//! Instances and assemblies: objects connected to other objects or shapes at a location.
use super::*;
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tkv3d::prs_mgr::{PrsMgrPresentableObject as _, PrsMgrTypeOfPresentation3d};

/// What an instance or assembly part refers to: a named object, else a shape by its name.
/// Example: `ConnectedPart::shape("b1", &b1)`.
#[derive(Clone, Copy)]
pub struct ConnectedPart<'a> {
    name: &'a str,
    shape: Option<&'a Shape>,
}

impl<'a> ConnectedPart<'a> {
    /// The displayed object `name`; `ConnectedPart::object("b")`.
    pub fn object(name: &'a str) -> Self {
        Self { name, shape: None }
    }

    /// The object `name` when displayed, else a new presentation of `shape`.
    /// Example: `ConnectedPart::shape("b1", &b1)`.
    pub fn shape(name: &'a str, shape: &'a Shape) -> Self {
        Self {
            name,
            shape: Some(shape),
        }
    }
}

fn translation([x, y, z]: [f64; 3]) -> Transform3 {
    let mut trsf = Transform3::default();
    trsf.set_translation_part(Vector3::new(x, y, z));
    trsf
}

impl ViewerSession {
    /// Display as `name` an instance of `part` translated by `offset`, as the interactive
    /// viewer connects one object; `session.connect_instance("b1", [2., 0., 0.], part)?`.
    pub fn connect_instance(
        &mut self,
        name: &str,
        offset: [f64; 3],
        part: ConnectedPart<'_>,
    ) -> Result<(), Failure> {
        // Every step leaves the viewer as it is; one update follows, as after a command.
        self.connect_instance_quietly(name, offset, part)?;
        self.context.update_current_viewer();
        Ok(())
    }

    /// The same connection without the viewer update.
    /// Example: `session.connect_instance_quietly("s_0_0", [0., 0., 0.], part)?`.
    pub fn connect_instance_quietly(
        &mut self,
        name: &str,
        offset: [f64; 3],
        part: ConnectedPart<'_>,
    ) -> Result<(), Failure> {
        let origin = self.connected_part(part, false)?;
        let instance = AisConnectedInteractive::new(PrsMgrTypeOfPresentation3d::AllView);
        instance.connect_trsf(&origin, &translation(offset))?;
        self.bind_and_display(name, instance.cast(), false)
    }

    /// Display as `name` an assembly of `parts` whose own location is `offset`.
    /// Example: `session.connect_assembly("bc", [0., -6., 0.], &parts)?`.
    pub fn connect_assembly(
        &mut self,
        name: &str,
        offset: [f64; 3],
        parts: &[ConnectedPart<'_>],
    ) -> Result<(), Failure> {
        let assembly = AisMultipleConnectedInteractive::new();
        for part in parts.iter().filter(|part| part.name != name) {
            let object = self.connected_part(*part, true)?;
            assembly.connect(&object)?;
        }
        assembly.set_local_transformation(&translation(offset))?;
        self.bind_and_display(name, assembly.cast(), true)
    }

    /// Add to the assembly `name` an instance of `part` translated by `offset`, then
    /// redisplay the assembly and recompute its selection.
    /// Example: `session.add_connected("z4", [0., 4., 0.], ConnectedPart::object("s"))?`.
    pub fn add_connected(
        &mut self,
        name: &str,
        offset: [f64; 3],
        part: ConnectedPart<'_>,
    ) -> Result<(), Failure> {
        let assembly = self.connected_name(name)?;
        let object = self.connected_part(part, false)?;
        let multi = assembly.cast::<AisMultipleConnectedInteractive>();
        let value = multi.get().ok_or_else(|| not_an_assembly(name))?;
        value.connect_trsf(&object, &translation(offset))?;
        self.context.display(&assembly, false)?;
        self.context.recompute_selection_only(&assembly)?;
        self.context.update_current_viewer();
        Ok(())
    }

    /// Remove the `index`-th (from 1) child of the assembly `name`.
    /// Example: `session.disconnect_child("z4", 1)?`.
    pub fn disconnect_child(&self, name: &str, index: usize) -> Result<(), Failure> {
        let assembly = self.connected_name(name)?;
        let multi = assembly.cast::<AisMultipleConnectedInteractive>();
        let value = multi.get().ok_or_else(|| not_an_assembly(name))?;
        let child = value
            .children()
            .get(index.wrapping_sub(1))
            .cloned()
            .ok_or_else(|| {
                martensite_tkernel::standard::exceptions::program_error(&format!(
                    "child {index} of {name:?}; expected 1..={}",
                    value.children().len()
                ))
            })?;
        self.context.disconnect(&assembly, &child.cast())?;
        self.context.update_current_viewer();
        Ok(())
    }

    /// Replace the location of `name` by a translation; `session.set_translation("b1", [2., 1., 0.])?`.
    pub fn set_translation(&self, name: &str, offset: [f64; 3]) -> Result<(), Failure> {
        let object = self.connected_name(name)?;
        let location = martensite_tkmath::toploc::Location::new(translation(offset));
        self.with_update(|context| context.set_location(&object, &location))
    }

    /// The object a connection refers to: an instance or assembly as it is, another named
    /// object replaced by an instance of it, or an instance of a new shape presentation.
    fn connected_part(
        &mut self,
        part: ConnectedPart<'_>,
        load: bool,
    ) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
        if let Some(object) = self.object(part.name).cloned() {
            return self.connected_object(part.name, object);
        }
        let shape = part.shape.ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "connected part {:?}; expected a displayed object or a shape",
                part.name
            ))
        })?;
        let instance = AisConnectedInteractive::new(PrsMgrTypeOfPresentation3d::AllView);
        instance.connect(&AisShape::new(shape).cast())?;
        let instance: Handle<dyn AisInteractiveObject> = instance.cast();
        if load {
            self.context.load(&instance, -1)?;
        }
        self.objects.push(part.name, instance.clone());
        Ok(instance)
    }

    fn connected_object(
        &mut self,
        name: &str,
        object: Handle<dyn AisInteractiveObject>,
    ) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
        let connected = !object.cast::<AisConnectedInteractive>().is_null()
            || !object.cast::<AisMultipleConnectedInteractive>().is_null();
        if connected {
            return Ok(object);
        }
        self.context.remove(&object, false)?;
        let value = object.get().expect("a bound object is never null");
        let instance = AisConnectedInteractive::new(PrsMgrTypeOfPresentation3d::AllView);
        if value.has_display_mode() {
            instance.set_display_mode(value.display_mode());
        }
        instance.connect_datum(&object, &value.local_transformation_geom())?;
        if !value.transform_persistence().is_null() {
            instance.set_transform_persistence(&value.transform_persistence());
        }
        self.bind_and_display(name, instance.clone().cast(), false)?;
        Ok(instance.cast())
    }

    /// Bind `object` to `name`, removing what the name held, and display it with the
    /// context defaults; `self.bind_and_display("z", object, true)?`.
    fn bind_and_display(
        &mut self,
        name: &str,
        object: Handle<dyn AisInteractiveObject>,
        update: bool,
    ) -> Result<(), Failure> {
        if let Some(replaced) = self.objects.remove(name) {
            self.context.remove(&replaced, false)?;
        }
        self.objects.remove_object(&object);
        self.objects.push(name, object.clone());
        self.context.display(&object, update)
    }

    fn connected_name(&self, name: &str) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
        self.object(name).cloned().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "object {name:?} not displayed; expected a displayed object name"
            ))
        })
    }
}

fn not_an_assembly(name: &str) -> Failure {
    martensite_tkernel::standard::exceptions::program_error(&format!(
        "object {name:?} is not an assembly; expected a multiple connected object"
    ))
}
