//! Where the view cube is drawn: next to a far object, with orthographic persistence, and
//! in only some of the views.
use super::prelude::*;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_DEFAULT, Graphic3dTransModeFlags, Graphic3dTransformPers,
    Graphic3dTransformPersMembers as _,
};
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};

const RULE: &str = "==================================";

/// Keep the cube's corner anchoring and add orthographic persistence, so a perspective
/// camera does not distort it; `persist_orthographic(&cube)?`.
pub(super) fn persist_orthographic(cube: &Handle<AisViewCube>) -> Outcome {
    let current = cube.transform_persistence();
    let current = current
        .get()
        .ok_or("view cube without transform persistence")?;
    let flags = Graphic3dTransModeFlags::TRIEDRON_PERS | Graphic3dTransModeFlags::ORTHO_PERS;
    let ortho =
        Graphic3dTransformPers::with_corner(flags, current.corner_2d()?, current.offset_2d()?)?;
    cube.set_transform_persistence(&Handle::new(ortho).cast());
    Ok(())
}

/// `largedist`: a tiny box a million units from the origin, with the layer origin moved
/// next to it, keeps the cube's labels steady.
pub(crate) fn far_origin_cube() -> Outcome {
    banner(&[
        RULE,
        "0030850: Visualization, GlText - text within trihedron persistence jitters when camera is far from origin",
        RULE,
    ]);
    let b = box_sized([0.001, 0.001, 0.001])?;
    let mut run = SessionRun::new()?;
    run.session().set_axonometric()?;
    let request = DisplayRequest::default().with_display_mode(1);
    run.session().display("b", &b, &request)?;
    run.session()
        .translate_location("b", [1_000_000., 0., 0.])?;
    run.session().fit()?;
    run.session()
        .set_z_layer_origin(GRAPHIC3D_ZLAYER_ID_DEFAULT, [1_000_000., 0., 0.])?;
    let cube = run.session().view_cube("vc")?;
    run.session().display_object("vc", cube.cast())?;
    run.dump("largedist.png")?;
    run.finish()
}

/// `view`: a cube hidden in the second view stays visible in the first.
pub(crate) fn per_view_visibility() -> Outcome {
    banner(&[
        RULE,
        "0028954: Visualization - implement interactive object AisViewCube for camera manipulations",
        "Check view affinity",
        RULE,
    ]);
    let mut run = SessionRun::new()?;
    run.session().open_view(409, 409)?;
    let cube = run.session().view_cube("vc")?;
    run.session().display_object("vc", cube.cast())?;
    run.session().hide_in_active_view("vc")?;
    let hidden = "Error: hiding Cube in View2 fails.";
    expect_pixel(run.session(), (100, 350), "BLACK", true, hidden)?;
    run.dump("view_v2.png")?;
    run.session().activate_view(0)?;
    let shown = "Error: showing Cube in View1 fails.";
    expect_pixel(run.session(), (100, 350), "BLACK", false, shown)?;
    run.dump("view_v1.png")?;
    run.session().activate_view(1)?;
    run.finish()
}
