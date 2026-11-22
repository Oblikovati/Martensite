//! Native Phong-family shading state is immutable and independent of Vulkan handles.
use crate::{
    PrimitiveDraw, RenderError, RenderLightSet, RenderMaterial, draw::DrawStream, error::rejected,
};
use martensite_tkernel::ncollection::{Mat4, Vec4};
/// Legacy surface algorithms; `RenderShading::Phong` interpolates normals per fragment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RenderShading {
    /// Derive a face normal from fragment position derivatives.
    PhongFacet,
    /// Illuminate vertices and interpolate front/back colors.
    Gouraud,
    /// Interpolate nodal normals and illuminate fragments.
    Phong,
}
/// One snapshot of geometry normals, transforms, materials and lights; `draw.with_lighting(state)?`.
///
/// The view matrix, eye, normalization flag and lights change with the camera and the layer, so
/// they are inline; the rest is a shared [`LightingBody`] that reads reach through `Deref`. A
/// replayed draw record sets these four without copying the 400-byte body (#594).
#[derive(Clone, Debug, PartialEq)]
pub struct RenderLighting {
    pub(crate) view: [f32; 16],
    pub(crate) normalize_fixed: bool,
    pub(crate) eye: [f32; 4],
    pub(crate) lights: RenderLightSet,
    pub(crate) body: std::sync::Arc<LightingBody>,
}
/// The part of a lighting snapshot that its lowering fixes; `lighting.material` reads it.
#[derive(Clone, Debug, PartialEq)]
pub struct LightingBody {
    /// Renewed on every mutable access, so a cache keyed on it sees each change (#594).
    pub(crate) identity: LightingIdentity,
    pub(crate) normals: DrawStream<[f32; 3]>,
    pub(crate) material: RenderMaterial,
    pub(crate) world: [f32; 16],
    pub(crate) fixed: Option<crate::fixed_lighting::FixedLightingInterpolation>,
    pub(crate) fixed_source_opacity: Option<[f32; 2]>,
    pub(crate) normal: [f32; 16],
    pub(crate) shading: RenderShading,
    pub(crate) vertex_colors: bool,
    pub(crate) front_vertex_colors_only: bool,
    pub(crate) pbr: bool,
}
impl std::ops::Deref for RenderLighting {
    type Target = LightingBody;
    fn deref(&self) -> &LightingBody {
        &self.body
    }
}
impl std::ops::DerefMut for RenderLighting {
    /// Copies the body first when a record or another frame still shares it.
    fn deref_mut(&mut self) -> &mut LightingBody {
        let body = std::sync::Arc::make_mut(&mut self.body);
        body.identity = LightingIdentity::next();
        body
    }
}
/// A process-unique number for one state of a lighting body. It is not part of equality:
/// two bodies with the same values are equal whatever their identities.
#[derive(Clone, Copy, Debug)]
pub struct LightingIdentity(pub(crate) u64);
impl LightingIdentity {
    fn next() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self(NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}
impl PartialEq for LightingIdentity {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl RenderLighting {
    /// Snapshot world-space shading; `RenderLighting::new(normals, material, model, view, false, RenderShading::Phong)?`.
    pub fn new(
        normals: Vec<[f32; 3]>,
        material: RenderMaterial,
        world: [f32; 16],
        view: [f32; 16],
        perspective: bool,
        shading: RenderShading,
    ) -> Result<Self, RenderError> {
        validate_lighting_components(&normals, &material, &world, &view)?;
        let normal = inverse_normal_transform(world)?;
        let eye = lighting_eye(view, perspective)?;
        Ok(Self {
            view,
            normalize_fixed: false,
            eye,
            lights: RenderLightSet::default(),
            body: std::sync::Arc::new(LightingBody {
                identity: LightingIdentity::next(),
                normals: normals.into(),
                material,
                world,
                fixed: None,
                fixed_source_opacity: None,
                normal,
                shading,
                vertex_colors: false,
                front_vertex_colors_only: false,
                pbr: false,
            }),
        })
    }
    /// Attach enabled lights without changing the geometry snapshot; `state.with_lights(lights)`.
    pub fn with_lights(mut self, lights: RenderLightSet) -> Self {
        self.lights = lights;
        self
    }
    /// Multiply lit coefficients by vertex colors, optionally on front faces only; `state.with_vertex_colors(true, false)`.
    pub fn with_vertex_colors(mut self, enabled: bool, front_only: bool) -> Self {
        self.vertex_colors = enabled;
        self.front_vertex_colors_only = front_only;
        self
    }
    /// Select metallic-roughness shading (`Graphic3dTypeOfShadingModel_Pbr`/`PbrFacet`);
    /// the surface algorithm stays `Phong` or `PhongFacet`. Example: `state.with_pbr(true)`.
    pub fn with_pbr(mut self, enabled: bool) -> Self {
        self.pbr = enabled;
        self
    }
    /// Whether the PBR material model shades this snapshot; `state.is_pbr()`.
    pub fn is_pbr(&self) -> bool {
        self.pbr
    }
    /// Inspect nodal normals; `state.normals()`.
    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }
    /// Inspect the resolved surface algorithm; `state.shading()`.
    pub fn shading(&self) -> RenderShading {
        self.shading
    }
    #[cfg(test)]
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }
    pub(crate) fn encode_into(&self, bytes: &mut Vec<u8>) {
        bytes.clear();
        bytes.reserve(304 + 80 * self.lights.source_count());
        self.encode_coefficients(bytes);
        let settings = [
            self.fixed.map_or(self.shading as u32, |mode| mode as u32),
            self.lights.source_count() as u32,
            u32::from(self.vertex_colors),
            u32::from(self.front_vertex_colors_only)
                + 2 * u32::from(self.is_fixed_lighting() && self.normalize_fixed),
        ];
        bytes.extend_from_slice(settings.map(u32::to_ne_bytes).as_flattened());
        for light in self.lights.sources.iter() {
            encode_light_source(bytes, light);
        }
    }
    fn encode_coefficients(&self, bytes: &mut Vec<u8>) {
        append_coefficients(bytes, self.world);
        append_coefficients(bytes, self.normal);
        append_coefficients(bytes, self.eye);
        append_coefficients(bytes, self.lights.ambient);
        for record in self.packed_material_records() {
            append_coefficients(bytes, record);
        }
    }
}
impl RenderLighting {
    /// The eight material vec4 slots: Phong records, or the front PBR record in slots 0-2 and
    /// the back one in slots 4-6 (GlMaterialPBR order) for metallic-roughness shading.
    fn packed_material_records(&self) -> [[f32; 4]; 8] {
        if !self.pbr {
            return self.material.packed_common();
        }
        let [f0, f1, f2, b0, b1, b2] = self.material.packed_pbr();
        [f0, f1, f2, [0.; 4], b0, b1, b2, [0.; 4]]
    }
}
fn inverse_normal_transform(world: [f32; 16]) -> Result<[f32; 16], RenderError> {
    let identity = crate::identity_clip();
    // A bitwise match preserves the cofactor path for signed zeros in authored matrices.
    if world.map(f32::to_bits) == identity.map(f32::to_bits) {
        let mut normal = identity;
        // The general inverse's first off-diagonal cofactors produce negative zero.
        normal[1] = -0.;
        normal[4] = -0.;
        return Ok(normal);
    }
    Mat4::from_data(world)
        .inverted()
        .map(|inverse| inverse.transposed().data())
        .map_err(|error| rejected((world, error), "an invertible model matrix"))
}
#[cfg(test)]
#[test]
fn identity_normals_are_bit_identical_to_the_general_inverse() {
    let identity = Mat4::<f32>::identity();
    let inverse = identity.inverted().unwrap().transposed().data();
    assert_eq!(
        inverse_normal_transform(identity.data())
            .unwrap()
            .map(f32::to_bits),
        inverse.map(f32::to_bits),
    );
}

pub(crate) fn lighting_eye(view: [f32; 16], perspective: bool) -> Result<[f32; 4], RenderError> {
    let inverse = Mat4::from_data(view)
        .inverted()
        .map_err(|error| rejected((view, error), "an invertible world-view matrix"))?;
    let vector = if perspective {
        Vec4::new(0., 0., 0., 1.)
    } else {
        Vec4::new(0., 0., 1., 0.)
    };
    let mut eye = (inverse * vector).data();
    eye[3] = f32::from(perspective);
    Ok(eye)
}
fn encode_light_source(bytes: &mut Vec<u8>, light: &crate::light_snapshot::PackedRenderLight) {
    let fields = [light.kind, u32::from(light.cast_shadows), 0, 0];
    bytes.extend_from_slice(fields.map(u32::to_ne_bytes).as_flattened());
    append_coefficients(bytes, light.color);
    append_coefficients(bytes, light.position);
    append_coefficients(bytes, light.direction);
    append_coefficients(bytes, light.params);
}
fn append_coefficients<const N: usize>(bytes: &mut Vec<u8>, coefficients: [f32; N]) {
    bytes.extend_from_slice(coefficients.map(f32::to_ne_bytes).as_flattened());
}
fn validate_lighting_components(
    normals: &[[f32; 3]],
    material: &RenderMaterial,
    world: &[f32; 16],
    view: &[f32; 16],
) -> Result<(), RenderError> {
    validate_material_opacity(material)?;
    if normals
        .iter()
        .flatten()
        .chain(world)
        .chain(view)
        .chain(material.packed_common().iter().flatten())
        .any(|v| !v.is_finite())
    {
        return Err(rejected(
            (normals, material, world, view),
            "finite lighting normals, common coefficients and matrices",
        ));
    }
    Ok(())
}
fn validate_material_opacity(material: &RenderMaterial) -> Result<(), RenderError> {
    let opacity = material.common.map(|face| face.diffuse[3]);
    if opacity
        .iter()
        .any(|value| !value.is_finite() || !(0. ..=1.).contains(value))
    {
        return Err(rejected(
            opacity,
            "finite diffuse opacity in [0,1] for both material faces",
        ));
    }
    Ok(())
}
impl RenderLighting {
    pub(crate) fn uses_translucent_material(&self) -> bool {
        self.material.common.iter().any(|face| face.diffuse[3] < 1.)
    }
    pub(crate) fn validate_opaque_marker_lighting(&self) -> Result<(), RenderError> {
        // OpenGL 4.6 §15.2 makes actual Points front-facing; unused back opacity cannot reject them.
        if self.material.common[0].diffuse[3] < 1. {
            return Err(rejected(
                self.material.common.map(|face| face.diffuse[3]),
                "opaque marker diffuse opacity equal to 1",
            ));
        }
        Ok(())
    }
}
impl PrimitiveDraw {
    /// Attach surface or native ordered nodal line/opaque marker lighting with one normal per vertex; `draw.with_lighting(state)?`.
    pub fn with_lighting(mut self, lighting: RenderLighting) -> Result<Self, RenderError> {
        self.validate_lighting_topology(&lighting)?;
        if self.topology() == crate::PrimitiveTopology::Points {
            lighting.validate_opaque_marker_lighting()?;
        } else if self.topology() == crate::PrimitiveTopology::Triangles {
            self.surface_blend |= lighting.uses_translucent_material();
        }
        self.lighting = Some(lighting);
        self.unlit_back_color = None;
        Ok(self)
    }
    fn validate_lighting_topology(&self, lighting: &RenderLighting) -> Result<(), RenderError> {
        if self.rejects_lighting_topology(lighting) {
            return Err(rejected(
                (
                    lighting.normals.len(),
                    self.vertices.len(),
                    self.topology(),
                    self.textured_style.is_some(),
                    self.capping_pass,
                ),
                "one normal per ordinary triangle or Gouraud/Phong line/marker vertex",
            ));
        }
        Ok(())
    }
    fn rejects_lighting_topology(&self, lighting: &RenderLighting) -> bool {
        lighting.normals.len() != self.vertices.len()
            || (self.topology() != crate::PrimitiveTopology::Triangles
                && self.point_style.is_none()
                && self.line_style.is_none())
            || ((self.point_style.is_some() || self.line_style.is_some())
                && lighting.shading == RenderShading::PhongFacet)
            || self.textured_style.is_some()
            || self.capping_pass.is_mask()
    }
    /// Inspect a retained lighting snapshot; `draw.lighting()`.
    pub fn lighting(&self) -> Option<&RenderLighting> {
        self.lighting.as_ref()
    }
}

#[cfg(test)]
#[path = "lighting_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "lighting_alpha_tests.rs"]
mod alpha_tests;

#[cfg(test)]
#[path = "lighting_marker_back_tests.rs"]
mod marker_back_tests;
