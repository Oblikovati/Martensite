//! Viewer lights and their interactive light source presentations: clearing, adding and
//! removing global lights, then restacking the presentations as the light command does.
use super::ViewerSession;

/// Sets up a light presentation before it is displayed.
pub type LightPresentationSetup<'a> = &'a dyn Fn(&AisLightSource) -> Result<(), Failure>;
use martensite_tkernel::ncollection::{List, Vec2};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::graphic3d::{Graphic3dCLight, Graphic3dTransformPersMembers as _};
use martensite_tkv3d::ais::{
    AisInteractiveObject, AisKindOfInteractive, AisLightSource, AisLightSourceMembers as _,
    AisLightSourceSource as _,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

impl ViewerSession {
    /// Remove every active light of the view and its presentation.
    /// Example: `session.clear_lights()?`.
    pub fn clear_lights(&mut self) -> Result<(), Failure> {
        let view = self.view().clone();
        let view = view.get().ok_or_else(null_view)?.view_base();
        while let Some(light) = view.active_lights().into_iter().next() {
            self.remove_light_presentation(&light)?;
            self.viewer.del_light(&light)?;
        }
        self.update_light_presentations()
    }

    /// Add `light` to the viewer and switch it on in every view; with `presentation`, display
    /// its presentation under the light's name after `presentation` sets it up (a new light, or a
    /// change to an existing light, which reuses
    /// the light's displayed presentation). Example: `session.add_light(&light, Some(&|_| Ok(())))?`.
    pub fn add_light(
        &mut self,
        light: &Handle<Graphic3dCLight>,
        presentation: Option<LightPresentationSetup<'_>>,
    ) -> Result<(), Failure> {
        self.viewer.add_light(light);
        self.viewer.set_light_on_one(light)?;
        if let Some(setup) = presentation {
            let name = light_name(light)?;
            let shown = match self.light_presentation(&name) {
                Some(shown) => shown,
                None => AisLightSource::new(light)?,
            };
            setup(shown.get().ok_or_else(null_view)?)?;
            self.display_object(&name, shown.cast())?;
        }
        self.update_light_presentations()
    }

    /// The view's active light at `index` in activation order, resolving
    /// a numeric name; `session.active_light(2)?`.
    pub fn active_light(&self, index: usize) -> Result<Handle<Graphic3dCLight>, Failure> {
        let view = self.view().clone();
        let lights = view
            .get()
            .ok_or_else(null_view)?
            .view_base()
            .active_lights();
        lights.into_iter().nth(index).ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "light index {index}; expected an index below the active light count"
            ))
        })
    }

    /// Delete `light` from the viewer with its presentation.
    /// Example: `session.remove_light(&light)?`.
    pub fn remove_light(&mut self, light: &Handle<Graphic3dCLight>) -> Result<(), Failure> {
        self.remove_light_presentation(light)?;
        self.viewer.del_light(light)?;
        self.update_light_presentations()
    }

    fn remove_light_presentation(
        &mut self,
        light: &Handle<Graphic3dCLight>,
    ) -> Result<(), Failure> {
        let name = light_name(light)?;
        if self.light_presentation(&name).is_some() {
            self.remove(&[&name])?;
        }
        Ok(())
    }

    /// The light presentation bound to `name`, if any.
    fn light_presentation(&self, name: &str) -> Option<Handle<AisLightSource>> {
        let object = self.object(name)?.cast::<AisLightSource>();
        (!object.is_null()).then_some(object)
    }

    /// Restack the displayed light presentations by light id and redisplay them; the 2D and
    /// trihedron-persistent ones line up along the top of the view.
    fn update_light_presentations(&self) -> Result<(), Failure> {
        let mut list = List::default();
        let context = &self.context;
        context.displayed_objects_by_kind(AisKindOfInteractive::LightSource, -1, &mut list)?;
        let mut presentations: Vec<Handle<AisLightSource>> = list
            .iter()
            .map(|object| object.cast::<AisLightSource>())
            .filter(|p| !p.is_null())
            .collect();
        // The context keeps objects unordered; the light ids give the stacking order.
        presentations.sort_by_key(light_id);
        let mut top_stack = 0;
        for presentation in presentations {
            top_stack = restack(&presentation, top_stack)?;
            let value = presentation.get().ok_or_else(null_view)?;
            let object: Handle<dyn AisInteractiveObject> = presentation.clone().cast();
            context.redisplay(&object, false, false)?;
            context.set_transform_persistence(&object, &value.transform_persistence())?;
        }
        Ok(())
    }
}

/// Offset a 2D or trihedron-persistent presentation after the ones before it.
fn restack(presentation: &Handle<AisLightSource>, top_stack: i32) -> Result<i32, Failure> {
    let value = presentation.get().ok_or_else(null_view)?;
    let persistence = value.transform_persistence();
    let Some(persistence) = persistence.get().filter(|p| p.is_trihedron_or_2d()) else {
        return Ok(top_stack);
    };
    let size = value.size() as i32;
    persistence.set_offset_2d(Vec2::new(top_stack + size, size))?;
    Ok(top_stack + size + size / 2)
}

fn light_id(presentation: &Handle<AisLightSource>) -> String {
    let light = presentation.get().map(|p| p.light());
    light
        .and_then(|l| {
            l.get()
                .map(|l| String::from_utf8_lossy(l.get_id().as_bytes()).into_owned())
        })
        .unwrap_or_default()
}

fn light_name(light: &Handle<Graphic3dCLight>) -> Result<String, Failure> {
    let light = light.get().ok_or_else(null_view)?;
    Ok(String::from_utf8_lossy(light.name().as_bytes()).into_owned())
}

fn null_view() -> Failure {
    martensite_tkernel::standard::exceptions::program_error(
        "a null view, light or presentation; expected a live one",
    )
}
