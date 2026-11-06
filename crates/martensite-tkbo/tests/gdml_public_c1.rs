//! Native replay of the GDML oblique rectangular revolution construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateCone, PerformCut, PerformFuse, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;

fn final_placement(
    mut shape: martensite_tkbrep::topods::Shape,
) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), [1., 0., 0.], 165.);
    Translate(&shape, 254., 381., 2195.46361349)
}

fn inverted_placement(
    mut shape: martensite_tkbrep::topods::Shape,
    z: f64,
) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), [1., 0., 0.], -180.);
    shape = Translate(&shape, 0., 0., z);
    final_placement(shape)
}

#[test]
fn GdmlPublic__C1() {
    let source = "tests/boolean/gdml_public/C1";
    let outer = final_placement(Translate(
        &CreateCone(469.61773982, 666.83785218, 736.03547960614355),
        0.,
        0.,
        1752.6372651538566,
    ));
    let inner = inverted_placement(
        Translate(
            &CreateCone(298.82011237, 666.83785218, 98.61005618565314),
            0.,
            0.,
            80.06860779434688,
        ),
        2667.3514087387584,
    );
    let upper = inverted_placement(
        Translate(
            &CreateCone(100.584, 470.63373982, 99.15452893970652),
            0.,
            0.,
            26.951401570293473,
        ),
        1878.4709592669262,
    );
    let lower = final_placement(Translate(
        &Translate(
            &CreateCone(100.584, 299.83611237, 743.6190068744344),
            0.,
            0.,
            375.3845984255655,
        ),
        0.,
        0.,
        1472.0709592705846,
    ));
    let fused = PerformFuse(&outer, &inner);
    assert!(!fused.is_null(), "{source}: fuse result");
    let upper_cut = PerformCut(&fused, &upper);
    assert!(!upper_cut.is_null(), "{source}: upper cut");
    let result = PerformCut(&upper_cut, &lower);
    assert!(!result.is_null(), "{source}: lower cut");
}
