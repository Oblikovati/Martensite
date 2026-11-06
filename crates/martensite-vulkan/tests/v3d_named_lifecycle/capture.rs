//! Existing capture/END lifecycle retains and validates every actual constructed driver.
use super::*;
use crate::hatch_surface::scene::SourceViewOutput;

impl<F: SourceDriverFactory> SourceViewOutput for NamedViewerScene<F> {
    fn source_view_handle(&self) -> &Handle<dyn V3dViewSource> {
        &self.current
    }
    fn source_window_size(&self) -> (i32, i32) {
        self.current.window().size()
    }
    fn close(&self) {
        self.close_views();
    }
}

impl<F: SourceDriverFactory> NamedViewerScene<F> {
    fn close_views(&self) {
        if self.closed.replace(true) {
            return;
        }
        for owner in self.owners.iter().chain(&self.retired_owners) {
            owner.session.close();
        }
    }
}

impl<F: SourceDriverFactory> Drop for NamedViewerScene<F> {
    fn drop(&mut self) {
        self.close_views();
    }
}

pub(super) fn finish_named(
    scene: NamedViewerScene<driver_factory::QualifiedVulkanFactory>,
) -> Outcome {
    scene.audit()?;
    if scene.current.is_null() || scene.renderers.len() != scene.drivers.extent() {
        return Err(refused(
            "capture driver/view ownership incomplete",
            "all real original drivers and a current view",
        ));
    }
    capture_and_finalize(scene)
}

fn capture_and_finalize(
    scene: NamedViewerScene<driver_factory::QualifiedVulkanFactory>,
) -> Outcome {
    let renderers = scene.renderers.clone();
    let size = scene.source_window_size();
    let view = crate::session_run::render_view(scene.active_session()?)?;
    let observed = view
        .get()
        .ok_or_else(|| refused("null render view", "the actual Vulkan capture view"))?;
    let record = crate::tie_edges::SubmittedDraws::install(observed);
    let capture = crate::source_output::capture_case(renderers[0].clone(), scene, "bug23670_2.png");
    observed.set_frame_hooks(None);
    let completed = capture?;
    let witness = crate::session_run::write_ties_sidecar(&record, "bug23670_2.png", size);
    finalize_captured(completed, &renderers, witness)
}

fn finalize_captured(
    completed: crate::source_output::SourceCompletion<
        NamedViewerScene<driver_factory::QualifiedVulkanFactory>,
    >,
    renderers: &[Handle<martensite_vulkan::Graphic3dRenderDriver>],
    witness: Outcome,
) -> Outcome {
    println!("TEST COMPLETED");
    let result = completed.finish_after_end();
    let additional = validate_additional_drivers(&renderers[1..]);
    result.and(witness).and(additional)
}

fn validate_additional_drivers(
    drivers: &[Handle<martensite_vulkan::Graphic3dRenderDriver>],
) -> Outcome {
    for driver in drivers {
        crate::source_output::validate_driver(driver)?;
    }
    Ok(())
}
