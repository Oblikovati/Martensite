//! Native replay of the GDML keyhole construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateBox, CreateCylinder, PerformCut, PerformFuse, RotateAroundAxis, Translate,
};
use martensite_tkmath::gp::Point3;

const FINAL_AXIS: [f64; 3] = [
    -0.70710678118654746,
    -4.3297802811774677e-17,
    0.70710678118654757,
];

fn place(mut shape: martensite_tkbrep::topods::Shape) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), FINAL_AXIS, 180.);
    Translate(&shape, -0.75, 0.00016331, -20.)
}

fn rotated_box(
    origin: Point3,
    size: [f64; 3],
    angle: f64,
    translation: [f64; 3],
) -> martensite_tkbrep::topods::Shape {
    let mut shape = CreateBox(origin, size[0], size[1], size[2]);
    RotateAroundAxis(&mut shape, Point3::default(), [0., 0., 1.], angle);
    place(Translate(
        &shape,
        translation[0],
        translation[1],
        translation[2],
    ))
}

fn placed_cylinder(radius: f64, translation: [f64; 3]) -> martensite_tkbrep::topods::Shape {
    let mut shape = Translate(&CreateCylinder(radius, 1.5), 0., 0., -0.75);
    RotateAroundAxis(&mut shape, Point3::default(), [0., 0., 1.], -180.);
    place(Translate(
        &shape,
        translation[0],
        translation[1],
        translation[2],
    ))
}

fn circle_strip(
    radius: f64,
    box_origin: Point3,
    box_size: [f64; 3],
    box_angle: f64,
    circle_translation: [f64; 3],
    box_translation: [f64; 3],
) -> martensite_tkbrep::topods::Shape {
    let circle = placed_cylinder(radius, circle_translation);
    let strip = rotated_box(box_origin, box_size, box_angle, box_translation);
    PerformCut(&circle, &strip)
}

#[test]
fn GdmlPublic__B4() {
    let source = "tests/boolean/gdml_public/B4";
    let base = place(CreateBox(
        Point3::new(-19.79487096, -17.14305166, -0.75),
        39.58974192,
        34.28610332,
        1.5,
    ));
    let first_body_cut = rotated_box(
        Point3::new(-17.667114695, -17.667114695, -0.78),
        [35.33422939, 35.33422939, 1.56],
        -98.21312067,
        [34.8064247, -2.52384605, 0.],
    );
    let second_body_cut = rotated_box(
        Point3::new(-17.667114695, -17.667114695, -0.81),
        [35.33422939, 35.33422939, 1.62],
        -81.78687933,
        [-34.8064247, -2.52384605, 0.],
    );
    let arc_twenty = circle_strip(
        20.,
        Point3::new(-20.2, -8.657158565, -0.765),
        [40.4, 17.31431713, 1.53],
        0.,
        [0., 20.00016331, 0.],
        [0., 8.48589299, 0.],
    );
    let arc_fifteen = circle_strip(
        15.,
        Point3::new(-15.15, -8.657131075, -0.765),
        [30.3, 17.31426215, 1.53],
        -180.,
        [0., -15.00021775, 0.],
        [0., -8.48592048, 0.],
    );

    let key_base = place(Translate(
        &CreateBox(
            Point3::new(-7.5000001, -13.0050001, -0.75),
            15.0000002,
            26.0100002,
            1.5,
        ),
        0.,
        -5.99521775,
        0.,
    ));
    let key_first_cut = rotated_box(
        Point3::new(-2.88499581, -2.88499581, -0.78),
        [5.76999162, 5.76999162, 1.56],
        -135.,
        [7.5400001, -19.04021785, 0.],
    );
    let key_second_cut = rotated_box(
        Point3::new(-2.88499581, -2.88499581, -0.81),
        [5.76999162, 5.76999162, 1.62],
        -45.,
        [-7.5400001, -19.04021785, 0.],
    );
    let key_body = PerformCut(&PerformCut(&key_base, &key_first_cut), &key_second_cut);
    assert!(!key_body.is_null(), "{source}: key body cuts");

    let middle_arc = circle_strip(
        15.,
        Point3::new(-15.15, -1.014857565, -0.765),
        [30.3, 2.02971513, 1.53],
        0.,
        [0., 20.00016331, 0.],
        [0., 5.99492469, 0.],
    );
    let left_arc = circle_strip(
        4.,
        Point3::new(-4.04, -3.4483557, -0.765),
        [8.08, 6.8967114, 1.53],
        135.,
        [-3.5, -15.00021775, 0.],
        [-3.0616443, -14.56186205, 0.],
    );
    let right_arc = circle_strip(
        4.,
        Point3::new(-4.04, -3.4483557, -0.765),
        [8.08, 6.8967114, 1.53],
        -135.,
        [3.5, -15.00021775, 0.],
        [3.0616443, -14.56186205, 0.],
    );
    let key_with_arcs = PerformFuse(
        &PerformFuse(&PerformFuse(&key_body, &middle_arc), &left_arc),
        &right_arc,
    );
    assert!(!key_with_arcs.is_null(), "{source}: key feature fuses");

    let body = PerformCut(&PerformCut(&base, &first_body_cut), &second_body_cut);
    assert!(!body.is_null(), "{source}: body cuts");
    let body = PerformFuse(&PerformFuse(&body, &arc_twenty), &arc_fifteen);
    assert!(!body.is_null(), "{source}: body feature fuses");
    let body = PerformCut(&body, &key_with_arcs);
    assert!(!body.is_null(), "{source}: keyhole cut");
    let first_hole = placed_cylinder(2., [12.5, 0.00016331, 0.]);
    let second_hole = placed_cylinder(2., [-12.5, 0.00016331, 0.]);
    let result = PerformCut(&PerformCut(&body, &first_hole), &second_hole);
    assert!(!result.is_null(), "{source}: paired hole cuts");
}
