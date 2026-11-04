//! Exact role replacement preserves group identity and restores original aspect handles.
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;

/// An exact-handle role swap, independent of the caller's semantic resolver.
pub struct PresentationAspectRoleScope {
    groups: Vec<Handle<dyn Graphic3dGroup>>,
    original: Handle<dyn Graphic3dAspectsSource>,
    replacement: Handle<dyn Graphic3dAspectsSource>,
    active: bool,
}
impl PresentationAspectRoleScope {
    /// Replace only exact role bindings; `PresentationAspectRoleScope::new(groups, role, clone)?`.
    pub fn new(
        groups: Vec<Handle<dyn Graphic3dGroup>>,
        original: Handle<dyn Graphic3dAspectsSource>,
        replacement: Handle<dyn Graphic3dAspectsSource>,
    ) -> std::io::Result<Self> {
        validate_scope(&groups, &original, &replacement)?;
        let mut scope = Self {
            groups,
            replacement,
            original,
            active: false,
        };
        scope.activate()?;
        Ok(scope)
    }
    fn activate(&mut self) -> std::io::Result<()> {
        self.require(&self.original)?;
        self.active = true;
        self.replace(&self.original, &self.replacement);
        self.require(&self.replacement)
    }
    fn require(&self, expected: &Handle<dyn Graphic3dAspectsSource>) -> std::io::Result<()> {
        if self
            .groups
            .iter()
            .any(|g| g.is_null() || g.aspects().object_address() != expected.object_address())
        {
            return Err(std::io::Error::other(
                "role group binding changed; expected recorded aspect handle",
            ));
        }
        Ok(())
    }
    fn replace(
        &self,
        from: &Handle<dyn Graphic3dAspectsSource>,
        to: &Handle<dyn Graphic3dAspectsSource>,
    ) {
        let map = aspect_map(from, to);
        for group in &self.groups {
            group.replace_aspects(&map);
        }
    }
    /// Restore owned defaults but refuse interference; `scope.restore()?`.
    pub fn restore(&mut self) -> std::io::Result<()> {
        let interference = self.require(&self.replacement);
        self.restore_owned_defaults();
        let restoration = self.require(&self.original);
        self.active = restoration.is_err();
        restoration?;
        interference
    }
    fn restore_owned_defaults(&self) {
        self.replace(&self.replacement, &self.original);
        for group in &self.groups {
            if group.aspects() != self.original {
                group.set_group_primitives_aspect(&self.original);
            }
        }
    }
}
impl Drop for PresentationAspectRoleScope {
    fn drop(&mut self) {
        if self.active {
            self.restore_owned_defaults();
        }
    }
}

fn aspect_map(
    previous: &Handle<dyn Graphic3dAspectsSource>,
    replacement: &Handle<dyn Graphic3dAspectsSource>,
) -> Graphic3dAspectsReplaceMap {
    [(
        previous.object_address() as usize,
        (previous.clone(), replacement.clone()),
    )]
    .into()
}

fn validate_scope(
    groups: &[Handle<dyn Graphic3dGroup>],
    original: &Handle<dyn Graphic3dAspectsSource>,
    replacement: &Handle<dyn Graphic3dAspectsSource>,
) -> std::io::Result<()> {
    if original.is_null() || replacement.is_null() || groups.is_empty() {
        return Err(std::io::Error::other(
            "role null or groups empty; expected computed role bindings",
        ));
    }
    Ok(())
}
