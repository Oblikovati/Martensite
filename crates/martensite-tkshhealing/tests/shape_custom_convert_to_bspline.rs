//! Plane conversion, explicit curve storage and copy isolation contracts.
mod healing_support;

use healing_support::{edges_of, square_face};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools::{BRepToolsModification, Modifier, NewCurve, NewSurface};
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomCurve, GeomLine, GeomSurface};
use martensite_tkmath::gp::Point3;
use martensite_tkshhealing::shape_custom::ShapeCustomConvertToBSpline;

#[test]
fn plane_is_kept_by_default() {
    let face = square_face(0.);
    let modification = ShapeCustomConvertToBSpline::new();
    let mut surface = NewSurface::default();
    assert!(!modification.new_surface(&face, &mut surface).unwrap());
    let edge = &edges_of(&face.clone().into_shape())[0];
    assert!(
        !modification
            .new_curve(edge, &mut NewCurve::default())
            .unwrap()
    );
}

#[test]
fn plane_mode_converts_the_plane_on_the_face_bounds() {
    let face = square_face(0.);
    let edge = store_first_pcurve(&face);
    let modification = ShapeCustomConvertToBSpline::new();
    modification.set_plane_mode(true);
    let mut surface = NewSurface::default();
    assert!(modification.new_surface(&face, &mut surface).unwrap());
    assert_unit_square_surface(&surface);
    // The fixture explicitly owns its stored p-curve; construction does not insert it (#243).
    assert!(
        modification
            .new_curve(&edge, &mut NewCurve::default())
            .unwrap()
    );
}

fn assert_unit_square_surface(surface: &NewSurface) {
    let spline = surface.surface.cast::<GeomBSplineSurface>();
    let spline = spline.get().expect("a B-spline surface");
    assert_eq!((spline.u_degree(), spline.v_degree()), (1, 1));
    // The plane of a polygon face is framed at its centre: the UV box is [-0.5, 0.5]^2.
    assert_eq!(spline.bounds(), (-0.5, 0.5, -0.5, 0.5));
    assert!(!surface.rev_wires && !surface.rev_face);
}

#[test]
fn modifier_rebuilds_a_plane_face_on_a_bspline() {
    let face = square_face(2.).into_shape();
    let modification = ShapeCustomConvertToBSpline::new();
    modification.set_plane_mode(true);
    let modification: Handle<dyn BRepToolsModification> = Handle::new(modification).cast();
    let modifier = Modifier::with_modification(&face, &modification).unwrap();
    assert!(modifier.is_done());
    let result = modifier.modified_shape(&face).unwrap();
    let rebuilt = martensite_tkbrep::topods::face(&result).unwrap();
    let surface = martensite_tkbrep::brep::BRepTool::surface(&rebuilt).unwrap();
    assert!(!surface.cast::<GeomBSplineSurface>().is_null());
}

fn store_first_pcurve(face: &Face) -> Edge {
    let edge = edges_of(&face.clone().into_shape())[0].clone();
    let projected = BRepTool::curve_on_face(&edge, face).unwrap();
    assert!(!projected.is_stored);
    BRepBuilder::default()
        .update_edge_pcurve_on_face(
            &edge,
            &projected.pcurve,
            face,
            BRepTool::edge_tolerance(&edge).unwrap(),
        )
        .unwrap();
    assert!(BRepTool::curve_on_face(&edge, face).unwrap().is_stored);
    edge
}

fn assert_bottom_edge(curve: &Handle<dyn GeomCurve>) {
    for parameter in [0., 0.25, 0.5, 1.] {
        let actual = curve.get().unwrap().value(parameter).unwrap();
        assert!(actual.distance(&Point3::new(parameter, 0., 0.)) < 1e-14);
    }
}

#[test]
fn stored_plane_curve_copy_preserves_geometry_and_isolates_mutation() {
    let face = square_face(0.);
    let edge = store_first_pcurve(&face);
    let source = BRepTool::curve_with_location(&edge).unwrap();
    let modification = ShapeCustomConvertToBSpline::new();
    modification.set_plane_mode(true);
    let mut copied = NewCurve::default();
    assert!(modification.new_curve(&edge, &mut copied).unwrap());
    assert!(copied.curve != source.curve);
    assert_eq!(copied.location, source.location);
    assert_eq!(copied.tolerance, BRepTool::edge_tolerance(&edge).unwrap());
    assert_bottom_edge(&source.curve);
    assert_bottom_edge(&copied.curve);
    assert_curve_mutations_are_isolated(&source.curve, &copied.curve);
    assert!(BRepTool::curve_on_face(&edge, &face).unwrap().is_stored);
}

fn assert_curve_mutations_are_isolated(
    source: &Handle<dyn GeomCurve>,
    copied: &Handle<dyn GeomCurve>,
) {
    let copied_line = copied.cast::<GeomLine>();
    let source_line = source.cast::<GeomLine>();
    copied_line
        .get()
        .unwrap()
        .set_location(Point3::new(5., 6., 7.));
    assert_bottom_edge(source);
    source_line
        .get()
        .unwrap()
        .set_location(Point3::new(8., 9., 10.));
    assert_eq!(
        copied.get().unwrap().value(0.).unwrap(),
        Point3::new(5., 6., 7.)
    );
}

#[test]
fn generated_plane_curve_does_not_request_a_stored_curve_copy() {
    let face = square_face(0.);
    let edge = edges_of(&face.clone().into_shape())[0].clone();
    let source = BRepTool::curve_with_location(&edge).unwrap();
    let modification = ShapeCustomConvertToBSpline::new();
    modification.set_plane_mode(true);
    assert!(!BRepTool::curve_on_face(&edge, &face).unwrap().is_stored);
    assert_unchanged_curve_output(&modification, &edge, &source.curve);
    assert_bottom_edge(&source.curve);
    assert_generated_pcurve_geometry(&modification, &edge, &face);
}

fn assert_unchanged_curve_output(
    modification: &ShapeCustomConvertToBSpline,
    edge: &Edge,
    source: &Handle<dyn GeomCurve>,
) {
    let mut unchanged = NewCurve {
        curve: source.clone(),
        tolerance: 0.125,
        ..NewCurve::default()
    };
    let original_location = unchanged.location.clone();
    assert!(!modification.new_curve(edge, &mut unchanged).unwrap());
    assert!(unchanged.curve == *source);
    assert_eq!(unchanged.location, original_location);
    assert_eq!(unchanged.tolerance, 0.125);
}

fn assert_generated_pcurve_geometry(
    modification: &ShapeCustomConvertToBSpline,
    edge: &Edge,
    face: &Face,
) {
    let mut pcurve = Handle::default();
    let mut tolerance = 0.;
    assert!(
        modification
            .new_curve2d(edge, face, edge, face, &mut pcurve, &mut tolerance)
            .unwrap()
    );
    let surface = BRepTool::surface(face).unwrap();
    for parameter in [0., 0.25, 0.5, 1.] {
        let uv = pcurve.get().unwrap().value(parameter).unwrap();
        let actual = surface.get().unwrap().value(uv.x(), uv.y()).unwrap();
        assert!(actual.distance(&Point3::new(parameter, 0., 0.)) < 1e-14);
    }
    assert_eq!(tolerance, BRepTool::edge_tolerance(edge).unwrap());
    assert!(!BRepTool::curve_on_face(edge, face).unwrap().is_stored);
}
