//! Explicit dump dimensions borrow the actual view without resizing its physical window.
use super::*;
use martensite_tkv3d::v3d::V3dViewSource;

struct SizedSourceView<'a, V> {
    scene: &'a V,
    size: (i32, i32),
}
impl<V: SourceViewOutput> SourceViewOutput for SizedSourceView<'_, V> {
    fn source_view_handle(&self) -> &Handle<dyn V3dViewSource> {
        self.scene.source_view_handle()
    }
    fn source_window_size(&self) -> (i32, i32) {
        self.size
    }
}

/// Publish an explicitly sized first dump under the same surviving owner; `capture_case_sized(driver, scene, "wide.png", (4096, 100))?`.
pub(crate) fn capture_case_sized<V: SourceViewOutput>(
    driver: Handle<Graphic3dRenderDriver>,
    scene: V,
    basename: &'static str,
    size: (i32, i32),
) -> Result<SourceCompletion<V>, Box<dyn Error>> {
    let screenshot = source_dump(
        &SizedSourceView {
            scene: &scene,
            size,
        },
        basename,
    )?;
    Ok(completion(driver, scene, basename, screenshot))
}

impl<V: SourceViewOutput> SourceCompletion<V> {
    /// Retain an explicitly sized later dump until the same END; `completion.capture_additional_sized("tall.png", (100, 4096))?`.
    pub(crate) fn capture_additional_sized(
        &mut self,
        basename: &'static str,
        size: (i32, i32),
    ) -> Result<(), Box<dyn Error>> {
        let screenshot = source_dump(
            &SizedSourceView {
                scene: &self.scene,
                size,
            },
            basename,
        )?;
        self.additional.push((screenshot, basename));
        Ok(())
    }
}

/// Check only injected CPU image publication; this does not observe validation or execute END.
#[cfg(test)]
pub(crate) fn recorded_cpu_publication<V: SourceViewOutput>(
    captured: &SourceCompletion<V>,
) -> Result<(), Box<dyn Error>> {
    require_source_captures(
        &captured.screenshot,
        captured.basename,
        &captured.additional,
    )
}
