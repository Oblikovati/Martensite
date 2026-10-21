//! Container deletion is valid reshape work; an unsupported split is never empty success.
use super::*;
use martensite_tkbrep::topods::{Face, Shape, ShapeIterator, Wire};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

fn remove_wire_edges(context: &Handle<ShapeBuildReShape>, wire: &Wire) {
    for edge in ShapeIterator::new(&wire.clone().into_shape(), true, true) {
        context.remove(&edge).unwrap();
    }
}

fn first_wire(face: &Face) -> Wire {
    let shape = face.clone().into_shape();
    let children = ShapeIterator::new(&shape, true, true);
    topods::wire(children.value().unwrap()).unwrap()
}

fn assert_empty_rebuilt_wire(context: &Handle<ShapeBuildReShape>, wire: &Wire) {
    let result = context
        .apply(&wire.clone().into_shape(), ShapeEnum::Shape)
        .unwrap();
    assert!(!result.is_null());
    assert_eq!(result.shape_type().unwrap(), ShapeEnum::Wire);
    assert!(!ShapeIterator::new(&result, true, true).more());
    assert!(context.status_flag(ShapeExtendStatus::Done4));
    assert!(!context.status_flag(ShapeExtendStatus::Fail1));
}

#[test]
fn removing_all_child_edges_builds_an_empty_wire_with_deletion_history() {
    let face = face_fixture();
    let wire = first_wire(&face);
    let source_edges: Vec<Shape> =
        ShapeIterator::new(&wire.clone().into_shape(), true, true).collect();
    let context = Handle::new(ShapeBuildReShape::new());
    remove_wire_edges(&context, &wire);
    assert_empty_rebuilt_wire(&context, &wire);
    for edge in source_edges {
        assert!(context.value(&edge).unwrap().is_null());
        assert_eq!(context.status(&edge, false).unwrap().0, -1);
    }
    assert_eq!(
        ShapeIterator::new(&wire.into_shape(), true, true).count(),
        4
    );
}

#[test]
fn all_removed_wires_are_a_failed_perform_without_done_or_published_result() {
    let face = face_fixture();
    let context = Handle::new(ShapeBuildReShape::new());
    remove_wire_edges(&context, &first_wire(&face));
    let composer = initialized_composer(&face, &context);
    assert!(composer.load_wires().unwrap().is_empty());
    assert!(!composer.perform().unwrap());
    assert!(composer.status(ShapeExtendStatus::Fail6));
    assert!(!composer.status(ShapeExtendStatus::Done1));
    assert!(composer.result().is_null());
}

#[test]
fn failed_all_removed_rerun_retains_prior_result_without_claiming_completion() {
    let face = face_fixture();
    let context = Handle::new(ShapeBuildReShape::new());
    let composer = initialized_composer(&face, &context);
    assert!(composer.perform().unwrap());
    let previous = composer.result();
    remove_wire_edges(&context, &first_wire(&face));
    assert!(!composer.perform().unwrap());
    assert!(composer.status(ShapeExtendStatus::Fail6));
    assert!(!composer.status(ShapeExtendStatus::Done1));
    assert!(composer.result().is_same(&previous));
}

fn interior_rectangle() -> Wire {
    let points = [
        Point3::new(0.5, 0.5, 0.),
        Point3::new(1.5, 0.5, 0.),
        Point3::new(1.5, 1.5, 0.),
        Point3::new(0.5, 1.5, 0.),
    ];
    let mut builder = BRepBuilderAPIMakeWire::new();
    for index in 0..4 {
        let edge = BRepBuilderAPIMakeEdge::from_points(points[index], points[(index + 1) % 4])
            .unwrap()
            .edge()
            .unwrap();
        builder.add_edge(&edge).unwrap();
    }
    let mut wire = builder.wire().unwrap();
    wire.set_orientation(Orientation::Reversed);
    wire
}

#[test]
fn kept_outer_wire_and_deleted_hole_publish_only_the_valid_square() {
    let mut face = face_fixture();
    let hole = interior_rectangle();
    BRepBuilder::default()
        .add(&mut face, &hole.clone().into_shape())
        .unwrap();
    let context = Handle::new(ShapeBuildReShape::new());
    remove_wire_edges(&context, &hole);
    let composer = initialized_composer(&face, &context);
    assert_eq!(composer.load_wires().unwrap().length(), 1);
    assert!(composer.perform().unwrap());
    assert!(composer.status(ShapeExtendStatus::Done1));
    assert!(!composer.status(ShapeExtendStatus::Fail));
    assert_valid_square(&composer.result());
    assert_original_hole_is_untouched(&face, &hole);
}

fn assert_original_hole_is_untouched(face: &Face, hole: &Wire) {
    assert_eq!(
        ShapeIterator::new(&face.clone().into_shape(), true, true).count(),
        2
    );
    assert_eq!(
        ShapeIterator::new(&hole.clone().into_shape(), true, true).count(),
        4
    );
}

fn assert_valid_square(result: &Shape) {
    assert_eq!(result.shape_type().unwrap(), ShapeEnum::Face);
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let wire = first_wire(&topods::face(result).unwrap());
    let mut points: Vec<(f64, f64)> = ShapeIterator::new(&wire.into_shape(), true, true)
        .map(|edge| square_vertex(&edge))
        .collect();
    points.sort_by(|left, right| left.partial_cmp(right).unwrap());
    assert_eq!(points, vec![(0., 0.), (0., 2.), (2., 0.), (2., 2.)]);
}

fn square_vertex(edge: &Shape) -> (f64, f64) {
    let vertex = ShapeIterator::new(edge, true, true);
    let point = BRepTool::pnt(&topods::vertex(vertex.value().unwrap()).unwrap()).unwrap();
    assert_eq!(point.z(), 0.);
    (point.x(), point.y())
}

fn record_edge_replacement(
    context: &Handle<ShapeBuildReShape>,
    source: &Shape,
    empty_container: bool,
) {
    if !empty_container {
        context.remove(source).unwrap();
        return;
    }
    let mut replacement = martensite_tkbrep::topods::Compound::default();
    Builder.make_compound(&mut replacement);
    context.replace(source, &replacement.into_shape()).unwrap();
}

fn split_after_replacement(
    empty_container: bool,
) -> (
    ShapeFixComposeShell,
    ShapeFixWireSegment,
    martensite_tkernel::standard::Failure,
) {
    let face = face_fixture();
    let context = Handle::new(ShapeBuildReShape::new());
    let composer = initialized_composer(&face, &context);
    let mut wires = composer.load_wires().unwrap();
    let source = wires.value(1).unwrap().edge(1).unwrap().into_shape();
    record_edge_replacement(&context, &source, empty_container);
    let line = Line2::new(Point2::new(3., 0.), Direction2::new(0., 1.).unwrap());
    let error = composer
        .split_segments_by_line(&mut wires, line, true, 1)
        .unwrap_err();
    let segment = wires.value(1).unwrap().clone();
    (composer, segment, error)
}

#[test]
fn empty_edge_container_is_an_explicit_unsupported_split_not_empty_success() {
    let (composer, segment, error) = split_after_replacement(true);
    assert_eq!(error.exception_type(), "Standard_NotImplemented");
    assert_eq!(segment.nb_edges(), 4);
    assert!(!composer.status(ShapeExtendStatus::Done1));
    assert!(composer.result().is_null());
}

#[test]
fn null_edge_replacement_is_a_typed_failure_not_empty_success() {
    let (composer, segment, error) = split_after_replacement(false);
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(segment.nb_edges(), 4);
    assert!(!composer.status(ShapeExtendStatus::Done1));
    assert!(composer.result().is_null());
}

// This pins a known legacy defect, not successful modelling acceptance. The checked
// publication boundary must reject this invalid face; the port keeps native behavior.
#[test]
fn incompatible_empty_container_keeps_legacy_invalid_success_and_cache_status_loss() {
    let face = face_fixture();
    let context = Handle::new(ShapeBuildReShape::new());
    let wire = first_wire(&face);
    let edge = ShapeIterator::new(&wire.clone().into_shape(), true, true)
        .value()
        .unwrap()
        .clone();
    record_edge_replacement(&context, &edge, true);
    assert_incompatible_cache_status(&context, &wire);
    let composer = initialized_composer(&face, &context);
    for _ in 0..2 {
        assert_legacy_invalid_success(&composer);
    }
}

fn assert_legacy_invalid_success(composer: &ShapeFixComposeShell) {
    assert!(composer.perform().unwrap());
    assert!(composer.status(ShapeExtendStatus::Done1));
    assert!(
        !BRepCheckAnalyzer::new(&composer.result(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let result_wire = first_wire(&topods::face(&composer.result()).unwrap());
    assert_eq!(
        ShapeIterator::new(&result_wire.into_shape(), true, true).count(),
        3
    );
}

fn assert_incompatible_cache_status(context: &Handle<ShapeBuildReShape>, wire: &Wire) {
    let source = wire.clone().into_shape();
    let partial = context.apply(&source, ShapeEnum::Shape).unwrap();
    assert!(context.status_flag(ShapeExtendStatus::Fail1));
    assert!(!context.status_flag(ShapeExtendStatus::Done4));
    assert_eq!(ShapeIterator::new(&partial, true, true).count(), 3);
    assert!(
        context
            .apply(&source, ShapeEnum::Shape)
            .unwrap()
            .is_same(&partial)
    );
    assert!(context.status_flag(ShapeExtendStatus::Done1));
    assert!(!context.status_flag(ShapeExtendStatus::Fail1));
}

#[test]
fn splitting_an_empty_sequence_completes_as_a_noop_without_perform_success() {
    let context = Handle::new(ShapeBuildReShape::new());
    let composer = initialized_composer(&face_fixture(), &context);
    let mut wires = Sequence::default();
    let line = Line2::new(Point2::new(3., 0.), Direction2::new(0., 1.).unwrap());
    composer
        .split_segments_by_line(&mut wires, line, true, 1)
        .unwrap();
    assert!(wires.is_empty());
    assert!(composer.status(ShapeExtendStatus::Ok));
    assert!(!composer.status(ShapeExtendStatus::Done));
    assert!(!composer.status(ShapeExtendStatus::Fail));
    assert!(composer.result().is_null());
}
