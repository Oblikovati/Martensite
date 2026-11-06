//! `GL_SPHERE_MAP` texture-coordinate generation of the fixed-function pipeline.
//!
//! Global texture parameters select `GL_SPHERE_MAP` for S and T
//! when a texture's `GenMode` is `Graphic3dTOTM_SPHERE` (surface textures with that mode and
//! the view's environment texture, `Graphic3dTextureEnv`). OpenGL 2.1 section 2.11.4: with `u`
//! the unit eye-space vector to the vertex and `n'` the eye-space normal (normalized only under
//! `GL_NORMALIZE`), `r = u - 2 n' (n'. u)`, `m = 2 sqrt(rx^2 + ry^2 + (rz + 1)^2)`,
//! `s = rx / m + 1/2`, `t = ry / m + 1/2`. The arithmetic follows Mesa's fixed-function vertex
//! program in single precision and in its operation order: `normalize(v) = v * rsq(v. v)`,
//! `r = u - n' * (2 (n'. u))`, `1/m = rsq((r + (0, 0, 1)). (r + (0, 0, 1))) * 0.5`.
use crate::{RenderError, error::rejected};
use martensite_tkernel::ncollection::{Mat4, Vec4};

/// The eye-space inputs of sphere-map generation for one draw: the object-space normal per
/// vertex, the inverse transpose of the loaded model-view and the `GL_NORMALIZE` state.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SphereTexgenInput {
    pub normals: Vec<[f32; 3]>,
    pub normal_matrix: Mat4<f32>,
    pub normalize: bool,
}

impl SphereTexgenInput {
    /// Freeze the normals and the normal matrix of `model_view`;
    /// `SphereTexgenInput::new(normals, &model_view, false)?`.
    pub fn new(
        normals: Vec<[f32; 3]>,
        model_view: &Mat4<f32>,
        normalize: bool,
    ) -> Result<Self, RenderError> {
        let inverse = model_view
            .inverted()
            .map_err(|_| rejected(model_view.data(), "an invertible sphere-map model-view"))?;
        Ok(Self {
            normals,
            normal_matrix: inverse.transposed(),
            normalize,
        })
    }

    /// The generated S and T of vertex `index` at eye-space `eye`;
    /// `input.coordinates(0, eye)`. A vertex without a normal uses GL's current normal (0,0,1).
    pub fn coordinates(&self, index: usize, eye: Vec4<f32>) -> [f32; 2] {
        let object = self.normals.get(index).copied().unwrap_or([0., 0., 1.]);
        let normal = self.normal_matrix * Vec4::new(object[0], object[1], object[2], 0.);
        let mut normal = [normal.x(), normal.y(), normal.z()];
        if self.normalize {
            normal = normalized(normal);
        }
        sphere_map(normalized([eye.x(), eye.y(), eye.z()]), normal)
    }
}

/// `v * rsq(v . v)`, with `rsq` rounded as `1 / sqrt`.
fn normalized(v: [f32; 3]) -> [f32; 3] {
    let scale = 1. / dot(v, v).sqrt();
    v.map(|value| value * scale)
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// S and T from the unit eye vector `u` and the eye normal `n`.
fn sphere_map(u: [f32; 3], n: [f32; 3]) -> [f32; 2] {
    let twice = {
        let projection = dot(n, u);
        projection + projection
    };
    let r = [0, 1, 2].map(|axis| u[axis] - n[axis] * twice);
    let lifted = [r[0], r[1], r[2] + 1.];
    let inverse_m = 1. / dot(lifted, lifted).sqrt() * 0.5;
    [r[0] * inverse_m + 0.5, r[1] * inverse_m + 0.5]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_normal_facing_the_viewer_maps_the_view_axis_to_the_centre() {
        let input = SphereTexgenInput::new(vec![[0., 0., 1.]], &Mat4::identity(), false).unwrap();
        // u = (0,0,-1), n = (0,0,1): r = (0,0,1), m = 4, so (s,t) = (0.5,0.5).
        assert_eq!(input.coordinates(0, Vec4::new(0., 0., -5., 1.)), [0.5, 0.5]);
    }
    #[test]
    fn an_oblique_normal_follows_the_specification_formula() {
        let n = normalized([0.6, 0., 0.8]);
        let input = SphereTexgenInput::new(vec![n], &Mat4::identity(), false).unwrap();
        let [s, t] = input.coordinates(0, Vec4::new(0., 0., -2., 1.));
        // u = (0,0,-1), n.u = -0.8, r = u + 1.6 n = (0.96, 0, 0.28), m = 2 sqrt(0.9216 + 1.6384) = 3.2.
        assert!((s - (0.96 / 3.2 + 0.5)).abs() < 1e-6, "s={s}");
        assert_eq!(t, 0.5);
    }
    #[test]
    fn normals_follow_the_inverse_transpose_and_normalize_only_on_request() {
        let mut scale = Mat4::<f32>::identity();
        scale.set_value(0, 0, 2.);
        let raw = SphereTexgenInput::new(vec![[1., 0., 1.]], &scale, false).unwrap();
        let unit = SphereTexgenInput::new(vec![[1., 0., 1.]], &scale, true).unwrap();
        let eye = Vec4::new(0.4, 0.3, -1., 1.);
        assert_ne!(raw.coordinates(0, eye), unit.coordinates(0, eye));
        assert!(SphereTexgenInput::new(vec![], &Mat4::from_data([0.; 16]), false).is_err());
    }
    #[test]
    fn missing_normals_use_the_current_normal() {
        let input = SphereTexgenInput::new(vec![], &Mat4::identity(), false).unwrap();
        assert_eq!(input.coordinates(3, Vec4::new(0., 0., -1., 1.)), [0.5, 0.5]);
    }
}
