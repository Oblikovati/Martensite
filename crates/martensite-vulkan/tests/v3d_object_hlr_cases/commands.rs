//! Thin typed equivalents of VHLRType and the all-object VAspects width command.
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::{Failure, Handle, exceptions::program_error};
use martensite_tkv3d::{
    ais::{AisShape, AisShapeParameters},
    prs_mgr::PrsMgrPresentableObjectMembers,
    prs3d::Prs3dTypeOfHlr,
};

pub(super) fn shape(session: &ViewerSession, name: &str) -> Result<Handle<AisShape>, Failure> {
    let object = session.object(name).ok_or_else(|| {
        program_error(&format!(
            "vhlrtype name={name:?}; expected a bound AisShape"
        ))
    })?;
    let shape = object.cast::<AisShape>();
    if shape.is_null() {
        return Err(program_error(&format!(
            "vhlrtype name={name:?}; expected AisShape, found other presentation"
        )));
    }
    Ok(shape)
}

pub(super) fn set_object_algorithm(
    session: &ViewerSession,
    name: &str,
    kind: Prs3dTypeOfHlr,
) -> Result<(), Failure> {
    require_algorithm(kind)?;
    let shape = shape(session, name)?;
    let recompute = shape.type_of_hlr() != kind && session.view().view_base().computed_mode();
    session.update_view(|_| {
        // Explicit object arguments establish an own type even if inheritance already agrees.
        shape.set_type_of_hlr(kind);
        if recompute {
            session.context().redisplay(&shape.cast(), false, false)?;
        }
        Ok(())
    })
}

pub(super) fn set_all_line_width(
    session: &ViewerSession,
    names: &[&str],
    width: f64,
) -> Result<(), Failure> {
    let objects = names
        .iter()
        .map(|name| shape(session, name).map(|s| s.cast()))
        .collect::<Result<Vec<_>, _>>()?;
    session.update_context(|context| {
        for object in &objects {
            context.set_width(object, width, false)?;
            object.synchronize_aspects();
        }
        Ok(())
    })
}

fn require_algorithm(kind: Prs3dTypeOfHlr) -> Result<(), Failure> {
    match kind {
        Prs3dTypeOfHlr::Algo | Prs3dTypeOfHlr::PolyAlgo => Ok(()),
        invalid => Err(program_error(&format!(
            "vhlrtype algorithm={invalid:?}; expected Algo or PolyAlgo"
        ))),
    }
}
