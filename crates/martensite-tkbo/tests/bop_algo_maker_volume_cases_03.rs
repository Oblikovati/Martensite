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
fn VolumeMaker__E3() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(0., -1398., -8.3266726846886741e-16),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 1.5654144647214707e-13),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1750.,
        ),
        plane_face(
            Point3::new(-837.85723364880107, -1532.5, -1479.1857140209454),
            Direction3::new(
                -0.89100652418836812,
                -2.2204460492503136e-16,
                0.45399049973954653,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-771.03174431000002, -1532.5, -1434.59193091),
            Direction3::new(-2.2204460492503131e-16, 1.1102230246251565e-16, -1.).unwrap(),
        ),
        plane_face(
            Point3::new(0., -1450., -4.4408920985006262e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(
                -793.75341161103245,
                -1532.5000000000002,
                -1557.8287844128681,
            ),
            Direction3::new(
                0.45399049973954719,
                1.1102230246251564e-16,
                0.89100652418836768,
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
    expect_surface(&make_volume(inputs), 2.15489e7);
}

#[test]
fn VolumeMaker__E7() {
    use support::cone_face;
    let axis = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        cone_face(
            Point3::new(0., -60.919306349999999, 0.),
            axis,
            28.800000000062262,
            0.,
        ),
        plane_face(
            Point3::new(0., -460.31437495679268, 4.4341760108900527e-14),
            axis,
        ),
        plane_face(
            Point3::new(0., -558., -5.8286708792820718e-16),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        sphere_face(510.),
        cone_face(
            Point3::new(0., -309.47272469000001, 0.),
            axis,
            43.999999999485127,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    let result = make_volume(inputs);
    expect_surface(&result, 5.7053e6);
    expect_shape_count(&result, ShapeEnum::Solid, 8);
}

#[test]
fn VolumeMaker__F8() {
    use support::{cone_face, cylinder_face};
    let downward = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(27.577164466275352, -1038.2137499999999, 27.577164466275359),
            Direction3::new(
                0.70710678118654746,
                4.4408920985006262e-16,
                0.70710678118654768,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -729., 1.3849699165291441e-13),
            Direction3::new(0., 1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., -786.50743635718959, 3.8387493219403887e-14),
            downward,
        ),
        cone_face(
            Point3::new(0., 518.47000000000003, 0.),
            downward,
            14.999999999912038,
            0.,
        ),
        plane_face(
            Point3::new(
                -27.577164466275352,
                -1038.2137499999999,
                -27.577164466275359,
            ),
            Direction3::new(
                0.70710678118654746,
                4.4408920985006262e-16,
                0.70710678118654768,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 7.5439654523279387e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            260.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 5.41197e6);
}

#[test]
fn VolumeMaker__F9() {
    use support::{cone_face, cylinder_face};
    let down = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(
                2.3537565147188571e-15,
                -592.35294118000002,
                1.1044592656221142e-15,
            ),
            Direction3::new(
                0.90529096719956037,
                4.7161388709779336e-17,
                0.42479202523927467,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -620., 2.2759572004815709e-14),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        cylinder_face(Point3::new(0., 0., 0.), down, 1750.),
        cone_face(
            Point3::new(0., -5411.2352941199997, 0.),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            19.885165113840024,
            0.,
        ),
        plane_face(
            Point3::new(
                2.6273419411410461e-15,
                -592.35294118000002,
                1.448127868774439e-15,
            ),
            Direction3::new(
                0.87578064704701541,
                5.3591496750491317e-17,
                0.48270928959147968,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -564.70588236000003, -3.0694401265129993e-15),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
        cylinder_face(Point3::new(0., 0., 2.9226621123257246e-14), down, 1720.),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 9.42987e7);
}

#[test]
fn VolumeMaker__G2() {
    use support::{cone_face, cylinder_face};
    let down = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
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
        plane_face(
            Point3::new(0., -729., 1.3849699165291441e-13),
            Direction3::new(0., 1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., -786.50743635718959, 3.8387493219403887e-14),
            down,
        ),
        cone_face(
            Point3::new(0., 518.47000000000003, 0.),
            down,
            14.999999999912038,
            0.,
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
        cylinder_face(
            Point3::new(0., 0., 7.5439654523279387e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            260.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 6.28616e6);
}

#[test]
fn VolumeMaker__E4() {
    use support::cone_face;
    let down = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(460.8377555733228, -1160., 121.87519451048833),
            Direction3::new(
                -0.17364817766693036,
                1.1223734950417248e-17,
                0.98480775301220813,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(356.99281047643007, -1160.0000000363402, 62.947464405791337),
            Direction3::new(
                -0.95325038083096469,
                -0.25112060959207294,
                -0.16808376151111859,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(474.72960978667726, -1160., 43.090574269511663),
            Direction3::new(
                -0.17364817766693036,
                1.1223734950417248e-17,
                0.98480775301220813,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(509.6380121876166, -1160.0000000132395, 89.862931946685578),
            Direction3::new(
                -0.91703127426762787,
                -0.36456495599572464,
                -0.1616973558068471,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -629.99999999095576, 8.7908261660158493e-14),
            down,
        ),
        plane_face(
            Point3::new(0., -730., 9.9010491907414171e-14),
            Direction3::new(0., 1., -1.1102230246251565e-16).unwrap(),
        ),
        cone_face(
            Point3::new(0., 161.80722892, 0.),
            down,
            21.380811808638473,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 3.76654e6);
}
