//! Native replay of analytic volume-maker regressions.
#![allow(non_snake_case)]

#[path = "bop_algo_maker_volume_support.rs"]
mod support;

use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction3, Point3};
use support::{
    cone_face, expect_empty_surface, expect_shape_count, expect_surface, make_volume, plane_face,
};

#[test]
fn VolumeMaker__C3() {
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-513.57358775546049, 376.87261141864394, 391.24411461018769),
            Direction3::new(
                -0.35888381816183362,
                -0.86162916044152549,
                0.35888381816183351,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(328.92245019629729, 589.54856741821357, -265.84852531445716),
            Direction3::new(
                -0.70710678118654768,
                1.0835061328294932e-16,
                -0.70710678118654746,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-482.79588435174912, 402.18608718514611, 482.79588435174912),
            Direction3::new(
                -0.35888381816183385,
                -0.86162916044152549,
                0.35888381816183329,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-463.10033014697382, 385.77899253621149, 463.10033014697387),
            Direction3::new(
                -0.35888381816183385,
                -0.86162916044152549,
                0.35888381816183329,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(279.85740961131154, 573.09168907578135, -196.41880943129888),
            Direction3::new(
                -0.70710678118654779,
                1.8999111705548982e-16,
                -0.70710678118654724,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-347.34130504162755, 289.34762072607049, 347.34130504162755),
            Direction3::new(
                -0.60926382221627495,
                0.50753836296070431,
                0.60926382221627418,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-456.95672492948086, 334.52761535111603, 456.95672492948091),
            Direction3::new(
                0.60926382221627451,
                -0.50753836296070476,
                -0.60926382221627429,
            )
            .unwrap(),
        ),
    ]
    .into_iter()
    .collect();
    expect_empty_surface(&make_volume(inputs));
}

#[test]
fn VolumeMaker__C5() {
    let down = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
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
        cone_face(
            Point3::new(0., 518.47000000000003, 0.),
            down,
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
        plane_face(
            Point3::new(0., -786.50743635718959, 3.8387493219403887e-14),
            down,
        ),
        plane_face(
            Point3::new(0., -796.42849000000001, 9.2386370775798815e-15),
            Direction3::new(0., 1., -1.1102230246251565e-16).unwrap(),
        ),
        cone_face(
            Point3::new(0., -440.74363604000001, 0.),
            down,
            45.110284878807235,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    let result = make_volume(inputs);
    expect_surface(&result, 5.5768e6);
    expect_shape_count(&result, ShapeEnum::Solid, 17);
}

#[test]
fn VolumeMaker__C6() {
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
            Point3::new(27.577164468182502, -1038.2137499999999, 27.577164468182474),
            Direction3::new(
                -0.70710678118654746,
                2.2204460492503131e-16,
                -0.70710678118654757,
            )
            .unwrap(),
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
        plane_face(
            Point3::new(0., -786.50743635718959, 3.8387493219403887e-14),
            down,
        ),
        plane_face(
            Point3::new(0., -796.42849000000001, 9.2386370775798815e-15),
            Direction3::new(0., 1., -1.1102230246251565e-16).unwrap(),
        ),
        cone_face(
            Point3::new(0., -440.74363604000001, 0.),
            down,
            45.110284878807235,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    let result = make_volume(inputs);
    expect_surface(&result, 6.45353e6);
    expect_shape_count(&result, ShapeEnum::Solid, 30);
}

#[test]
fn VolumeMaker__C7() {
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(293.59338837266205, 530.33173154802842, 419.81194880703725),
            Direction3::new(
                -0.70710678124078374,
                5.96999808083709e-11,
                0.70710678113231129,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(395.5430009713703, 487.72110861164703, 395.54300097350261),
            Direction3::new(
                0.46390378819883044,
                -0.75470958021241763,
                0.46390378833371382,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(442.46179031859833, 408.75822975397654, 345.44673994835938),
            Direction3::new(
                -0.46390378819883077,
                0.75470958021241707,
                -0.46390378833371432,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(315.11890869790136, 487.8695244290447, 428.7509684330779),
            Direction3::new(
                -0.70710678124078397,
                5.9699945265738569e-11,
                0.70710678113231118,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(434.85318048286939, 423.03013402881749, 325.11020804792412),
            Direction3::new(
                0.53366026198141492,
                0.65605902900241919,
                0.53366026200789074,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(413.38188493162187, 435.84243835913986, 340.76201850744235),
            Direction3::new(
                -0.53366026198141514,
                -0.65605902900241886,
                -0.53366026200789107,
            )
            .unwrap(),
        ),
    ]
    .into_iter()
    .collect();
    expect_surface(&make_volume(inputs), 1744.95);
}
