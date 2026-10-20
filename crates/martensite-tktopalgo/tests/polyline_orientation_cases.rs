//! Original closed-polyline face orientation regressions, with the command builders.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Shape, Wire};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeWire,
};

fn original_polyline(points: &[Point3; 4]) -> Result<Wire, Failure> {
    let mut maker = BRepBuilderAPIMakePolygon::new();
    for point in points {
        maker.add_point(*point)?;
    }
    assert!(maker.is_done());
    maker.wire()
}

fn command_wire_from_polygon(polygon: &Wire) -> Result<Wire, Failure> {
    let mut maker = BRepBuilderAPIMakeWire::new();
    maker.add_wire(polygon)?;
    assert!(maker.is_done());
    maker.wire()
}

fn assert_original_orientations(points: &[Point3; 4]) -> Result<(), Failure> {
    let polygon = original_polyline(points)?;
    let wire = command_wire_from_polygon(&polygon)?;
    let face_maker = BRepBuilderAPIMakeFace::from_wire(&wire, false)?;
    assert!(face_maker.is_done());
    let face = face_maker.face()?;
    for shape in [
        polygon.into_shape(),
        wire.into_shape(),
        face.clone().into_shape(),
    ] {
        assert_eq!(shape.orientation(), Orientation::Forward);
    }
    assert_exploded_orientations(&face)?;
    Ok(())
}

fn assert_exploded_orientations(face: &Shape) -> Result<(), Failure> {
    for kind in [ShapeEnum::Wire, ShapeEnum::Edge] {
        for shape in Explorer::new(face, kind, ShapeEnum::Shape)? {
            assert_eq!(shape.orientation(), Orientation::Forward);
        }
    }
    Ok(())
}

#[test]
fn bug24954_1_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_1_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_1_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

fn bug24954_1_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(9.08, 8.62, 3.75),
        Point3::new(1.87, 3.04, 9.0),
        Point3::new(4.26, 1.37, -0.08),
        Point3::new(9.08, 8.62, 3.75),
    ])
}

#[test]
fn bug24954_2_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_2_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_2_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

#[allow(clippy::approx_constant)] // Original coordinates include literal 3.14, not pi.
fn bug24954_2_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(-1.15, 3.42, 1.95),
        Point3::new(4.54, 3.13, 1.36),
        Point3::new(2.97, -3.14, 2.08),
        Point3::new(-1.15, 3.42, 1.95),
    ])
}

#[test]
fn bug24954_3_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_3_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_3_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

fn bug24954_3_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(-4.62, 0.1, 0.75),
        Point3::new(-4.12, 4.64, -2.91),
        Point3::new(-1.55, -1.07, 4.48),
        Point3::new(-4.62, 0.1, 0.75),
    ])
}

#[test]
fn bug24954_4_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_4_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_4_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

#[allow(clippy::approx_constant)] // Original coordinates include literal 3.14, not pi.
fn bug24954_4_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(-2.92, -4.7, -2.87),
        Point3::new(4.73, 1.75, 1.76),
        Point3::new(3.14, -1.23, 2.74),
        Point3::new(-2.92, -4.7, -2.87),
    ])
}

#[test]
fn bug24954_5_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_5_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_5_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

fn bug24954_5_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(-4.05, 3.84, 0.17),
        Point3::new(4.22, 3.55, 3.19),
        Point3::new(-1.32, -1.44, -1.57),
        Point3::new(-4.05, 3.84, 0.17),
    ])
}

#[test]
fn bug24954_6_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_6_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_6_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

fn bug24954_6_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(-2.36, -1.62, 0.17),
        Point3::new(3.26, -1.05, -3.8),
        Point3::new(2.75, -4.01, 4.95),
        Point3::new(-2.36, -1.62, 0.17),
    ])
}

#[test]
fn bug24954_7_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_7_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_7_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

fn bug24954_7_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(4.72, 2.03, -2.52),
        Point3::new(-4.78, 1.89, -3.82),
        Point3::new(-1.42, 4.56, 1.55),
        Point3::new(4.72, 2.03, -2.52),
    ])
}

#[test]
fn bug24954_8_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_8_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_8_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

fn bug24954_8_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(2.02, -1.4, -3.75),
        Point3::new(-2.93, -1.87, 1.72),
        Point3::new(-5.0, -0.18, 0.39),
        Point3::new(2.02, -1.4, -3.75),
    ])
}

#[test]
fn bug24954_9_original_polyline_face_keeps_forward_orientation() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "polyline_orientation_cases::bug24954_9_original_polyline_face_keeps_forward_orientation",
        || Ok(bug24954_9_original_polyline_face_keeps_forward_orientation_body()?),
    );
    Ok(())
}

fn bug24954_9_original_polyline_face_keeps_forward_orientation_body() -> Result<(), Failure> {
    assert_original_orientations(&[
        Point3::new(1.21, 0.16, 4.2),
        Point3::new(3.12, 4.16, 0.93),
        Point3::new(-2.33, -2.55, 2.41),
        Point3::new(1.21, 0.16, 4.2),
    ])
}
