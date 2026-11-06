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
fn VolumeMaker__A8() {
    let shared = Direction3::new(
        -0.15037445914067274,
        0.27563735581879711,
        -0.94942696934275683,
    )
    .unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-4.2580937183769834, -506.18339008981781, 147.62942729477928),
            Direction3::new(
                0.98768834059988952,
                -6.6728564553435301e-12,
                -0.15643446501022945,
            )
            .unwrap(),
        ),
        sphere_face(535.),
        plane_face(
            Point3::new(-27.47908454454036, -513.55172659614686, -173.4961116930572),
            shared,
        ),
        plane_face(
            Point3::new(194.60851143, 27.3, 469.82650764),
            Direction3::new(-1.2670433353083948e-16, -1., -6.6853539210366213e-16).unwrap(),
        ),
        plane_face(
            Point3::new(
                -19.268639075459625,
                -528.60152622385317,
                -121.65739916694265,
            ),
            shared,
        ),
        plane_face(
            Point3::new(
                -50.337753508376991,
                -521.07662641018214,
                -143.30609453522072,
            ),
            Direction3::new(
                0.98768834059988952,
                6.6728564553435301e-12,
                -0.15643446501022945,
            )
            .unwrap(),
        ),
        sphere_face(520.),
    ]
    .into_iter()
    .collect();
    let result = make_volume(inputs);
    expect_surface(&result, 1.75676e7);
    for (kind, expected) in [
        (ShapeEnum::Wire, 68),
        (ShapeEnum::Face, 68),
        (ShapeEnum::Shell, 24),
        (ShapeEnum::Solid, 24),
    ] {
        expect_shape_count(&result, kind, expected);
    }
}

#[test]
fn VolumeMaker__A9() {
    use support::sphere_face_at;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-1.9428902930940247e-15, 0., -1.9428902930940236e-15),
            Direction3::new(1.110223024625157e-16, -1., 1.1102230246251563e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-6.6613381477509392e-15, 78.75, 10.00000000000006),
            Direction3::new(-6.6613381477509392e-16, 6.106226635438361e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(10., 78.75, 6.6861338147750938e-14),
            Direction3::new(1., -2.2204460492503128e-16, 6.6613381477509402e-16).unwrap(),
        ),
        plane_face(
            Point3::new(6.6613381477509392e-15, 78.75, -9.9999999999999396),
            Direction3::new(-6.6613381477509392e-16, 6.106226635438361e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(11.15, 75., 6.0171246429124051e-14),
            Direction3::new(-1., 4.4408920985006262e-16, 3.8453235056139087e-17).unwrap(),
        ),
        plane_face(
            Point3::new(3.3306690738754696e-14, 150., 5.0494651785678038e-14),
            Direction3::new(-4.4408920985006262e-16, -1., 1.3473797619095955e-16).unwrap(),
        ),
        sphere_face_at(
            Point3::new(0., 570., 0.),
            Direction3::new(0., -1.2246467991473532e-16, -1.).unwrap(),
            500.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 9.88757e6);
}

#[test]
fn VolumeMaker__B1() {
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(24.324478110342437, 508.53654736, -12.393940642889628),
            Direction3::new(-0.89100652418836779, 0., 0.45399049973954686).unwrap(),
        ),
        sphere_face(520.),
        plane_face(
            Point3::new(-139.11177319288961, 495.97571681, -40.241482479657549),
            Direction3::new(
                0.45399049973954658,
                -1.1228384600540191e-16,
                -0.89100652418836801,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(197.01514608, -27.3, 475.63663767),
            Direction3::new(2.2204460492503131e-16, 1., 2.7755575615628918e-16).unwrap(),
        ),
        plane_face(
            Point3::new(
                497.86372504673375,
                3.6839557552926462e-16,
                147.60224008685657,
            ),
            Direction3::new(
                -0.23344536385590542,
                1.349434342598039e-17,
                0.97236992039767656,
            )
            .unwrap(),
        ),
        sphere_face(510.),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 1.67121e7);
}

#[test]
fn VolumeMaker__B2() {
    use support::sphere_face_at;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(45.874506818569543, -472.21980812485742, 129.82890639137648),
            Direction3::new(
                0.98768834059513788,
                4.0913845243535237e-16,
                -0.15643446504023051,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(24.57793391, -547.91916668, 155.17896747),
            Direction3::new(
                0.04311918230233603,
                -0.96126169593831889,
                0.27224380257292363,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(17.422936824496691, -479.11074202028237, 110.00409378390755),
            Direction3::new(
                -0.15037445916777556,
                -0.27563735581699866,
                -0.94942696933898629,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-3.5099102111873783, -472.21980812485742, 137.65062964338799),
            Direction3::new(
                0.98768834059513788,
                4.0913845243535237e-16,
                -0.15643446504023051,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(17.23877250204459, -483.34943929843018, 108.84132601407637),
            Direction3::new(
                0.15037445916777487,
                0.27563735581699939,
                0.94942696933898629,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(18.110056564649678, -403.72991228925224, 114.34239708406143),
            Direction3::new(
                0.043119182302335579,
                -0.96126169593831889,
                0.27224380257292374,
            )
            .unwrap(),
        ),
        sphere_face_at(
            Point3::new(-2.3316069e-9, 4.841695e-9, 3.4334278e-9),
            Direction3::new(
                -0.98768834059513788,
                -3.1637180626632703e-16,
                0.15643446504023009,
            )
            .unwrap(),
            500.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 9.8784e6);
}

#[test]
fn VolumeMaker__C4() {
    use support::cylinder_face;
    let upward = Direction3::new(0., 1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(0., 515., 1.1102230246251565e-15),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1585.55182012, 516., 565.97921),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(-1691.058216960648, 515.99999999999966, 449.32377732345128),
            Direction3::new(
                -0.96646577672161749,
                -2.2204460492503128e-16,
                0.25679544860818826,
            )
            .unwrap(),
        ),
        cylinder_face(Point3::new(0., 0., -4.829470157119431e-14), upward, 1750.),
        plane_face(
            Point3::new(-1642.72714221, 516., 478.80098361),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(0., 537., 2.3314683517128287e-15),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1691.0582169606487, 516.00000000000114, 449.32377732344884),
            Direction3::new(
                -0.96646577672161782,
                -6.6613381477509373e-16,
                -0.25679544860818687,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 4.7545301029572329e-14),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    assert!(!make_volume(inputs).is_null());
}

#[test]
fn VolumeMaker__C8() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(1634.597165131441, 820., -503.19389260275346),
            Direction3::new(
                0.26050450864264807,
                2.2204460492503131e-16,
                0.96547263087922519,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., 790., 8.3266726846886741e-16),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1557.8287844128695, 820.00000000000034, -793.75341161103233),
            Direction3::new(
                0.8910065241883679,
                2.2204460492503128e-16,
                -0.4539904997395468,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -9.1038288019262836e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(1547.2842888890548, 820., -704.20625497119863),
            Direction3::new(
                0.45399049973954719,
                1.1102230246251564e-16,
                0.89100652418836768,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., 825., 4.163336342344337e-15),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1434.59193091, 820., -771.03174431000002),
            Direction3::new(1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1688.5837544647095, 820., -455.61486384201993),
            Direction3::new(
                -0.96547263087922508,
                -4.2558268217990881e-34,
                0.26050450864264851,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 9.9142916099026479e-14),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 2.12115e7);
}
