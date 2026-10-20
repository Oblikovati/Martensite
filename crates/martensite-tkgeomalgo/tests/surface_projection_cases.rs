//! Whole original bug27762_2 and bug24081_1/_2, source
//!; default Grad/full surface bounds.
#[path = "common/isolated_case.rs"]
mod isolated_case;
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::{
    GeomCurve, GeomEllipse, GeomSurface, GeomSurfaceOfRevolution, GeomToroidalSurface,
};
use martensite_tkgeomalgo::geom_api::ProjectPointOnSurf;
use martensite_tkgeombase::extrema::ExtAlgo;
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, Point3};
use std::fmt::Write;
#[path = "support/curve_segment.rs"]
mod curve_segment;

#[derive(Default)]
struct ProjectionPublication {
    log: String,
    points: Vec<Point3>,
    segments: Vec<Handle<dyn GeomCurve>>,
}

fn surface_projection_output(
    surface: &Handle<dyn GeomSurface>,
    query: Point3,
) -> ProjectionPublication {
    let projection =
        ProjectPointOnSurf::with_bounds(query, surface, surface.bounds(), ExtAlgo::Grad).unwrap();
    // tests/parse.rules rejects the command's otherwise successful "projection failed." text.
    assert!(
        projection.is_done(),
        "projection failed. query={query:?}; expected completed surface projection"
    );
    let mut output = ProjectionPublication::default();
    for index in 1..=projection.nb_points().unwrap() {
        let point = projection.point(index).unwrap();
        let parameters = projection.parameters(index).unwrap();
        append_projection_solution(&mut output, index, query, point, parameters);
    }
    output
}

fn append_projection_solution(
    output: &mut ProjectionPublication,
    index: i32,
    query: Point3,
    point: Point3,
    parameters: (f64, f64),
) {
    write!(output.log, "ext_{index} ").unwrap();
    if query.distance(&point) > precision::confusion() {
        output
            .segments
            .push(curve_segment::extrema_segment(query, point));
    } else {
        output.points.push(point);
        output.log.push_str(" Point on surface ");
    }
    writeln!(output.log, " Parameters: {} {}", parameters.0, parameters.1).unwrap();
}

fn revolution_surface(center_x: f64) -> Handle<dyn GeomSurface> {
    let normal = Direction3::new(0.0, -1.0, 0.0).unwrap();
    let frame =
        Axis2::with_x_direction(Point3::new(center_x, 0.0, 0.0), normal, Direction3::Z).unwrap();
    let ellipse: Handle<dyn GeomCurve> =
        Handle::new(GeomEllipse::new(frame, 5.0, 2.0).unwrap()).cast();
    let axis = Axis1::new(Point3::default(), Direction3::Z);
    Handle::new(GeomSurfaceOfRevolution::new(&ellipse, axis).unwrap()).cast()
}

#[test]
fn bug27762_2_original_zero_major_torus_four_named_projections() {
    run_projection_case(
        "surface_projection_cases::bug27762_2_original_zero_major_torus_four_named_projections",
        bug27762_2_original_zero_major_torus_four_named_projections_body,
    );
}

fn bug27762_2_original_zero_major_torus_four_named_projections_body() {
    let frame = Axis3::new(Point3::default(), Direction3::Z, Direction3::X).unwrap();
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomToroidalSurface::new(frame, 0.0, 10.0).unwrap()).cast();
    let output = surface_projection_output(&surface, Point3::new(1.0, 0.0, 0.0));
    let matches = output
        .log
        .split_whitespace()
        .filter(|word| word.starts_with("ext_"))
        .count();
    assert_eq!(matches, 4, "original lmatch ext_*: {}", output.log);
}

#[test]
fn bug24081_1_original_ellipse_revolution_projection() {
    run_projection_case(
        "surface_projection_cases::bug24081_1_original_ellipse_revolution_projection",
        bug24081_1_original_ellipse_revolution_projection_body,
    );
}

fn bug24081_1_original_ellipse_revolution_projection_body() {
    let surface = revolution_surface(10.0);
    let query = Point3::new(7.9, 0.0, 0.0);
    let _publication = surface_projection_output(&surface, query);
}

#[test]
fn bug24081_2_original_axis_crossing_ellipse_revolution_projection() {
    run_projection_case(
        "surface_projection_cases::bug24081_2_original_axis_crossing_ellipse_revolution_projection",
        bug24081_2_original_axis_crossing_ellipse_revolution_projection_body,
    );
}

fn bug24081_2_original_axis_crossing_ellipse_revolution_projection_body() {
    let surface = revolution_surface(2.1);
    let query = Point3::new(0.01, 0.0, 0.0);
    let _publication = surface_projection_output(&surface, query);
}

#[test]
fn projection_publication_retains_point_at_confusion_boundary() {
    let mut output = ProjectionPublication::default();
    let point = Point3::new(precision::confusion(), 0.0, 0.0);
    append_projection_solution(&mut output, 1, Point3::default(), point, (2.0, 3.0));
    assert_eq!(output.points, [point]);
    assert!(output.segments.is_empty());
    assert_eq!(output.log, "ext_1  Point on surface  Parameters: 2 3\n");
}

#[test]
fn projection_publication_retains_segment_above_confusion_boundary() {
    let distance = f64::from_bits(precision::confusion().to_bits() + 1);
    let point = Point3::new(distance, 0.0, 0.0);
    let mut output = ProjectionPublication::default();
    append_projection_solution(&mut output, 2, Point3::default(), point, (4.0, 5.0));
    assert!(output.points.is_empty());
    assert_eq!(output.segments.len(), 1);
    assert_eq!(output.segments[0].value(0.0).unwrap(), Point3::default());
    assert_eq!(output.segments[0].value(distance).unwrap(), point);
    assert_eq!(output.log, "ext_2  Parameters: 4 5\n");
}

// bugs/begin installs CPU/elapsed1000 before any surface or projection fixture.
fn run_projection_case(identity: &str, body: fn()) {
    isolated_case::run_isolated(identity, 1000, || {
        body();
        println!("TEST COMPLETED");
    });
}
