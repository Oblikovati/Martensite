//! Shared persistence per authored display command retains actual object identities.
use super::*;
use crate::case_view::viewer_session::{DisplayRequest, corner_persistence};
use crate::primitive_shapes::box_sized;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::{Graphic3dNameOfMaterial, Graphic3dTransModeFlags};

const DISPLAY_NAMES: [&str; 18] = [
    "b1", "b2", "b3", "b4", "s", "t", "p1", "p2", "p3", "p4", "p5", "p6", "p7", "p8", "p9", "p10",
    "p11", "p12",
];

pub(super) fn initial(session: &mut ViewerSession, shapes: &[(String, Shape)]) -> Outcome {
    session.clear()?;
    initial_objects(session, shapes)?;
    materials_and_colors(session)?;
    session.set_location("s", [0., 0., -1.])?;
    session.display(
        "bb",
        &box_sized([1000.; 3])?,
        &DisplayRequest::default().with_display_mode(0),
    )?;
    session.fit()?;
    Ok(())
}

fn initial_objects(session: &mut ViewerSession, shapes: &[(String, Shape)]) -> Outcome {
    let persistence = corner_persistence(Graphic3dTransModeFlags::TMF_2D, Corner::CENTER, [0, 0])?;
    let request = DisplayRequest::default()
        .with_display_mode(1)
        .with_highlight_mode(1);
    session.update_session(|session| {
        for name in DISPLAY_NAMES {
            let (_, shape) = shapes
                .iter()
                .find(|(key, _)| key == name)
                .expect("source object name; expected constructed input shape");
            session.display_persistent(name, shape, persistence.clone(), &request)?;
        }
        Ok(())
    })?;
    Ok(())
}

fn materials_and_colors(session: &mut ViewerSession) -> Outcome {
    session.update_session(|session| {
        for name in ["b1", "b2", "b3", "b4", "t"] {
            session.set_material(name, Graphic3dNameOfMaterial::Plastified)?;
        }
        Ok(())
    })?;
    for (name, color) in [
        ("b1", NamedColor::RED),
        ("b2", NamedColor::GREEN),
        ("b3", NamedColor::BLUE),
        ("b4", NamedColor::MAGENTA),
        ("t", NamedColor::GOLD),
    ] {
        session.set_color(name, Color::named(color))?;
    }
    Ok(())
}

pub(super) fn reanchor(session: &mut ViewerSession, corner: Corner) -> Outcome {
    let persistence = corner_persistence(Graphic3dTransModeFlags::TMF_2D, corner, [0, 0])?;
    session.update_session(|session| {
        for name in DISPLAY_NAMES {
            session.redisplay_persistent(name, persistence.clone())?;
        }
        Ok(())
    })?;
    Ok(())
}
