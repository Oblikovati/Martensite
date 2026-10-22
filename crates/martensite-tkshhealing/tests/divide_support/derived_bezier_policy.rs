//! Further-derived policies retain their virtual hooks through inherited phases.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Builder, Compound, Shape},
};
use martensite_tkernel::{
    message::Msg,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkg3d::{
    geom::{GeomBezierSurface, GeomPlane},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::Plane3;
use martensite_tkshhealing::shape_upgrade::{
    ShapeDivider, ShapeUpgradeFaceDivide, ShapeUpgradeShapeConvertToBezier, ShapeUpgradeShapeDivide,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

struct DerivedBezierPolicy {
    converter: ShapeUpgradeShapeConvertToBezier,
    calls: SharedState<Vec<&'static str>>,
    suppress_surface: bool,
}
impl ShapeDivider for DerivedBezierPolicy {
    fn shape_divide_base(&self) -> &ShapeUpgradeShapeDivide {
        self.converter.shape_divide_base()
    }
    fn perform(&self, new_context: bool) -> Result<bool, Failure> {
        self.converter.perform_with_policy(self, new_context)
    }
    fn split_face_tool(&self) -> Handle<ShapeUpgradeFaceDivide> {
        self.calls.write().push("face-tool");
        let face = ShapeDivider::split_face_tool(&self.converter);
        if self.suppress_surface {
            face.set_split_surface_tool(&Handle::default());
        }
        face
    }
    fn face_msg(&self) -> Msg {
        self.record_message("face-message")
    }
    fn wire_msg(&self) -> Msg {
        self.record_message("wire-message")
    }
    fn edge_msg(&self) -> Msg {
        self.record_message("edge-message")
    }
}
impl DerivedBezierPolicy {
    fn record_message(&self, key: &'static str) -> Msg {
        self.calls.write().push(key);
        let mut message = Msg::new();
        message.set_text(key.as_bytes());
        message
    }
}

fn plane_shape() -> Shape {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 3.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}
fn derived_policy(source: Shape, suppress_surface: bool) -> DerivedBezierPolicy {
    let converter = ShapeUpgradeShapeConvertToBezier::new(source);
    converter.set_surface_conversion(true);
    DerivedBezierPolicy {
        converter,
        calls: SharedState::new(Vec::new()),
        suppress_surface,
    }
}
fn compound_planes() -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    for _ in 0..2 {
        Builder.add(&mut compound, &plane_shape()).unwrap();
    }
    compound.into_shape()
}
fn assert_virtual_trace(policy: &DerivedBezierPolicy, count: usize) {
    let one = ["face-tool", "face-message", "wire-message", "edge-message"];
    assert_eq!(*policy.calls.read(), one.repeat(count));
}
fn count_bezier_faces(shape: &Shape) -> usize {
    Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .filter(|shape| {
            !BRepTool::surface(&topods::face(shape).unwrap())
                .unwrap()
                .cast::<GeomBezierSurface>()
                .is_null()
        })
        .count()
}

#[test]
fn further_derived_policy_retains_all_virtual_hooks() {
    let policy = derived_policy(plane_shape(), false);
    let dispatch: &dyn ShapeDivider = &policy;
    assert!(
        policy
            .converter
            .perform_with_policy(dispatch, true)
            .unwrap()
    );
    assert_virtual_trace(&policy, 1);
    assert_eq!(count_bezier_faces(&policy.converter.result()), 1);
}
#[test]
fn further_derived_surface_tool_can_refuse_surface_conversion() {
    let source = plane_shape();
    let policy = derived_policy(source.clone(), true);
    assert!(!policy.perform(true).unwrap());
    assert_virtual_trace(&policy, 1);
    assert!(policy.converter.result().is_same(&source));
    let face = topods::face(&policy.converter.result()).unwrap();
    assert!(
        !BRepTool::surface(&face)
            .unwrap()
            .cast::<GeomPlane>()
            .is_null()
    );
}
#[test]
fn further_derived_compound_recursion_preserves_the_originating_policy() {
    let policy = derived_policy(compound_planes(), false);
    assert!(policy.perform(true).unwrap());
    assert_virtual_trace(&policy, 2);
    assert_eq!(count_bezier_faces(&policy.converter.result()), 2);
}
#[test]
fn converter_policy_entrypoint_refuses_an_unrelated_base_before_mutation() {
    let own = ShapeUpgradeShapeConvertToBezier::new(plane_shape());
    let other = derived_policy(plane_shape(), false);
    let context = own.context();
    let result = own.result();
    let failure = own.perform_with_policy(&other, true).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert_eq!(own.context(), context);
    assert_eq!(own.result(), result);
    assert!(other.calls.read().is_empty());
}

#[test]
fn explicit_self_policy_retains_the_existing_entrypoint_behavior() {
    let direct = ShapeUpgradeShapeConvertToBezier::new(plane_shape());
    let explicit = ShapeUpgradeShapeConvertToBezier::new(plane_shape());
    direct.set_surface_conversion(true);
    explicit.set_surface_conversion(true);
    assert_eq!(
        direct.perform(true).unwrap(),
        explicit.perform_with_policy(&explicit, true).unwrap()
    );
    assert_eq!(count_bezier_faces(&direct.result()), 1);
    assert_eq!(count_bezier_faces(&explicit.result()), 1);
}
