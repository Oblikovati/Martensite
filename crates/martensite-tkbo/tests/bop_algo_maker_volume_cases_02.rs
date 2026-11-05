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
fn VolumeMaker__D6() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-1677.4339210099999, -825., 347.58541137999993),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(0., -672., -2.3314683517128287e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1726.8665102250791, -825., 273.50878573665614),
            Direction3::new(
                0.98768834059513766,
                -2.1722168295619746e-32,
                0.15643446504023142,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 7.2275518903097691e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(1665.7013364039071, -825., 339.75605834598957),
            Direction3::new(
                -0.15643446504023129,
                4.4408920985006257e-16,
                0.98768834059513766,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -691., -3.3861802251067274e-15),
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
    expect_surface(&make_volume(inputs), 2.0232e7);
}

#[test]
fn VolumeMaker__D7() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(1441.7792892394632, -1532.5, 900.69424230424909),
            Direction3::new(
                -0.6072926454886024,
                -2.2204460492503121e-16,
                0.79447822042863747,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(1344.61246598, -1532.5, 870.19410379999999),
            Direction3::new(-2.2204460492503131e-16, 1.1102230246251565e-16, -1.).unwrap(),
        ),
        plane_face(
            Point3::new(0., -1455., -4.7184478546569153e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 1.5654144647214707e-13),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(1481.6804735199999, -1532.5, 1027.81075758),
            Direction3::new(-1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., -1497., -8.3266726846886741e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1384.513650145994, -1532.5, 1058.3108959980543),
            Direction3::new(
                -0.7944782204286378,
                9.7927001272767777e-33,
                -0.60729264548860207,
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
    assert!(!make_volume(inputs).is_null());
}

#[test]
fn VolumeMaker__D8() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-335.23319463083521, 698.25, 1031.741684575172),
            Direction3::new(
                0.95105651629515375,
                3.3306690738754676e-16,
                0.30901699437494723,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-361.48545975741939, 698.25, 1112.3677894628579),
            Direction3::new(
                0.94413350938426932,
                -2.7755575615628914e-17,
                0.32956322072971667,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-377.03714595623018, -18.500000000000256, 1080.1369248570181),
            Direction3::new(
                -0.32956322072971639,
                -2.2204460492503131e-16,
                0.94413350938426943,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-379.81702109239041, -18.500000000000014, 1016.6899885855038),
            Direction3::new(
                0.93679437761793805,
                -2.2204460492503121e-16,
                0.34988039965596873,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-409.2680104425807, 698.25, 1095.6886109971422),
            Direction3::new(
                0.94413350938426932,
                -2.7755575615628914e-17,
                0.32956322072971667,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-382.41124430916489, 698.25, 1016.4126070048281),
            Direction3::new(
                0.95105651629515375,
                3.3306690738754676e-16,
                0.30901699437494723,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-863.07307891000005, 156.50000000000011, -657.71745162000002),
            Direction3::new(4.4909464723405365e-17, 1., 3.2628636031510525e-17).unwrap(),
        ),
        plane_face(
            Point3::new(-415.13750551487595, 698.25, 939.25789966596847),
            Direction3::new(
                -0.34988039965596873,
                5.5511151231257827e-16,
                0.93679437761793805,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-340.44108716728596, 698.25, 967.50583750729675),
            Direction3::new(
                -0.30901699437494723,
                -3.3306690738754691e-16,
                0.95105651629515375,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-377.03714596623058, 698.24999999999989, 1080.1369248570184),
            Direction3::new(
                0.32956322072971667,
                1.1102230246251565e-16,
                -0.94413350938426932,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., 1240., 6.0146332359067856e-14),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 2.0539125955565396e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1026.3199999999999,
        ),
    ]
    .into_iter()
    .collect();
    let result = make_volume(inputs);
    expect_surface(&result, 6.84902e7);
    for (kind, expected) in [
        (ShapeEnum::Wire, 129),
        (ShapeEnum::Face, 129),
        (ShapeEnum::Shell, 31),
        (ShapeEnum::Solid, 31),
    ] {
        expect_shape_count(&result, kind, expected);
    }
}

#[test]
fn VolumeMaker__D9() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(0., 825., 4.163336342344337e-15),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-771.03174431000002, 820., -1434.59193091),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(-793.75341161103199, 820.00000000000023, -1557.828784412869),
            Direction3::new(
                -0.45399049973954675,
                1.1102230246251565e-16,
                -0.8910065241883679,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -9.1038288019262836e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(-837.85723364880118, 820., -1479.1857140209454),
            Direction3::new(
                -0.89100652418836812,
                -2.2204460492503136e-16,
                0.45399049973954653,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-860.57890103, 820., -1513.23500145),
            Direction3::new(1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., 920., 1.1102230246251565e-14),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 9.9142916099026479e-14),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 2.40251e7);
}

#[test]
fn VolumeMaker__E1() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(
                -306.53078964627537,
                -1038.2137499999999,
                -251.37646071372467,
            ),
            Direction3::new(
                -0.70710678118654746,
                4.4408920985006262e-16,
                0.70710678118654768,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(
                -27.577164468182303,
                -1038.2137499999999,
                -27.577164468182389,
            ),
            Direction3::new(
                0.70710678118654768,
                2.2204460492503131e-16,
                0.70710678118654746,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 7.5439654523279387e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            260.,
        ),
        plane_face(
            Point3::new(
                -251.37646071372464,
                -1038.2137499999999,
                -306.53078964627542,
            ),
            Direction3::new(
                -0.70710678118654746,
                4.4408920985006262e-16,
                0.70710678118654768,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -630.00000000193427, 1.275057837093401e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., -729., 1.3849699165291441e-13),
            Direction3::new(0., 1., -1.1102230246251565e-16).unwrap(),
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 891858.);
}

#[test]
fn VolumeMaker__E2() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(973.57839899499993, -780., 1417.99061547),
            Direction3::new(1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(
            Point3::new(965.23713319219564, -780.00000000000034, 1459.6976661987558),
            Direction3::new(
                0.55157308145357831,
                -2.2204460492503123e-16,
                0.83412657062091267,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 7.2275518903097691e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(945.99926726850424, -780., 1412.4748846758775),
            Direction3::new(
                0.83412657062091222,
                2.2204460492503131e-16,
                -0.55157308145357886,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -770., 1.1102230246251565e-15),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., -777., -1.7763568394002505e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(937.65800154999999, -780., 1370.7678338199999),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -1.0788592241794959e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 1.95938e7);
}
