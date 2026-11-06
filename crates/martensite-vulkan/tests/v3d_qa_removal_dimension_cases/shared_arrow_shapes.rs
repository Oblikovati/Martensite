//! Literal vertices and circle edges preserve the QA constructor overloads.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};

pub(super) fn vertices() -> Result<Vec<Shape>, Failure> {
    let points = [
        [602.51, 50., 0.],
        [602.51, 200., 0.],
        [102.51, 200., 0.],
        [102.51, 170., 0.],
        [502.51, 170., 0.],
        [502.51, 80., 0.],
        [102.51, 80., 0.],
        [102.51, 50., 0.],
    ];
    points
        .into_iter()
        .map(|[x, y, z]| {
            Ok(BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?
                .vertex()?
                .into_shape())
        })
        .collect()
}

pub(super) fn first_radius_edge() -> Result<Shape, Failure> {
    let circle = circle([191.09, -88.58, 0.], 1., 88.58)?;
    let edge = BRepBuilderAPIMakeEdge::from_circ_points(
        circle,
        Point3::new(191.09, 0., 0.),
        Point3::new(191.09, -177.16, 0.),
    )?
    .edge()?;
    Ok(edge.into_shape())
}

pub(super) fn complete_radius_edge(
    center: [f64; 3],
    normal_z: f64,
    radius: f64,
) -> Result<Shape, Failure> {
    Ok(
        BRepBuilderAPIMakeEdge::from_circ(circle(center, normal_z, radius)?)?
            .edge()?
            .into_shape(),
    )
}

fn circle([x, y, z]: [f64; 3], normal_z: f64, radius: f64) -> Result<Circle3, Failure> {
    Circle3::new(
        Axis2::new(Point3::new(x, y, z), Direction3::new(0., 0., normal_z)?)?,
        radius,
    )
}
