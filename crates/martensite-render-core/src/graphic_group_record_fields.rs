//! The fields of a recorded group draw that a replay computes again (#594): those derived from
//! the camera and those derived from the model matrix. A record is keyed on the class of its
//! model (mirrored or not, implicit stack or not) instead of its bits, so one record replays
//! under every placement of a connected presentation (phase 5, instancing.md step 1).
use crate::group_flipping::FlipCarry;
use crate::group_record_frame::RecordCameraParts;
use crate::{PrimitiveDraw, RenderProjectionFactors};
use martensite_tkernel::ncollection::Mat4;
use martensite_tkmath::gp::Transform3;
use martensite_tkservice::graphic3d::Graphic3dCStructureFields;

/// The camera- and placement-derived fields of an ordinary surface draw.
#[derive(Clone, Copy)]
pub(crate) struct ReplayedFields {
    clip: [f32; 16],
    depth_clamp: bool,
    view: [f32; 16],
    eye: [f32; 4],
    factors: Option<RenderProjectionFactors>,
    normalize: bool,
    world: [f32; 16],
    normal: [f32; 16],
}

/// The replayed fields of a group, computed only when a replay or a kept record needs them.
pub(super) struct LazyReplayedFields<'a> {
    parts: &'a RecordCameraParts,
    structure: &'a Graphic3dCStructureFields,
    model: Mat4<f64>,
    local: Transform3,
    fields: std::cell::OnceCell<Option<ReplayedFields>>,
}

impl<'a> LazyReplayedFields<'a> {
    /// The fields of a group with transformation `local` and model `model` in `structure`;
    /// `LazyReplayedFields::new(&parts, &fields, model, local)`.
    pub(super) fn new(
        parts: &'a RecordCameraParts,
        structure: &'a Graphic3dCStructureFields,
        model: Mat4<f64>,
        local: Transform3,
    ) -> Self {
        Self {
            parts,
            structure,
            model,
            local,
            fields: std::cell::OnceCell::new(),
        }
    }
    /// The fields, or `None` for a model that a lowering rejects; `lazy.get()`.
    pub(super) fn get(&self) -> Option<&ReplayedFields> {
        self.fields
            .get_or_init(|| ReplayedFields::current(self.parts, self.structure, self.model))
            .as_ref()
    }
    /// The model stack the group leaves for the structure's next group; `lazy.carry()`.
    pub(super) fn carry(&self) -> FlipCarry {
        FlipCarry::ordinary(self.structure, &self.local)
    }
}

impl ReplayedFields {
    /// The fields of a group without persistence, as `persistent_transform::ordinary_clip`,
    /// `retained_camera`, `lighting_matrices` and `native_normalize_state` compute them, or
    /// `None` for a model that is not finite or not invertible, which lowering rejects;
    /// `ReplayedFields::current(&parts, &fields, model)`.
    fn current(
        parts: &RecordCameraParts,
        structure: &Graphic3dCStructureFields,
        model: Mat4<f64>,
    ) -> Option<Self> {
        let world = Mat4::<f32>::from_other(&model).data();
        if !world.iter().all(|value| value.is_finite()) {
            return None;
        }
        let normal = crate::lighting::inverse_normal_transform(world).ok()?;
        let normalize = !parts.unit_axial_scale
            || crate::fixed_normalization::structure_scale_normalizes(structure);
        Some(Self::placed(parts, (world, normal), normalize))
    }

    /// The fields of a placement with world matrix `world` and its normal matrix `normal`
    /// under the camera `parts`, which a retained placement frame also patches its templates
    /// with (draw_instances.rs); `ReplayedFields::placed(&parts, (world, normal), false)`.
    pub(crate) fn placed(
        parts: &RecordCameraParts,
        (world, normal): ([f32; 16], [f32; 16]),
        normalize: bool,
    ) -> Self {
        Self {
            clip: (parts.projection_view * Mat4::from_data(world)).data(),
            depth_clamp: false,
            view: parts.view.data(),
            eye: parts.eye,
            factors: parts
                .factors
                .and_then(|factors| factors.with_model_world(world)),
            normalize,
            world,
            normal,
        }
    }

    /// Whether `draw` is an ordinary surface whose replayed fields are these.
    pub(super) fn certifies(&self, draw: &PrimitiveDraw) -> bool {
        ordinary_surface(draw)
            && bits(&draw.clip_transform) == bits(&self.clip)
            && draw.depth_clamp == self.depth_clamp
            && draw
                .projection_factors
                .is_none_or(|f| Some(f) == self.factors)
            && draw
                .fixed_function_factors
                .is_none_or(|f| Some(f) == self.factors)
            && draw.lighting.as_ref().is_none_or(|lighting| {
                bits(&lighting.view) == bits(&self.view)
                    && lighting.eye.map(f32::to_bits) == self.eye.map(f32::to_bits)
                    && lighting.normalize_fixed == self.normalize
                    && bits(&lighting.world) == bits(&self.world)
                    && bits(&lighting.normal) == bits(&self.normal)
            })
    }

    /// Set the replayed fields of a recorded draw.
    pub(crate) fn patch(&self, draw: &mut PrimitiveDraw) {
        draw.clip_transform = self.clip;
        draw.depth_clamp = self.depth_clamp;
        if draw.projection_factors.is_some() {
            draw.projection_factors = self.factors;
        }
        if draw.fixed_function_factors.is_some() {
            draw.fixed_function_factors = self.factors;
        }
        if let Some(lighting) = &mut draw.lighting {
            lighting.view = self.view;
            lighting.eye = self.eye;
            lighting.normalize_fixed = self.normalize;
            lighting.world = self.world;
            lighting.normal = self.normal;
        }
    }
}

fn bits(matrix: &[f32; 16]) -> [u32; 16] {
    matrix.map(f32::to_bits)
}

/// A triangle draw with no state that lowering derives from the camera or the model besides
/// the fields `ReplayedFields` recomputes: no lines, points, text, textures, edges or user
/// program.
fn ordinary_surface(draw: &PrimitiveDraw) -> bool {
    draw.topology() == crate::PrimitiveTopology::Triangles
        && draw.line_style.is_none()
        && draw.point_style.is_none()
        && draw.projected_dash.is_none()
        && draw.guard_band_source.is_none()
        && draw.quad_corners.is_empty()
        && draw.mesh_edges.is_none()
        && draw.polygon_edge_style.is_none()
        && draw.textured_style.is_none()
        && draw.surface_texture.is_none()
        && draw.user_program.is_none()
        && !draw.annotation
        && !draw.stencil_rect
}
