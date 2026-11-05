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
fn VolumeMaker__E5() {
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
            Point3::new(356.99281047643018, -1160.0000000363402, 62.947464405791308),
            Direction3::new(
                -0.95325038083096458,
                -0.25112060959207305,
                -0.16808376151111853,
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
            Point3::new(0., -1219.9999999945176, 1.5341142011343817e-13),
            down,
        ),
        plane_face(
            Point3::new(0., -1280., 1.6007275826179778e-13),
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
    expect_surface(&make_volume(inputs), 1.00927e7);
}

#[test]
fn VolumeMaker__E6() {
    use support::cone_face;
    let down = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-460.8377555733228, -1160., -121.8751945104883),
            Direction3::new(
                0.17364817766693036,
                -5.955424826592936e-17,
                -0.98480775301220813,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(
                -356.99281047643029,
                -1160.0000000363407,
                -62.947464405791337,
            ),
            Direction3::new(
                0.95325038083096492,
                -0.25112060959207222,
                0.16808376151111856,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-474.72960978667726, -1160., -43.090574269511656),
            Direction3::new(
                0.17364817766693036,
                -5.955424826592936e-17,
                -0.98480775301220813,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(
                -509.63801218761682,
                -1160.0000000132397,
                -89.862931946685592,
            ),
            Direction3::new(
                0.91703127426762765,
                -0.36456495599572514,
                0.1616973558068471,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -899.99999999085912, 1.1788428332502699e-13),
            down,
        ),
        plane_face(
            Point3::new(0., -1000., 1.289865135722934e-13),
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
    expect_surface(&make_volume(inputs), 6.47464e6);
}
