//! Native replay of the GDML partial-revolution cone construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{PerformCut, PerformFuse, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCone;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn partial_cone(
    r1: f64,
    r2: f64,
    height: f64,
    seam_degrees: f64,
    z: f64,
) -> martensite_tkbrep::topods::Shape {
    let mut maker = BRepPrimAPIMakeCone::with_angle(r1, r2, height, 300_f64.to_radians())
        .expect("GDML B9 partial cone");
    let mut shape = maker.shape().expect("GDML B9 partial cone shape").clone();
    RotateAroundAxis(&mut shape, Point3::default(), [0., 0., 1.], seam_degrees);
    Translate(&shape, 0., 0., z)
}

fn final_placement(
    mut shape: martensite_tkbrep::topods::Shape,
) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), [1., 0., 0.], -161.32582176);
    Translate(&shape, 0., -746.52181701, 1844.22896887)
}

fn inverted_placement(
    mut shape: martensite_tkbrep::topods::Shape,
    shift: f64,
) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), [1., 0., 0.], -180.);
    shape = Translate(&shape, 0., 0., shift);
    final_placement(shape)
}

#[test]
fn GdmlPublic__B9() {
    let source = "tests/boolean/gdml_public/B9";
    let outer = final_placement(partial_cone(
        378.9195177,
        466.55429973,
        487.95842250363467,
        120.,
        2109.8582757863655,
    ));
    let inner = inverted_placement(
        partial_cone(
            176.81924281,
            466.55429973,
            52.03490172044269,
            -60.,
            31.755811739557313,
        ),
        2681.60741175578,
    );
    let upper = inverted_placement(
        partial_cone(
            88.29261617,
            379.81136231,
            52.355242979245794,
            -60.,
            15.856892340754207,
        ),
        2177.9102405158328,
    );
    let lower = final_placement(Translate(
        &partial_cone(
            88.29261617,
            177.71108741,
            497.8901659718403,
            120.,
            491.62130272815972,
        ),
        0.,
        0.,
        1665.3060030573038,
    ));
    let fused = PerformFuse(&outer, &inner);
    assert!(!fused.is_null(), "{source}: fuse result");
    let upper_cut = PerformCut(&fused, &upper);
    assert!(!upper_cut.is_null(), "{source}: upper cut");
    let result = PerformCut(&upper_cut, &lower);
    assert!(!result.is_null(), "{source}: lower cut");
}
