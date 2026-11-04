//! Every namespace binding is joined to the real existing viewer resource graph.
use super::*;
use names::{ResolvedNames, text};

impl<F: SourceDriverFactory> NamedViewerScene<F> {
    pub(super) fn initialize(&mut self, input: &str) -> Outcome {
        if self.closed.get() {
            return Err(refused(input, "an open invocation-local namespace"));
        }
        let names = self.resolve(input)?;
        if self.views.is_bound1(&names.view) {
            return Err(refused(input, "a new view name in this source protocol"));
        }
        if !self.drivers.is_bound1(&names.driver) {
            self.new_driver(&names)?;
        } else {
            self.additional_view(&names)?;
        }
        self.audit()
    }

    fn new_driver(&mut self, names: &ResolvedNames) -> Outcome {
        let built = self.factory.create()?;
        if self.drivers.is_bound2(&built.core) {
            return Err(refused(
                text(&names.driver),
                "a distinct actual driver for a new driver name",
            ));
        }
        let owner = DriverOwner::new(&built)?;
        self.drivers.bind(names.driver.clone(), built.core)?;
        self.contexts
            .bind(names.context.clone(), owner.session.context().clone())?;
        self.current = owner.session.view().clone();
        self.views.bind(names.view.clone(), self.current.clone())?;
        self.owners.push(owner);
        if let Some(renderer) = built.renderer {
            self.renderers.push(renderer);
        }
        Ok(())
    }

    fn additional_view(&mut self, names: &ResolvedNames) -> Outcome {
        let core = self.drivers.find1(&names.driver)?.clone();
        let owner = self.owner_index(&core)?;
        if self.contexts.is_bound1(&names.context) {
            self.reuse_context(owner, names)?;
            self.owners[owner].session.open_view(409, 409)?;
        } else {
            self.owners[owner].session.open_viewer()?;
            self.contexts.bind(
                names.context.clone(),
                self.owners[owner].session.context().clone(),
            )?;
        }
        self.current = self.owners[owner].session.view().clone();
        self.views.bind(names.view.clone(), self.current.clone())?;
        Ok(())
    }

    fn reuse_context(&mut self, owner: usize, names: &ResolvedNames) -> Outcome {
        let context = self.contexts.find1(&names.context)?.clone();
        if context.current_viewer().clone().driver() != self.owners[owner].core {
            return Err(refused(
                text(&names.context),
                "the named context's actual driver owner",
            ));
        }
        if self.owners[owner].session.context() == &context {
            return Ok(());
        }
        let view = self.view_for_context(&context)?;
        self.owners[owner].session.activate_view_handle(&view)?;
        Ok(())
    }

    fn view_for_context(
        &self,
        context: &Handle<AisInteractiveContext>,
    ) -> Outcome<Handle<dyn V3dViewSource>> {
        let viewer = context.current_viewer().clone();
        self.views
            .items()
            .find(|(_, view)| view.viewer() == viewer)
            .map(|(_, view)| view.clone())
            .ok_or_else(|| {
                refused(
                    "context has no named view",
                    "an actual registered view of this context",
                )
            })
    }

    pub(super) fn context_for_view(
        &self,
        view: &Handle<dyn V3dViewSource>,
    ) -> Outcome<Handle<AisInteractiveContext>> {
        self.contexts
            .items()
            .find(|(_, context)| context.current_viewer().clone() == view.viewer())
            .map(|(_, context)| context.clone())
            .ok_or_else(|| {
                refused(
                    format!("view {:?} has no context", view.object_address()),
                    "an actual registered viewer/context owner",
                )
            })
    }

    pub(super) fn active_session(&self) -> Outcome<&ViewerSession> {
        let owner = self.owner_index(&self.current.viewer().driver())?;
        Ok(&self.owners[owner].session)
    }

    pub(super) fn active_session_mut(&mut self) -> Outcome<&mut ViewerSession> {
        let owner = self.owner_index(&self.current.viewer().driver())?;
        Ok(&mut self.owners[owner].session)
    }

    fn owner_index(&self, core: &Handle<dyn Graphic3dGraphicDriver>) -> Outcome<usize> {
        self.owners
            .iter()
            .position(|owner| &owner.core == core)
            .ok_or_else(|| {
                refused(
                    format!("{:?}", core.object_address()),
                    "an actual owned driver session",
                )
            })
    }

    pub(super) fn audit(&self) -> Outcome {
        for (name, core) in self.drivers.items() {
            self.audit_driver(name, core)?;
        }
        for (name, view) in self.views.items() {
            self.audit_view(name, view)?;
        }
        for (_, context) in self.contexts.items() {
            self.audit_context(context)?;
        }
        self.audit_current()?;
        Ok(())
    }

    fn audit_current(&self) -> Outcome {
        if self.current.is_null() && self.views.is_empty() {
            return Ok(());
        }
        if !self.views.is_bound2(&self.current) {
            return Err(refused(
                "current view unregistered",
                "a real current named view",
            ));
        }
        let owner = self.owner_index(&self.current.viewer().driver())?;
        let session = &self.owners[owner].session;
        let context = self.context_for_view(&self.current)?;
        if session.view() != &self.current || session.context() != &context {
            let observed = format!("current {:?}", self.current.object_address());
            return Err(refused(observed, "the active actual owner view/context"));
        }
        Ok(())
    }

    fn audit_driver(
        &self,
        name: &AsciiString,
        core: &Handle<dyn Graphic3dGraphicDriver>,
    ) -> Outcome {
        if core.is_null() || !self.owners.iter().any(|owner| &owner.core == core) {
            return Err(refused(text(name), "an actual owned driver handle"));
        }
        if !self
            .contexts
            .items()
            .any(|(_, ctx)| !ctx.is_null() && ctx.current_viewer().clone().driver() == *core)
        {
            return Err(refused(text(name), "a real retained context of the driver"));
        }
        Ok(())
    }

    fn audit_view(&self, name: &AsciiString, view: &Handle<dyn V3dViewSource>) -> Outcome {
        if view.is_null()
            || view.viewer().is_null()
            || view.window().is_null()
            || !view.viewer().defined_views().contains(view)
        {
            return Err(refused(
                text(name),
                "a live actual defined view, not a label-only or removed binding",
            ));
        }
        self.context_for_view(view)?;
        Ok(())
    }

    fn audit_context(&self, context: &Handle<AisInteractiveContext>) -> Outcome {
        if context.is_null() {
            return Err(refused("null context", "an actual viewer/context handle"));
        }
        let viewer = context.current_viewer().clone();
        if !self.drivers.is_bound2(&viewer.driver()) {
            return Err(refused(
                "context driver unregistered",
                "an actual named driver",
            ));
        }
        let defined = viewer.defined_views();
        let named: Vec<_> = self
            .views
            .items()
            .filter(|(_, view)| view.viewer() == viewer)
            .map(|(_, view)| view.clone())
            .collect();
        require_membership(&defined, &named)
    }

    pub(super) fn close_named(&mut self, input: &str) -> Outcome {
        let names = self.resolve(input)?;
        if !self.views.is_bound1(&names.view) {
            return Err(refused(input, "an existing named view"));
        }
        let view = self.views.find1(&names.view)?.clone();
        let context = self.context_for_view(&view)?;
        if view == self.current {
            self.activate_other(&view)?;
        }
        view.window().unmap();
        view.remove();
        self.views.unbind1(&names.view);
        self.remove_empty_context(&context)?;
        println!("3D View - {} was deleted.\n", text(&names.view));
        self.audit()
    }

    fn activate_other(&mut self, removed: &Handle<dyn V3dViewSource>) -> Outcome {
        let next = self
            .views
            .items()
            .find(|(_, view)| *view != removed)
            .map(|(_, view)| view.clone());
        let Some(next) = next else {
            self.current = Handle::default();
            return Ok(());
        };
        let owner = self.owner_index(&next.viewer().driver())?;
        self.owners[owner].session.activate_view_handle(&next)?;
        self.current = next;
        Ok(())
    }

    fn remove_empty_context(&mut self, context: &Handle<AisInteractiveContext>) -> Outcome {
        if !context.current_viewer().clone().defined_views().is_empty() {
            return Ok(());
        }
        let core = context.current_viewer().clone().driver();
        context.remove_all(false)?;
        self.contexts.unbind2(context);
        if !self
            .contexts
            .items()
            .any(|(_, context)| context.current_viewer().clone().driver() == core)
        {
            self.drivers.unbind2(&core);
            self.retire_driver(&core);
        }
        Ok(())
    }
}

fn require_membership(
    defined: &[Handle<dyn V3dViewSource>],
    named: &[Handle<dyn V3dViewSource>],
) -> Outcome {
    if named.len() != defined.len() || named.iter().any(|view| !defined.contains(view)) {
        return Err(refused(
            format!("defined{} named{}", defined.len(), named.len()),
            "exact real view membership",
        ));
    }
    Ok(())
}
