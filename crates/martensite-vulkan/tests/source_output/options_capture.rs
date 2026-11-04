//! Explicit dump options borrow the real view while the original owner survives until END.
use super::*;
use martensite_tkservice::image::ImagePixMap;
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dViewSource};

struct OptionsSourceView<'scene, 'options, 'name, V> {
    scene: &'scene V,
    options: &'options V3dImageDumpOptions<'name>,
}

impl<V: SourceViewOutput> SourceViewOutput for OptionsSourceView<'_, '_, '_, V> {
    fn source_view_handle(&self) -> &Handle<dyn V3dViewSource> {
        self.scene.source_view_handle()
    }
    fn source_window_size(&self) -> (i32, i32) {
        self.scene.source_window_size()
    }
    fn source_dump(
        &self,
        image: &ImagePixMap,
    ) -> Result<bool, martensite_tkernel::standard::Failure> {
        self.scene.source_dump_options(image, self.options)
    }
}

/// Publish one requested original dump, retaining its genuine owner; `capture_case_options(driver, scene, name, &options)?`.
pub(crate) fn capture_case_options<V: SourceViewOutput>(
    driver: Handle<Graphic3dRenderDriver>,
    scene: V,
    basename: &'static str,
    options: &V3dImageDumpOptions,
) -> Result<SourceCompletion<V>, Box<dyn Error>> {
    let screenshot = source_dump(
        &OptionsSourceView {
            scene: &scene,
            options,
        },
        basename,
    )?;
    Ok(completion(driver, scene, basename, screenshot))
}

impl<V: SourceViewOutput> SourceCompletion<V> {
    /// Keep the requested capture through the same authored END; `capture.capture_additional_options(name, &options)?`.
    pub(crate) fn capture_additional_options(
        &mut self,
        basename: &'static str,
        options: &V3dImageDumpOptions,
    ) -> Result<(), Box<dyn Error>> {
        let screenshot = source_dump(
            &OptionsSourceView {
                scene: &self.scene,
                options,
            },
            basename,
        )?;
        self.additional.push((screenshot, basename));
        Ok(())
    }
}
