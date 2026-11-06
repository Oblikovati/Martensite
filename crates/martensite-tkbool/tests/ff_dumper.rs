//! Literal diagnostic transcripts exercise both explicitly selected source configurations.
use martensite_tkbool::top_ope_brep::TopOpeBRepFFDumperView as TopOpeBRepFFDumper;
use martensite_tkbool::{top_ope_brep::*, top_ope_brep_ds::*};
use martensite_tkbrep::{
    brep_adaptor::{BRepAdaptorCurve2d, BRepAdaptorSurface},
    topexp::Explorer,
    topods::{self, Face, Shape},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkgeomalgo::{
    int_patch::*,
    int_surf::{IntSurfTransition, IntSurfTypeTrans},
};
use martensite_tkmath::gp::{Direction3, Line3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex};

fn face() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
        .unwrap()
        .face()
        .unwrap()
}
fn point() -> TopOpeBRepVPointInter<'static> {
    let mut source = IntPatchPoint::new();
    source.set_value_tangent(Point3::new(1.23456789, -0., 1e8), 1e-7, false);
    source.set_parameter(2.5);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    point.set_index(5);
    point
}
fn line() -> TopOpeBRepLineInter {
    let source = IntPatchGLine::from_line(
        &Line3::new(Point3::default(), Direction3::X),
        false,
        IntPatchLineTransitions::Undecided,
    )
    .unwrap();
    source
        .add_vertex(point().point_of_intersection().unwrap())
        .unwrap();
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &Handle::new(source).cast(),
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line.set_index(7);
    line.change_vpoint(1).unwrap().change_keep(true);
    line
}
#[test]
fn release_methods_ignore_missing_context_invalid_indices_and_uninitialized_points() {
    let filler = TopOpeBRepFacesFiller::new();
    let mut dumper = TopOpeBRepFFDumper::new(Some(&filler));
    dumper.init(None).unwrap();
    let mut output = b"untouched".to_vec();
    dumper.dump_line_index(i32::MIN, &mut output).unwrap();
    dumper
        .dump_line(&TopOpeBRepLineInter::new(), &mut output)
        .unwrap();
    let point = TopOpeBRepVPointInter::new();
    dumper.dump_vp(&point, &mut output).unwrap();
    dumper
        .dump_vp_on_surface(&point, i32::MAX, &mut output)
        .unwrap();
    dumper
        .dump_dsp(&point, TopOpeBRepDSKind::Vertex, -1, true, &mut output)
        .unwrap();
    assert_eq!(dumper.explore_index(&Shape::default(), 1).unwrap(), 0);
    assert_eq!(output, b"untouched");
    assert!(dumper.p_faces_filler_dummy().is_none());
}
#[test]
fn debug_point_text_preserves_six_digit_format_signed_zero_and_discarded_flag() {
    let filler = TopOpeBRepFacesFiller::new();
    let dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    let mut output = Vec::new();
    dumper.dump_vp(&point(), &mut output).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "VP 5 on 0 : on curve : 2.5 NOT kept\ndinp L0P5 1.23457 -0 1e+08; #draw\n"
    );
    assert!(std::ptr::eq(
        dumper.p_faces_filler_dummy().unwrap(),
        &filler
    ));
}
#[test]
fn debug_line_dump_keeps_exact_spacing_and_orientation_direction_order() {
    let face = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_context(&face, &face, &mut pair, &hds).unwrap();
    let mut dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    let mut line = line();
    line.set_faces(
        &Face::unchecked(face.oriented(Orientation::Reversed)),
        &face,
    );
    let mut output = Vec::new();
    dumper.dump_line(&line, &mut output).unwrap();
    let expected = concat!(
        "\n--------------------------------------------------\nline 7/0 is a LINE\n",
        "has no vertex on restriction\nis not closed by vertices\nis not periodic\n\n",
        "dinp L7P1K 1.23457 -0 1e+08; #draw 1e-07\n\n",
        "VP 1 on 0 : on curve : 2.5\ndinp L7P1K 1.23457 -0 1e+08; #draw\n\n",
        "LINE dir : 1 0 0\ntransition from f1 / f2 FORWARDtransition from f2 / f1 REVERSED",
        "\n--------------------------------------------------\n"
    );
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}
#[test]
fn debug_geometry_dump_distinguishes_stored_point_vertex_and_unknown_tolerances() {
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let pindex = hds
        .change_ds()
        .add_point(&TopOpeBRepDSPoint::new(Point3::default(), 0.125));
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::default())
        .unwrap()
        .vertex()
        .unwrap();
    let vindex = hds.change_ds().add_shape(&vertex);
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_storage(&hds).unwrap();
    let dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    let mut output = Vec::new();
    dumper
        .dump_dsp(&point(), TopOpeBRepDSKind::Point, pindex, true, &mut output)
        .unwrap();
    dumper
        .dump_dsp(
            &point(),
            TopOpeBRepDSKind::Vertex,
            vindex,
            false,
            &mut output,
        )
        .unwrap();
    dumper
        .dump_dsp(&point(), TopOpeBRepDSKind::Unknown, -9, true, &mut output)
        .unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "VP 5 on 0 gives new DSP 1 tol = 0.125\nVP 5 on 0 equals new DSV 1 tol = 1e-07\nVP 5 on 0 gives new DS??? -9 tol = 1e-07\n"
    );
}
#[test]
fn debug_edge_maps_use_shape_identity_and_report_missing_edges() {
    let face = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_context(&face, &face, &mut pair, &hds).unwrap();
    let dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    let mut explorer = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let mut index = 1;
    while explorer.more() {
        for operand in [1, 2] {
            assert_eq!(
                dumper
                    .explore_index(&explorer.current().reversed(), operand)
                    .unwrap(),
                index
            );
        }
        index += 1;
        explorer.next().unwrap();
    }
    assert!(dumper.explore_index(&Shape::default(), 1).is_err());
    assert_eq!(dumper.explore_index(&Shape::default(), 0).unwrap(), 0);
}
#[test]
fn debug_boundary_dump_intentionally_repeats_edge_label_without_trailing_newline() {
    let face = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_context(&face, &face, &mut pair, &hds).unwrap();
    let dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    let explorer = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = topods::edge(explorer.current()).unwrap();
    let arc = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, &face).unwrap()).cast();
    let mut transition = IntSurfTransition::new();
    transition.set_type(false, IntSurfTypeTrans::In);
    let mut source = IntPatchPoint::new();
    source.set_arc(true, &arc, 0.375, &transition, &transition);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    let mut output = Vec::new();
    dumper.dump_vp_on_surface(&point, 1, &mut output).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "on edge REVERSED (ds0) (ex1) of face of 1 : par : 0.375\non edge REVERSED (ds0) (ex1) of face of 1"
    );
}
#[test]
fn borrowed_diagnostics_are_thread_safe() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<TopOpeBRepFFDumper<'static, 'static>>();
}

#[test]
fn debug_init_preserves_line_identity_for_equal_faces_and_resets_changed_faces() {
    let first = face();
    let changed = Face::unchecked(first.reversed());
    let (mut pair1, mut pair2) = (
        TopOpeBRepFacesIntersector::new(),
        TopOpeBRepFacesIntersector::new(),
    );
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let (mut filler1, mut filler2) = (TopOpeBRepFacesFiller::new(), TopOpeBRepFacesFiller::new());
    filler1
        .bind_context(&first, &first, &mut pair1, &hds)
        .unwrap();
    filler2
        .bind_context(&changed, &first, &mut pair2, &hds)
        .unwrap();
    let mut dumper = TopOpeBRepFFDumper::new_debug(&filler1).unwrap();
    dumper.dump_line(&line(), &mut Vec::new()).unwrap();
    dumper.init(Some(&filler1)).unwrap();
    let mut output = Vec::new();
    dumper.dump_vp(&point(), &mut output).unwrap();
    assert!(String::from_utf8(output).unwrap().contains("dinp L7P5"));
    dumper.init(Some(&filler2)).unwrap();
    let mut output = Vec::new();
    dumper.dump_vp(&point(), &mut output).unwrap();
    assert!(String::from_utf8(output).unwrap().contains("dinp L0P5"));
    assert!(dumper.init(None).is_err());
    assert!(dumper.p_faces_filler_dummy().is_none());
}

#[test]
fn debug_map_repeated_edge_occurrences_replace_with_the_last_exploration_index() {
    use martensite_tkbrep::topods::{Builder, Wire};
    let first = face();
    let mut explorer = Explorer::new(&first, ShapeEnum::Wire, ShapeEnum::Shape).unwrap();
    let wire_shape = explorer.current().clone();
    let mut wire = Wire::unchecked(wire_shape);
    wire.set_free(true).unwrap();
    explorer = Explorer::new(&first, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = explorer.current().clone();
    Builder.add(&mut wire, &edge.reversed()).unwrap();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(&first, &first, &mut pair, &hds)
        .unwrap();
    let dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    assert_eq!(dumper.explore_index(&edge, 1).unwrap(), 5);
}

struct FailingDiagnosticSink;
impl std::io::Write for FailingDiagnosticSink {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("named sink failure"))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[test]
fn debug_io_failure_is_returned_without_silent_output_loss() {
    let filler = TopOpeBRepFacesFiller::new();
    let mut dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    let error = dumper
        .dump_vp(&point(), &mut FailingDiagnosticSink)
        .unwrap_err();
    assert!(error.message().contains("named sink failure"));
    assert!(dumper.dump_line_index(1, &mut Vec::new()).is_err());
}

#[test]
fn indexed_dump_reads_the_prepared_intersection_line() {
    let first = face();
    let plane = Plane3::new(Point3::default(), Direction3::X).unwrap();
    let second = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(&first, &second).unwrap();
    assert_eq!(pair.nb_lines(), 1);
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_context(&first, &second, &mut pair, &hds)
        .unwrap();
    let mut dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    let mut output = Vec::new();
    dumper.dump_line_index(1, &mut output).unwrap();
    assert!(
        String::from_utf8(output).unwrap().starts_with(
            "\n--------------------------------------------------\nline 1/1 is a LINE\n"
        )
    );
    assert!(dumper.dump_line_index(2, &mut Vec::new()).is_err());
}

#[test]
fn restriction_and_periodic_line_headers_preserve_flags() {
    use martensite_tkmath::gp::{Axis2, Circle3};
    let face = face();
    let mut pair = TopOpeBRepFacesIntersector::new();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler.bind_context(&face, &face, &mut pair, &hds).unwrap();
    let mut dumper = TopOpeBRepFFDumper::new_debug(&filler).unwrap();
    let explorer = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = topods::edge(explorer.current()).unwrap();
    let arc = Handle::new(BRepAdaptorCurve2d::with_edge_on_face(&edge, &face).unwrap()).cast();
    let source = IntPatchRLine::new_undecided(false);
    source.set_arc_on_s1(&arc);
    let mut restriction = TopOpeBRepLineInter::new();
    restriction
        .set_line(
            &Handle::new(source).cast(),
            &BRepAdaptorSurface::default(),
            &BRepAdaptorSurface::default(),
        )
        .unwrap();
    let mut output = Vec::new();
    dumper.dump_line(&restriction, &mut output).unwrap();
    assert!(
        String::from_utf8(output)
            .unwrap()
            .contains("is a RESTRICTION of 1\n++++ line restriction\n")
    );
    let circle = IntPatchGLine::from_circle(
        &Circle3::new(Axis2::default(), 1.).unwrap(),
        false,
        IntPatchLineTransitions::Undecided,
    );
    let mut periodic = TopOpeBRepLineInter::new();
    periodic
        .set_line(
            &Handle::new(circle).cast(),
            &BRepAdaptorSurface::default(),
            &BRepAdaptorSurface::default(),
        )
        .unwrap();
    let mut output = Vec::new();
    dumper.dump_line(&periodic, &mut output).unwrap();
    assert!(String::from_utf8(output).unwrap().contains("is periodic\n"));
}
