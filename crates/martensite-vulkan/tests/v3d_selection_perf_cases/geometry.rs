//! The authored grid and incremental spiral retain their exact construction arithmetic.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakePolygon;

pub(super) fn spheres() -> Result<Vec<(String, Shape)>, Failure> {
    let mut shapes = Vec::with_capacity(100);
    for i in 0..10 {
        for j in 0..10 {
            let shape = crate::primitive_shapes::sphere(100.)?;
            let offset = [f64::from(i) * 110., -f64::from(j) * 110., 0.];
            shapes.push((
                format!("sph{}", i * 10 + j),
                crate::primitive_shapes::translated(&shape, offset)?,
            ));
        }
    }
    Ok(shapes)
}

pub(super) fn spiral_points() -> Vec<[f64; 3]> {
    let mut z = 0_f64;
    (0..1000)
        .map(|_| {
            let x = 0.2 * z * martensite_cmath::cos(z);
            let y = 0.2 * z * martensite_cmath::sin(z);
            z += 0.3;
            [x, y, z]
        })
        .collect()
}

pub(super) fn spiral_wire(points: &[[f64; 3]]) -> Result<Shape, Failure> {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for &[x, y, z] in points {
        polygon.add_point(Point3::new(x, y, z))?;
    }
    Ok(polygon.wire()?.into_shape())
}

#[test]
fn spiral_keeps_the_source_increment_before_appending_z() {
    let points = spiral_points();
    assert_eq!(points.len(), 1000);
    assert_eq!(points[0], [0., 0., 0.3]);
    assert_eq!(
        points[1],
        [
            0.2 * 0.3 * martensite_cmath::cos(0.3_f64),
            0.2 * 0.3 * martensite_cmath::sin(0.3_f64),
            0.6
        ]
    );
    assert!(points[999][2] > 299.999999999);
}

#[test]
fn sphere_grid_builds_every_authored_named_solid() {
    let shapes = spheres().unwrap();
    assert_eq!(shapes.len(), 100);
    for (index, (name, shape)) in shapes.iter().enumerate() {
        assert_eq!(name, &format!("sph{index}"));
        assert_eq!(
            shape.shape_type().unwrap(),
            martensite_tkg3d::top_abs::ShapeEnum::Solid
        );
        crate::shape_checks::check_shape_valid(shape).unwrap();
    }
}
