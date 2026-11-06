//! Face selection of a restored shape with the selection BVH built by a background thread.
use super::prelude::*;
use crate::dataset_shape::read_dataset_shape;
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorMembers as _;

/// `v3d/mesh/C1`: with background BVH building on one thread, a rubber band over the whole
/// view selects every face of the motor part once the build has finished.
pub(crate) fn prebuilt_face_rectangle() -> Result<(), Box<dyn Error>> {
    // The grid's `begin` opens the view before the case runs.
    let mut run = SessionRun::new()?;
    banner(&[
        "========",
        "OCC31757: Visualization - Prebuild BVH for Select3DSensitiveEntity in separate threads",
        "========",
    ]);
    run.session().clear()?;
    let s = read_dataset_shape("occ/Motor-c.brep")?;
    show(&mut run, "s", &s)?;
    run.session().fit()?;
    // Build the selection BVH on one thread, then wait for it after activating faces.
    run.session()
        .context()
        .main_selector()
        .set_to_prebuild_bvh(true, 1);
    run.session().set_object_selection_mode("s", 4, true)?;
    run.session().context().main_selector().wait_for_bvh_build();
    run.session().select_pixels(&[(0, 0), (512, 512)], false)?;
    run.dump("C1.png")?;
    run.finish()
}
