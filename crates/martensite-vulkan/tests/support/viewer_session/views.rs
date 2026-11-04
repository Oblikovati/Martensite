//! Switching between the views of a session and hiding objects in one of them.
use super::*;
use martensite_tkv3d::select_mgr::SelectMgrSelectableObjectMembers as _;

pub(super) struct RetainedViewerOwner {
    pub(super) viewer: Handle<V3dViewer>,
    pub(super) context: Handle<AisInteractiveContext>,
    pub(super) controller: AisViewController,
}

impl ViewerSession {
    /// Make view `index` (in opening order) active and redraw it; `session.activate_view(0)?`.
    pub fn activate_view(&mut self, index: usize) -> Result<(), Failure> {
        if index >= self.views.len() {
            return Err(martensite_tkernel::standard::exceptions::program_error(
                &format!(
                    "view index {index}; expected below the {} open views",
                    self.views.len()
                ),
            ));
        }
        self.activate_owner_for_view(index)?;
        self.active = index;
        self.view().redraw();
        Ok(())
    }

    /// Activate a real retained view by identity; `session.activate_view_handle(&view)?`.
    pub fn activate_view_handle(
        &mut self,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let index = self.retained_view_index(view)?;
        require_live_retained_view(view)?;
        self.activate_view(index)
    }

    fn retained_view_index(&self, view: &Handle<dyn V3dViewSource>) -> Result<usize, Failure> {
        self.views
            .iter()
            .position(|(_, candidate)| candidate == view)
            .ok_or_else(|| {
                martensite_tkernel::standard::exceptions::program_error(&format!(
                    "view {:?}; expected an actual retained view handle",
                    view.object_address()
                ))
            })
    }

    fn activate_owner_for_view(&mut self, index: usize) -> Result<(), Failure> {
        let target = &self.views[index].1;
        let owner = target.viewer().object_address();
        if owner == self.viewer.object_address() {
            return Ok(());
        }
        let previous = self.earlier_viewers.iter_mut().find(|retained| retained.viewer.object_address() == owner)
            .ok_or_else(|| martensite_tkernel::standard::exceptions::program_error(&format!(
                "view {:?} viewer {owner:?}; expected a registered viewer/context/controller owner", target.object_address()
            )))?;
        std::mem::swap(&mut self.viewer, &mut previous.viewer);
        std::mem::swap(&mut self.context, &mut previous.context);
        std::mem::swap(&mut self.controller, &mut previous.controller);
        Ok(())
    }

    /// Hide the object `name` in the active view only, keeping it displayed in the others.
    /// Example: `session.hide_in_active_view("vc")?`.
    pub fn hide_in_active_view(&self, name: &str) -> Result<(), Failure> {
        let object = self.object(name).cloned().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?} not displayed; expected a displayed object name"
            ))
        })?;
        let view = self.view().clone();
        self.with_update(|context| context.set_view_affinity(&object, &view, false))
    }
}

impl ViewerSession {
    /// A new viewer of the same driver with its own context, pointer
    /// input and a first view of the active view's size, made active. Named objects stay bound
    /// (the interactive viewer's name map is global), but they are displayed in the viewer
    /// that displayed them; `session.open_viewer()?`.
    pub fn open_viewer(&mut self) -> Result<(), Failure> {
        let (width, height) = self.views[self.active].0.size();
        let replacement = Self::with_graphic_driver(&self.viewer.driver(), width, height)?;
        let earlier = RetainedViewerOwner {
            viewer: std::mem::replace(&mut self.viewer, replacement.viewer),
            context: std::mem::replace(&mut self.context, replacement.context),
            controller: std::mem::replace(&mut self.controller, replacement.controller),
        };
        self.earlier_viewers.push(earlier);
        self.views.extend(replacement.views);
        self.active = self.views.len() - 1;
        Ok(())
    }

    /// Close the active view keeping the context and its objects, then open a new view of
    /// the same size, as closing and reopening a named view does; `session.reopen_view()?`.
    pub fn reopen_view(&mut self) -> Result<(), Failure> {
        let (window, view) = self.views.remove(self.active);
        view.remove();
        let (width, height) = window.size();
        self.open_view(width, height)
    }

    /// Display the named non-shape object again with its own or the context's modes.
    /// Example: `session.display_again("b1c")?`.
    pub fn display_again(&self, name: &str) -> Result<(), Failure> {
        let object = self.object(name).cloned().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?} not registered; expected a bound object name"
            ))
        })?;
        let value = object.get().expect("a bound object is never null");
        let mode = if value.has_display_mode() {
            value.display_mode()
        } else if value.accept_display_mode(self.context.display_mode()) {
            self.context.display_mode()
        } else {
            0
        };
        let selection = if self.context.get_auto_activate_selection() {
            value.global_selection_mode()
        } else {
            -1
        };
        self.with_update(|context| {
            context.display_with_modes(&object, mode, selection, false, PrsMgrDisplayStatus::None)
        })
    }
}

impl ViewerSession {
    /// Display `shape` as `name`, then keep it visible in the active
    /// view only, as one command; `session.display_in_active_view("b2", &b2)?`.
    pub fn display_in_active_view(&mut self, name: &str, shape: &Shape) -> Result<(), Failure> {
        self.display(name, shape, &DisplayRequest::default())?;
        let object = self.named(name)?;
        let active = self.view().clone();
        let views: Vec<_> = self.views.iter().map(|(_, view)| view.clone()).collect();
        self.with_update(|context| {
            for view in &views {
                context.set_view_affinity(&object, view, false)?;
            }
            context.set_view_affinity(&object, &active, true)
        })
    }
}

fn require_live_retained_view(view: &Handle<dyn V3dViewSource>) -> Result<(), Failure> {
    let viewer = view.viewer();
    if viewer.is_null() || !viewer.defined_views().contains(view) {
        return Err(martensite_tkernel::standard::exceptions::program_error(
            &format!(
                "view {:?}; expected a live defined view",
                view.object_address()
            ),
        ));
    }
    Ok(())
}
