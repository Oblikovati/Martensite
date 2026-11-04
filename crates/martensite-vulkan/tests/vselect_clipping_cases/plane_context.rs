//! Named live planes for these six cases; attachments coexist until explicit deletion.
use super::prelude::*;
use martensite_tkernel::ncollection::Vec4;
use martensite_tkservice::graphic3d::Graphic3dClipPlane;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
use martensite_tkv3d::v3d::V3dViewSource;

type ClippedObject = Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>;
type ClippedView = Handle<dyn V3dViewSource>;

pub(super) struct ClippingPlanes {
    entries: Vec<NamedPlane>,
    active_view: ClippedView,
}

struct NamedPlane {
    name: String,
    plane: Handle<Graphic3dClipPlane>,
    views: Vec<ClippedView>,
    objects: Vec<ClippedObject>,
}

impl ClippingPlanes {
    pub(super) fn new(active_view: &ClippedView) -> Self {
        Self {
            entries: Vec::new(),
            active_view: active_view.clone(),
        }
    }

    pub(super) fn create(&mut self, name: &str) -> Result<(), Failure> {
        self.delete(name)?;
        self.entries.push(NamedPlane {
            name: name.to_owned(),
            plane: Handle::new(Graphic3dClipPlane::default()),
            views: Vec::new(),
            objects: Vec::new(),
        });
        Ok(())
    }

    pub(super) fn ensure(&mut self, name: &str) -> Result<(), Failure> {
        if self.entries.iter().any(|entry| entry.name == name) {
            return Ok(());
        }
        self.create(name)
    }

    pub(super) fn plane(&self, name: &str) -> Result<&Handle<Graphic3dClipPlane>, Failure> {
        self.entries
            .iter()
            .find(|entry| entry.name == name)
            .map(|entry| &entry.plane)
            .ok_or_else(|| missing_plane(name))
    }

    pub(super) fn equation(&self, name: &str, [a, b, c, d]: [f64; 4]) -> Result<(), Failure> {
        let plane = self.plane(name)?;
        plane.set_equation(Vec4::new(a, b, c, d))?;
        self.redraw();
        Ok(())
    }

    pub(super) fn on_view(&mut self, name: &str, view: &ClippedView) -> Result<(), Failure> {
        let entry = self.entry_mut(name)?;
        view.add_clip_plane(&entry.plane);
        if !entry.views.contains(view) {
            entry.views.push(view.clone());
        }
        self.redraw();
        Ok(())
    }

    pub(super) fn on_named_view(&mut self, name: &str, target: &str) -> Result<(), Failure> {
        if target != "Driver1/Viewer1/View1" {
            return Err(Failure::with_message(
                "Standard_ProgramError",
                &format!(
                    "clipping view {target:?}; expected this family's initialized Driver1/Viewer1/View1"
                ),
            ));
        }
        let active = self.active_view.clone();
        self.on_view(name, &active)
    }

    pub(super) fn on_object(
        &mut self,
        name: &str,
        object: &ClippedObject,
        override_global: bool,
    ) -> Result<(), Failure> {
        let entry = self.entry_mut(name)?;
        object.add_clip_plane(&entry.plane);
        object.clip_planes().set_override_global(override_global);
        if !entry.objects.contains(object) {
            entry.objects.push(object.clone());
        }
        self.redraw();
        Ok(())
    }

    pub(super) fn delete(&mut self, name: &str) -> Result<bool, Failure> {
        let Some(index) = self.entries.iter().position(|entry| entry.name == name) else {
            return Ok(false);
        };
        let entry = self.entries.remove(index);
        for view in &entry.views {
            view.remove_clip_plane(&entry.plane);
            view.redraw();
        }
        for object in &entry.objects {
            object.remove_clip_plane(&entry.plane);
        }
        self.redraw();
        Ok(true)
    }

    fn entry_mut(&mut self, name: &str) -> Result<&mut NamedPlane, Failure> {
        self.entries
            .iter_mut()
            .find(|entry| entry.name == name)
            .ok_or_else(|| missing_plane(name))
    }

    fn redraw(&self) {
        self.active_view.redraw();
        for view in self.entries.iter().flat_map(|entry| &entry.views) {
            if view != &self.active_view {
                view.redraw();
            }
        }
    }
}

fn missing_plane(name: &str) -> Failure {
    Failure::with_message(
        "Standard_ProgramError",
        &format!("clipping plane {name:?}; expected an existing named plane"),
    )
}
