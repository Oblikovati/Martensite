//! `StepTidyCartesianPointReducer`.
use crate::step_tidy_fixture::{Fixture, ORIGIN, ReplacedEntities, Z_AXIS, name, one_item};
use martensite_tkdestep::step_data::{StepDataLogical, StepDataSelectType};
use martensite_tkdestep::step_geom::{
    StepGeomAxis1Placement, StepGeomAxis2Placement3d, StepGeomBSplineCurveForm,
    StepGeomBSplineCurveWithKnots, StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve,
    StepGeomBSplineSurfaceForm, StepGeomBSplineSurfaceWithKnots,
    StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface, StepGeomCartesianPoint,
    StepGeomDirection, StepGeomKnotType, StepGeomLine, StepGeomRationalBSplineSurface,
    StepGeomVector,
};
use martensite_tkdestep::step_repr::{StepReprRepresentation, StepReprRepresentationContext};
use martensite_tkdestep::step_shape::{
    StepShapeGeometricCurveSet, StepShapeGeometricSetSelect, StepShapeVertexPoint,
};
use martensite_tkdestep::step_tidy::StepTidyCartesianPointReducer;
use martensite_tkdestep::step_visual::{
    StepVisualLayeredItem, StepVisualPresentationLayerAssignment,
    StepVisualPresentationStyleAssignment, StepVisualStyledItem,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::Handle;

type Point = Handle<StepGeomCartesianPoint>;

const UNKNOWN: StepDataLogical = StepDataLogical::LUnknown;

fn replaced(fixture: &Fixture) -> ReplacedEntities {
    fixture.replaced_by(&mut StepTidyCartesianPointReducer::new(&fixture.ws))
}

/// Two equal points, each held by its own sharing entity: one is replaced.
fn assert_merged_in(share: impl Fn(&Fixture, &Point)) {
    let fixture = Fixture::new();
    let first = fixture.add_cartesian_point("", ORIGIN);
    let second = fixture.add_cartesian_point("", ORIGIN);
    share(&fixture, &first);
    share(&fixture, &second);
    replaced(&fixture).assert_one_of(&first, &second);
}

fn add_placement_on(fixture: &Fixture, point: &Point, direction: &Handle<StepGeomDirection>) {
    let placement = Handle::new(StepGeomAxis2Placement3d::new());
    placement.init(&name(""), point, true, direction, false, &Handle::default());
    fixture.add(&placement);
}

fn empty_integers() -> Handle<HArray1<i32>> {
    Handle::new(HArray1::default())
}

fn empty_reals() -> Handle<HArray1<f64>> {
    Handle::new(HArray1::default())
}

fn point_net(point: &Point) -> Handle<HArray2<Point>> {
    let net = HArray2::with_bounds(1, 1, 1, 1).expect("bounds");
    net.change_array2()
        .set_value(1, 1, point.clone())
        .expect("set");
    Handle::new(net)
}

fn knotted_surface(net: &Handle<HArray2<Point>>) -> Handle<StepGeomBSplineSurfaceWithKnots> {
    let surface = Handle::new(StepGeomBSplineSurfaceWithKnots::new());
    surface.init(
        &name(""),
        1,
        1,
        net,
        StepGeomBSplineSurfaceForm::BssfUnspecified,
        UNKNOWN,
        UNKNOWN,
        UNKNOWN,
        &empty_integers(),
        &empty_integers(),
        &empty_reals(),
        &empty_reals(),
        StepGeomKnotType::KtUnspecified,
    );
    surface
}

fn rational_surface() -> Handle<StepGeomRationalBSplineSurface> {
    let surface = Handle::new(StepGeomRationalBSplineSurface::new());
    surface.init(
        &name(""),
        1,
        1,
        &Handle::new(HArray2::with_bounds(1, 1, 1, 1).expect("bounds")),
        StepGeomBSplineSurfaceForm::BssfUnspecified,
        UNKNOWN,
        UNKNOWN,
        UNKNOWN,
        &Handle::new(HArray2::with_bounds(1, 1, 1, 1).expect("bounds")),
    );
    surface
}

/// The combined surface over its knotted part, which it shares the control net with.
fn combined_surface(
    fixture: &Fixture,
    point: &Point,
    rational: &Handle<StepGeomRationalBSplineSurface>,
) -> Handle<StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface> {
    let net = point_net(point);
    let knotted = knotted_surface(&net);
    fixture.add(&knotted);
    let combined = Handle::new(StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::new());
    combined.init(
        &name(""),
        1,
        1,
        &net,
        StepGeomBSplineSurfaceForm::BssfUnspecified,
        UNKNOWN,
        UNKNOWN,
        UNKNOWN,
        &knotted,
        rational,
    );
    combined
}

#[test]
fn different_names() {
    let fixture = Fixture::new();
    let first = fixture.add_cartesian_point("FirstPt", ORIGIN);
    let second = fixture.add_cartesian_point("SecondPt", ORIGIN);
    let direction = fixture.add_direction("", Z_AXIS);
    add_placement_on(&fixture, &first, &direction);
    add_placement_on(&fixture, &second, &direction);
    replaced(&fixture).assert_empty();
}

#[test]
fn step_geom_axis2_placement3d() {
    let fixture = Fixture::new();
    let first = fixture.add_cartesian_point("", ORIGIN);
    let second = fixture.add_cartesian_point("", ORIGIN);
    let direction = fixture.add_direction("", Z_AXIS);
    add_placement_on(&fixture, &first, &direction);
    add_placement_on(&fixture, &second, &direction);
    replaced(&fixture).assert_one_of(&first, &second);
}

#[test]
fn step_shape_vertex_point() {
    assert_merged_in(|fixture, point| {
        let vertex = Handle::new(StepShapeVertexPoint::new());
        vertex.init(&name(""), &point.cast());
        fixture.add(&vertex);
    });
}

#[test]
fn step_shape_geometric_curve_set() {
    assert_merged_in(|fixture, point| {
        let elements = HArray1::with_bounds(1, 1).expect("bounds");
        let mut select = StepShapeGeometricSetSelect::default();
        select.set_value(&point.cast()).expect("a point");
        elements.change_array1().set_value(1, select).expect("set");
        let elements = Handle::new(elements);
        fixture.add(&elements);
        let set = Handle::new(StepShapeGeometricCurveSet::new());
        set.init(&name(""), &elements);
        fixture.add(&set);
    });
}

#[test]
fn step_visual_presentation_layer_assignment() {
    assert_merged_in(|fixture, point| {
        let items = HArray1::with_bounds(1, 1).expect("bounds");
        let mut item = StepVisualLayeredItem::default();
        item.set_value(&point.cast()).expect("a layered item");
        items.change_array1().set_value(1, item).expect("set");
        let items = Handle::new(items);
        fixture.add(&items);
        let assignment = Handle::new(StepVisualPresentationLayerAssignment::new());
        assignment.init(&name(""), &name(""), &items);
        fixture.add(&assignment);
    });
}

#[test]
fn step_visual_styled_item() {
    assert_merged_in(|fixture, point| {
        let styles = HArray1::<Handle<StepVisualPresentationStyleAssignment>>::with_bounds(1, 1);
        let styled = Handle::new(StepVisualStyledItem::new());
        styled.init(
            &name(""),
            &Handle::new(styles.expect("bounds")),
            &point.cast(),
        );
        fixture.add(&styled);
    });
}

#[test]
fn step_geom_b_spline_curve_with_knots() {
    assert_merged_in(|fixture, point| {
        let curve = Handle::new(StepGeomBSplineCurveWithKnots::new());
        curve.init(
            &name(""),
            1,
            &one_item(point.clone()),
            StepGeomBSplineCurveForm::BscfUnspecified,
            UNKNOWN,
            UNKNOWN,
            &empty_integers(),
            &empty_reals(),
            StepGeomKnotType::KtUnspecified,
        );
        fixture.add(&curve);
    });
}

#[test]
fn step_geom_line() {
    assert_merged_in(|fixture, point| {
        let line = Handle::new(StepGeomLine::new());
        line.init(&name(""), point, &Handle::new(StepGeomVector::new()));
        fixture.add(&line);
    });
}

#[test]
fn step_geom_b_spline_surface_with_knots() {
    assert_merged_in(|fixture, point| fixture.add(&knotted_surface(&point_net(point))));
}

#[test]
fn step_geom_axis1_placement() {
    assert_merged_in(|fixture, point| {
        let axis = Handle::new(StepGeomAxis1Placement::new());
        axis.init(
            &name(""),
            point,
            false,
            &Handle::new(StepGeomDirection::new()),
        );
        fixture.add(&axis);
    });
}

#[test]
fn step_repr_representation() {
    assert_merged_in(|fixture, point| {
        let representation = Handle::new(StepReprRepresentation::new());
        representation.init(
            &name(""),
            &one_item(point.cast()),
            &Handle::new(StepReprRepresentationContext::new()),
        );
        fixture.add(&representation);
    });
}

#[test]
fn step_geom_b_spline_curve_with_knots_and_rational_b_spline_curve() {
    assert_merged_in(|fixture, point| {
        let curve = Handle::new(StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::new());
        curve.init_fields(
            &name(""),
            1,
            &one_item(point.clone()),
            StepGeomBSplineCurveForm::BscfUnspecified,
            UNKNOWN,
            UNKNOWN,
            &empty_integers(),
            &empty_reals(),
            StepGeomKnotType::KtUnspecified,
            &empty_reals(),
        );
        fixture.add(&curve);
    });
}

/// adds only the first combined surface to the model: the second point is held by the
/// second knotted surface alone, whose net the combined one would share.
#[test]
fn step_geom_b_spline_surface_with_knots_and_rational_b_spline_surface() {
    let fixture = Fixture::new();
    let first = fixture.add_cartesian_point("", ORIGIN);
    let second = fixture.add_cartesian_point("", ORIGIN);
    let rational = rational_surface();
    fixture.add(&combined_surface(&fixture, &first, &rational));
    let _not_added = combined_surface(&fixture, &second, &rational);
    replaced(&fixture).assert_one_of(&first, &second);
}
