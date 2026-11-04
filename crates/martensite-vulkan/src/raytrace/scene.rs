//! GlView::updateRaytraceGeometry, addRaytraceStructure and addRaytraceGroups for a fresh build: the triangle sets and materials of every visible ray-traced
//! structure in the ray-traced, non-immediate layers.
use super::material::RaytraceMaterial;
use super::primitive_triangles::{is_raytraced, triangle_set};
use super::textures::{RaytraceTextures, apply_material_textures};
use super::triangle_set::RaytraceTriangleSet;
use crate::graphic_group::RaytraceGroupCommand;
use crate::{Graphic3dDrawGroup, Graphic3dRenderStructure, RenderError, RenderMaterialColorSpace};
use martensite_tkernel::ncollection::Mat4;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

/// The ray-traced geometry of one view.
#[derive(Clone, Debug, Default)]
pub(crate) struct RaytraceScene {
    pub sets: Vec<RaytraceTriangleSet>,
    pub materials: Vec<RaytraceMaterial>,
    pub textures: RaytraceTextures,
    /// Whether materials take their textures (the device's HasRayTracingTextures).
    pub textured: bool,
}

/// Collect the scene of `view` (identified by `view_id` for view affinity); `textured` materials
/// take the textures of their aspects. Example:
/// `collect_scene(&*view, view.identification(), RenderMaterialColorSpace::Linear, true)?`.
pub(crate) fn collect_scene(
    view: &dyn Graphic3dCView,
    view_id: i32,
    space: RenderMaterialColorSpace,
    textured: bool,
) -> Result<RaytraceScene, RenderError> {
    let mut scene = RaytraceScene {
        textured,
        ..Default::default()
    };
    for layer in view.layers() {
        let settings = layer.layer_settings();
        if !settings.is_raytracable() || layer.is_immediate() {
            continue;
        }
        for priority in layer.array_of_structures() {
            for weak in priority.iter() {
                scene.add_structure(&weak.upgrade(), view_id, space)?;
            }
        }
    }
    Ok(scene)
}

/// Whether any group of the batches holds a ray-traced array (GlStructure::IsRaytracable).
fn is_raytracable(batches: &[Vec<Handle<dyn Graphic3dGroup>>]) -> bool {
    batches.iter().flatten().any(|group| {
        let draw = group.cast::<Graphic3dDrawGroup>();
        draw.get().is_some_and(|draw| {
            draw.raytrace_commands().1.iter().any(|command| {
                matches!(command, RaytraceGroupCommand::Primitive(p) if is_raytraced(p.kind))
            })
        })
    })
}

impl RaytraceScene {
    fn add_structure(
        &mut self,
        structure: &Handle<dyn Graphic3dCStructure>,
        view_id: i32,
        space: RenderMaterialColorSpace,
    ) -> Result<(), RenderError> {
        let retained = structure.cast::<Graphic3dRenderStructure>();
        let Some(retained) = retained.get() else {
            return Ok(());
        };
        if !retained.is_visible_in_view(view_id) {
            return Ok(());
        }
        let batches = retained.instanced_group_batches();
        if !is_raytracable(&batches) {
            return Ok(());
        }
        let transform = structure_transform(&retained.cstructure_base().fields.read());
        // The instanced structure's groups take this structure's transformation, as natively.
        for group in batches.iter().flatten() {
            if let Some(draw) = group.cast::<Graphic3dDrawGroup>().get() {
                self.add_group(draw, &transform, space)?;
            }
        }
        Ok(())
    }
    /// One group: its material (the group aspects, else the default structure material), a
    /// new material per aspects node, and the triangle set of each ray-traced array.
    fn add_group(
        &mut self,
        group: &Graphic3dDrawGroup,
        transform: &Mat4<f32>,
        space: RenderMaterialColorSpace,
    ) -> Result<(), RenderError> {
        let (aspects, commands) = group.raytrace_commands();
        let mut material_id = self.push_aspects_material(group, &aspects, space)?;
        for command in commands {
            match command {
                RaytraceGroupCommand::Aspects(aspects) => {
                    if !aspects.is_null() {
                        material_id = self.push_aspects_material(group, &aspects, space)?;
                    }
                }
                RaytraceGroupCommand::Primitive(primitive) => {
                    if let Some(mut set) = triangle_set(&primitive, material_id)? {
                        set.transform = *transform;
                        self.sets.push(set);
                    }
                }
            }
        }
        Ok(())
    }
    /// convertMaterial of `aspects` (the default material without aspects), with its textures
    /// when the device textures ray-traced materials.
    fn push_aspects_material(
        &mut self,
        group: &Graphic3dDrawGroup,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        space: RenderMaterialColorSpace,
    ) -> Result<i32, RenderError> {
        let Some(source) = aspects.get() else {
            return Ok(self.push_material(RaytraceMaterial::default()));
        };
        let mut material = RaytraceMaterial::of_aspects(source, space);
        if self.textured {
            let images = &group.texture_images;
            apply_material_textures(&mut material, source, images, &mut self.textures)?;
        }
        Ok(self.push_material(material))
    }
    fn push_material(&mut self, material: RaytraceMaterial) -> i32 {
        self.materials.push(material);
        self.materials.len() as i32 - 1
    }
}

/// `theTrsf->Trsf().GetMat4(aMat4)` in float, identity without a transformation.
fn structure_transform(fields: &Graphic3dCStructureFields) -> Mat4<f32> {
    let mut model = Mat4::<f64>::identity();
    if let Some(datum) = fields.trsf.get() {
        datum.transformation().get_mat4(&mut model);
    }
    Mat4::<f32>::from_other(&model)
}
