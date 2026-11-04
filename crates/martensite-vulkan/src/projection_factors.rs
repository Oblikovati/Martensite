//! Explicit triangle projection retains the rounding boundary before camera projection.
use crate::{RenderError, error::rejected};
use martensite_tkernel::ncollection::Mat4;

/// Select a projection operation; `RenderProjectionBehavior::FactorizedTrianglesV2`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderProjectionBehavior {
    /// Replay the previously composed clip matrix.
    #[default]
    PrecomposedV1,
    /// Project ordinary triangles as `(projection * view) * (model * vertex)`.
    FactorizedTrianglesV2,
    /// As version two, and plain lines too: native draws them through the same expression, so
    /// a moved grid's far lines otherwise round to another slope (v3d/bugs/bug25672).
    FactorizedLinesV3,
    /// As version three, and a fixed-function frame multiplies by `projection * (view * model)`:
    /// The float view-model product is loaded with `glLoadMatrixf` and Mesa composes it with
    /// the projection, so a moved group rounds to other samples (demo/samples/dimensions).
    FixedFunctionModelViewV4,
}
impl RenderProjectionBehavior {
    /// Whether ordinary triangles use the factorized route; `behavior.factorizes_triangles()`.
    pub(crate) fn factorizes_triangles(self) -> bool {
        self != Self::PrecomposedV1
    }
    /// Whether plain line draws are factorized as well; `behavior.factorizes_lines()`.
    pub(crate) fn factorizes_lines(self) -> bool {
        matches!(
            self,
            Self::FactorizedLinesV3 | Self::FixedFunctionModelViewV4
        )
    }
    /// Whether fixed-function frames keep their own products; `behavior.composes_model_view()`.
    pub(crate) fn composes_model_view(self) -> bool {
        self == Self::FixedFunctionModelViewV4
    }
}

/// Immutable source matrices; `RenderProjectionFactors::new(projection, view, model)?`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderProjectionFactors {
    projection: [f32; 16],
    world_view: [f32; 16],
    model_world: [f32; 16],
    camera_isometry: bool,
}
impl RenderProjectionFactors {
    pub(crate) fn retained(p: [f32; 16], v: [f32; 16], m: [f32; 16]) -> Option<Self> {
        Self::new(p, v, m).ok()
    }
    pub(crate) fn retained_camera(
        p: [f32; 16],
        v: [f32; 16],
        m: [f32; 16],
        camera: &martensite_tkservice::graphic3d::Graphic3dCamera,
    ) -> Option<Self> {
        // One locked source snapshot prevents a scale/view ABA from inventing rigid provenance.
        Self::retained_camera_snapshot(p, v, m, camera.copy_members())
    }
    fn retained_camera_snapshot(
        p: [f32; 16],
        v: [f32; 16],
        m: [f32; 16],
        snapshot: martensite_tkservice::graphic3d::Graphic3dCamera,
    ) -> Option<Self> {
        let mut factors = Self::retained(p, v, m)?;
        let scale = snapshot.axial_scale();
        factors.camera_isometry = [scale.x(), scale.y(), scale.z()] == [1.; 3]
            && v.map(f32::to_bits) == snapshot.orientation_matrix_f().data().map(f32::to_bits);
        Some(factors)
    }
    pub(crate) fn has_camera_isometry(&self) -> bool {
        self.camera_isometry
    }
    /// Retain the actual factors, never recover them from an MVP; `Self::new(p, v, m)?`.
    pub fn new(p: [f32; 16], v: [f32; 16], m: [f32; 16]) -> Result<Self, RenderError> {
        if !p.iter().chain(&v).chain(&m).all(|value| value.is_finite()) {
            return Err(rejected(
                (p, v, m),
                "three finite column-major projection factors",
            ));
        }
        Ok(Self {
            projection: p,
            world_view: v,
            model_world: m,
            camera_isometry: false,
        })
    }
    /// Read projection/view/model in source order; `factors.matrices()[2]`.
    pub fn matrices(&self) -> [[f32; 16]; 3] {
        [self.projection, self.world_view, self.model_world]
    }
    /// Compose as a fixed-function frame does, in binary32; `factors.model_view_product()`.
    pub(crate) fn model_view_product(&self) -> [f32; 16] {
        let model_view = Mat4::from_data(self.world_view) * Mat4::from_data(self.model_world);
        (Mat4::from_data(self.projection) * model_view).data()
    }
    /// Compose only the camera factors in binary32; `factors.camera_product()`.
    pub fn camera_product(&self) -> [f32; 16] {
        (Mat4::from_data(self.projection) * Mat4::from_data(self.world_view)).data()
    }
    pub(crate) fn model(&self) -> [f32; 16] {
        self.model_world
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn identity() -> [f32; 16] {
        Mat4::<f32>::identity().data()
    }
    #[test]
    fn source_factors_are_retained_without_recovering_a_product() {
        let mut model = identity();
        model[12] = 16_777_216.;
        let mut view = identity();
        view[12] = -16_777_216.;
        let factors = RenderProjectionFactors::new(identity(), view, model).unwrap();
        assert_eq!(factors.matrices(), [identity(), view, model]);
        assert_eq!(factors.model(), model);
        assert_eq!(factors.camera_product(), view);
        // Independent IEEE32 example: rounding the world coordinate loses the unit.
        assert_eq!((1_f32 + model[12]) + view[12], 0.);
        assert_eq!(1_f32 + (model[12] + view[12]), 1.);
    }
    #[test]
    fn nonfinite_factors_are_refused_and_default_replays_the_prior_operation() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut projection = identity();
            projection[7] = value;
            assert!(RenderProjectionFactors::new(projection, identity(), identity()).is_err());
        }
        assert_eq!(
            RenderProjectionBehavior::default(),
            RenderProjectionBehavior::PrecomposedV1
        );
        fn send_sync<T: Send + Sync>() {}
        send_sync::<RenderProjectionFactors>();
        send_sync::<RenderProjectionBehavior>();
    }
    #[test]
    fn each_version_states_which_primitives_it_factorizes() {
        use RenderProjectionBehavior as B;
        assert!(!B::PrecomposedV1.factorizes_triangles() && !B::PrecomposedV1.factorizes_lines());
        assert!(B::FactorizedTrianglesV2.factorizes_triangles());
        assert!(!B::FactorizedTrianglesV2.factorizes_lines());
        assert!(
            B::FactorizedLinesV3.factorizes_triangles() && B::FactorizedLinesV3.factorizes_lines()
        );
        assert!(!B::FactorizedLinesV3.composes_model_view());
        let fixed = B::FixedFunctionModelViewV4;
        assert!(fixed.factorizes_triangles() && fixed.factorizes_lines());
        assert!(fixed.composes_model_view());
    }
    #[test]
    fn fixed_function_product_groups_view_with_model_not_with_projection() {
        // Independent IEEE32 example: the unit survives only when view and model cancel first.
        let (mut projection, mut view, mut model) = (identity(), identity(), identity());
        projection[12] = 1.;
        view[12] = -33_554_432.;
        model[12] = 33_554_432.;
        let factors = RenderProjectionFactors::new(projection, view, model).unwrap();
        let camera = Mat4::from_data(factors.camera_product());
        let precomposed = (camera * Mat4::from_data(model)).data();
        assert_eq!(factors.model_view_product()[12], 1.);
        assert_eq!(precomposed[12], 0.);
    }
    #[test]
    fn selecting_projection_preserves_every_independent_behavior() {
        let old = crate::RenderBehaviorConfiguration::default();
        let selected = old.with_projection(RenderProjectionBehavior::FactorizedTrianglesV2);
        assert_eq!(old.choices(), selected.choices());
        assert_eq!(old.projection(), RenderProjectionBehavior::PrecomposedV1);
        assert_eq!(
            selected.projection(),
            RenderProjectionBehavior::FactorizedTrianglesV2
        );
    }
    #[test]
    fn unavailable_float_factors_do_not_add_a_legacy_retention_failure() {
        let mut projection = identity();
        projection[0] = f32::INFINITY;
        assert!(RenderProjectionFactors::retained(projection, identity(), identity()).is_none());
        assert!(RenderProjectionFactors::retained(identity(), identity(), identity()).is_some());
    }
    #[test]
    fn camera_provenance_is_exact_and_does_not_infer_rigidity_from_nearby_values() {
        use martensite_tkservice::graphic3d::Graphic3dCamera;
        let camera = Graphic3dCamera::default();
        let view = camera.orientation_matrix_f().data();
        let retained =
            RenderProjectionFactors::retained_camera(identity(), view, identity(), &camera)
                .unwrap();
        assert!(retained.has_camera_isometry());
        assert!(
            !RenderProjectionFactors::new(identity(), view, identity())
                .unwrap()
                .has_camera_isometry()
        );
    }
    #[test]
    fn one_changed_camera_word_does_not_receive_an_isometry_marker() {
        use martensite_tkservice::graphic3d::Graphic3dCamera;
        let camera = Graphic3dCamera::default();
        let view = camera.orientation_matrix_f().data();
        let mut changed = view;
        changed[0] = f32::from_bits(changed[0].to_bits() + 1);
        assert!(
            !RenderProjectionFactors::retained_camera(identity(), changed, identity(), &camera)
                .unwrap()
                .has_camera_isometry()
        );
    }
    #[test]
    fn axial_scale_never_receives_the_rounded_camera_isometry_marker() {
        use martensite_tkservice::graphic3d::Graphic3dCamera;
        let camera = Graphic3dCamera::default();
        camera
            .set_axial_scale(martensite_tkmath::gp::Xyz::new(2., 1., 1.))
            .unwrap();
        let factors = RenderProjectionFactors::retained_camera(
            identity(),
            camera.orientation_matrix_f().data(),
            identity(),
            &camera,
        )
        .unwrap();
        assert!(!factors.has_camera_isometry());
    }
    fn mutated_camera_and_snapshot() -> (
        martensite_tkservice::graphic3d::Graphic3dCamera,
        martensite_tkservice::graphic3d::Graphic3dCamera,
        [f32; 16],
    ) {
        let camera = martensite_tkservice::graphic3d::Graphic3dCamera::default();
        let snapshot = camera.copy_members();
        let view = snapshot.orientation_matrix_f().data();
        camera
            .set_axial_scale(martensite_tkmath::gp::Xyz::new(2., 1., 1.))
            .unwrap();
        (camera, snapshot, view)
    }
    #[test]
    fn owned_provenance_snapshot_is_isolated_from_later_source_scale_changes() {
        let (camera, snapshot, view) = mutated_camera_and_snapshot();
        assert_ne!(camera.orientation_matrix_f().data(), view);
        assert_eq!(snapshot.orientation_matrix_f().data(), view);
        let factors = RenderProjectionFactors::retained_camera_snapshot(
            identity(),
            view,
            identity(),
            snapshot,
        )
        .unwrap();
        assert!(factors.has_camera_isometry());
        let live = RenderProjectionFactors::retained_camera(identity(), view, identity(), &camera)
            .unwrap();
        assert!(!live.has_camera_isometry());
    }
}
