//! A caught nested failure retains the live conversion depth until reconstruction.
use martensite_tkbrep::topods::{Builder, Shape, Wire};
use martensite_tkernel::{
    impl_transient,
    message::Msg,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tkshhealing::{
    shape_build::ShapeBuildReShape,
    shape_upgrade::{
        ShapeDivider, ShapeUpgradeFaceDivide, ShapeUpgradeShapeConvertToBezier,
        ShapeUpgradeShapeDivide, ShapeUpgradeTool, ShapeUpgradeWireDivide, WireDivideData,
        WireDivider,
    },
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};

#[derive(Default)]
struct InjectedWireFailure {
    parent: ShapeUpgradeWireDivide,
}
impl_transient!(InjectedWireFailure,"DepthTest_InjectedWireFailure",ShapeUpgradeWireDivide,parent,interfaces:[dyn WireDivider]);
impl WireDivider for InjectedWireFailure {
    fn wire_divide_data(&self) -> &WireDivideData {
        self.parent.wire_divide_data()
    }
    fn wire_divide_base(&self) -> &ShapeUpgradeTool {
        self.parent.wire_divide_base()
    }
    fn perform(&self) -> Result<(), Failure> {
        Err(Failure::with_message(
            "Standard_ProgramError",
            "injected nested wire failure",
        ))
    }
}
#[derive(Debug, PartialEq, Eq)]
struct NestedFailureState {
    exception_type: &'static str,
    result_null: bool,
    source_wire: bool,
    modes: (bool, bool),
    same_context: bool,
}
#[derive(Default)]
struct DepthTrace {
    injected: bool,
    nested: bool,
    failure: Option<NestedFailureState>,
    factories: Vec<(bool, bool)>,
}
struct ReentrantDepthPolicy {
    converter: ShapeUpgradeShapeConvertToBezier,
    trace: SharedState<DepthTrace>,
    failure_input: Shape,
}
impl ShapeDivider for ReentrantDepthPolicy {
    fn shape_divide_base(&self) -> &ShapeUpgradeShapeDivide {
        self.converter.shape_divide_base()
    }
    fn perform(&self, new_context: bool) -> Result<bool, Failure> {
        self.converter.perform_with_policy(self, new_context)
    }
    fn split_face_tool(&self) -> Handle<ShapeUpgradeFaceDivide> {
        let modes = self.current_modes();
        self.trace.write().factories.push(modes);
        let face = ShapeDivider::split_face_tool(&self.converter);
        if self.trace.read().nested {
            face.set_wire_divide_tool(&Handle::new(InjectedWireFailure::default()).cast());
        }
        face
    }
    fn face_msg(&self) -> Msg {
        let inject = {
            let mut trace = self.trace.write();
            let inject = !trace.injected;
            trace.injected = true;
            inject
        };
        if inject {
            self.catch_nested_failure();
        }
        ShapeDivider::face_msg(&self.converter)
    }
}
impl ReentrantDepthPolicy {
    fn new(inject: bool) -> Self {
        let converter = ShapeUpgradeShapeConvertToBezier::new(plane());
        converter.set_surface_conversion(true);
        converter.set_3d_conversion(true);
        Self {
            converter,
            failure_input: free_wire(),
            trace: SharedState::new(DepthTrace {
                injected: !inject,
                ..Default::default()
            }),
        }
    }
    fn current_modes(&self) -> (bool, bool) {
        (
            self.converter.get_surface_conversion(),
            self.converter.get_3d_conversion(),
        )
    }
    fn catch_nested_failure(&self) {
        let saved = self.converter.with(|fields| fields.shape.clone());
        let context = self.converter.context();
        self.converter.init(&self.failure_input);
        self.trace.write().nested = true;
        let failure = self.perform(false).unwrap_err();
        self.record_failure(&failure, &context);
        self.trace.write().nested = false;
        self.converter.init(&saved);
    }
    fn record_failure(&self, failure: &Failure, context: &Handle<ShapeBuildReShape>) {
        let state = NestedFailureState {
            exception_type: failure.exception_type(),
            result_null: self.converter.result().is_null(),
            source_wire: self
                .converter
                .with(|fields| fields.shape.is_same(&self.failure_input)),
            modes: self.current_modes(),
            same_context: self.converter.context() == *context,
        };
        assert_eq!(failure.message(), "injected nested wire failure");
        self.trace.write().failure = Some(state);
    }
}
fn plane() -> Shape {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 3.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}
fn free_wire() -> Shape {
    let edge = BRepBuilderAPIMakeEdge::from_points(Point3::default(), Point3::new(1., 0., 0.))
        .unwrap()
        .edge()
        .unwrap();
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    Builder
        .add(&mut wire.clone().into_shape(), &edge.into_shape())
        .unwrap();
    wire.into_shape()
}
fn expected_nested_failure() -> NestedFailureState {
    NestedFailureState {
        exception_type: "Standard_ProgramError",
        result_null: true,
        source_wire: true,
        modes: (true, false),
        same_context: true,
    }
}
#[test]
fn caught_nested_failure_retains_live_outer_postprocess_boundary() {
    let policy = ReentrantDepthPolicy::new(true);
    assert!(policy.perform(true).unwrap());
    assert_eq!(policy.trace.read().failure, Some(expected_nested_failure()));
    assert_eq!(
        policy.trace.read().factories,
        [(true, false), (true, false), (false, true)]
    );
    assert_eq!(policy.current_modes(), (true, true));
    assert!(!policy.converter.result().checked().unwrap());
}
#[test]
fn init_does_not_reset_depth_retained_after_a_caught_nested_failure() {
    let policy = ReentrantDepthPolicy::new(true);
    assert!(policy.perform(true).unwrap());
    policy.converter.init(&plane());
    policy.trace.write().factories.clear();
    assert!(policy.perform(true).unwrap());
    assert_eq!(policy.trace.read().factories, [(true, true)]);
    assert_eq!(policy.current_modes(), (true, true));
    assert!(!policy.converter.result().checked().unwrap());
}
#[test]
fn a_new_operation_recovers_first_level_phases_after_nested_failure() {
    let failed = ReentrantDepthPolicy::new(true);
    assert!(failed.perform(true).unwrap());
    let fresh = ReentrantDepthPolicy::new(false);
    assert!(fresh.perform(true).unwrap());
    assert_eq!(fresh.trace.read().factories, [(true, false), (false, true)]);
    assert!(fresh.converter.result().checked().unwrap());
}
