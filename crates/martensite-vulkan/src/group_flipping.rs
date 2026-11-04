//! GlFlipper saves ModelWorld before testing axis orientation.
use crate::RenderMatrixState;
use martensite_tkernel::ncollection::Mat4;
use martensite_tkmath::gp::Axis2;
use martensite_tkservice::graphic3d::{Graphic3dFlipper, Graphic3dGroupMembers};

#[derive(Clone, Copy)]
struct FlipModelSnapshot {
    model: Mat4<f32>,
    clip: [f32; 16],
    changed: bool,
}

/// The actual current model and saved matrices after a group returns, including residual
/// group-transform and persistence pushes; `FlipCarry::default()` starts a new structure.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct FlipCarry {
    state: Option<RenderMatrixState<f32>>,
}

pub(crate) struct GroupFlipStack {
    current: FlipModelSnapshot,
    baseline: FlipModelSnapshot,
    models: RenderMatrixState<f32>,
    local_pops: usize,
    projection: Mat4<f64>,
    view: Mat4<f32>,
    baseline_view: Mat4<f32>,
    precise_view: Option<Mat4<f64>>,
}
const _: fn(&FlipCarry) -> &(dyn Send + Sync) = |value| value;
const _: fn(&GroupFlipStack) -> &(dyn Send + Sync) = |value| value;

impl GroupFlipStack {
    /// The stack of a group in `context` that inherits the earlier groups' pushed flips;
    /// `GroupFlipStack::scene(context, clip, &carry)?`.
    pub fn scene(
        context: &crate::text_submission::TextSceneContext<'_>,
        clip: [f32; 16],
        carry: &FlipCarry,
    ) -> Result<Self, crate::RenderError> {
        let (model, view) = crate::persistent_transform::lighting_matrices(context)?;
        let ordinary = context.structure.trsf_pers.is_null()
            && context.group.transform_persistence().is_null();
        let mut stack = Self::new(
            Mat4::from_data(model),
            Mat4::from_data(view),
            context.camera.projection_matrix(),
            ordinary.then(|| context.camera.orientation_matrix()),
            clip,
        );
        let incoming =
            crate::persistent_transform::flipping_model_state(context, carry.state.clone())?;
        stack.models = incoming.model;
        stack.local_pops = incoming.local_pops;
        stack.view = incoming.view;
        stack.refresh_model();
        Ok(stack)
    }
    /// The pushed flips the next group of the structure inherits.
    pub fn carry(&self) -> FlipCarry {
        let mut state = self.models.clone();
        for _ in 0..self.local_pops {
            // Native Pop asserts and leaves the current model unchanged on an empty stack.
            let _ = state.pop();
        }
        FlipCarry { state: Some(state) }
    }
    pub fn new(
        model: Mat4<f32>,
        view: Mat4<f32>,
        projection: Mat4<f64>,
        precise_view: Option<Mat4<f64>>,
        clip: [f32; 16],
    ) -> Self {
        let current = FlipModelSnapshot {
            model,
            clip,
            changed: false,
        };
        let mut models = RenderMatrixState::default();
        models.set_current(&model);
        Self {
            current,
            baseline: current,
            models,
            local_pops: 0,
            projection,
            view,
            baseline_view: view,
            precise_view,
        }
    }
    pub fn apply(&mut self, plane: Option<Axis2>) {
        let Some(plane) = plane else {
            if self.models.pop().is_ok() {
                self.refresh_model();
            }
            return;
        };
        let rotation = Graphic3dFlipper::new(plane).compute(&(self.view * self.current.model));
        self.push((!rotation.is_identity()).then_some(rotation));
    }
    /// Save the current model, then apply `rotation` (none for an identity flip).
    fn push(&mut self, rotation: Option<Mat4<f32>>) {
        self.models.push();
        let Some(rotation) = rotation else {
            return;
        };
        *self.models.change_current() *= rotation;
        self.refresh_model();
    }
    fn refresh_model(&mut self) {
        let model = *self.models.current();
        let changed = model != self.baseline.model || self.view != self.baseline_view;
        if self.current.model == model && self.current.changed == changed {
            return;
        }
        self.current.model = model;
        self.current.changed = changed;
        self.current.clip = if self.current.changed {
            self.flipped_clip()
        } else {
            self.baseline.clip
        };
    }
    /// The current model stack, including command pushes, for custom element observations.
    pub fn model_state(&self) -> &RenderMatrixState<f32> {
        &self.models
    }
    /// The group's effective world-view after applying its actual persistence anchor.
    pub fn world_view(&self) -> Mat4<f32> {
        self.view
    }
    fn flipped_clip(&self) -> [f32; 16] {
        if let Some(view) = self.precise_view {
            return (self.projection * view * Mat4::<f64>::from_other(&self.current.model))
                .data()
                .map(|value| value as f32);
        }
        (Mat4::<f32>::from_other(&self.projection) * self.view * self.current.model).data()
    }
    pub fn clip(&self) -> [f32; 16] {
        self.current.clip
    }
    pub fn override_model(&self) -> Option<Mat4<f32>> {
        self.current.changed.then_some(self.current.model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkmath::gp::{Direction3, Point3};
    fn stack(signs: [f32; 3]) -> GroupFlipStack {
        let mut view = Mat4::<f32>::identity().data();
        for (index, sign) in [0, 5, 10].into_iter().zip(signs) {
            view[index] = sign;
        }
        GroupFlipStack::new(
            Mat4::identity(),
            Mat4::from_data(view),
            Mat4::identity(),
            None,
            Mat4::<f32>::identity().data(),
        )
    }
    #[test]
    fn eight_axis_signs_choose_native_half_turns() {
        for (signs, expected) in [
            ([1., 1., 1.], [1., 1., 1.]),
            ([-1., 1., 1.], [-1., -1., 1.]),
            ([1., -1., 1.], [-1., -1., 1.]),
            ([-1., -1., 1.], [-1., -1., 1.]),
            ([1., 1., -1.], [-1., 1., -1.]),
            ([-1., 1., -1.], [-1., 1., -1.]),
            ([1., -1., -1.], [1., -1., -1.]),
            ([-1., -1., -1.], [1., -1., -1.]),
        ] {
            let mut flips = stack(signs);
            flips.apply(Some(Axis2::default()));
            let matrix = flips.current.model.data();
            assert_eq!([matrix[0], matrix[5], matrix[10]], expected);
        }
    }
    #[test]
    fn nested_identity_push_restores_each_saved_model_and_exact_clip() {
        let mut flips = stack([-1., -1., 1.]);
        let initial = flips.clip();
        flips.apply(Some(Axis2::default()));
        let rotated = flips.current.model;
        flips.apply(Some(Axis2::default()));
        assert_eq!(flips.current.model, rotated);
        flips.apply(None);
        assert_eq!(flips.current.model, rotated);
        flips.apply(None);
        assert_eq!(flips.clip(), initial);
        assert!(flips.override_model().is_none());
        flips.apply(None);
        assert_eq!(flips.clip(), initial);
    }
    #[test]
    fn offset_reference_origin_is_fixed_by_the_rotation() {
        let mut flips = stack([-1., -1., 1.]);
        let plane = Axis2::with_x_direction(
            Point3::new(2., 3., 0.),
            Direction3::new(0., 0., 1.).unwrap(),
            Direction3::new(1., 0., 0.).unwrap(),
        )
        .unwrap();
        flips.apply(Some(plane));
        let matrix = flips.current.model.data();
        assert_eq!([matrix[12], matrix[13], matrix[14]], [4., 6., 0.]);
        assert_eq!(flips.override_model(), Some(flips.current.model));
    }
    #[test]
    fn the_next_group_inherits_a_pushed_flip_until_it_pops_it() {
        let mut label = stack([-1., -1., 1.]);
        let initial = label.clip();
        label.apply(Some(Axis2::default()));
        let flipped = (label.current.model, label.clip());
        let carry = label.carry();
        // The shaded text group: same frame, the carried push, then the disabling flipper.
        let mut text = stack([-1., -1., 1.]);
        text.models = carry.state.unwrap();
        text.refresh_model();
        assert_eq!((text.current.model, text.clip()), flipped);
        assert_eq!(text.override_model(), Some(flipped.0));
        text.apply(None);
        assert_eq!(text.clip(), initial);
        assert_eq!(text.models.depth(), 0);
        assert!(text.models.current().is_identity());
    }
    #[test]
    fn no_rotation_preserves_the_original_clip_precision() {
        let mut flips = stack([1.; 3]);
        flips.current.clip[0] = 0.12345679;
        flips.precise_view = Some(Mat4::identity());
        flips.apply(Some(Axis2::default()));
        assert_eq!(flips.clip()[0], 0.12345679);
        assert!(flips.override_model().is_none());
        flips.apply(None);
        assert_eq!(flips.clip()[0], 0.12345679);
    }
}
