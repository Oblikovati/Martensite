//! Resolve the existing operation handle and its typed frame aspect without computation.
use super::*;
use martensite_tkv3d::{ais::AisInteractiveObjectMembers, prs_mgr::PrsMgrPresentation};
pub(super) struct FrameRole {
    pub scale: Handle<AisColorScale>,
    pub presentation: Handle<PrsMgrPresentation>,
    pub group: Handle<dyn Graphic3dGroup>,
    pub aspect: Handle<dyn Graphic3dAspectsSource>,
}
impl FrameRole {
    pub(super) fn resolve(
        session: &ViewerSession,
        scale: &Handle<AisColorScale>,
    ) -> io::Result<Self> {
        require_operation_scale(session, scale)?;
        let presentation = existing_scale_presentation(session, scale)?;
        let group = unique_frame_group(presentation.groups())?;
        Ok(Self {
            scale: scale.clone(),
            presentation,
            aspect: group.aspects(),
            group,
        })
    }
}
fn require_operation_scale(
    session: &ViewerSession,
    scale: &Handle<AisColorScale>,
) -> io::Result<()> {
    if !scale
        .dynamic_type()
        .is_some_and(|kind| kind.name() == "AisColorScale")
        || scale.get_context() != *session.context()
        || !session.context().is_displayed_mode(&scale.cast(), 0)
    {
        return Err(refused(
            "scale type/context/display differs; expected actual displayed mode0 operation handle",
        ));
    }
    Ok(())
}
fn existing_scale_presentation(
    session: &ViewerSession,
    scale: &Handle<AisColorScale>,
) -> io::Result<Handle<PrsMgrPresentation>> {
    let presentation = session
        .context()
        .main_prs_mgr()
        .presentation(&scale.cast(), 0, false, &Handle::default())
        .map_err(|error| refused(error.to_string()))?;
    if presentation.is_null() || presentation.must_be_updated() || !presentation.is_displayed() {
        return Err(refused(
            "mode0 absent/dirty/hidden; expected existing computed presentation",
        ));
    }
    Ok(presentation)
}
pub(super) fn unique_frame_group(
    groups: Vec<Handle<dyn Graphic3dGroup>>,
) -> io::Result<Handle<dyn Graphic3dGroup>> {
    let matches: Vec<_> = groups.into_iter().filter(is_frame_group).collect();
    if matches.len() != 1 {
        return Err(refused(format!(
            "typed frame bindings {}; expected exactly one",
            matches.len()
        )));
    }
    let group = matches[0].clone();
    if group.aspects().line_type() == martensite_tkservice::aspect::AspectTypeOfLine::Empty {
        return Err(refused(
            "frame is Empty; expected drawable actual frame aspect",
        ));
    }
    Ok(group)
}
fn is_frame_group(group: &Handle<dyn Graphic3dGroup>) -> bool {
    if group.is_null() {
        return false;
    }
    group
        .aspects()
        .dynamic_type()
        .is_some_and(|kind| kind.name() == "Graphic3d_AspectLine3d")
}
