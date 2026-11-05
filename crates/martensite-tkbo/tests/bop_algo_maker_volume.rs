//! Native replay of BOPAlgoMakerVolume; see the recorded capture.
#![allow(non_snake_case)]
#[path = "shape_dump/mod.rs"]
mod shape_dump;
use martensite_tkbo::bop_algo::BOPAlgoMakerVolume;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeSphere};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape},
    brep_gprop::BRepGProp,
};
use shape_dump::dump_result;
use std::fmt::Write;

#[path = "bop_algo_maker_volume_support.rs"]
mod support;
use support::{cone_face, faces, plane_face};
fn run(label: &str, arguments: &List<Shape>, intersect: bool, avoid: bool, rows: &mut String) {
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(arguments);
    maker.set_intersect(intersect);
    maker.set_avoid_internal_shapes(avoid);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    let (errors, warnings) = (maker.options().has_errors(), maker.options().has_warnings());
    let null = maker.shape().is_null();
    writeln!(
        rows,
        "case {label} errors {} warnings {} faces {} null {}",
        i32::from(errors),
        i32::from(warnings),
        maker.faces().extent(),
        i32::from(null)
    )
    .unwrap();
    if errors || null {
        return;
    }
    writeln!(rows, "type {}", maker.shape().shape_type().unwrap() as i32).unwrap();
    dump_result(&maker.bounding_box().clone().into_shape(), rows);
    dump_result(maker.shape(), rows);
}

#[test]
fn maker_volume_matches_native_volumes() {
    let mut rows = String::new();
    let mut box_maker = BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0).unwrap();
    let box_shape = box_maker.shape().unwrap().clone();
    run("box-faces", &faces(&box_shape), true, false, &mut rows);
    run(
        "box-faces-no-intersect",
        &faces(&box_shape),
        false,
        false,
        &mut rows,
    );
    let mut sphere = BRepPrimAPIMakeSphere::at_center(Point3::new(10.0, 10.0, 10.0), 5.0).unwrap();
    let solids: List<Shape> = [box_shape.clone(), sphere.shape().unwrap().clone()]
        .into_iter()
        .collect();
    run("box-sphere", &solids, true, false, &mut rows);
    let mut with_edge = faces(&box_shape);
    let edge =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(2.0, 2.0, 2.0), Point3::new(8.0, 8.0, 8.0));
    with_edge.append(edge.unwrap().edge().unwrap().into_shape());
    run("internal-edge", &with_edge, true, false, &mut rows);
    run("internal-edge-avoided", &with_edge, true, true, &mut rows);
    run("nothing", &List::default(), true, false, &mut rows);
    let expected = include_str!("fixtures/tkbo_maker_volume.expected");
    for (line, (actual, wanted)) in rows.lines().zip(expected.lines()).enumerate() {
        assert_eq!(actual, wanted, "line {}", line + 1);
    }
    assert_eq!(rows.lines().count(), expected.lines().count());
}

#[test]
fn VolumeMaker__A1() {
    let mut first = BRepPrimAPIMakeSphere::new(550.0).unwrap();
    let mut second = BRepPrimAPIMakeSphere::new(510.0).unwrap();
    let mut arguments = faces(first.shape().unwrap());
    for face in faces(second.shape().unwrap()).iter() {
        arguments.append(face.clone());
    }
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(
        !maker.options().has_errors(),
        "tests/boolean/volumemaker/A1: mkvolume failed"
    );
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    let area = properties.mass();
    assert!((area - 1.03384e7).abs() / 1.03384e7 <= 0.01, "area {area}");
}

#[test]
fn VolumeMaker__A2() {
    let faces: List<Shape> = [
        plane_face(
            Point3::new(1.8446341100608551e-14, 112.0915332, -696.20434567),
            Direction3::new(
                0.86602540378443893,
                -0.47367733686073005,
                -0.16009303715192008,
            )
            .unwrap(),
        ),
        cone_face(
            Point3::new(0., 112.0915332, -696.20434567),
            Direction3::new(0., -0.32018607430384038, 0.94735467372146087).unwrap(),
            79.818517025525367,
            0.,
        ),
        plane_face(
            Point3::new(0., 85.262913594124683, -616.82482165921328),
            Direction3::new(0., 0.32018607430384038, -0.94735467372146087).unwrap(),
        ),
        plane_face(
            Point3::new(0., 101.92376450277938, -666.12032900071165),
            Direction3::new(0., -0.32018607430384038, 0.94735467372146087).unwrap(),
        ),
        cone_face(
            Point3::new(0., -213.31402525, 266.59354374),
            Direction3::new(0., 0.32018607430384038, -0.94735467372146098).unwrap(),
            10.181482974667482,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&faces);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 2.66661e6).abs() / 2.66661e6 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__E8() {
    let sphere = BRepPrimAPIMakeSphere::at_center(Point3::new(0., 0.3175, 0.), 0.3175)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let mut arguments: List<Shape> = [
        plane_face(
            Point3::new(0., 0.59246307, -3.3217190109624537e-17),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
        cone_face(
            Point3::new(0., 0.635, 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            75.000001447462765,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    for face in faces(&sphere).iter() {
        arguments.append(face.clone());
    }
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 1.58904).abs() / 1.58904 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__E9() {
    let sphere = BRepPrimAPIMakeSphere::new(510.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let mut arguments: List<Shape> = [
        plane_face(
            Point3::new(0., -482.01391336, -1.5690752863672962e-15),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
        cone_face(
            Point3::new(0., -309.47272469, 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            43.999999999485127,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    for face in faces(&sphere).iter() {
        arguments.append(face.clone());
    }
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 3.69404e6).abs() / 3.69404e6 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__F4() {
    let arguments: List<Shape> = [
        plane_face(
            Point3::new(0., 0.59246307, -3.3217190109624537e-17),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
        cone_face(
            Point3::new(0., 0.635, 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            75.000001447462765,
            0.,
        ),
        plane_face(
            Point3::new(0., 0.635, 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 0.161139).abs() / 0.161139 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__F3() {
    let arguments: List<Shape> = [
        plane_face(
            Point3::new(
                670.97596720634715,
                3.2079534447360114e-15,
                -573.06761400170967,
            ),
            Direction3::new(
                -0.76040596560003137,
                -7.9534447360005325e-17,
                0.64944804833018333,
            )
            .unwrap(),
        ),
        cone_face(
            Point3::new(731.30150714141485, 5.8e-15, -624.59049250040596),
            Direction3::new(
                -0.76040596560003104,
                2.4874806615124344e-18,
                0.64944804833018366,
            )
            .unwrap(),
            4.8990924536466087,
            0.,
        ),
        plane_face(
            Point3::new(
                670.97596723162394,
                5.9973401323890082e-15,
                -573.06761402329801,
            ),
            Direction3::new(
                -0.76040596560003104,
                2.4874806615124344e-18,
                0.64944804833018366,
            )
            .unwrap(),
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 1846.27).abs() / 1846.27 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__F5() {
    let arguments: List<Shape> = [
        plane_face(
            Point3::new(62.947464405791067, -1160.0000000363409, -356.99281047643018),
            Direction3::new(
                -0.16808376151111815,
                -0.25112060959207205,
                0.95325038083096503,
            )
            .unwrap(),
        ),
        cone_face(
            Point3::new(0., 237.27272727, 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            14.543833993447443,
            0.,
        ),
        plane_face(
            Point3::new(0., -1219.9999999945176, 1.5341142011343817e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., -1280., 1.6007275826179778e-13),
            Direction3::new(0., 1., -1.1102230246251565e-16).unwrap(),
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 3.32328e6).abs() / 3.32328e6 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__F6() {
    let arguments: List<Shape> = [
        plane_face(
            Point3::new(254., -263.1159033714531, -208.40966385538604),
            Direction3::new(0., 0.25881904510252102, 0.9659258262890682).unwrap(),
        ),
        cone_face(
            Point3::new(254., -309.36134456, -381.),
            Direction3::new(0., 0.25881904510252074, 0.96592582628906831).unwrap(),
            74.999999999529862,
            0.,
        ),
        plane_face(
            Point3::new(254., -288.63806394797888, -303.65966385643014),
            Direction3::new(0., 0.25881904510252074, 0.96592582628906831).unwrap(),
        ),
        cone_face(
            Point3::new(254., 0., 773.5522558),
            Direction3::new(0., -0.25881904510252102, -0.9659258262890682).unwrap(),
            15.000000000112278,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 4.90446e6).abs() / 4.90446e6 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__F2() {
    let arguments: List<Shape> = [
        plane_face(
            Point3::new(0., -746.52181701, 1844.22896887),
            Direction3::new(
                -0.86602540378443904,
                -0.4736773368607301,
                -0.16009303715192008,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -54.262442849244721, -204.00237530028846),
            Direction3::new(0., -0.32018607430384038, 0.94735467372146087).unwrap(),
        ),
        cone_face(
            Point3::new(0., -746.52181701, 1844.22896887),
            Direction3::new(0., 0.32018607430384038, -0.94735467372146098).unwrap(),
            10.181482974865283,
            0.,
        ),
        plane_face(
            Point3::new(0., 85.262913376439656, -616.82482172668711),
            Direction3::new(0., -0.32018607430384038, 0.94735467372146098).unwrap(),
        ),
        cone_face(
            Point3::new(0., -213.31402525, 266.59354374),
            Direction3::new(0., 0.32018607430384038, -0.94735467372146098).unwrap(),
            10.181482974667482,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 8.92077e6).abs() / 8.92077e6 <= 0.01,
        "area {}",
        properties.mass()
    );
}
