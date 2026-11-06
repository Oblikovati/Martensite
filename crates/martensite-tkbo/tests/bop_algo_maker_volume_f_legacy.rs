//! Native replay of additional analytic volume-maker regressions.
#![allow(non_snake_case)]
#[path = "bop_algo_maker_volume_support.rs"]
mod support;
use martensite_tkbo::bop_algo::BOPAlgoMakerVolume;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use support::{cone_face, plane_face};

#[test]
fn VolumeMaker__F7() {
    let arguments: List<Shape> = [
        plane_face(
            Point3::new(-306.53078964627537, -1038.21375, -251.37646071372467),
            Direction3::new(
                -0.70710678118654746,
                4.4408920985006262e-16,
                0.70710678118654768,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-27.577164468182303, -1038.21375, -27.577164468182389),
            Direction3::new(
                0.70710678118654768,
                2.2204460492503131e-16,
                0.70710678118654746,
            )
            .unwrap(),
        ),
        cone_face(
            Point3::new(0., 107.1882, 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            20.000000000057945,
            0.,
        ),
        plane_face(
            Point3::new(-251.37646071372464, -1038.21375, -306.53078964627542),
            Direction3::new(
                -0.70710678118654746,
                4.4408920985006262e-16,
                0.70710678118654768,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -900.49999998788303, 1.3190004643924646e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., -999., 1.4283574323314951e-13),
            Direction3::new(0., 1., -1.1102230246251565e-16).unwrap(),
        ),
        cone_face(
            Point3::new(0., 164.44, 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            15.140999999959531,
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
        (properties.mass() - 7.80813e6).abs() / 7.80813e6 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__F1() {
    let arguments: List<Shape> = [
        plane_face(
            Point3::new(-2.4595121467478056e-14, -746.52181701, 1844.22896887),
            Direction3::new(
                0.86602540378443849,
                -0.47367733686073094,
                -0.16009303715192033,
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
            Point3::new(0., -746.52181701, 1844.22896887),
            Direction3::new(
                -0.86602540378443904,
                -0.4736773368607301,
                -0.16009303715192008,
            )
            .unwrap(),
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
        (properties.mass() - 1.13448e7).abs() / 1.13448e7 <= 0.01,
        "area {}",
        properties.mass()
    );
}

#[test]
fn VolumeMaker__G1() {
    use martensite_tkbrep::{topexp, toptools::IndexedMapOfShape};
    use martensite_tkg3d::top_abs::ShapeEnum;
    use support::cylinder_face;
    let axis = Direction3::new(
        -0.41716766026590824,
        0.90882954575006414,
        -5.4874902763032048e-16,
    )
    .unwrap();
    let reverse = Direction3::new(
        0.41716766026590824,
        -0.90882954575006414,
        -5.4874902763032048e-16,
    )
    .unwrap();
    let inputs: List<Shape> = [
        cone_face(
            Point3::new(-59.81469844, 384.36473473, 127.),
            reverse,
            89.995898744693349,
            0.,
        ),
        plane_face(
            Point3::new(-59.810258374411447, 384.355061729966, 127.),
            axis,
        ),
        plane_face(
            Point3::new(-59.814609898410502, 384.36454183582288, 127.),
            reverse,
        ),
        cylinder_face(
            Point3::new(96.320637969732303, 44.212751846651756, 127.),
            axis,
            2.90794958,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut surface = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut surface, false, false).unwrap();
    assert!((surface.mass() - 138971.).abs() / 138971. <= 0.01);
    let mut volume = GProps::default();
    assert!(
        BRepGProp::volume_properties_eps(maker.shape(), &mut volume, 1e-10, false, false).unwrap()
            <= 1e-10
    );
    assert!((volume.mass() - 246.42).abs() / 246.42 <= 0.01);
    for (kind, expected) in [
        (ShapeEnum::Vertex, 7),
        (ShapeEnum::Edge, 12),
        (ShapeEnum::Wire, 11),
        (ShapeEnum::Face, 9),
        (ShapeEnum::Shell, 4),
        (ShapeEnum::Solid, 4),
    ] {
        let mut found = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(maker.shape(), kind, &mut found).unwrap();
        assert_eq!(found.extent(), expected, "{kind:?}");
    }
}

#[test]
fn VolumeMaker__G3() {
    use support::cylinder_face;
    let axis = Direction3::new(
        -0.76040596560003104,
        2.4874806615124344e-18,
        0.64944804833018366,
    )
    .unwrap();
    let inputs: List<Shape> = [
        cone_face(
            Point3::new(731.30150714141485, 5.8e-15, -624.59049250040596),
            axis,
            4.8990924536466087,
            0.,
        ),
        plane_face(
            Point3::new(
                670.82388601322714,
                5.9978376286039971e-15,
                -572.93772439204361,
            ),
            Direction3::new(
                0.76040596560003104,
                -2.4874806615124344e-18,
                -0.64944804833018366,
            )
            .unwrap(),
        ),
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
        cylinder_face(
            Point3::new(
                -2.9683633329113945e-10,
                -6.6972588473441807e-14,
                -3.4731328923953697e-10,
            ),
            Direction3::new(
                0.76040596560003137,
                7.9534447360005325e-17,
                -0.64944804833018333,
            )
            .unwrap(),
            2.45,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 3714.75).abs() / 3714.75 <= 0.01);
}

#[test]
fn VolumeMaker__G4() {
    use support::cylinder_face;
    let axis = Direction3::new(0., 1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        cylinder_face(Point3::new(0., 0., 6.1367577686155528e-14), axis, 276.),
        plane_face(
            Point3::new(0., -547.5, 5.8286708792820718e-16),
            Direction3::new(0., -1., -1.1102230246251565e-16).unwrap(),
        ),
        plane_face(Point3::new(0., -558., -5.8286708792820718e-16), axis),
        cone_face(
            Point3::new(0., -60.91930635, 0.),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            28.800000000062262,
            0.,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 1.25206e6).abs() / 1.25206e6 <= 0.01);
}

#[test]
fn VolumeMaker__G5() {
    use support::cylinder_face;
    let downward = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(2.6273419411410461e-15, -592.35294118, 1.448127868774439e-15),
            Direction3::new(
                0.87578064704701541,
                5.3591496750491317e-17,
                0.48270928959147968,
            )
            .unwrap(),
        ),
        plane_face(Point3::new(0., -620., 2.2759572004815709e-14), downward),
        cylinder_face(Point3::new(0., 0., 0.), downward, 1750.),
        cone_face(
            Point3::new(0., -5411.23529412, 0.),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            19.885165113840024,
            0.,
        ),
        plane_face(
            Point3::new(
                2.3537565147188571e-15,
                -592.35294118,
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
            Point3::new(0., -564.70588236, -3.0694401265129993e-15),
            downward,
        ),
        cylinder_face(Point3::new(0., 0., 2.9226621123257246e-14), downward, 1720.),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 9.42987e7).abs() / 9.42987e7 <= 0.01);
}

#[test]
fn VolumeMaker__G6() {
    use support::cylinder_face;
    let upward = Direction3::new(0., 1., 1.1102230246251565e-16).unwrap();
    let downward = Direction3::new(0., -1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(
                2.3537565147188571e-15,
                -592.35294118,
                1.1044592656221142e-15,
            ),
            Direction3::new(
                0.90529096719956037,
                4.7161388709779336e-17,
                0.42479202523927467,
            )
            .unwrap(),
        ),
        cylinder_face(Point3::new(0., 0., 2.9698465908722937e-14), upward, 1750.),
        cone_face(
            Point3::new(0., -5411.23529412, 0.),
            upward,
            19.885165113840024,
            0.,
        ),
        plane_face(
            Point3::new(2.6273419411410461e-15, -592.35294118, 1.448127868774439e-15),
            Direction3::new(
                0.87578064704701541,
                5.3591496750491317e-17,
                0.48270928959147968,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -564.70588236, -3.0694401265129993e-15),
            downward,
        ),
        plane_face(Point3::new(0., -620., -2.3869795029440866e-14), upward),
        cylinder_face(
            Point3::new(0., 0., -4.6157522248790883e-14),
            downward,
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 9.42987e7).abs() / 9.42987e7 <= 0.01);
}

#[test]
fn VolumeMaker__G7() {
    use support::sphere_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(548.64325385, 27.3, -3.6783467500131954e-15),
            Direction3::new(-4.4408920985006262e-16, 1., -1.3473797619095955e-16).unwrap(),
        ),
        plane_face(
            Point3::new(
                486.6025401200996,
                2.0625399133056258e-14,
                116.82293399833648,
            ),
            Direction3::new(
                0.97236992039767656,
                -4.4408920985006271e-16,
                0.23344536385590584,
            )
            .unwrap(),
        ),
        sphere_face(550.),
        plane_face(
            Point3::new(
                374.64624787088081,
                -1.5154544286133377e-15,
                396.61870189058595,
            ),
            Direction3::new(
                0.76040596560003082,
                -5.5511151231257839e-17,
                -0.64944804833018399,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(531.76355117, -27.3, 127.66513349),
            Direction3::new(5.7436601950102579e-16, 1., -4.9008054356520772e-16).unwrap(),
        ),
        plane_face(
            Point3::new(
                321.42888552646951,
                -5.5487008260301958e-15,
                376.34487115994881,
            ),
            Direction3::new(
                0.64944804833018377,
                1.1102230246251565e-16,
                0.76040596560003071,
            )
            .unwrap(),
        ),
        sphere_face(535.),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 1.73885e7).abs() / 1.73885e7 <= 0.01);
}

#[test]
fn VolumeMaker__G8() {
    use support::sphere_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-503.03467413, 27.3, 3.0309088572266766e-15),
            Direction3::new(-1.110223024625157e-16, 1., 1.1102230246251563e-16).unwrap(),
        ),
        plane_face(
            Point3::new(
                -17.608213675459623,
                -491.58544067385321,
                -111.17388574694267,
            ),
            Direction3::new(
                -0.15037445914067274,
                0.27563735581879711,
                -0.94942696934275683,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(
                431.33015951945981,
                1.2520186166709012e-14,
                103.55320945467923,
            ),
            Direction3::new(
                0.97236992039767667,
                -2.2204460492503128e-16,
                0.23344536385590553,
            )
            .unwrap(),
        ),
        sphere_face(510.),
        plane_face(
            Point3::new(
                169.38660019244185,
                -3.7886360715333475e-15,
                480.27376534356694,
            ),
            Direction3::new(
                0.92387953251128674,
                1.3877787807814457e-16,
                -0.38268343236509,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(
                479.78505977673376,
                -9.0927265716800359e-15,
                143.26193656685658,
            ),
            Direction3::new(
                0.23344536385590553,
                3.3306690738754696e-16,
                -0.97236992039767667,
            )
            .unwrap(),
        ),
        sphere_face(500.),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 1.90007e7).abs() / 1.90007e7 <= 0.01);
}

#[test]
fn VolumeMaker__G9() {
    use support::sphere_face;
    let shared =
        Direction3::new(0.85649044251752993, 0.27563735586738991, 0.436403677718875).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-109.65165515927141, 528.22159206517972, -55.870308878274699),
            shared,
        ),
        sphere_face(535.),
        plane_face(
            Point3::new(-156.41603332072859, 513.17179243482019, -79.697949681725305),
            shared,
        ),
        plane_face(
            Point3::new(-120.6399035968881, 520.69669224978725, -92.108607390229196),
            Direction3::new(
                0.45399049974768874,
                -7.7904639963905031e-12,
                -0.89100652418421933,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-118.04565524587703, 462.03266975772823, -60.147265544976598),
            Direction3::new(
                -0.24559468238333207,
                0.96126169592386956,
                -0.12513674094604033,
            )
            .unwrap(),
        ),
        sphere_face(520.),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 1.61207e7).abs() / 1.61207e7 <= 0.01);
}

#[test]
fn VolumeMaker__H1() {
    use support::sphere_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-154.47938423072861, 505.59171548482027, -78.711177681725303),
            Direction3::new(
                0.85649044251753015,
                0.27563735586738969,
                0.43640367771887478,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-514.05668202, 3.6783467500131954e-15, -27.3),
            Direction3::new(3.8453235056139149e-17, 1.3473797619095952e-16, -1.).unwrap(),
        ),
        plane_face(
            Point3::new(-498.38812535, -5.2642366932254001e-15, 0.),
            Direction3::new(-1., 1.1102230246251565e-16, 0.).unwrap(),
        ),
        sphere_face(550.),
        plane_face(
            Point3::new(-26.94591530454035, -501.66571339614683, -170.12981358305723),
            Direction3::new(
                -0.15037445914067232,
                0.27563735581879711,
                -0.94942696934275683,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(197.01514608, -27.300000000000004, 475.63663767),
            Direction3::new(2.2204460492503131e-16, 1., 2.7755575615628918e-16).unwrap(),
        ),
        sphere_face(535.),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 2.08517e7).abs() / 2.08517e7 <= 0.01);
}

#[test]
fn VolumeMaker__H2() {
    use support::cylinder_face;
    let upward = Direction3::new(0., 1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(Point3::new(0., -1020., -2.3314683517128287e-15), upward),
        plane_face(
            Point3::new(771.03174431, -825., -1591.87807199),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(793.75341161103165, -824.99999999999977, -1557.8287844128686),
            Direction3::new(
                0.45399049973954675,
                1.1102230246251565e-16,
                -0.8910065241883679,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(273.50878573665523, -825., -1726.8665102250795),
            Direction3::new(0.1564344650402309, 0., -0.98768834059513777).unwrap(),
        ),
        cylinder_face(Point3::new(0., 0., 7.2275518903097691e-14), upward, 1750.),
        plane_face(
            Point3::new(183.77345414000004, -825., -1677.43392101),
            Direction3::new(1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(
            Point3::new(347.58541138000004, -825., 1677.43392101),
            Direction3::new(1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(Point3::new(0., -1030., -2.2759572004815709e-14), upward),
        cylinder_face(
            Point3::new(0., 0., -1.0788592241794959e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1720.,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 1.98614e7).abs() / 1.98614e7 <= 0.01);
}

#[test]
fn VolumeMaker__H3() {
    use support::sphere_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-176.68935041632815, -136.40000000000003, 561.3921855049216),
            Direction3::new(
                -0.96814764037810808,
                -6.8091391435688272e-18,
                -0.25038000405444022,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(
                -112.59552164700079,
                1.0295987166949039e-14,
                561.18334660015273,
            ),
            Direction3::new(
                0.96814764037810797,
                1.8717419577616017e-16,
                0.25038000405444111,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-123.93810201, -33.500000000000007, 479.23308199),
            Direction3::new(-8.1345814275900847e-17, -1., 9.4975092332914232e-17).unwrap(),
        ),
        plane_face(
            Point3::new(-189.13079556159209, -31.5, 731.31452385509169),
            Direction3::new(-1.195351741235474e-16, -1., -3.0786269887577617e-17).unwrap(),
        ),
        plane_face(
            Point3::new(
                -105.15960170591688,
                -8.4237957633761796e-15,
                406.62200896164194,
            ),
            Direction3::new(
                -0.2503800040544415,
                1.1231727684501573e-16,
                0.96814764037810774,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(
                -142.71660231371661,
                5.1487492657709883e-14,
                551.84415500900138,
            ),
            Direction3::new(
                0.25038000405444183,
                -5.3742990079214548e-16,
                -0.96814764037810774,
            )
            .unwrap(),
        ),
        sphere_face(500.),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 9.87124e6).abs() / 9.87124e6 <= 0.01);
}

#[test]
fn VolumeMaker__H4() {
    use support::cylinder_face;
    let upward = Direction3::new(0., 1., 1.1102230246251565e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(Point3::new(0., -1010., -1.1102230246251565e-14), upward),
        plane_face(
            Point3::new(265.67943276, -825., 1616.2687468249999),
            Direction3::new(-2.2204460492503131e-16, -1.1102230246251565e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(-793.75341161103267, -824.99999999999977, 1557.8287844128686),
            Direction3::new(
                -0.45399049973954719,
                1.1102230246251564e-16,
                0.89100652418836768,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(793.75341161103267, -824.99999999999977, 1557.8287844128686),
            Direction3::new(
                0.45399049973954719,
                1.1102230246251564e-16,
                0.89100652418836768,
            )
            .unwrap(),
        ),
        cylinder_face(Point3::new(0., 0., 7.2275518903097691e-14), upward, 1750.),
        plane_face(
            Point3::new(704.20625497119897, -825., 1547.2842888890546),
            Direction3::new(
                -0.89100652418836812,
                -2.2204460492503136e-16,
                0.45399049973954653,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-704.20625497119897, -825., 1547.2842888890546),
            Direction3::new(
                0.89100652418836812,
                -2.2204460492503136e-16,
                0.45399049973954653,
            )
            .unwrap(),
        ),
        plane_face(Point3::new(0., -1030., -2.2759572004815709e-14), upward),
        plane_face(
            Point3::new(771.03174431, -825., 1434.59193091),
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
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 2.26035e7).abs() / 2.26035e7 <= 0.01);
}

#[test]
fn VolumeMaker__H5() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(335.2331946308351, 698.25, -1031.741684575172),
            Direction3::new(
                -0.95105651629515375,
                -4.4408920985006262e-16,
                -0.30901699437494717,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(379.81702147596855, -944.5, -1016.689989615124),
            Direction3::new(
                0.93679437761793805,
                -2.2204460492503121e-16,
                0.34988039965596873,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(0., -199., -3.8302694349567901e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-448.0746418400002, -1690.0000000000005, -1379.03194863),
            Direction3::new(-4.566617150598209e-16, -1., 3.1642628651144884e-17).unwrap(),
        ),
        plane_face(
            Point3::new(377.03714586714227, 698.25, -1080.1369248874196),
            Direction3::new(
                -0.32956322072971639,
                -2.2204460492503131e-16,
                0.94413350938426943,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., 2.0539125955565396e-15),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
            1144.32,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 3.27453e7).abs() / 3.27453e7 <= 0.01);
}

#[test]
fn VolumeMaker__H6() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-688.33310993156408, -895., 1539.8682429696005),
            Direction3::new(
                -0.95105651629515375,
                -4.4408920985006262e-16,
                -0.30901699437494717,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(-2110.49026701, -620., -29.999999999999964),
            Direction3::new(1.110223024625157e-16, 1., 1.1102230246251563e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-178.79823340500002, -895., 1677.20835547),
            Direction3::new(-1., 1.1102230246251568e-16, 2.2204460492503131e-16).unwrap(),
        ),
        plane_face(
            Point3::new(0., -1170., -3.0531133177191805e-14),
            Direction3::new(0., 1., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-265.64370669, -895., 1615.2543608350002),
            Direction3::new(-2.2204460492503131e-16, 1.1102230246251565e-16, -1.).unwrap(),
        ),
        plane_face(
            Point3::new(-790.07290042162936, -895., 1571.9532661481319),
            Direction3::new(
                0.30901699437494717,
                -1.6653345369377348e-16,
                -0.95105651629515375,
            )
            .unwrap(),
        ),
        cylinder_face(
            Point3::new(0., 0., -2.0206059048177849e-13),
            Direction3::new(0., -1., 1.1102230246251565e-16).unwrap(),
            1650.,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 3.20115e7).abs() / 3.20115e7 <= 0.01);
}

#[test]
fn VolumeMaker__H7() {
    use support::cylinder_face;
    let y_axis = Direction3::new(2.5254770070057677e-16, 1., 2.2204460492503128e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-0.8, -1.1969506538713359e-32, -0.8),
            Direction3::new(1., 1.4961883173391698e-32, -1.2217304763960305e-16).unwrap(),
        ),
        plane_face(
            Point3::new(
                1.1999999999999995,
                1.4200000000000004,
                3.330669073875467e-17,
            ),
            Direction3::new(-2.5254770070057677e-16, -1., -2.2204460492503128e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1.1999999999999997, 1.4400000000000004, 0.80000000000000027),
            Direction3::new(-4.4408920985006262e-16, 1., 1.0419471428105878e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-1.6, 0.95999999999999985, -1.2),
            Direction3::new(1., 2.2204460492503126e-16, 4.9303806576313216e-32).unwrap(),
        ),
        plane_face(
            Point3::new(-1.31220258, 0.45464310000000052, -1.6),
            Direction3::new(9.773843811168237e-17, -2.7052603405912107e-16, 1.).unwrap(),
        ),
        plane_face(
            Point3::new(1.2, 1.28, 3.2),
            Direction3::new(-6.8996496966039912e-17, 1.0419471428105876e-16, -1.).unwrap(),
        ),
        cylinder_face(
            Point3::new(-1.2, 2.2204460492503131e-16, -0.4),
            y_axis,
            0.15,
        ),
        cylinder_face(Point3::new(-1.2, 2.2204460492503131e-16, 0.4), y_axis, 0.15),
        cylinder_face(
            Point3::new(-1.2, 2.2204460492503131e-16, -1.2),
            Direction3::new(1.110223024625157e-16, 1., 1.1102230246251563e-16).unwrap(),
            0.15,
        ),
        cylinder_face(Point3::new(-1.2, 2.2204460492503131e-16, 1.2), y_axis, 0.15),
        cylinder_face(Point3::new(-1.2, 2.2204460492503131e-16, 2.), y_axis, 0.15),
        cylinder_face(Point3::new(-1.2, 2.2204460492503131e-16, 2.8), y_axis, 0.15),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 8.13019).abs() / 8.13019 <= 0.01);
}

#[test]
fn VolumeMaker__H8() {
    use support::cylinder_face;
    let axis = Direction3::new(-6.8996496966039912e-17, -1., -4.4408920985006262e-16).unwrap();
    let inputs: List<Shape> = [
        cylinder_face(
            Point3::new(1.2, 1.1102230246251565e-16, -1.20008083),
            axis,
            0.2,
        ),
        plane_face(
            Point3::new(1.1999709254655291, 0.3199650988081304, -0.8),
            Direction3::new(
                -0.76822127962268527,
                0.64018439963410856,
                2.2100838024345804e-16,
            )
            .unwrap(),
        ),
        plane_face(Point3::new(-1.2, 0.9500000000000004, -1.20008083), axis),
        cylinder_face(
            Point3::new(1.2, 1.1102230246251565e-16, -1.20008083),
            axis,
            0.125,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 2.10324).abs() / 2.10324 <= 0.01);
}

#[test]
fn VolumeMaker__H9() {
    use support::cylinder_face;
    let downward = Direction3::new(-6.8996496966039912e-17, -1., -4.4408920985006262e-16).unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(1.1999999990042425, 0.31999999413033342, -0.8),
            Direction3::new(
                0.76822127962268527,
                -0.64018439963410856,
                -1.2909433957963266e-16,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(1.27965339, 0.75030347500000016, -0.8),
            Direction3::new(9.9920072216264089e-16, 1., 2.7052603405912097e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-3.1086244689504381e-16, -0.0099999999999997435, -1.5),
            Direction3::new(-4.4408920985006262e-16, 2.2204460492503131e-16, -1.).unwrap(),
        ),
        plane_face(
            Point3::new(3.1086244689504381e-16, -0.010000000000000056, -0.1),
            Direction3::new(-4.4408920985006262e-16, 2.2204460492503131e-16, -1.).unwrap(),
        ),
        plane_face(
            Point3::new(-1.19699792, 0.1162048950000002, -0.8),
            Direction3::new(-4.4408920985006262e-16, 1., 1.0419471428105878e-16).unwrap(),
        ),
        plane_face(
            Point3::new(1.1969979147476388, 0.47260249268594856, -0.8),
            Direction3::new(
                0.76822127962268527,
                -0.64018439963410856,
                -1.2909433957963266e-16,
            )
            .unwrap(),
        ),
        plane_face(
            Point3::new(1.6382686137389917, 0.31996510442169662, -0.8),
            Direction3::new(
                -0.98328601660540405,
                -0.18206759609627698,
                5.4350981971514161e-17,
            )
            .unwrap(),
        ),
        cylinder_face(Point3::new(1.2, 0., -0.4), downward, 0.2),
        cylinder_face(
            Point3::new(1.2, 1.1102230246251565e-16, -1.20008083),
            downward,
            0.2,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 16.5007).abs() / 16.5007 <= 0.01);
}

#[test]
fn VolumeMaker__I1() {
    use support::cylinder_face;
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(635., 431.8, -139.7),
            Direction3::new(0., 1., 0.).unwrap(),
        ),
        plane_face(
            Point3::new(1016., 127., -139.7),
            Direction3::new(1., 0., 1.1102230246251565e-16).unwrap(),
        ),
        cylinder_face(
            Point3::new(635., 127., -1.4210854715202004e-14),
            Direction3::new(0., -1.2246467991473532e-16, -1.).unwrap(),
            419.1,
        ),
        plane_face(
            Point3::new(635., 127., 177.8),
            Direction3::new(1.1102230246251565e-16, 0., -1.).unwrap(),
        ),
        plane_face(
            Point3::new(635., -177.8, -139.7),
            Direction3::new(0., 1., 0.).unwrap(),
        ),
        plane_face(
            Point3::new(254., 127., -139.7),
            Direction3::new(1., 0., 1.1102230246251565e-16).unwrap(),
        ),
        plane_face(
            Point3::new(635., 127., -254.),
            Direction3::new(0., 1.2246467991473532e-16, 1.).unwrap(),
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 4.544e6).abs() / 4.544e6 <= 0.01);
}

#[test]
fn VolumeMaker__I2() {
    use support::cylinder_face;
    let tilted = Direction3::new(
        2.7755575615628914e-16,
        -0.71130314921411464,
        -0.7028853604380183,
    )
    .unwrap();
    let inputs: List<Shape> = [
        plane_face(
            Point3::new(-254., -309.31813765, -164.27444069),
            Direction3::new(-1., -1.9742628343647336e-16, -1.9508987770756001e-16).unwrap(),
        ),
        plane_face(
            Point3::new(
                7.3045963355777887e-14,
                -496.51592627408689,
                -349.25687086146246,
            ),
            tilted,
        ),
        plane_face(
            Point3::new(
                -7.3045963355777887e-14,
                -122.12034902591307,
                20.707989481462448,
            ),
            tilted,
        ),
        plane_face(
            Point3::new(-127., -81.632281828743373, -200.94876776038524),
            Direction3::new(0., -0.70288536043801819, 0.71130314921411486).unwrap(),
        ),
        plane_face(
            Point3::new(254., -309.31813764999993, -164.27444068999995),
            Direction3::new(-1., -1.9742628343647336e-16, -1.9508987770756001e-16).unwrap(),
        ),
        plane_face(
            Point3::new(-127., -438.69804493125667, 160.39323204038519),
            Direction3::new(0., 0.70288536043801819, -0.71130314921411486).unwrap(),
        ),
        plane_face(
            Point3::new(0., -535.56033360266497, 64.677242648464215),
            Direction3::new(0., 0.7028853604380183, -0.71130314921411464).unwrap(),
        ),
        cylinder_face(
            Point3::new(-127., -141.76929448894901, -140.09155135332165),
            Direction3::new(0., -0.70288536043801819, 0.71130314921411486).unwrap(),
            76.2,
        ),
    ]
    .into_iter()
    .collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    let mut properties = GProps::default();
    BRepGProp::surface_properties(maker.shape(), &mut properties, false, false).unwrap();
    assert!((properties.mass() - 2.07211e6).abs() / 2.07211e6 <= 0.01);
}
