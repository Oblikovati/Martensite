//! Resolve a live POINT aspect by presentation and type, never by rank or coordinates.
use super::*;
use crate::session_run::presentation_role_scope::PresentationAspectRoleScope;
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};
use martensite_tkv3d::{
    ais::AisInteractiveObjectMembers as _, prs_mgr::PrsMgrPresentableObjectMembers as _,
};
type Bindings = Vec<(usize, i32, usize, usize)>;

pub(super) struct PointLightingScope {
    role: PresentationAspectRoleScope,
    object: Handle<PointFrameText>,
    bindings: Bindings,
    pub(super) original: Handle<dyn Graphic3dAspectsSource>,
    pub(super) replacement: Handle<dyn Graphic3dAspectsSource>,
}
impl PointLightingScope {
    /// Resolve a computed owned POINT role before swapping; `PointLightingScope::new(session, &object)?`.
    pub(super) fn new(
        session: &ViewerSession,
        object: &Handle<PointFrameText>,
    ) -> io::Result<Self> {
        let group = point_group(session, object)?;
        let original = group.aspects();
        let replacement = unlit_clone(&original)?;
        let bindings = bindings(object);
        let role =
            PresentationAspectRoleScope::new(vec![group], original.clone(), replacement.clone())?;
        Ok(Self {
            role,
            object: object.clone(),
            bindings,
            original,
            replacement,
        })
    }
    /// Restore exact original bindings and refuse interference; `scope.restore()?`.
    pub(super) fn restore(&mut self) -> io::Result<()> {
        let interference = self.replacement.shading_model() != Graphic3dTypeOfShadingModel::Unlit;
        self.role.restore()?;
        if interference || bindings(&self.object) != self.bindings {
            return Err(io::Error::other(
                "point lighting bindings/model changed; expected exact owned inverse restoration",
            ));
        }
        Ok(())
    }
}
fn point_group(
    session: &ViewerSession,
    object: &Handle<PointFrameText>,
) -> io::Result<Handle<dyn Graphic3dGroup>> {
    require_context(session, object)?;
    let own = object.display_mode();
    let mode = if own >= 0 {
        own
    } else {
        session.context().display_mode()
    };
    let groups = existing_point_groups(object, mode);
    if groups.len() != 1 {
        return Err(io::Error::other(format!(
            "point role groups={}, mode={mode}; expected one existing POINT fill binding",
            groups.len()
        )));
    }
    Ok(groups[0].clone())
}
fn unlit_clone(
    original: &Handle<dyn Graphic3dAspectsSource>,
) -> io::Result<Handle<dyn Graphic3dAspectsSource>> {
    if original.is_null() || original.shading_model() != Graphic3dTypeOfShadingModel::Default {
        return Err(io::Error::other(format!(
            "point role model={:?}; expected original default effective PhongFacet",
            original.shading_model()
        )));
    }
    let replacement = Handle::new(Graphic3dAspectFillArea3d::default());
    *replacement.aspects_base().fields.write() = original.aspects_base().fields.read().clone();
    replacement.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    Ok(replacement.cast())
}
fn bindings(object: &Handle<PointFrameText>) -> Bindings {
    object
        .presentations()
        .into_iter()
        .flat_map(|presentation| {
            let identity = presentation.object_address() as usize;
            let mode = presentation.mode();
            presentation.groups().into_iter().map(move |group| {
                (
                    identity,
                    mode,
                    group.object_address() as usize,
                    group.aspects().object_address() as usize,
                )
            })
        })
        .collect()
}

fn require_context(session: &ViewerSession, object: &Handle<PointFrameText>) -> io::Result<()> {
    let actual = object.get_context().object_address();
    let expected = session.context().object_address();
    if actual != expected || !session.context().is_displayed(&object.cast()) {
        return Err(io::Error::other(format!(
            "point owner context={actual:p}; expected displayed context={expected:p}"
        )));
    }
    Ok(())
}

fn existing_point_groups(
    object: &Handle<PointFrameText>,
    mode: i32,
) -> Vec<Handle<dyn Graphic3dGroup>> {
    object
        .presentations()
        .into_iter()
        .filter(|presentation| presentation.mode() == mode)
        .flat_map(|presentation| presentation.groups())
        .filter(is_point_fill_group)
        .collect()
}
fn is_point_fill_group(group: &Handle<dyn Graphic3dGroup>) -> bool {
    let aspect = group.aspects().cast::<Graphic3dAspectFillArea3d>();
    !aspect.is_null() && aspect.interior_style() == AspectInteriorStyle::Point
}
