//! Interactive objects other than shapes: displaying one under a name and moving it.
use super::ViewerSession;
use super::display::{DisplayRequest, display_new};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tkmath::toploc::Location;
use martensite_tkv3d::ais::AisInteractiveObject;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

impl ViewerSession {
    /// Display `object` under `name` with the context's default modes, replacing an object
    /// already bound to the name; `session.display_object("c1", object.cast())?`.
    pub fn display_object(
        &mut self,
        name: &str,
        object: Handle<dyn AisInteractiveObject>,
    ) -> Result<(), Failure> {
        if let Some(replaced) = self.objects.remove(name) {
            self.context.remove(&replaced, false)?;
        }
        self.objects.push(name, object.clone());
        let context = self.context.clone();
        self.with_update(|_| display_new(&context, &object, &DisplayRequest::default()))
    }

    /// Prepend a translation to the local transformation of the object `name`, as
    /// `SetLocation(translation * current)`; `session.translate_location("c1", [0., 0., 20.])?`.
    pub fn translate_location(&self, name: &str, [x, y, z]: [f64; 3]) -> Result<(), Failure> {
        let object = self.object(name).cloned().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?} not displayed; expected a displayed object name"
            ))
        })?;
        let current = object
            .get()
            .map_or_else(Transform3::default, |o| o.local_transformation());
        let mut translation = Transform3::default();
        translation.set_translation_part(Vector3::new(x, y, z));
        let location = Location::new(translation.multiplied(current));
        self.with_update(|context| context.set_location(&object, &location))
    }
}

impl ViewerSession {
    /// The child inherits the parent's transformation; then
    /// update. Example: `session.add_child("b1", "b2")?`.
    pub fn add_child(&self, parent: &str, child: &str) -> Result<(), Failure> {
        use martensite_tkv3d::prs_mgr::PrsMgrPresentableObject as _;
        let (parent, child) = (self.named(parent)?, self.named(child)?);
        let value = parent.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(
                "a null parent; expected a live object",
            )
        })?;
        value.add_child(&child.cast())?;
        self.with_update(|_| Ok(()))
    }

    /// The child keeps its world placement under
    /// the parent's transformation; then update. Example: `session.add_child_in_place("m", "c1")?`.
    pub fn add_child_in_place(&self, parent: &str, child: &str) -> Result<(), Failure> {
        use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
        let (parent, child) = (self.named(parent)?, self.named(child)?);
        let value = parent.get().ok_or_else(null_parent)?;
        value.add_child_with_current_transformation(&child.cast())?;
        self.with_update(|_| Ok(()))
    }

    /// The object no longer propagates its visual state (display,
    /// erase, highlight) to its children; then update. Example: `session.ignore_parent_visual_state("p")?`.
    pub fn ignore_parent_visual_state(&self, name: &str) -> Result<(), Failure> {
        use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
        let parent = self.named(name)?;
        parent
            .get()
            .ok_or_else(null_parent)?
            .set_propagate_visual_state(false);
        self.with_update(|_| Ok(()))
    }
}

fn null_parent() -> Failure {
    martensite_tkernel::standard::exceptions::program_error("a null parent; expected a live object")
}
