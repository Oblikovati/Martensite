//! Native replay of analytic volume-maker regressions.
#![allow(non_snake_case, unused_imports)]

#[path = "bop_algo_maker_volume_support.rs"]
mod support;

use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction3, Point3};
use support::{expect_shape_count, expect_surface, make_volume, plane_face, sphere_face};

#[test]
fn VolumeMaker__C9() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(0., -875., -1.4432899320127035e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1084.71007164, -980., -2128.6796868400002),
            Direction3::new(1.110223024625157e-16, -1., 1.1102230246251563e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-308.43155740558768, -800., -2407.1041518310276),
            Direction3::new(
                0.80901699437494767,
                9.2444637330587307e-33,
                -0.5877852522924728,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-381.4805392043022, -800., -2409.117597419774),
            Direction3::new(
                -0.15639998811787861,
                -1.5422229525403152e-32,
                -0.98769380058635958,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -1.0103029524088925e-13),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            2440.,
        ),
        plane_face(
            Point3::new(-309.45754782652125, -800., -2369.8806246344966),
            Direction3::new(
                0.98769380058635958,
                -4.4408920985006262e-16,
                -0.15639998811787881,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-330.11602392243287, -800., -2299.7847255135234),
            Direction3::new(
                -0.58778525229247269,
                -1.6653345369377348e-16,
                -0.80901699437494767,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -2.0206059048177849e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            2340.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 4.46244e7);
}

#[test]
fn VolumeMaker__D1() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(0., -870., -1.3877787807814457e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1084.71007164, -800., -2202.7571169349999),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, -1.).unwrap(),
        ),
        plane_face(
            Point3::new(1107.4195555676831, -800.00000000000159, -2173.2457127376779),
            Direction3::new(
                0.45402160145796139,
                -6.6613381477509383e-16,
                -0.89099067638755247,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -8.8817841970012523e-14),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            2440.,
        ),
        plane_face(
            Point3::new(1026.7956776490237, -800., -2158.1910908915443),
            Direction3::new(
                -0.89099067638755203,
                -4.4408920985006247e-16,
                -0.45402160145796222,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -875., -1.4432899320127035e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1004.0861936399999, -800., -2128.6796868400002),
            Direction3::new(1., 1.1102230246251568e-16, -2.2204460492503131e-16).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 1.561806239891439e-13),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            2365.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 3.77486e7);
}

#[test]
fn VolumeMaker__D2() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(0., -232.5, 3.8857805861880479e-15),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-755.97177315543559, -267.5000000000008, 1577.8348070102672),
            Direction3::new(
                -0.43208574880198153,
                -4.4408920985006262e-16,
                0.90183252640511413,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(
                -319.93542909460012,
                1.5603261213365471e-30,
                1669.6231077529578,
            ),
            Direction3::new(
                -0.98213123972955296,
                -3.0594629635162616e-32,
                -0.18819731121164149,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -229.5, 4.2188474935755949e-15),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-734.36208166999995, -267.5, 1594.2540655599998),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 2.9226621123257246e-14),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 1.93904e7);
}

#[test]
fn VolumeMaker__D3() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-2591.6564295634707, -1492.5, -1185.6605612884966),
            Direction3::new(
                0.46174861323503397,
                1.1102230246251565e-16,
                -0.88701083317822171,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-2755.6429359899998, -1492.5, -878.10963635499979),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(0., -1695., -1.3877787807814457e-14),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-2168.52484422, -1617.5000000000002, -5.7395311755081831e-13),
            Direction3::new(1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 1.6042722705833512e-13),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            2950.,
        ),
        plane_face(
            Point3::new(0., -1395., 1.6653345369377348e-14),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-2680.4726796899999, -1492.5, -1314.27713212),
            Direction3::new(1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-2613.5191306921497, -1492.5, -1360.5119453640414),
            Direction3::new(
                0.88701083317822182,
                -1.7180717263929126e-32,
                0.46174861323503402,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -2.0206059048177849e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            2750.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 8.24049e7);
}

#[test]
fn VolumeMaker__D4() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(1665.7013364039071, -825., -339.75605834598957),
            Direction3::new(
                0.15643446504023129,
                4.4408920985006257e-16,
                0.98768834059513766,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -658., -8.3266726846886741e-16),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1726.86651022508, -825.00000000000114, -273.50878573665358),
            Direction3::new(
                0.98768834059513788,
                -6.6613381477509373e-16,
                -0.15643446504022995,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 7.2275518903097691e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(-1677.4339210099999, -825., -347.58541138000004),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(0., -672., -2.3314683517128287e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1616.2687468249999, -825., -265.67943276),
            Direction3::new(1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -1.0788592241794959e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 1.99784e7);
}

#[test]
fn VolumeMaker__D5() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(0., -1450., -4.4408920985006262e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-1236.2999231598978, -1532.4999999999993, 1236.2999231598976),
            Direction3::new(
                -0.70710678118654757,
                3.3306690738754696e-16,
                0.70710678118654746,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 1.5654144647214707e-13),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(-690.68380099000012, -1532.5, 1609.6129695899999),
            Direction3::new(-1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-1200.91009663, -1532.5, 1289.33293185),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(0., -1455., -4.7184478546569153e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-538.51475828369678, -1532.4999999999993, 1657.3780060989745),
            Direction3::new(
                0.30901699437494751,
                -4.4408920985006262e-16,
                -0.95105651629515353,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -1.8139656443594276e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 1.96647e7);
}
