//! Snapshot only coordinate inputs; binding-time Eye planes follow the final draw order.
use crate::{RenderError, RenderVertex, error::rejected, surface_texture_uv::SurfaceUvTransform};
use martensite_tkernel::ncollection::{Mat4, Vec4};
use martensite_tkservice::graphic3d::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceTexgen {
    pub mode: Graphic3dTypeOfTextureMode,
    pub planes: [[f32; 4]; 2],
    pub model_view: Mat4<f32>,
    pub input: Vec<[f32; 4]>,
    pub transform: SurfaceUvTransform,
    /// The eye-space normal inputs of `GL_SPHERE_MAP`: a draw of any mode keeps them, since a
    /// sphere-map generation left enabled by an earlier texture applies to it
    /// (`crate::texture_coordinate_frame`); absent when the model-view is singular.
    pub sphere: Option<crate::surface_sphere_texgen::SphereTexgenInput>,
}

impl SurfaceTexgen {
    /// Freeze live parameters before frame submission; `SurfaceTexgen::snapshot(uv, params, top_down, world, view)`.
    pub fn snapshot(
        input: Vec<[f32; 4]>,
        parameters: &Graphic3dTextureParams,
        top_down: bool,
        world: [f32; 16],
        view: [f32; 16],
    ) -> Box<Self> {
        Box::new(Self {
            mode: parameters.gen_mode(),
            planes: [
                parameters.gen_plane_s().data(),
                parameters.gen_plane_t().data(),
            ],
            model_view: Mat4::from_data(view) * Mat4::from_data(world),
            input,
            transform: SurfaceUvTransform::read(parameters, top_down),
            sphere: None,
        })
    }
}
impl SurfaceTexgen {
    /// Attach the `GL_SPHERE_MAP` inputs; a Sphere-mode snapshot rejects a singular model-view,
    /// other modes then keep none. Example: `snapshot.attach_sphere(normals, normalize)?`.
    pub fn attach_sphere(
        &mut self,
        normals: Vec<[f32; 3]>,
        normalize: bool,
    ) -> Result<(), RenderError> {
        let input = crate::surface_sphere_texgen::SphereTexgenInput::new(
            normals,
            &self.model_view,
            normalize,
        );
        self.sphere = match self.mode == Graphic3dTypeOfTextureMode::Sphere {
            true => Some(input?),
            false => input.ok(),
        };
        Ok(())
    }
}
/// The texture coordinate of a Sprite-mode texture on faces under the fixed-function pipeline
/// (the recorded sphere_texgen capture: every face pixel shows the texel at the centre).
const SPRITE_FACE_COORDINATES: [f32; 4] = [0.5, 0.5, 0., 1.];
#[derive(Clone)]
pub(crate) struct TextureGenerationBinding {
    mode: Graphic3dTypeOfTextureMode,
    planes: [[f32; 4]; 2],
    inverse_bound: Option<Mat4<f32>>,
}
impl TextureGenerationBinding {
    /// Coordinates without generation planes: the supplied UVs (`Manual`) or the point-sprite
    /// centre (`Sprite`). Example: `TextureGenerationBinding::fixed(Manual)`.
    pub fn fixed(mode: Graphic3dTypeOfTextureMode) -> Self {
        Self {
            mode,
            planes: [[0.; 4]; 2],
            inverse_bound: None,
        }
    }
    /// GL's initial generation state: `GL_EYE_LINEAR` with the eye planes (1, 0, 0, 0) and
    /// (0, 1, 0, 0) given under an identity model-view. Example: `TextureGenerationBinding::initial()`.
    pub fn initial() -> Self {
        Self {
            mode: Graphic3dTypeOfTextureMode::Eye,
            planes: [[1., 0., 0., 0.], [0., 1., 0., 0.]],
            inverse_bound: Some(Mat4::identity()),
        }
    }
    /// GL_EYE_PLANE is transformed at binding under the loaded matrix; `Self::new(snapshot)?`.
    pub fn new(snapshot: &SurfaceTexgen) -> Result<Self, RenderError> {
        let inverse_bound = if snapshot.mode == Graphic3dTypeOfTextureMode::Eye {
            Some(snapshot.model_view.inverted().map_err(|_| {
                rejected(
                    snapshot.model_view.data(),
                    "invertible binding-time Eye model-view",
                )
            })?)
        } else {
            None
        };
        Ok(Self {
            mode: snapshot.mode,
            planes: snapshot.planes,
            inverse_bound,
        })
    }
    pub fn coordinates(
        &self,
        snapshot: &SurfaceTexgen,
        vertices: &[RenderVertex],
    ) -> Result<Vec<[f32; 4]>, RenderError> {
        if snapshot.input.len() != vertices.len() {
            return Err(rejected(
                (snapshot.input.len(), vertices.len()),
                "one raw homogeneous UV per texgen vertex",
            ));
        }
        if self.mode == Graphic3dTypeOfTextureMode::Sphere && snapshot.sphere.is_none() {
            return Err(rejected(
                snapshot.model_view.data(),
                "an invertible sphere-map model-view",
            ));
        }
        Ok(vertices
            .iter()
            .zip(&snapshot.input)
            .enumerate()
            .map(|(index, (vertex, input))| {
                self.vertex_coordinates(snapshot, vertex, (index, input))
            })
            .collect())
    }
    fn vertex_coordinates(
        &self,
        snapshot: &SurfaceTexgen,
        vertex: &RenderVertex,
        (index, input): (usize, &[f32; 4]),
    ) -> [f32; 4] {
        let position = Vec4::new(
            vertex.position[0],
            vertex.position[1],
            vertex.position[2],
            vertex.weight,
        );
        let position = self.inverse_bound.map_or(position, |inverse| {
            inverse * (snapshot.model_view * position)
        });
        if self.mode == Graphic3dTypeOfTextureMode::Sprite {
            // GL_COORD_REPLACE substitutes the point coordinate after the texture matrix; Mesa
            // (llvmpipe and radeonsi) supplies the centre (0.5, 0.5) on faces.
            return SPRITE_FACE_COORDINATES;
        }
        if self.mode == Graphic3dTypeOfTextureMode::Sphere
            && let Some(sphere) = snapshot.sphere.as_ref()
        {
            let [s, t] = sphere.coordinates(index, snapshot.model_view * position);
            return snapshot.transform.generated([s, t, input[2], input[3]]);
        }
        snapshot
            .transform
            .generated(self.generated_input(input, position))
    }
    fn generated_input(&self, input: &[f32; 4], position: Vec4<f32>) -> [f32; 4] {
        if self.mode == Graphic3dTypeOfTextureMode::Manual {
            return *input;
        }
        let s = Vec4::from_array(self.planes[0]).dot(&position);
        let t = Vec4::from_array(self.planes[1]).dot(&position);
        [s, t, input[2], input[3]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn texgen_fixture(mode: Graphic3dTypeOfTextureMode, translation: f32) -> Box<SurfaceTexgen> {
        let parameters = Graphic3dTextureParams::default();
        parameters.set_gen_mode(
            mode,
            Vec4::new(0.0625, 0., 0., 0.5),
            Vec4::new(0., 0.0625, 0., 0.5),
        );
        let mut matrix = Mat4::<f32>::identity();
        matrix.set_value(0, 3, translation);
        SurfaceTexgen::snapshot(
            vec![[0., 0., 7., 2.]],
            &parameters,
            false,
            matrix.data(),
            Mat4::<f32>::identity().data(),
        )
    }
    #[test]
    fn object_generation_preserves_supplied_r_q_and_ignores_model_translation() {
        let snapshot = texgen_fixture(Graphic3dTypeOfTextureMode::Object, 4.);
        let binding = TextureGenerationBinding::new(&snapshot).unwrap();
        let vertices = [RenderVertex::new([-8., -8., 0.], [1.; 4])];
        assert_eq!(
            binding.coordinates(&snapshot, &vertices).unwrap(),
            vec![[0., 0., 7., 2.]]
        );
    }
    #[test]
    fn eye_generation_retains_the_binding_matrix_until_the_image_is_rebound() {
        let first = texgen_fixture(Graphic3dTypeOfTextureMode::Eye, -4.);
        let second = texgen_fixture(Graphic3dTypeOfTextureMode::Eye, 4.);
        let vertices = [RenderVertex::new([-8., -8., 0.], [1.; 4])];
        let binding = TextureGenerationBinding::new(&first).unwrap();
        assert_eq!(
            binding.coordinates(&second, &vertices).unwrap(),
            vec![[0.5, 0., 7., 2.]]
        );
        let rebound = TextureGenerationBinding::new(&second).unwrap();
        assert_eq!(
            rebound.coordinates(&second, &vertices).unwrap(),
            vec![[0., 0., 7., 2.]]
        );
    }
    #[test]
    fn sprite_generation_on_faces_samples_the_centre_without_the_texture_matrix() {
        let snapshot = texgen_fixture(Graphic3dTypeOfTextureMode::Sprite, 4.);
        let binding = TextureGenerationBinding::new(&snapshot).unwrap();
        let vertices = [RenderVertex::new([-8., 3., 1.], [1.; 4])];
        let coordinates = binding.coordinates(&snapshot, &vertices).unwrap();
        assert_eq!(coordinates, vec![SPRITE_FACE_COORDINATES]);
    }
    #[test]
    fn manual_compatibility_keeps_r_q_and_rejects_wrong_shape_or_singular_eye_binding() {
        let mut snapshot = texgen_fixture(Graphic3dTypeOfTextureMode::Manual, 0.);
        let binding = TextureGenerationBinding::new(&snapshot).unwrap();
        let vertices = [RenderVertex::new([9.; 3], [1.; 4])];
        assert_eq!(
            binding.coordinates(&snapshot, &vertices).unwrap(),
            snapshot.input
        );
        assert!(binding.coordinates(&snapshot, &[]).is_err());
        snapshot.mode = Graphic3dTypeOfTextureMode::Eye;
        snapshot.model_view = Mat4::from_data([0.; 16]);
        assert!(TextureGenerationBinding::new(&snapshot).is_err());
    }
}
