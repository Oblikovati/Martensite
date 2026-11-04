//! Shapes and trihedrons kept at a window corner by trihedron persistence.
use super::prelude::*;
use martensite_tkmath::{gp::Transform3, gp::Vector3, toploc::Location};
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use martensite_tkservice::graphic3d::Graphic3dTransModeFlags as Flags;
use martensite_tkservice::graphic3d::{Graphic3dCameraProjection, Graphic3dStereoMode};
use martensite_tkv3d::v3d::V3dStereoDumpOptions;

/// `bug23120`: a centred trihedron-persistent box keeps its place through zoom and pan.
pub(crate) fn centered_trihedron_box() -> Outcome {
    banner(&[
        "============",
        "OCC23120: Transform persistence is not restored",
        "============",
        "",
    ]);
    let b1 = box_sized([10., 20., 30.])?;
    let mut offset = Transform3::default();
    offset.set_translation_part(Vector3::new(50., 50., 0.));
    let b2 = box_sized([10., 20., 30.])?.moved(&Location::new(offset), true)?;
    let mut run = SessionRun::new()?;
    let centred = corner_persistence(Flags::TRIEDRON_PERS, Corner::CENTER, [0, 0])?;
    run.session()
        .display_persistent("b1", &b1, centred, &DisplayRequest::default())?;
    run.session()
        .display("b2", &b2, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    run.session().zoom(0.5)?;
    run.session().pan(100, 0)?;
    if !run.session().check_color(300, 130, [0.78, 0.55, 0.9])? {
        println!("Error : Transform persistence is not restored");
    }
    run.dump("bug23120.png")?;
    run.finish()
}

/// `bug27817`: a face of a trihedron-persistent box highlights under the cursor.
pub(crate) fn trihedron_face_highlight() -> Outcome {
    banner(&[
        "========",
        "StdSelectBRepOwner - copy transformation persistence flag",
        "========",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().display_trihedron("t")?;
    let corner = || corner_persistence(Flags::TRIEDRON_PERS, Corner::LEFT_LOWER, [40, 40]);
    run.session().redisplay_persistent("t", corner()?)?;
    let b = box_sized([30., 40., 50.])?;
    let request = DisplayRequest::default()
        .with_display_mode(1)
        .with_highlight_mode(1);
    run.session()
        .display_persistent("b", &b, corner()?, &request)?;
    run.session().set_object_selection_mode("b", 4, true)?;
    run.session().move_cursor(50, 350)?;
    if run.session().pixel_color_name(50, 350)? != "CYAN2" {
        println!("Error: face is not highlighted");
    }
    run.dump("bug27817.png")?;
    run.finish()
}

/// `bug27793`: trihedron-persistent trihedrons keep their corners under a strong zoom of an
/// anaglyph stereo perspective view, dumped as the blended stereo pair.
pub(crate) fn stereo_corner_trihedrons() -> Outcome {
    banner(&[
        "========",
        "Object drifts at zoom within Graphic3dTMF_TriedronPers applied",
        "========",
    ]);
    let b = box_sized([100., 200., 300.])?;
    let mut run = SessionRun::new()?;
    place_stereo_corner_trihedrons(&mut run, &b)?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    run.session()
        .set_stereo_mode(Graphic3dStereoMode::Anaglyph)?;
    run.session().fit()?;
    run.session().zoom(0.05)?;
    run.dump_stereo("bug27793.png", V3dStereoDumpOptions::Blended)?;
    run.finish()
}

fn place_stereo_corner_trihedrons(
    run: &mut SessionRun,
    shape: &martensite_tkbrep::topods::Shape,
) -> Outcome {
    run.session().clear()?;
    run.session().display_trihedron("t1")?;
    run.session().display_trihedron("t2")?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("b", shape, &shaded)?;
    run.session().set_axonometric()?;
    let corner = |at| corner_persistence(Flags::TRIEDRON_PERS, at, [100, 100]);
    run.session()
        .redisplay_persistent("t1", corner(Corner::LEFT_LOWER)?)?;
    run.session()
        .redisplay_persistent("t2", corner(Corner::RIGHT_UPPER)?)?;
    run.session().set_location("t2", [0., 0., -50.])?;
    Ok(())
}
