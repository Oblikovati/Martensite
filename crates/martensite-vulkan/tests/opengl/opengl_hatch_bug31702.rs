//! Two genuine viewers share one driver and independently evaluate the source named-pixel predicates.
use super::source_output::{SourceCompletion, capture_case};
use crate::hatch_surface::scene::{HatchBoxScene, HatchSourceView, SourceViewOutput, box_shape};
use martensite_tkernel::{quantity::Color, standard::Handle};
use martensite_tkservice::{
    graphic3d::Graphic3dBufferType,
    image::{ImageFormat, ImagePixMap},
};
use martensite_vulkan::Graphic3dRenderDriver;
use std::{error::Error, io::Write};

#[derive(Clone, Copy)]
enum NamedHatchView {
    First,
    Second,
    SecondSolidControl,
}

struct ObservedHatchView {
    completion: SourceCompletion,
    pixel_name: &'static str,
}

impl NamedHatchView {
    fn basename(self) -> &'static str {
        match self {
            Self::First => "bug31702_v1.png",
            Self::Second | Self::SecondSolidControl => "bug31702_v2.png",
        }
    }

    fn pixel_failure(self) -> &'static str {
        match self {
            Self::First => "Error: no hatching in first view",
            Self::Second | Self::SecondSolidControl => "Error: no hatching in second view",
        }
    }
}

/// Keep both views alive through the inherited END; `case_body()?`.
pub(super) fn case_body() -> Result<(), Box<dyn Error>> {
    let completions = case_commands()?;
    std::io::stdout().write_all(b"TEST COMPLETED\n")?;
    finish_views(completions)
}

/// Render both real views while omitting only source completion; `commands_without_end()?`.
pub(super) fn commands_without_end() -> Result<(), Box<dyn Error>> {
    let completions = case_commands()?;
    finish_views(completions)
}

/// Exercise the actual second-view predicate on an honest solid control; `second_pixel_failure_body()?`.
pub(super) fn second_pixel_failure_body() -> Result<(), Box<dyn Error>> {
    source_heading()?;
    let driver = Graphic3dRenderDriver::new(super::device_options::validation_options())?;
    let first = source_view(&driver, NamedHatchView::First)?;
    record_control_pixel("first", first.pixel_name)?;
    let second = source_view(&driver, NamedHatchView::SecondSolidControl)?;
    record_control_pixel("second", second.pixel_name)?;
    std::io::stdout().write_all(b"TEST COMPLETED\n")?;
    let names = (first.pixel_name, second.pixel_name);
    finish_views([first.completion, second.completion])?;
    require_control_pixels(names);
    Ok(())
}

fn record_control_pixel(stage: &str, name: &str) -> std::io::Result<()> {
    // Report the actual named readback, rather than supplying a predicted name to the predicate.
    writeln!(
        std::io::stdout(),
        "{{\"control_{stage}_named_pixel\":{name:?}}}"
    )
}

fn require_control_pixels(names: (&str, &str)) {
    assert_eq!(
        names.0, "BLACK",
        "first={:?}; expected actual first HATCH BLACK",
        names.0
    );
    assert_ne!(
        names.1, "BLACK",
        "second={:?}; expected actual second Solid nonBLACK",
        names.1
    );
}

fn case_commands() -> Result<[SourceCompletion; 2], Box<dyn Error>> {
    source_heading()?;
    // The isolated worker has no current view: the initial clear and close take empty branches.
    let driver = Graphic3dRenderDriver::new(super::device_options::validation_options())?;
    let first = source_view(&driver, NamedHatchView::First)?;
    let second = source_view(&driver, NamedHatchView::Second)?;
    Ok([first.completion, second.completion])
}

fn source_heading() -> std::io::Result<()> {
    let issue = "0031702: Visualization, Renderer - hatching interior is ignored in second View within the Viewer";
    writeln!(std::io::stdout(), "============\n{issue}\n============\n")
}

fn source_view(
    driver: &Handle<Graphic3dRenderDriver>,
    named: NamedHatchView,
) -> Result<ObservedHatchView, Box<dyn Error>> {
    let scene = initialized_box_scene(driver)?;
    scene.source_hatch_default()?;
    if matches!(named, NamedHatchView::SecondSolidControl) {
        scene.hatch(false);
    }
    let pixel_name = check_named_pixel(&scene, named)?;
    let completion = capture_case(driver.clone(), scene, named.basename())?;
    Ok(ObservedHatchView {
        completion,
        pixel_name,
    })
}

fn initialized_box_scene(
    driver: &Handle<Graphic3dRenderDriver>,
) -> Result<HatchBoxScene, Box<dyn Error>> {
    // The source names v1/v1 and v2/v1 resolve distinct viewer/context pairs on this SAME driver.
    let initialized = HatchSourceView::new(driver)?;
    let scene = initialized.with_shape(box_shape([100., 200., 300.]));
    scene.source_display_shaded()?;
    scene.source_fit()?;
    Ok(scene)
}

fn check_named_pixel(
    scene: &HatchBoxScene,
    named: NamedHatchView,
) -> Result<&'static str, Box<dyn Error>> {
    let actual = source_pixel_name(scene, 201, 200)?;
    if actual != "BLACK" {
        // A source error remains soft: its image, the other view and END must still execute.
        writeln!(std::io::stdout(), "{}", named.pixel_failure())?;
    }
    Ok(actual)
}

fn source_pixel_name(
    scene: &HatchBoxScene,
    x: i32,
    y: i32,
) -> Result<&'static str, Box<dyn Error>> {
    let (width, height) = scene.source_window_size();
    if x < 0 || x >= width || y < 0 || y > height {
        return Err(std::io::Error::other(format!(
            "vreadpixel received({x},{y}); expected within({width},{height})"
        ))
        .into());
    }
    let image = source_rgb_image(scene, width, height)?;
    let color = image.pixel_color(x, y, true)?;
    Ok(Color::string_name(color.rgb().name()))
}

fn source_rgb_image(
    scene: &HatchBoxScene,
    width: i32,
    height: i32,
) -> Result<ImagePixMap, Box<dyn Error>> {
    let image = allocated_rgb_image(width, height)?;
    // VReadPixel calls ToPixMap directly, without VDump's immediate-update suppression.
    let buffer = Graphic3dBufferType::Rgb;
    if !scene
        .source_view_handle()
        .to_pix_map_sized(&image, width, height, buffer, true)?
    {
        return Err(std::io::Error::other(format!(
            "Error: image dump failed: received RGB({width},{height}); expected actual view image"
        ))
        .into());
    }
    Ok(image)
}

fn allocated_rgb_image(width: i32, height: i32) -> Result<ImagePixMap, Box<dyn Error>> {
    let image = ImagePixMap::default();
    if !image.init_trash(ImageFormat::Rgb, width as usize, height as usize, 0) {
        return Err(std::io::Error::other(format!(
            "Error: image allocation failed: received RGB({width},{height}); expected complete image"
        ))
        .into());
    }
    Ok(image)
}

fn finish_views(completions: [SourceCompletion; 2]) -> Result<(), Box<dyn Error>> {
    let [first, second] = completions;
    // The capture wrapper retires the current second view before the still-live first view.
    second.finish_after_end()?;
    first.finish_after_end()
}
