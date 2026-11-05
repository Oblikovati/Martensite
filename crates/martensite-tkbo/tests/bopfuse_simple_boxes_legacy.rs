//! Regressions of self-contained box fuse regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn fused_boxes(origin: [f64; 3], size: [f64; 3], expected_area: f64) {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let second = CreateBox(
        Point3::new(origin[0], origin[1], origin[2]),
        size[0],
        size[1],
        size[2],
    );
    ValidateResult(&PerformFuse(&first, &second), expected_area, -1., false);
}

fn fused_rotated_boxes(origin: [f64; 3], size: [f64; 3], pivot: [f64; 3], angle: f64, area: f64) {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let mut second = CreateBox(
        Point3::new(origin[0], origin[1], origin[2]),
        size[0],
        size[1],
        size[2],
    );
    RotateAboutZ(
        &mut second,
        Point3::new(pivot[0], pivot[1], pivot[2]),
        angle,
    );
    ValidateResult(&PerformFuse(&first, &second), area, -1., false);
}

macro_rules! box_fuse_case {
    ($name:ident, $origin:expr, $size:expr, $area:expr) => {
        #[test]
        fn $name() {
            fused_boxes($origin, $size, $area);
        }
    };
}
macro_rules! rotated_box_fuse_case {
    ($name:ident, $origin:expr, $size:expr, $pivot:expr, $angle:expr, $area:expr) => {
        #[test]
        fn $name() {
            fused_rotated_boxes($origin, $size, $pivot, $angle, $area);
        }
    };
}

#[path = "bopfuse_simple_boxes_legacy/a_to_f.rs"]
mod a_to_f;
#[path = "bopfuse_simple_boxes_legacy/g_to_l.rs"]
mod g_to_l;
#[path = "bopfuse_simple_boxes_legacy/m_to_r.rs"]
mod m_to_r;
#[path = "bopfuse_simple_boxes_legacy/s_to_x.rs"]
mod s_to_x;
#[path = "bopfuse_simple_boxes_legacy/unmapped_cylinders.rs"]
mod unmapped_cylinders;
#[path = "bopfuse_simple_boxes_legacy/y_to_z.rs"]
mod y_to_z;
#[path = "bopfuse_simple_boxes_legacy/zc_to_zj.rs"]
mod zc_to_zj;
#[path = "bopfuse_simple_boxes_legacy/zh_to_zp.rs"]
mod zh_to_zp;
#[path = "bopfuse_simple_boxes_legacy/zq_to_zx.rs"]
mod zq_to_zx;
