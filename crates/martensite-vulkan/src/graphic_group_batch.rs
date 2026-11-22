use super::*;
use std::sync::Weak;

type Commands = crate::draw::DrawStream<GroupCommand>;

pub(crate) struct BatchGraphicInput {
    geometry: crate::primitive_cache::BatchGeometryStamp,
    material: BatchMaterial,
    commands: Weak<Vec<GroupCommand>>,
    closed: bool,
}

impl BatchGraphicInput {
    pub(crate) fn commands_expired(&self) -> bool {
        self.commands.strong_count() == 0
    }
}

impl Graphic3dDrawGroup {
    fn probe_sources(&self) -> (Commands, Handle<dyn Graphic3dAspectsSource>) {
        let submissions = self.submissions.read();
        (submissions.commands.clone(), submissions.aspects.clone())
    }

    pub(crate) fn probe_graphic_input(&self) -> Option<BatchGraphicInput> {
        let closed = self.probe_group_state()?;
        let (commands, aspects) = self.probe_sources();
        let (primitive, style) = probe_primitive(&commands, &aspects)?;
        Some(BatchGraphicInput {
            geometry: crate::primitive_cache::BatchGeometryStamp::capture(primitive)?,
            material: probe_material(style)?,
            commands: commands.witness()?,
            closed,
        })
    }

    pub(crate) fn matches_batch_seed(
        &self,
        saved: &BatchGraphicInput,
        draw: &PrimitiveDraw,
    ) -> bool {
        let (commands, aspects) = self.probe_sources();
        let Some((primitive, _)) = probe_primitive(&commands, &aspects) else {
            return false;
        };
        let Some(cache) = &primitive.retained else {
            return false;
        };
        crate::primitive_cache::GeometryStamp::capture(primitive)
            .is_some_and(|source| cache.matches_batch_streams(&source, draw))
            && saved.material.matches_draw(draw)
            && self.probe_unchanged(saved)
    }

    pub(crate) fn probe_unchanged(&self, saved: &BatchGraphicInput) -> bool {
        if self.probe_group_state() != Some(saved.closed) {
            return false;
        }
        let (commands, aspects) = self.probe_sources();
        if commands.identity().map(|id| id.get()) != Some(saved.commands.as_ptr().addr()) {
            return false;
        }
        let Some((primitive, style)) = probe_primitive(&commands, &aspects) else {
            return false;
        };
        probe_material(style).as_ref() == Some(&saved.material)
            && saved.geometry.probe_matches(primitive)
    }

    fn probe_group_state(&self) -> Option<bool> {
        let fields = self.base.fields.read();
        (fields.trsf.is_identity() && fields.trsf_pers.is_null() && fields.flipper.is_null())
            .then_some(fields.is_closed)
    }
}

fn probe_primitive<'a>(
    commands: &'a Commands,
    base_aspects: &'a Handle<dyn Graphic3dAspectsSource>,
) -> Option<(&'a PrimitiveSubmission, &'a dyn Graphic3dAspectsSource)> {
    let mut style = base_aspects;
    let mut source = None;
    for command in commands.iter() {
        match command {
            GroupCommand::Aspects(aspects) => style = aspects,
            GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) if source.is_none() => {
                source = Some((primitive, style.get()?));
            }
            _ => return None,
        }
    }
    source
}

#[derive(PartialEq)]
struct BatchMaterial {
    front_coefficients: [[u32; 4]; 4],
    back_coefficients: Option<[[u32; 4]; 4]>,
    front_color: ([u64; 3], u32),
    back_color: ([u64; 3], u32),
    polygon_offset: (i32, u32, u32),
    shading: Graphic3dTypeOfShadingModel,
    culling: Graphic3dTypeOfBackfacingModel,
    alpha_mode: Graphic3dAlphaMode,
    alpha_cutoff: u32,
    blends: bool,
    distinguish_materials: bool,
    back_vertex_color: bool,
}

impl BatchMaterial {
    fn matches_draw(&self, draw: &PrimitiveDraw) -> bool {
        let Some(lighting) = &draw.lighting else {
            return false;
        };
        let mut front = self.front_coefficients;
        let mut back = self.back_coefficients.unwrap_or(front);
        let offset = draw.polygon_offset.map(|offset| {
            let (mode, factor, units) = offset.parameters();
            (mode, factor.to_bits(), units.to_bits())
        });
        if self.blends
            || offset != Some(self.polygon_offset)
            || draw.surface_blend
            || draw.surface_forced_blend
            || draw.surface_cutoff.is_some()
        {
            return false;
        }
        // Opaque surface lowering forces both diffuse alpha coefficients to one.
        front[0][3] = 1f32.to_bits();
        back[0][3] = 1f32.to_bits();
        lighting
            .material
            .common
            .map(|face| face.packed().map(|part| part.map(f32::to_bits)))
            == [front, back]
    }
}

fn probe_material(aspect: &dyn Graphic3dAspectsSource) -> Option<BatchMaterial> {
    let fields = aspect.aspects_base().fields.read();
    if !ordinary_surface(&fields) {
        return None;
    }
    let offset = fields.polygon_offset;
    let front_coefficients = common_bits(&fields.front_material, *fields.interior_color.rgb());
    let back_coefficients = fields
        .to_distinguish_materials
        .then(|| common_bits(&fields.back_material, *fields.back_interior_color.rgb()));
    Some(BatchMaterial {
        front_coefficients,
        back_coefficients,
        front_color: color_bits(&fields.interior_color),
        back_color: color_bits(&fields.back_interior_color),
        polygon_offset: (
            offset.mode.0,
            offset.factor.to_bits(),
            offset.units.to_bits(),
        ),
        shading: fields.shading_model,
        culling: fields.face_culling,
        alpha_mode: fields.alpha_mode,
        alpha_cutoff: fields.alpha_cutoff.to_bits(),
        blends: crate::style::surface_fields_use_blending(&fields),
        distinguish_materials: fields.to_distinguish_materials,
        back_vertex_color: fields.to_use_vertex_color_for_back_faces,
    })
}

fn common_bits(
    material: &Graphic3dMaterialAspect,
    color: martensite_tkernel::quantity::Color,
) -> [[u32; 4]; 4] {
    crate::material::common_coefficients(crate::RenderMaterialColorSpace::Linear, material, color)
        .packed()
        .map(|coefficient| coefficient.map(f32::to_bits))
}

fn ordinary_surface(fields: &Graphic3dAspectsFields) -> bool {
    use martensite_tkservice::aspect::AspectInteriorStyle;
    fields.interior_style == AspectInteriorStyle::Solid
        && fields.program.is_null()
        && fields.texture_set.is_null()
        && fields.hatch_style.is_null()
        && !fields.to_map_texture
        && !fields.to_draw_edges
        && !fields.to_draw_silhouette
}

fn color_bits(color: &martensite_tkernel::quantity::ColorRgba) -> ([u64; 3], u32) {
    let rgb = color.rgb();
    (
        [rgb.red(), rgb.green(), rgb.blue()].map(f64::to_bits),
        color.alpha().to_bits(),
    )
}
