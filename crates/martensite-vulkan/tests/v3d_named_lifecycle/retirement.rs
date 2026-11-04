//! Removed namespaces retire real owners without discarding their earlier capture resources.
use super::*;

impl<F: SourceDriverFactory> NamedViewerScene<F> {
    pub(super) fn retire_driver(&mut self, core: &Handle<dyn Graphic3dGraphicDriver>) {
        let Some(index) = self.owners.iter().position(|owner| &owner.core == core) else {
            return;
        };
        self.retired_owners.push(self.owners.remove(index));
    }

    pub(super) fn close_all(&mut self) -> Outcome {
        let names: Vec<_> = self
            .views
            .items()
            .map(|(name, _)| names::text(name))
            .collect();
        for name in names {
            self.close_named(&name)?;
        }
        self.audit()
    }

    pub(super) fn initialize_default(&mut self) -> Outcome {
        if !self.views.is_empty() {
            return Err(refused(
                self.views.extent(),
                "an empty namespace for this source default initialization",
            ));
        }
        let name = names::free_name(&self.views, "View");
        self.initialize(&name)
    }
}
