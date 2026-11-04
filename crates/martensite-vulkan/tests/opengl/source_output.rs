// Screenshot publication and post-END retirement are shared by literal source callers.
use crate::hatch_surface::scene::{HatchBoxScene, SourceViewOutput};
use martensite_tkernel::{standard::Handle, tcollection::AsciiString};
use martensite_tkservice::image::ImageAlienPixMap;
use martensite_vulkan::Graphic3dRenderDriver;
use std::{
    error::Error,
    io::Write,
    path::{Path, PathBuf},
};

// Live UI ownership is invocation-local and deliberately not a published modelling result.
/// Keep actual view resources alive until source completion; `capture_case(driver, scene, name)?`.
/// Invocation-local UI handles are retired on their worker thread, never published as model results.
pub(super) struct SourceCompletion<V: SourceViewOutput = HatchBoxScene> {
    driver: Handle<Graphic3dRenderDriver>,
    pub(super) scene: V,
    screenshot: Option<PathBuf>,
    basename: &'static str,
    additional: Vec<(Option<PathBuf>, &'static str)>,
}

/// Publish the actual source image without closing its view; `capture_case(driver, scene, "case.png")?`.
pub(super) fn capture_case<V: SourceViewOutput>(
    driver: Handle<Graphic3dRenderDriver>,
    scene: V,
    basename: &'static str,
) -> Result<SourceCompletion<V>, Box<dyn Error>> {
    let screenshot = source_dump(&scene, basename)?;
    Ok(completion(driver, scene, basename, screenshot))
}

fn completion<V: SourceViewOutput>(
    driver: Handle<Graphic3dRenderDriver>,
    scene: V,
    basename: &'static str,
    screenshot: Option<PathBuf>,
) -> SourceCompletion<V> {
    SourceCompletion {
        driver,
        scene,
        screenshot,
        basename,
        additional: Vec::new(),
    }
}

#[path = "../source_output/sized_capture.rs"]
pub(crate) mod sized_capture;

impl<V: SourceViewOutput> SourceCompletion<V> {
    /// Retire the view only after the caller END; `completion.finish_after_end()?`.
    pub(super) fn finish_after_end(self) -> Result<(), Box<dyn Error>> {
        self.scene.close();
        validate_driver(&self.driver)?;
        require_source_captures(&self.screenshot, self.basename, &self.additional)
    }
}

fn require_case_capture(screenshot: Option<&Path>, basename: &str) -> Result<(), Box<dyn Error>> {
    if screenshot.is_some() {
        return Ok(());
    }
    // This is a separate required-output qualification after source END, not a new color predicate.
    Err(std::io::Error::other(format!(
        "source capture absent; expected actual {basename} output"
    ))
    .into())
}

pub(crate) fn source_dump<V: SourceViewOutput>(
    scene: &V,
    basename: &str,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let image = ImageAlienPixMap::default();
    if !scene.source_dump(&image)? {
        // Native VDump returns success after this diagnostic; do not invent an earlier source guard.
        writeln!(std::io::stdout(), "Fail: view dump failed!")?;
        return Ok(None);
    }
    let path = capture_path(basename)?;
    if !save_case_pixmap(&image, &path)? {
        // VDump reports a failed save without aborting the source's END and cleanup.
        writeln!(std::io::stdout(), "Fail: image can not be saved!")?;
        return Ok(None);
    }
    Ok(Some(path))
}

fn save_case_pixmap(image: &ImageAlienPixMap, path: &Path) -> Result<bool, Box<dyn Error>> {
    let name = path.to_str().ok_or_else(|| {
        std::io::Error::other(format!("path={path:?}; expected UTF-8 capture filename"))
    })?;
    let success = image.save(&AsciiString::from(name))?;
    Ok(success)
}

#[path = "../support/case_capture_path.rs"]
mod case_capture_path;
pub(super) use case_capture_path::capture_path;

fn require_source_captures(
    first: &Option<PathBuf>,
    basename: &str,
    captures: &[(Option<PathBuf>, &'static str)],
) -> Result<(), Box<dyn Error>> {
    require_case_capture(first.as_deref(), basename)?;
    for (path, basename) in captures {
        require_case_capture(path.as_deref(), basename)?;
    }
    Ok(())
}

/// Preserve the source validation reporter for every real driver; `validate_driver(&driver)?`.
pub(crate) fn validate_driver(
    driver: &Handle<Graphic3dRenderDriver>,
) -> Result<(), Box<dyn Error>> {
    for diagnostic in driver.validation_errors().unwrap() {
        // These stored messages have ERROR severity; expose the real diagnostics to source grading.
        writeln!(std::io::stderr(), "Error: Vulkan validation: {diagnostic}")?;
    }
    Ok(())
}

#[path = "../source_output/options_capture.rs"]
pub(crate) mod options_capture;
