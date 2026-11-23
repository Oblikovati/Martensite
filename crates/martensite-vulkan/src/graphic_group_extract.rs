//! One spatial snapshot feeds the ordered native element program.
use super::elements::ElementVisit;
use super::{Graphic3dDrawGroup, GroupCommand, GroupGeometry, apply_command_clipping};
use crate::render_element_pass::{RenderElementPass, check_is_transparent};
use crate::{PrimitiveDraw, RenderClipping, RenderError, stencil_program::StencilProgram};
use martensite_tkernel::{ncollection::Mat4, standard::Handle};
use martensite_tkservice::graphic3d::Graphic3dClipPlaneSequenceSource;
use martensite_tkservice::graphic3d::*;
struct GroupSpatialSnapshot {
    base_model: Mat4<f64>,
    matrix: [f32; 16],
    depth_clamp: bool,
    clipping: Option<RenderClipping>,
}
struct GroupDrawPolicy {
    highlighted: bool,
    allow_culling: bool,
    /// The workspace highlight style: the structure style while highlighted, else null.
    highlight: Handle<dyn Graphic3dPresentationAttributesSource>,
}
impl GroupDrawPolicy {
    fn new(structure: &Graphic3dCStructureFields, allow_culling: bool) -> Self {
        // GlStructure::Render sets the workspace highlight style unless it is a box.
        let highlighted =
            structure.highlight && !crate::bound_box_highlight::is_bound_box_highlight(structure);
        Self {
            highlighted,
            allow_culling,
            highlight: match highlighted {
                true => structure.highlight_style.clone(),
                false => Handle::default(),
            },
        }
    }
}
impl Graphic3dDrawGroup {
    /// Lower the group's commands in `structure` with the current camera; a group draw
    /// record calls this when it cannot replay (graphic_group_record.rs).
    pub(super) fn lower_in_structure(
        &self,
        camera: &Graphic3dCamera,
        extent: Option<[u32; 2]>,
        structure: &Graphic3dCStructureFields,
        rendering: &Graphic3dRenderingParams,
        global_clipping: &Handle<dyn Graphic3dClipPlaneSequenceSource>,
        (pass, carry): (&RenderElementPass, &mut crate::group_flipping::FlipCarry),
    ) -> Result<StencilProgram, RenderError> {
        let persistence =
            extent.map(|raster| crate::render_scale::persistence_extent(raster, rendering));
        let origin = pass.layer_origin().map(|origin| origin.coords());
        let spatial =
            self.spatial_snapshot(camera, persistence, structure, (global_clipping, origin))?;
        let mut context =
            self.stencil_text_context((camera, extent), structure, rendering, &spatial);
        context.environment = pass.environment();
        context.fixed_function_lighting = pass.context_state().fixed_function_lighting;
        let policy = GroupDrawPolicy::new(structure, spatial.clipping.is_none());
        let mut program =
            self.extract_commands(spatial.matrix, policy, &mut context, (pass, carry))?;
        finish_group_spatial_state(&mut program.draws, spatial.depth_clamp, structure)?;
        Ok(program)
    }
    fn spatial_snapshot(
        &self,
        camera: &Graphic3dCamera,
        extent: Option<[u32; 2]>,
        structure: &Graphic3dCStructureFields,
        (global_clipping, origin): (
            &Handle<dyn Graphic3dClipPlaneSequenceSource>,
            Option<[f64; 3]>,
        ),
    ) -> Result<GroupSpatialSnapshot, RenderError> {
        crate::highlight_style::validate_structure(structure)?;
        let base_model = crate::persistent_transform::model_transform(self, structure);
        let (matrix, depth_clamp) = crate::persistent_transform::group_clip_transform(
            self, camera, extent, structure, base_model,
        )?;
        let clipping = if global_clipping.is_null() && structure.clip_planes.is_null() {
            None
        } else {
            let model = crate::persistent_transform::clipping_model(self, base_model)?;
            crate::retained_clipping::structure_clipping(global_clipping, structure, model, origin)?
        };
        Ok(GroupSpatialSnapshot {
            base_model,
            matrix,
            depth_clamp,
            clipping,
        })
    }
    fn stencil_text_context<'a>(
        &'a self,
        projection: (&'a Graphic3dCamera, Option<[u32; 2]>),
        structure: &'a Graphic3dCStructureFields,
        rendering: &'a Graphic3dRenderingParams,
        spatial: &'a GroupSpatialSnapshot,
    ) -> crate::text_submission::TextSceneContext<'a> {
        crate::text_submission::TextSceneContext {
            group: self,
            camera: projection.0,
            extent: projection.1,
            structure,
            rendering,
            fonts: &self.text_source,
            base_model: Some(&spatial.base_model),
            model_override: None,
            view_override: None,
            clipping: spatial.clipping.as_ref(),
            environment: None,
            fixed_function_lighting: false,
        }
    }
    fn extract_commands(
        &self,
        matrix: [f32; 16],
        policy: GroupDrawPolicy,
        context: &mut crate::text_submission::TextSceneContext<'_>,
        (pass, carry): (&RenderElementPass, &mut crate::group_flipping::FlipCarry),
    ) -> Result<StencilProgram, RenderError> {
        let (mut aspects, commands, effects) = self.retained_commands();
        let mut program = StencilProgram::default();
        // The transparent element pass replays commands without re-emitting their stencil state.
        let mut replayed = StencilProgram::default();
        let mut flips = crate::group_flipping::GroupFlipStack::scene(
            context,
            matrix,
            carry,
            (effects.flips, effects.calls_user_code),
        )?;
        let mut visit = ElementVisit {
            highlight: &policy.highlight,
            allow_face_culling: self.is_closed() && policy.allow_culling,
            pass,
            context: element_context_state(self, &commands, context, pass)?,
        };
        if effects.calls_user_code {
            // An injected text provider or render element may edit the live group.
            context.base_model = None;
        }
        for command in commands.iter() {
            if let GroupCommand::Element(element) = command {
                update_element_matrices(&mut visit, &flips);
                for (style, geometry) in
                    self.call_element(element.as_ref(), &mut aspects, &visit)?
                {
                    self.append_geometry(
                        &mut program,
                        &geometry,
                        &style,
                        &flips,
                        &policy,
                        context,
                    )?;
                }
                continue;
            }
            let lowering = pass.lowers_group_geometry();
            let target = if lowering {
                &mut program
            } else {
                &mut replayed
            };
            let Some(geometry) = command.advance(&mut aspects, &mut flips, target) else {
                continue;
            };
            if pass.skips_raytraced() && geometry.is_raytraced() {
                continue;
            }
            if lowering {
                pass.note_builtin(geometry.is_fill(), || {
                    check_is_transparent(&aspects, &policy.highlight)
                });
                self.append_geometry(&mut program, geometry, &aspects, &flips, &policy, context)?;
            }
        }
        *carry = flips.carry();
        Ok(program)
    }
    fn append_geometry(
        &self,
        program: &mut StencilProgram,
        geometry: &GroupGeometry,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        flips: &crate::group_flipping::GroupFlipStack,
        policy: &GroupDrawPolicy,
        context: &crate::text_submission::TextSceneContext<'_>,
    ) -> Result<(), RenderError> {
        let text = matches!(geometry, GroupGeometry::Text(..));
        let draws = self.lower_ordered_geometry(geometry, aspects, flips, policy, context)?;
        program.ordered_geometry(draws, text);
        Ok(())
    }
    fn lower_ordered_geometry(
        &self,
        geometry: &GroupGeometry,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        flips: &crate::group_flipping::GroupFlipStack,
        policy: &GroupDrawPolicy,
        context: &crate::text_submission::TextSceneContext<'_>,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let mut active = *context;
        let source_primitives = matches!(geometry, GroupGeometry::Primitives(_));
        active.model_override = flips.override_model();
        active.view_override = Some(flips.world_view());
        let mut lowered = geometry.lower(
            aspects,
            self.is_closed() && policy.allow_culling,
            flips.clip(),
            policy.highlighted,
            &active,
        )?;
        apply_command_clipping(&mut lowered, flips, context.clipping)?;
        if source_primitives {
            retain_projection_factors(&mut lowered, flips, context);
        }
        Ok(lowered)
    }
    pub(super) fn retained_commands(
        &self,
    ) -> (
        Handle<dyn Graphic3dAspectsSource>,
        crate::draw::DrawStream<GroupCommand>,
        super::commands::CommandEffects,
    ) {
        let state = self.submissions.read();
        (
            state.aspects.clone(),
            state.commands.clone(),
            state.command_effects,
        )
    }
}
fn retain_projection_factors(
    draws: &mut [PrimitiveDraw],
    flips: &crate::group_flipping::GroupFlipStack,
    context: &crate::text_submission::TextSceneContext<'_>,
) {
    let factors = crate::RenderProjectionFactors::retained_camera(
        context.camera.projection_matrix_f().data(),
        flips.world_view().data(),
        flips.model_state().current().data(),
        context.camera,
    );
    for draw in draws {
        draw.projection_factors = factors;
        draw.fixed_function_factors = factors;
    }
}
/// The context state of the group's elements, when it has any.
fn element_context_state(
    group: &Graphic3dDrawGroup,
    commands: &[GroupCommand],
    context: &crate::text_submission::TextSceneContext<'_>,
    pass: &RenderElementPass,
) -> Result<Option<crate::RenderContextState>, RenderError> {
    if !commands
        .iter()
        .any(|command| matches!(command, GroupCommand::Element(_)))
    {
        return Ok(None);
    }
    let [model_world, world_view, projection] = crate::persistent_transform::element_matrix_states(
        group,
        context.camera,
        context.persistence_extent(),
        context.structure,
    )?;
    Ok(Some(crate::RenderContextState {
        model_world,
        world_view,
        projection,
        ..pass.context_state().clone()
    }))
}
fn finish_group_spatial_state(
    draws: &mut [PrimitiveDraw],
    clamp: bool,
    structure: &Graphic3dCStructureFields,
) -> Result<(), RenderError> {
    for draw in draws {
        draw.depth_clamp = clamp;
        let mirrored = structure
            .trsf
            .get()
            .is_some_and(|datum| datum.transformation().is_negative());
        // Writing only a changed winding keeps a cached template's body shared (#594).
        if draw.mirrored != mirrored {
            draw.mirrored = mirrored;
        }
        crate::highlight_style::apply_highlight(draw, structure)?;
    }
    Ok(())
}

fn update_element_matrices(
    visit: &mut ElementVisit<'_>,
    flips: &crate::group_flipping::GroupFlipStack,
) {
    let Some(context) = &mut visit.context else {
        return;
    };
    context.model_world = flips.model_state().clone();
    context.world_view.set_current(&flips.world_view());
}
