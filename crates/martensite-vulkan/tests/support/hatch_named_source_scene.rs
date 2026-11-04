//! Literal named display batches retain separate AIS objects in one existing source view.
use super::*;
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::{
    prs_mgr::PrsMgrPresentableObject, select_mgr::SelectMgrSelectableObjectMembers,
};

/// Invocation-local ordered bindings, not a published modelling result or a second viewer.
/// Example: `let mut scene = NamedSourceScene::new(HatchSourceView::new(&driver)?);`.
pub struct NamedSourceScene {
    initialized: HatchSourceView,
    objects: Vec<(String, Handle<AisShape>)>,
}

impl NamedSourceScene {
    /// Retain an already initialized actual view; `NamedSourceScene::new(initialized)`.
    pub fn new(initialized: HatchSourceView) -> Self {
        let objects: Vec<(String, Handle<AisShape>)> = Vec::new();
        Self {
            initialized,
            objects,
        }
    }

    /// Display one batch and update once; `scene.source_display_group(&[("result", &shape)])?`.
    pub fn source_display_group(&mut self, shapes: &[(&str, &Shape)]) -> Result<(), Failure> {
        // A bare display retains the object/context defaults without installing an own mode.
        self.source_display_group_options(shapes, None)
    }

    /// Set each genuine object's own mode before its named display; `scene.source_display_group_mode(&shapes, 0)?`.
    pub fn source_display_group_mode(
        &mut self,
        shapes: &[(&str, &Shape)],
        mode: i32,
    ) -> Result<(), Failure> {
        self.source_display_group_options(shapes, Some(mode))
    }

    fn source_display_group_options(
        &mut self,
        shapes: &[(&str, &Shape)],
        mode: Option<i32>,
    ) -> Result<(), Failure> {
        let retained = &mut self.objects;
        self.initialized.source_context_command(|context| {
            for (name, shape) in shapes {
                let object = source_display_object(shape, mode)?;
                retained.push(((*name).to_owned(), object.clone()));
                display_source_object(context, &object, mode)?;
            }
            Ok(())
        })
    }

    /// Change the global mode without adding object-own modes; `scene.source_global_mode(1)?`.
    pub fn source_global_mode(&self, mode: i32) -> Result<(), Failure> {
        let requested: i32 = mode;
        self.initialized.source_global_display_mode(requested)
    }

    /// Apply default shading to the named AIS object; `scene.source_default_shading("a")?`.
    pub fn source_default_shading(&self, name: &str) -> Result<(), Failure> {
        let object = self.source_named_object(name)?;
        self.initialized.source_default_object_shading(object)
    }

    /// Fit the same view after the source displays; `scene.source_fit()?`.
    pub fn source_fit(&self) -> Result<(), Failure> {
        let view: &Handle<dyn V3dViewSource> = self.initialized.source_view_handle();
        view_output::with_source_view_update(
            &**view,
            || view.fit_all(0.01, false),
            || view.redraw(),
        )
    }

    /// Inspect literal bindings without creating another presentation; `scene.named_objects().count()`.
    pub fn named_objects(&self) -> impl Iterator<Item = (&str, &Handle<AisShape>)> {
        self.objects
            .iter()
            .map(|(name, object)| (name.as_str(), object))
    }

    /// Query actual context state without updating it; `scene.displayed_in_mode(object, 1)`.
    pub fn displayed_in_mode(&self, object: &Handle<AisShape>, mode: i32) -> bool {
        self.initialized
            .source_context_query(|context| context.is_displayed_mode(&object.cast(), mode))
    }
}

impl SourceViewOutput for NamedSourceScene {
    fn source_view_handle(&self) -> &Handle<dyn V3dViewSource> {
        // Delegation retains the canonical initialized view and its exact dynamic type.
        self.initialized.source_view_handle()
    }

    fn source_window_size(&self) -> (i32, i32) {
        // The original real window remains authoritative for source screenshot dimensions.
        self.initialized.source_window_size()
    }
}

fn source_display_object(shape: &Shape, mode: Option<i32>) -> Result<Handle<AisShape>, Failure> {
    let object: Handle<AisShape> = AisShape::new(shape);
    if let Some(mode) = mode {
        set_source_object_mode(&object, mode)?;
    }
    Ok(object)
}

fn set_source_object_mode(object: &Handle<AisShape>, mode: i32) -> Result<(), Failure> {
    if !object.accept_display_mode(mode) {
        return Err(martensite_tkernel::standard::exceptions::program_error(
            &format!(
                "mode={mode}; expected supported AIS shape display mode before named publication"
            ),
        ));
    }
    // ViewerCommands at sets this own mode before binding and Display, not after redraw.
    object.set_display_mode(mode);
    Ok(())
}

fn display_source_object(
    context: &AisInteractiveContext,
    object: &Handle<AisShape>,
    mode: Option<i32>,
) -> Result<(), Failure> {
    let Some(mode) = mode else {
        return context.display(&object.cast(), false);
    };
    let selection = match context.get_auto_activate_selection() {
        true => object.global_selection_mode(),
        false => -1,
    };
    context.display_with_modes(
        &object.cast(),
        mode,
        selection,
        false,
        PrsMgrDisplayStatus::None,
    )
}

#[cfg(test)]
#[path = "hatch_named_display_controls.rs"]
mod named_display_controls;

#[path = "hatch_named_source_commands.rs"]
mod source_commands;
