//! Existing-name display follows ViewerCommands's actual same-shape and changed-shape branches.
use super::*;
use martensite_tkernel::standard::catch_native_failure;

impl NamedSourceScene {
    /// Invoke the direct original projection command; `scene.source_axo()?`.
    pub fn source_axo(&self) -> Result<(), Failure> {
        let initialized = &self.initialized;
        initialized.source_axo()
    }

    /// Mutate actual DPI without substituting aspect widths; `scene.source_resolution(144)?`.
    pub fn source_resolution(&self, resolution: u32) -> Result<(), Failure> {
        let initialized = &self.initialized;
        initialized.source_resolution(resolution)
    }

    /// Retain the same named AIS owner through repeated top display; `scene.source_display_top("b1", &shape)?`.
    pub fn source_display_top(&self, name: &str, shape: &Shape) -> Result<(), Failure> {
        let object = self.source_named_object(name)?;
        catch_native_failure(|| {
            self.initialized.source_context_command(|context| {
                display_existing_top(context, object, shape, name)
            })
        })
    }

    /// Resolve the existing owner before a command can mutate it; `scene.source_named_object("b1")?`.
    pub(super) fn source_named_object(&self, name: &str) -> Result<&Handle<AisShape>, Failure> {
        self.objects
            .iter()
            .find(|(binding, _)| binding == name)
            .map(|(_, object)| object)
            .ok_or_else(|| {
                martensite_tkernel::standard::exceptions::program_error(&format!(
                    "name={name:?}; expected a displayed named AIS shape"
                ))
            })
    }
}

fn display_existing_top(
    context: &AisInteractiveContext,
    object: &Handle<AisShape>,
    shape: &Shape,
    name: &str,
) -> Result<(), Failure> {
    object.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOP);
    let mode = existing_named_mode(context, object);
    let selection = existing_named_selection(context, object);
    println!("Display {name}");
    reassign_named_shape(context, object, shape)?;
    display_existing_mode(context, object, mode, selection)
}

fn display_existing_mode(
    context: &AisInteractiveContext,
    object: &Handle<AisShape>,
    mode: i32,
    selection: i32,
) -> Result<(), Failure> {
    if selection == -1 {
        context.erase(&object.cast(), false)?;
    }
    context.display_with_modes(
        &object.cast(),
        mode,
        selection,
        false,
        PrsMgrDisplayStatus::None,
    )
}

fn reassign_named_shape(
    context: &AisInteractiveContext,
    object: &Handle<AisShape>,
    shape: &Shape,
) -> Result<(), Failure> {
    let changed = !object.shape().is_equal(shape);
    object.set_shape(shape);
    if changed {
        context.redisplay(&object.cast(), false, false)?;
    }
    Ok(())
}

fn existing_named_selection(context: &AisInteractiveContext, object: &AisShape) -> i32 {
    if context.get_auto_activate_selection() {
        return object.global_selection_mode();
    }
    -1
}

fn existing_named_mode(context: &AisInteractiveContext, object: &AisShape) -> i32 {
    if object.has_display_mode() {
        return object.display_mode();
    }
    if object.accept_display_mode(context.display_mode()) {
        return context.display_mode();
    }
    0
}
