//! Explicit stereo images retain the existing named owner and actual submitted-frame hooks.
use super::*;
use crate::{
    session_run,
    source_output::{self, SourceCompletion},
    tie_edges::SubmittedDraws,
};
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dStereoDumpOptions};
type Scene = NamedViewerScene<driver_factory::QualifiedVulkanFactory>;
pub(super) type CapturedScene = SourceCompletion<Scene>;

pub(super) fn options(stereo_options: V3dStereoDumpOptions) -> V3dImageDumpOptions<'static> {
    V3dImageDumpOptions {
        width: 512,
        height: 512,
        stereo_options,
        ..Default::default()
    }
}

pub(super) fn first(
    scene: Scene,
    name: &'static str,
    errors: &mut Vec<Box<dyn Error>>,
) -> Outcome<CapturedScene> {
    let render = session_run::render_view(scene.active_session()?)?;
    let renderer = first_renderer(&scene)?;
    observed_capture(&render, name, errors, || {
        source_output::options_capture::capture_case_options(
            renderer,
            scene,
            name,
            &options(V3dStereoDumpOptions::Mono),
        )
    })
}

fn first_renderer(scene: &Scene) -> Outcome<Handle<martensite_vulkan::Graphic3dRenderDriver>> {
    scene
        .renderers
        .first()
        .cloned()
        .ok_or_else(|| refused("missing renderer", "the real first named driver"))
}

pub(super) fn additional(
    captured: &mut CapturedScene,
    name: &'static str,
    stereo: V3dStereoDumpOptions,
    errors: &mut Vec<Box<dyn Error>>,
) -> Outcome {
    let render = session_run::render_view(captured.scene.active_session()?)?;
    observed_capture(&render, name, errors, || {
        captured.capture_additional_options(name, &options(stereo))
    })
}

fn observed_capture<T>(
    render: &Handle<martensite_vulkan::Graphic3dRenderView>,
    name: &'static str,
    errors: &mut Vec<Box<dyn Error>>,
    capture: impl FnOnce() -> Outcome<T>,
) -> Outcome<T> {
    let view = render
        .get()
        .ok_or("named stereo capture missing actual render view")?;
    let record = SubmittedDraws::install(view);
    let result = session_run::capture_then_clear(view, capture);
    observe(view, &record, name, errors);
    result
}

fn observe(
    view: &martensite_vulkan::Graphic3dRenderView,
    record: &SubmittedDraws,
    name: &'static str,
    errors: &mut Vec<Box<dyn Error>>,
) {
    let result = session_run::capture_witness::observe(view, record, name)
        .map_err(|error| -> Box<dyn Error> { error.into() })
        .and_then(|()| session_run::write_ties_sidecar(record, name, (512, 512)));
    if let Err(error) = result {
        errors.push(error);
    }
}

pub(super) fn finish(captured: CapturedScene, mut errors: Vec<Box<dyn Error>>) -> Outcome {
    let renderers = captured.scene.renderers.clone();
    captured.scene.audit()?;
    println!("TEST COMPLETED");
    collect_error(captured.finish_after_end(), &mut errors);
    for renderer in renderers.iter().skip(1) {
        collect_error(source_output::validate_driver(renderer), &mut errors);
    }
    require_completion(errors)
}

pub(super) fn collect_error(result: Outcome, errors: &mut Vec<Box<dyn Error>>) {
    if let Err(error) = result {
        errors.push(error);
    }
}

pub(super) fn require_completion(errors: Vec<Box<dyn Error>>) -> Outcome {
    if errors.is_empty() {
        return Ok(());
    }
    let causes = errors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("; ");
    Err(refused(
        causes,
        "all source captures and every retained renderer complete",
    ))
}
