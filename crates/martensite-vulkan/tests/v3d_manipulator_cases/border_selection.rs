//! Complete border-depth picking, including the nonselectable persistent cursor marker.
use super::prelude::*;
use crate::case_view::viewer_session::{ViewerSession, corner_persistence};
use crate::primitive_shapes::box_at;
use crate::viewer_objects::point_object;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_TOPMOST, Graphic3dCameraProjection, Graphic3dTransModeFlags,
};
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage as SelectionImage;

/// Replay both orthographic and perspective depth selections; `overlapped_border_selection()?`.
pub(crate) fn overlapped_border_selection() -> Outcome {
    banner(&[
        "# ===================================================================",
        "# 0027618: Visualization - selection returns entity overlapped by another entity on border cases",
        "# ===================================================================",
        "",
    ]);
    let mut run = SessionRun::new()?;
    border_scene(run.session())?;
    cursor_marker(run.session())?;
    orthographic_border(&mut run)?;
    perspective_border(&mut run)?;
    run.finish_without_image()
}

pub(super) fn border_scene(session: &mut ViewerSession) -> Outcome {
    let bottom = box_at([0.; 3], [0.1, 0.1, 0.05])?;
    let top = box_at([0.05, 0.05, 0.025], [0.02, 0.02, 0.04])?;
    session.clear()?;
    session.display_object("p0", point_object([0., 0., 0.])?)?;
    session.display_object("p1", point_object([0., 0.1, 0.05])?)?;
    let request = DisplayRequest::default()
        .with_display_mode(1)
        .with_highlight_mode(1);
    session.display("bb", &bottom, &request)?;
    session.display("bt", &top, &request)?;
    session.set_color("bt", Color::named(NamedColor::RED))?;
    session.set_axonometric()?;
    session.fit()?;
    Ok(())
}

pub(super) fn cursor_marker(session: &mut ViewerSession) -> Outcome {
    session.display_object("pp", point_object([245., -190., 0.])?)?;
    let point = session
        .object("pp")
        .cloned()
        .expect("pp; expected just-displayed cursor marker");
    let persistence = top_left_marker_persistence()?;
    session.update_context(|context| {
        context.set_z_layer(&point, GRAPHIC3D_ZLAYER_ID_TOPMOST);
        context.set_transform_persistence(&point, &persistence)?;
        context.display(&point, false)
    })?;
    session.set_object_selection_mode("pp", 0, false)?;
    Ok(())
}

fn border_checks(run: &mut SessionRun) -> Outcome {
    if run.session().pixel_color_name(235, 140)? == "CYAN2" {
        println!("Error: top should NOT be highlighted");
    }
    if run.session().pixel_color_name(235, 190)? != "CYAN2" {
        println!("Error: bottom should be highlighted");
    }
    Ok(())
}

fn border_dumps(run: &mut SessionRun, names: [&str; 2]) -> Outcome {
    run.selection_dump(names[0], SelectionImage::NORMALIZED_DEPTH)?;
    run.selection_dump(names[1], SelectionImage::COLORED_ENTITY)?;
    Ok(())
}

fn border_phase(run: &mut SessionRun, names: [&str; 2]) -> Outcome {
    run.session().move_cursor(245, 190)?;
    border_checks(run)?;
    border_dumps(run, names)
}

fn top_left_marker_persistence() -> Result<
    martensite_tkernel::standard::Handle<
        dyn martensite_tkservice::graphic3d::Graphic3dTransformPersSource,
    >,
    martensite_tkernel::standard::Failure,
> {
    corner_persistence(
        Graphic3dTransModeFlags::TMF_2D,
        AspectTypeOfTriedronPosition::LEFT_UPPER,
        [0, 0],
    )
}

fn orthographic_border(run: &mut SessionRun) -> Outcome {
    border_phase(run, ["bug27618_sel_depth.png", "bug27618_sel_entity.png"])
}

fn perspective_border(run: &mut SessionRun) -> Outcome {
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    run.session().move_cursor(0, 0)?;
    border_phase(
        run,
        [
            "bug27618_perps_sel_depth.png",
            "bug27618_persp_sel_entity.png",
        ],
    )
}

#[cfg(test)]
mod controls {
    use super::*;
    use martensite_tkservice::graphic3d::Graphic3dTransformPersMembers as _;
    use martensite_tkservice::null_driver::NullGraphicDriver;
    use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

    fn null_scene() -> ViewerSession {
        let mut session =
            ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap();
        border_scene(&mut session).unwrap();
        session
    }

    #[test]
    fn border_boxes_keep_explicit_display_and_highlight_modes() {
        let session = null_scene();
        for name in ["bb", "bt"] {
            let object = session.object(name).unwrap();
            assert!(session.context().is_displayed_mode(object, 1));
            assert_eq!(object.hilight_mode(), 1);
        }
        assert!(session.object("p0").is_some() && session.object("p1").is_some());
    }

    #[test]
    fn persistent_cursor_marker_is_topmost_and_has_no_active_selection() {
        let mut session = null_scene();
        cursor_marker(&mut session).unwrap();
        let point = session.object("pp").unwrap();
        let mut modes = Default::default();
        session.context().activated_modes(point, &mut modes);
        assert!(modes.is_empty());
        assert_eq!(
            session.context().get_z_layer(point),
            GRAPHIC3D_ZLAYER_ID_TOPMOST
        );
        assert_eq!(
            point.transform_persistence().mode(),
            Graphic3dTransModeFlags::TMF_2D
        );
        assert_eq!(
            point.transform_persistence().corner_2d().unwrap(),
            AspectTypeOfTriedronPosition::LEFT_UPPER
        );
    }
}
