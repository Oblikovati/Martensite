//! Native view naming labels only real driver/context/view resources.
mod capture;
#[cfg(test)]
mod controls;
mod driver_factory;
mod listing;
mod names;
mod registry;
mod retirement;
mod stereo_capture;
mod stereo_source;
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::banner;
use driver_factory::{DriverOwner, SourceDriverFactory};
use martensite_tkernel::{
    ncollection::DoubleMap,
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkservice::graphic3d::Graphic3dGraphicDriver;
use martensite_tkv3d::{ais::AisInteractiveContext, v3d::V3dViewSource};
use std::{cell::Cell, error::Error};
// Only the focused camera-dump host consumes this shared entry point.
#[allow(unused_imports)]
pub(crate) use stereo_source::camera_dumps;
type Outcome<T = ()> = Result<T, Box<dyn Error>>;

struct NamedViewerScene<F: SourceDriverFactory> {
    factory: F,
    drivers: DoubleMap<AsciiString, Handle<dyn Graphic3dGraphicDriver>>,
    contexts: DoubleMap<AsciiString, Handle<AisInteractiveContext>>,
    views: DoubleMap<AsciiString, Handle<dyn V3dViewSource>>,
    owners: Vec<DriverOwner>,
    retired_owners: Vec<DriverOwner>,
    renderers: Vec<Handle<martensite_vulkan::Graphic3dRenderDriver>>,
    current: Handle<dyn V3dViewSource>,
    closed: Cell<bool>,
}

impl<F: SourceDriverFactory> NamedViewerScene<F> {
    fn new(factory: F) -> Self {
        Self {
            factory,
            drivers: DoubleMap::default(),
            contexts: DoubleMap::default(),
            views: DoubleMap::default(),
            owners: Vec::new(),
            retired_owners: Vec::new(),
            renderers: Vec::new(),
            current: Handle::default(),
            closed: Cell::new(false),
        }
    }
}

/// Preserve all initialization/close/list operations and final mandatory capture; `named_views()?`.
pub(crate) fn named_views() -> Outcome {
    banner(&[
        "============",
        "OCC23670: Support for multiple 3D views",
        "============",
        "",
    ]);
    let mut scene = NamedViewerScene::new(driver_factory::QualifiedVulkanFactory);
    source_first_views(&mut scene)?;
    let before = scene.view_list()?;
    scene.initialize("Driver1/Viewer1/MyView")?;
    scene.close_named("MyView")?;
    scene.initialize("Driver1/Viewer1/View2")?;
    if before == scene.view_list()? {
        println!("Error. Different view lists are equal.");
    }
    capture::finish_named(scene)
}

fn source_first_views<F: SourceDriverFactory>(scene: &mut NamedViewerScene<F>) -> Outcome {
    scene.initialize("View1")?;
    scene.initialize("MyDriver/MyViewer/View1")?;
    scene.initialize("MyDriver/MyViewer/MyView")?;
    scene.close_named("View1")?;
    Ok(())
}

fn refused(value: impl std::fmt::Display, expected: &str) -> Box<dyn Error> {
    Failure::with_message(
        "Standard_ProgramError",
        &format!("named lifecycle {value}; expected {expected}"),
    )
    .into()
}
