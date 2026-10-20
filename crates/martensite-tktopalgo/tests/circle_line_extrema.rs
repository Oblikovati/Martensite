//! Original twice-mirrored circle/line extrema and dumped-normal assertions.
#[path = "common/geometry_io.rs"]
mod geometry_io;
use geometry_io::stream_bytes;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomLine, GeomTrimmedCurve};
use martensite_tkgeomalgo::geom_api::ExtremaCurveCurve;
use martensite_tkgeombase::geom_tools;
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Transform3, Vector3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPITransform,
};

fn original_twice_mirrored_circle() -> Result<Handle<dyn GeomCurve>, Failure> {
    let frame = Axis2::new(Point3::new(5., 5., 10.), Direction3::new(0., 1., 1.)?)?;
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomCircle::new(frame, 20.)?).cast();
    let mut shape = BRepBuilderAPIMakeEdge::from_curve(&curve)?
        .edge()?
        .into_shape();
    let mut reflection = Transform3::default();
    reflection.set_mirror_plane(Axis2::new(
        Point3::new(5., 5., 10.),
        Direction3::new(1., 0., 0.)?,
    )?);
    let mut transform = BRepBuilderAPITransform::new(reflection);
    for _ in 0..2 {
        transform.perform(&shape, false, false)?;
        shape = transform.shape()?.clone();
    }
    extracted_trimmed_curve(&topods::edge(&shape)?)
}

fn extracted_trimmed_curve(edge: &topods::Edge) -> Result<Handle<dyn GeomCurve>, Failure> {
    let found = BRepTool::curve_with_location(edge)?;
    let trimmed: Handle<dyn GeomCurve> = Handle::new(GeomTrimmedCurve::new(
        &found.curve,
        found.first,
        found.last,
        true,
        true,
    )?)
    .cast();
    Ok(trimmed.transformed(found.location.transformation())?.cast())
}

fn vector_between(first: Point3, last: Point3) -> Vector3 {
    Vector3::new(
        last.x() - first.x(),
        last.y() - first.y(),
        last.z() - first.z(),
    )
}

fn original_offset_chord_line(
    circle: &Handle<dyn GeomCurve>,
) -> Result<(Handle<dyn GeomCurve>, Vector3), Failure> {
    let first = circle.d0(0.63)?;
    let last = circle.d0(5.47)?;
    let chord = vector_between(first, last);
    let origin = Point3::new(first.x(), first.y() + 5., first.z() + 5.);
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
        origin,
        Direction3::from_vector(chord)?,
    ))
    .cast();
    Ok((line, chord))
}

fn connecting_segment(first: Point3, last: Point3) -> Result<Handle<dyn GeomCurve>, Failure> {
    let direction = Direction3::from_vector(vector_between(first, last))?;
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(first, direction)).cast();
    Ok(Handle::new(GeomTrimmedCurve::new(
        &line,
        0.,
        first.distance(&last),
        true,
        true,
    )?)
    .cast())
}

fn dumped_normal(first: Point3, last: Point3) -> Result<Vector3, Failure> {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    geom_tools::dump_curve(&connecting_segment(first, last)?, &mut stream)?;
    let output = String::from_utf8(stream_bytes(stream)).unwrap();
    let axis = output
        .lines()
        .find_map(|line| line.strip_prefix("  Axis   :"))
        .unwrap();
    let values: Vec<f64> = axis
        .split(',')
        .map(|value| value.trim().parse().unwrap())
        .collect();
    Ok(Vector3::new(values[0], values[1], values[2]))
}

#[test]
fn bug28394_2_all_four_original_extrema_have_dumped_normal_directions() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "circle_line_extrema::bug28394_2_all_four_original_extrema_have_dumped_normal_directions",
        || Ok(bug28394_2_all_four_original_extrema_have_dumped_normal_directions_body()?),
    );
    Ok(())
}

fn bug28394_2_all_four_original_extrema_have_dumped_normal_directions_body() -> Result<(), Failure>
{
    let circle = original_twice_mirrored_circle()?;
    let (line, chord) = original_offset_chord_line(&circle)?;
    let extrema = ExtremaCurveCurve::with_ranges(
        &circle,
        &line,
        (circle.first_parameter(), circle.last_parameter()),
        (line.first_parameter(), line.last_parameter()),
    )?;
    assert!(!extrema.extrema()?.is_parallel()?);
    assert_eq!(extrema.nb_extrema(), 4);
    for index in 1..=4 {
        assert_original_normal(&circle, chord, &extrema, index)?;
    }
    Ok(())
}

fn assert_original_normal(
    circle: &Handle<dyn GeomCurve>,
    chord: Vector3,
    extrema: &ExtremaCurveCurve,
    index: i32,
) -> Result<(), Failure> {
    let (first, last) = extrema.points(index)?;
    assert!(first.distance(&last) >= 1e-16);
    let normal = dumped_normal(first, last)?;
    let tangent = circle.d1(extrema.parameters(index)?.0)?.1;
    let line_dot = normal.dot(chord);
    let circle_dot = normal.dot(tangent);
    assert!(
        line_dot.abs() <= 1e-12 && circle_dot.abs() <= 1e-12,
        "extrema {index} is not normal: line dot {line_dot}, circle dot {circle_dot}"
    );
    Ok(())
}
