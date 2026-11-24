//! The pipeline a draw body selected on an earlier frame (#594). Selection runs a dozen device
//! admission checks and a locked map lookup for every draw; a recorded body replays with the
//! same inputs on every frame, so it keeps its last answer with the inputs it was given.
use super::pipeline::DrawPipeline;
use crate::PrimitiveDraw;
use std::sync::{Arc, Mutex};

/// Everything `RasterPipelines::select` reads besides the body itself.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct SelectionInputs {
    /// The serial of the pipeline set that selected.
    pipelines: usize,
    factorized: bool,
    window_depth: Option<[u32; 10]>,
    dynamic_lighting: bool,
    depth_pass: crate::depth_prepass::RenderDepthPass,
    depth_clamp: bool,
    face_culling: crate::RenderFaceCulling,
    /// The lighting body's identity, the eye's w and whether every light is directional.
    lighting: Option<(u64, u32, bool)>,
}

impl SelectionInputs {
    /// The inputs of one selection; `SelectionInputs::of(&pipelines, draw, ...)`.
    pub(super) fn of(
        pipelines: usize,
        draw: &PrimitiveDraw,
        (factorized, window_depth, dynamic_lighting): (bool, Option<[u32; 10]>, bool),
    ) -> Self {
        Self {
            pipelines,
            factorized,
            window_depth,
            dynamic_lighting,
            depth_pass: draw.depth_pass,
            depth_clamp: draw.depth_clamp,
            face_culling: draw.face_culling,
            lighting: draw.lighting.as_ref().map(|lighting| {
                (
                    lighting.body.identity.0,
                    lighting.eye[3].to_bits(),
                    super::directional_phong::directional_lights(&lighting.lights),
                )
            }),
        }
    }
}

/// A body's last selection. A copied body starts empty, and two memos always compare equal,
/// so the memo never changes what a draw is.
#[derive(Default)]
pub(crate) struct PipelineMemo(Mutex<Option<(SelectionInputs, Arc<DrawPipeline>)>>);

impl PipelineMemo {
    /// The pipeline selected before with `inputs`; `memo.get(&inputs)`.
    pub(super) fn get(&self, inputs: &SelectionInputs) -> Option<Arc<DrawPipeline>> {
        let slot = self.0.lock().ok()?;
        slot.as_ref()
            .filter(|(kept, _)| kept == inputs)
            .map(|(_, pipeline)| pipeline.clone())
    }
    /// Keep the pipeline selected with `inputs`; `memo.keep(inputs, pipeline.clone())`.
    pub(super) fn keep(&self, inputs: SelectionInputs, pipeline: Arc<DrawPipeline>) {
        if let Ok(mut slot) = self.0.lock() {
            *slot = Some((inputs, pipeline));
        }
    }
}

impl PipelineMemo {
    /// Forget the selection: the body is about to change in place.
    pub(crate) fn clear(&mut self) {
        if let Ok(slot) = self.0.get_mut() {
            *slot = None;
        }
    }
}

impl Clone for PipelineMemo {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl PartialEq for PipelineMemo {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl std::fmt::Debug for PipelineMemo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("PipelineMemo")
    }
}

#[cfg(test)]
mod tests {
    use super::SelectionInputs;
    use crate::PrimitiveDraw;

    fn lit_triangle() -> PrimitiveDraw {
        let vertex = |x: f32| crate::RenderVertex::new([x, 0., 0.], [1.; 4]);
        let vertices = vec![vertex(0.), vertex(1.), vertex(0.5)];
        let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
        draw.lighting = Some(
            crate::RenderLighting::new(
                vec![[0., 0., 1.]; 3],
                Default::default(),
                crate::identity_clip(),
                crate::identity_clip(),
                false,
                crate::RenderShading::Phong,
            )
            .unwrap(),
        );
        draw
    }

    fn inputs(draw: &PrimitiveDraw) -> SelectionInputs {
        SelectionInputs::of(1, draw, (false, None, true))
    }

    #[test]
    fn a_replayed_copy_keeps_the_inputs_and_a_lighting_write_changes_them() {
        let draw = lit_triangle();
        let replayed = draw.clone();
        assert!(inputs(&draw) == inputs(&replayed));
        let mut edited = draw.clone();
        edited.lighting.as_mut().unwrap().pbr = true;
        assert!(inputs(&draw) != inputs(&edited));
        assert_eq!(
            draw.lighting, replayed.lighting,
            "identity is not part of equality"
        );
    }

    #[test]
    fn per_frame_state_and_frame_inputs_change_the_inputs() {
        let draw = lit_triangle();
        let mut clamped = draw.clone();
        clamped.depth_clamp = true;
        assert!(inputs(&draw) != inputs(&clamped));
        assert!(inputs(&draw) != SelectionInputs::of(2, &draw, (false, None, true)));
        assert!(inputs(&draw) != SelectionInputs::of(1, &draw, (true, None, true)));
    }
}
