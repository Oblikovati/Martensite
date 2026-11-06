//! Presentation-local control removes only groups sharing the object's boundary aspect.
#[path = "presentation_aspect_scope.rs"]
mod aspect_scope;
pub use aspect_scope::PresentationAspectRoleScope;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::{ais::AisInteractiveObject, prs_mgr::PrsMgrPresentableObjectMembers};

pub struct BoundaryPatternScope {
    _role: PresentationLineRoleScope,
}
impl BoundaryPatternScope {
    /// Hide matching mode-zero boundaries; `let scope = BoundaryPatternScope::new(&object)`.
    pub fn new(object: &Handle<dyn AisInteractiveObject>) -> Self {
        let role = object
            .attributes()
            .un_free_boundary_aspect()
            .aspect()
            .cast();
        let groups = boundary_groups(object, &role);
        Self {
            _role: PresentationLineRoleScope::new(groups, role)
                .expect("boundary control expected existing matching mode-zero groups"),
        }
    }
}

/// An exact-handle line control, retaining the existing empty-line behavior.
pub struct PresentationLineRoleScope {
    scope: PresentationAspectRoleScope,
}
impl PresentationLineRoleScope {
    /// Hide only resolved role bindings; `PresentationLineRoleScope::new(groups, role)?`.
    pub fn new(
        groups: Vec<Handle<dyn Graphic3dGroup>>,
        original: Handle<dyn Graphic3dAspectsSource>,
    ) -> std::io::Result<Self> {
        let replacement = empty_clone(&original);
        Ok(Self {
            scope: PresentationAspectRoleScope::new(groups, original, replacement)?,
        })
    }
    /// Restore exact original handles and report interference; `scope.restore()?`.
    pub fn restore(&mut self) -> std::io::Result<()> {
        self.scope.restore()
    }
}

fn empty_clone(
    previous: &Handle<dyn Graphic3dAspectsSource>,
) -> Handle<dyn Graphic3dAspectsSource> {
    if previous.is_null() {
        return Handle::default();
    }
    let cloned = Handle::new(Graphic3dAspects::default());
    *cloned.aspects_base().fields.write() = previous.aspects_base().fields.read().clone();
    cloned.set_line_type(martensite_tkservice::aspect::AspectTypeOfLine::Empty);
    cloned.cast()
}

fn boundary_groups(
    object: &Handle<dyn AisInteractiveObject>,
    previous: &Handle<dyn Graphic3dAspectsSource>,
) -> Vec<Handle<dyn Graphic3dGroup>> {
    object
        .presentations()
        .into_iter()
        .filter(|presentation| presentation.mode() == 0)
        .flat_map(|presentation| presentation.groups())
        .filter(|group| group.aspects().object_address() == previous.object_address())
        .collect()
}

/// Capture exact presentation bindings; `let before = aspect_bindings(&object)`.
pub fn aspect_bindings(object: &Handle<dyn AisInteractiveObject>) -> Vec<(i32, usize)> {
    object
        .presentations()
        .into_iter()
        .flat_map(|presentation| {
            let mode = presentation.mode();
            presentation
                .groups()
                .into_iter()
                .map(move |group| (mode, group.aspects().object_address() as usize))
        })
        .collect()
}
