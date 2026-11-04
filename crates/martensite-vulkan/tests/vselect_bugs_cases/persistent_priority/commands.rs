//! Case-local command boundaries preserve existing-object display and actual owner mutation.
use super::*;
use crate::case_view::viewer_session::{HighlightStyle, corner_persistence};
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_TOP, Graphic3dDisplayPriority, Graphic3dNameOfMaterial,
    Graphic3dTransModeFlags, Graphic3dTransformPersSource,
};
use martensite_tkv3d::{
    prs_mgr::PrsMgrPresentableObjectMembers as _, prs3d::Prs3dTypeOfHighlight,
    select_mgr::SelectMgrEntityOwnerMembers as _,
};
pub(super) const PLASTIC: Graphic3dNameOfMaterial = Graphic3dNameOfMaterial::Plastified;

/// Keep the source global style separate from point-owned overrides; `white_dynamic(session)?`.
pub(super) fn white_dynamic(session: &ViewerSession) -> Result<(), Failure> {
    let style = HighlightStyle {
        display_mode: Some(1),
        color: Some(Color::named(NamedColor::WHITE)),
        ..HighlightStyle::default()
    };
    session.set_highlight_style(Prs3dTypeOfHighlight::Dynamic, &style)
}

/// Build the exact pixel-anchor policy without baking geometry; `top_left()?`.
pub(super) fn top_left() -> Result<Handle<dyn Graphic3dTransformPersSource>, Failure> {
    let flags = Graphic3dTransModeFlags::TMF_2D;
    let corner = AspectTypeOfTriedronPosition::LEFT_UPPER;
    corner_persistence(flags, corner, [0, 0])
}

/// Redisplay the existing object without replacing its selection identity; `persistent_existing(session,"tri1",Some(1))?`.
pub(super) fn persistent_existing(
    session: &ViewerSession,
    name: &str,
    mode: Option<i32>,
) -> Result<(), Failure> {
    let object = named(session, name)?;
    let persistence = top_left()?;
    session.update_context(|context| {
        context.set_z_layer(&object, GRAPHIC3D_ZLAYER_ID_TOP);
        context.set_transform_persistence(&object, &persistence)?;
        // Native existing-object VDisplay calls the real guarded setter, not a mode validator.
        if let Some(mode) = mode {
            object.set_display_mode(mode);
        }
        context.display(&object, false)
    })
}

/// Mutate the actual source global owner, never a replacement; `owner_priority(session,"tri1",6)?`.
pub(super) fn owner_priority(
    session: &ViewerSession,
    name: &str,
    priority: i32,
) -> Result<(), Failure> {
    let object = named(session, name)?;
    let owner = object.global_sel_owner();
    if owner.is_null() {
        return Err(program_error(&format!(
            "object {name:?} owner=null; expected actual global selection owner"
        )));
    }
    session.update_context(|_| {
        owner.set_priority(priority);
        Ok(())
    })
}

/// Keep presentation priority independent of owner priority; `display_priority(session,"box_1",5)?`.
pub(super) fn display_priority(
    session: &ViewerSession,
    name: &str,
    priority: i32,
) -> Result<(), Failure> {
    let value = match priority {
        4 => Graphic3dDisplayPriority::Below,
        5 => Graphic3dDisplayPriority::Normal,
        6 => Graphic3dDisplayPriority::Above,
        other => {
            return Err(program_error(&format!(
                "display priority {other}; expected original value4,5,6"
            )));
        }
    };
    session.set_display_priority(name, value)
}

/// Preserve exact named-pixel predicates and literal diagnostics; `pixel(run,(10,20),"CYAN",true,"Error")?`.
pub(super) fn pixel(
    run: &mut SessionRun,
    xy: (i32, i32),
    color: &str,
    required: bool,
    message: &str,
) -> Outcome {
    let actual = run.session().pixel_color_name(xy.0, xy.1)?;
    if pixel_is_fault(&actual, color, required) {
        println!("{message}");
    }
    Ok(())
}

/// Keep the original negative CYAN test distinct from equality; `pixel_is_fault("CYAN","CYAN",false)`.
pub(super) fn pixel_is_fault(actual: &str, color: &str, required: bool) -> bool {
    if required {
        return actual != color;
    }
    actual == color
}

/// Preserve move/check/dump order as one source probe; `white_dump(run,(1,2),(3,4),"Error","a.png")?`.
pub(super) fn white_dump(
    run: &mut SessionRun,
    cursor: (i32, i32),
    pixel_at: (i32, i32),
    message: &str,
    image: &'static str,
) -> Outcome {
    run.session().move_cursor(cursor.0, cursor.1)?;
    pixel(run, pixel_at, "WHITE", true, message)?;
    run.dump(image)
}
