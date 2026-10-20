//! Bnd_BoundSortBox: the face boxes of a BRepPrimAPI box.
//!
//! The math crate's bounding-box case that needs primitive shapes.
#![allow(non_snake_case)]
use martensite_tkbrep::topexp::Explorer;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::{BoundSortBox, BoundingBox3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;

/// The original case `InitializeWithFaceBoxes`.
///
/// `BRepBndLib::Add` of `BRepPrimAPI_MakeBox(1,1,1)` and of each of its six faces gives the
/// unit cube and its face rectangles, each enlarged by the vertex/edge/face tolerance
/// `Precision::Confusion()`; this now uses the original solid and face traversal, replacing the earlier surrogate.
#[test]
fn Bnd_BoundSortBox_Test__BUC60729_InitializeWithFaceBoxes() {
    let shape = BRepPrimAPIMakeBox::from_sizes(1.0, 1.0, 1.0)
        .unwrap()
        .solid()
        .unwrap();
    let mut aMainBox = BoundingBox3::default();
    BRepBndLib::add(&shape, &mut aMainBox, true).unwrap();

    // Initialize BoundSortBox with 6 boxes (for 6 faces of the cube)
    let aMaxNbrBox = 6;
    let mut aBoundSortBox = BoundSortBox::default();
    aBoundSortBox
        .initialize_with_count(&aMainBox, aMaxNbrBox)
        .unwrap();

    // Iterate through faces and add their bounding boxes
    let mut i = 1;
    for face in Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        let mut bounds = BoundingBox3::default();
        BRepBndLib::add(&face, &mut bounds, true).unwrap();
        aBoundSortBox.add(&bounds, i).unwrap();
        i += 1;
    }

    // Verify that 6 faces were processed
    assert_eq!(
        7, i,
        "Expected to process 6 faces (i should be 7 after loop)"
    );

    // The test passes if no exceptions occurred during initialization and addition
    // This test originally verified that the Bnd_BoundSortBox could handle
    // initialization and addition of multiple boxes without crashing
}
