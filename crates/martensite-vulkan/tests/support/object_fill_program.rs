//! One fill-program assignment path serves complete source bodies and the earlier shader fixtures.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::graphic3d::{Graphic3dGroupAspect, Graphic3dShaderProgram};
use martensite_tkv3d::{
    ais::{AisInteractiveContext, AisInteractiveObject},
    prs_mgr::PrsMgrPresentableObjectMembers,
};

/// Assign each actual object's fill program and refresh its aspects; `assign(&context, &[&object], program)?`.
pub(crate) fn assign(
    context: &AisInteractiveContext,
    objects: &[&Handle<dyn AisInteractiveObject>],
    program: impl Fn() -> Handle<Graphic3dShaderProgram>,
) -> Result<(), Failure> {
    for object in objects {
        let drawer = object.attributes();
        if drawer.set_shader_program(&program(), Graphic3dGroupAspect::FillArea, true)? {
            context.redisplay(object, false, false)?;
        } else {
            object.get().unwrap().synchronize_aspects();
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "object_fill_program_controls.rs"]
mod controls;
