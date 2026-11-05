//! Native replay of the GDML bracket construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateBox, CreateCylinder, PerformCut, PerformFuse, RotateAroundAxis, Translate,
};
use martensite_tkmath::gp::Point3;

const ASSEMBLY_AXIS: [f64; 3] = [
    -0.57735026918962573,
    0.57735026918962573,
    0.57735026918962584,
];

fn common_placement(
    mut shape: martensite_tkbrep::topods::Shape,
) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), ASSEMBLY_AXIS, 120.);
    Translate(&shape, -127., 76.2, 0.)
}

fn corner_box(x: f64, axis: [f64; 3]) -> martensite_tkbrep::topods::Shape {
    let mut shape = CreateBox(
        Point3::new(-54.959167605, -54.959167605, -79.248),
        109.91833521,
        109.91833521,
        158.496,
    );
    RotateAroundAxis(&mut shape, Point3::default(), axis, 45.);
    common_placement(Translate(&shape, x, -127.7620001, 0.))
}

fn cylinder_box_cut(x: f64, box_axis: [f64; 3]) -> martensite_tkbrep::topods::Shape {
    let mut box_shape = CreateBox(
        Point3::new(-76.962, -65.691176045, -77.724),
        153.924,
        131.38235209,
        155.448,
    );
    RotateAroundAxis(&mut box_shape, Point3::default(), box_axis, 45.);
    let box_shape = common_placement(Translate(&box_shape, x, -42.44932395, 0.));
    let mut cylinder = CreateCylinder(76.2, 152.4);
    cylinder = Translate(&cylinder, 0., 0., -76.2);
    RotateAroundAxis(&mut cylinder, Point3::default(), [0., 0., -1.], 90.);
    let cylinder = common_placement(Translate(&cylinder, x.signum() * 76.2, -50.8, 0.));
    PerformCut(&cylinder, &box_shape)
}

#[test]
fn GdmlPublic__C7() {
    let source = "tests/boolean/gdml_public/C7";
    let main = common_placement(CreateBox(
        Point3::new(-152.4000001, -127.0000001, -76.2),
        304.8000002,
        254.0000002,
        152.4,
    ));
    let first_cut = PerformCut(&main, &corner_box(-153.1620001, [0., 0., -1.]));
    assert!(!first_cut.is_null(), "{source}: first box cut");
    let body = PerformCut(&first_cut, &corner_box(153.1620001, [0., 0., 1.]));
    assert!(!body.is_null(), "{source}: second box cut");
    let first_feature = cylinder_box_cut(67.84932395, [0., 0., 1.]);
    assert!(!first_feature.is_null(), "{source}: first cylinder cut");
    let second_feature = cylinder_box_cut(-67.84932395, [0., 0., -1.]);
    assert!(!second_feature.is_null(), "{source}: second cylinder cut");
    let fused = PerformFuse(&body, &first_feature);
    assert!(!fused.is_null(), "{source}: first fuse");
    let result = PerformFuse(&fused, &second_feature);
    assert!(!result.is_null(), "{source}: second fuse");
}
