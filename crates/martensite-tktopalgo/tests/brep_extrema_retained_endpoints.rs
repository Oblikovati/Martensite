//! Point-curve initialization retains calculated endpoints without freezing live geometry.
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Edge, Vertex};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
use martensite_tktopalgo::brep_extrema::{BRepExtremaExtCC, BRepExtremaExtPC};

fn two_span_line() -> Handle<GeomBSplineCurve> {
    let mut poles = Array1::with_bounds(1, 3).unwrap();
    let mut knots = Array1::with_bounds(1, 3).unwrap();
    let mut mults = Array1::with_bounds(1, 3).unwrap();
    for index in 1..=3 {
        poles[index] = Point3::new(f64::from(index - 1), 0.0, 0.0);
        knots[index] = f64::from(index - 1);
        mults[index] = if index == 2 { 1 } else { 2 };
    }
    Handle::new(GeomBSplineCurve::new(&poles, &knots, &mults, 1, false).unwrap())
}

struct EndpointFixture {
    curve: Handle<GeomBSplineCurve>,
    edge: Edge,
    vertex: Vertex,
}

impl EndpointFixture {
    fn new() -> Self {
        let curve = two_span_line();
        let geometric: Handle<dyn GeomCurve> = curve.clone().cast();
        let edge = BRepBuilderAPIMakeEdge::from_curve_params(&geometric, 0.0, 2.0)
            .unwrap()
            .edge()
            .unwrap();
        let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1.0, 2.0, 0.0))
            .unwrap()
            .vertex()
            .unwrap();
        Self {
            curve,
            edge,
            vertex,
        }
    }

    fn calculate(&self) -> BRepExtremaExtPC {
        BRepExtremaExtPC::with_vertex_edge(&self.vertex, &self.edge).unwrap()
    }

    fn move_poles(&self) {
        for index in 1..=3 {
            self.curve
                .set_pole(index, Point3::new(f64::from(index + 9), 0.0, 0.0))
                .unwrap();
        }
    }
}

#[test]
fn reinitialization_preserves_calculated_endpoints_after_source_poles_change() {
    let fixture = EndpointFixture::new();
    let mut extrema = fixture.calculate();
    let (_, _, first, last) = extrema.trimmed_square_distances();
    assert_eq!(first, Point3::new(0.0, 0.0, 0.0));
    assert_eq!(last, Point3::new(2.0, 0.0, 0.0));
    fixture.move_poles();
    extrema.initialize(&fixture.edge).unwrap();
    assert!(!extrema.is_done());
    assert_eq!(
        extrema.trimmed_square_distances(),
        (f64::MAX, f64::MAX, first, last)
    );
    extrema.initialize(&fixture.edge).unwrap();
    assert_eq!(
        extrema.trimmed_square_distances(),
        (f64::MAX, f64::MAX, first, last)
    );
}

#[test]
fn the_next_perform_evaluates_current_poles_and_updates_endpoint_distances() {
    let fixture = EndpointFixture::new();
    let mut extrema = fixture.calculate();
    fixture.move_poles();
    extrema.initialize(&fixture.edge).unwrap();
    extrema.perform(&fixture.vertex).unwrap();
    assert_eq!(
        extrema.trimmed_square_distances(),
        (
            85.0,
            125.0,
            Point3::new(10.0, 0.0, 0.0),
            Point3::new(12.0, 0.0, 0.0)
        )
    );
}

#[test]
fn a_cloned_projector_retains_calculated_values_when_another_is_reinitialized() {
    let fixture = EndpointFixture::new();
    let mut extrema = fixture.calculate();
    let mut cloned = extrema.clone();
    fixture.move_poles();
    extrema.initialize(&fixture.edge).unwrap();
    assert!(!extrema.is_done());
    assert!(cloned.is_done());
    assert_eq!(cloned.square_distance(1).unwrap(), 4.0);
    cloned.perform(&fixture.vertex).unwrap();
    assert_eq!(cloned.trimmed_square_distances().0, 85.0);
    assert_eq!(extrema.trimmed_square_distances().2, Point3::default());
}

#[test]
fn non_geometric_initialization_leaves_computed_state_unchanged() {
    let fixture = EndpointFixture::new();
    let mut extrema = fixture.calculate();
    let before = extrema.trimmed_square_distances();
    let mut polygon_only = Edge::default();
    BRepBuilder::default().make_edge(&mut polygon_only).unwrap();
    extrema.initialize(&polygon_only).unwrap();
    assert!(extrema.is_done());
    assert_eq!(extrema.square_distance(1).unwrap(), 4.0);
    assert_eq!(extrema.trimmed_square_distances(), before);
}

#[test]
fn curve_curve_reinitialization_preserves_previous_endpoint_distances() {
    let fixture = EndpointFixture::new();
    let (mut extrema, second) = crossed_curve_pair(&fixture);
    let before = extrema.trimmed_square_distances();
    fixture.move_poles();
    extrema.initialize(&second).unwrap();
    assert_eq!(extrema.trimmed_square_distances(), before);
}

fn crossed_curve_pair(fixture: &EndpointFixture) -> (BRepExtremaExtCC, Edge) {
    let second = BRepBuilderAPIMakeEdge::from_points(
        Point3::new(1.0, -1.0, 3.0),
        Point3::new(1.0, 1.0, 3.0),
    )
    .unwrap()
    .edge()
    .unwrap();
    let extrema = BRepExtremaExtCC::with_edges(&fixture.edge, &second).unwrap();
    assert!(extrema.is_done());
    assert_eq!(extrema.trimmed_square_distances().0, [11.0; 4]);
    (extrema, second)
}

#[test]
fn curve_curve_next_perform_evaluates_changed_poles() {
    let fixture = EndpointFixture::new();
    let (mut extrema, second) = crossed_curve_pair(&fixture);
    fixture.move_poles();
    extrema.initialize(&second).unwrap();
    extrema.perform(&fixture.edge).unwrap();
    assert_eq!(
        extrema.trimmed_square_distances().0,
        [91.0, 91.0, 131.0, 131.0]
    );
}

#[test]
fn curve_curve_copies_keep_calculated_endpoints_independent() {
    let fixture = EndpointFixture::new();
    let (mut original, second) = crossed_curve_pair(&fixture);
    let mut cloned = original.clone();
    fixture.move_poles();
    original.initialize(&second).unwrap();
    assert!(original.is_done() && cloned.is_done());
    assert_eq!(cloned.square_distance(1).unwrap(), 9.0);
    cloned.perform(&fixture.edge).unwrap();
    assert_eq!(
        cloned.trimmed_square_distances().0,
        [91.0, 91.0, 131.0, 131.0]
    );
    assert_eq!(original.trimmed_square_distances().0, [11.0; 4]);
}

#[test]
fn curve_curve_non_geometric_calls_leave_calculation_unchanged() {
    let fixture = EndpointFixture::new();
    let (mut extrema, _) = crossed_curve_pair(&fixture);
    let before = extrema.trimmed_square_distances();
    let mut polygon_only = Edge::default();
    BRepBuilder::default().make_edge(&mut polygon_only).unwrap();
    extrema.initialize(&polygon_only).unwrap();
    extrema.perform(&polygon_only).unwrap();
    assert!(extrema.is_done());
    assert_eq!(extrema.trimmed_square_distances(), before);
}

#[test]
fn curve_curve_repeated_general_performs_retain_accumulated_parameter_state() {
    let fixture = EndpointFixture::new();
    let (mut original, _) = crossed_curve_pair(&fixture);
    let count = original.nb_ext().unwrap();
    assert_eq!(count, 1);
    assert_eq!(original.square_distance(1).unwrap(), 9.0);
    original.perform(&fixture.edge).unwrap();
    assert_eq!(original.nb_ext().unwrap(), 2 * count);
    let mut cloned = original.clone();
    cloned.perform(&fixture.edge).unwrap();
    assert_eq!(cloned.nb_ext().unwrap(), 3 * count);
    assert_eq!(original.nb_ext().unwrap(), 2 * count);
    assert_eq!(cloned.square_distance(3).unwrap(), 9.0);
}
