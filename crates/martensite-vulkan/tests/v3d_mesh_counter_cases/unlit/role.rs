//! Bind the existing shaded sphere presentation through exact aspect identity.
use super::*;
use martensite_tkv3d::ais::{AisShape, AisShapeSource};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;

pub(super) struct SphereAspectRole {
    pub object: Handle<dyn AisInteractiveObject>,
    pub groups: Vec<Handle<dyn Graphic3dGroup>>,
    pub aspect: Handle<dyn Graphic3dAspectsSource>,
}
impl SphereAspectRole {
    /// Resolve existing named shading bindings without computing; `SphereAspectRole::resolve(session)?`.
    pub fn resolve(session: &ViewerSession) -> io::Result<Self> {
        let object = resolve_sphere(session)?;
        let aspect = resolve_shading(&object)?;
        let groups = matching_groups(&object, &aspect)?;
        Ok(Self {
            object,
            groups,
            aspect,
        })
    }
    /// Borrow the actual shape identity for mesh checks; `role.shape()`.
    pub fn shape(&self) -> martensite_tkbrep::topods::Shape {
        self.object
            .cast::<AisShape>()
            .shape_base()
            .fields
            .read()
            .shape
            .clone()
    }
    /// Apply an exact clone with Unlit shading; `let scope = role.begin()?`.
    pub fn begin(&self) -> io::Result<role_scope::PresentationAspectRoleScope> {
        role_scope::PresentationAspectRoleScope::new(
            self.groups.clone(),
            self.aspect.clone(),
            unlit_clone(&self.aspect),
        )
    }
}
fn matching_groups(
    object: &Handle<dyn AisInteractiveObject>,
    aspect: &Handle<dyn Graphic3dAspectsSource>,
) -> io::Result<Vec<Handle<dyn Graphic3dGroup>>> {
    let presentation = mode_one_presentation(object)?;
    let existing = presentation.groups();
    if existing.iter().any(|group| group.is_null()) {
        return Err(refused("mode1 contains null group; expected live bindings"));
    }
    let groups: Vec<_> = existing
        .into_iter()
        .filter(|group| group.aspects() == *aspect)
        .collect();
    validate_groups(&groups)?;
    Ok(groups)
}
/// Clone every aspect field before selecting Unlit; `let clone = unlit_clone(&role)`.
pub(super) fn unlit_clone(
    original: &Handle<dyn Graphic3dAspectsSource>,
) -> Handle<dyn Graphic3dAspectsSource> {
    let clone = Handle::new(Graphic3dAspects::default());
    *clone.aspects_base().fields.write() = original.aspects_base().fields.read().clone();
    clone.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    clone.cast()
}

fn resolve_sphere(session: &ViewerSession) -> io::Result<Handle<dyn AisInteractiveObject>> {
    let object = session
        .object("s")
        .cloned()
        .ok_or_else(|| refused("s absent"))?;
    if object.cast::<AisShape>().is_null() || !session.context().is_displayed_mode(&object, 1) {
        return Err(refused(format!(
            "s={:?}; expected displayed mode1 AIS shape",
            object.object_address()
        )));
    }
    Ok(object)
}
fn resolve_shading(
    object: &Handle<dyn AisInteractiveObject>,
) -> io::Result<Handle<dyn Graphic3dAspectsSource>> {
    let drawer = object.attributes();
    if drawer.is_null() {
        return Err(refused("s drawer null; expected actual shading drawer"));
    }
    let shading = drawer.shading_aspect();
    if shading.is_null() || shading.aspect().is_null() {
        return Err(refused("s shading aspect null"));
    }
    let aspect = shading.aspect().cast::<dyn Graphic3dAspectsSource>();
    if aspect.shading_model() == Graphic3dTypeOfShadingModel::Unlit {
        return Err(refused("source shading model Unlit; expected lit source"));
    }
    Ok(aspect)
}
fn mode_one_presentation(
    object: &Handle<dyn AisInteractiveObject>,
) -> io::Result<Handle<martensite_tkv3d::prs_mgr::PrsMgrPresentation>> {
    let existing = object.presentations();
    if existing.iter().any(|presentation| presentation.is_null()) {
        return Err(refused("presentation=null; expected live computed mode1"));
    }
    let mut presentations = existing.into_iter().filter(|p| p.mode() == 1);
    let first = presentations
        .next()
        .ok_or_else(|| refused("mode1 presentation count0; expected1"))?;
    let extra = presentations.count();
    if extra != 0 {
        return Err(refused(format!(
            "mode1 presentation count={}; expected1",
            extra + 1
        )));
    }
    Ok(first)
}

fn validate_groups(groups: &[Handle<dyn Graphic3dGroup>]) -> io::Result<()> {
    let unique: std::collections::BTreeSet<_> = groups
        .iter()
        .map(|group| group.object_address() as usize)
        .collect();
    if groups.is_empty() || unique.len() != groups.len() {
        return Err(refused(format!(
            "matched groups={}, unique={}; expected nonempty distinct bindings",
            groups.len(),
            unique.len()
        )));
    }
    Ok(())
}
