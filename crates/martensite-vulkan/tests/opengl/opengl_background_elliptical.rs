//! The original background command operates on an initialized shape-free view.
use crate::hatch_surface::scene::HatchSourceView;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Handle,
};
use martensite_tkservice::{aspect::AspectGradientFillMethod, graphic3d::Graphic3dCubeMap};
use martensite_vulkan::Graphic3dRenderDriver;
use std::{error::Error, io::Write};

/// Execute the literal source and inherited completion; `case_body()?`.
pub(super) fn case_body() -> Result<(), Box<dyn Error>> {
    source_heading()?;
    let driver = Graphic3dRenderDriver::new(super::device_options::validation_options())?;
    case_view(driver)
}

/// Diagnose an explicit opt-in without changing the original default; `last_policy_diagnostic_body()?`.
pub(super) fn last_policy_diagnostic_body() -> Result<(), Box<dyn Error>> {
    source_heading()?;
    let policy = martensite_vulkan::RenderRasterPolicy::LastProvokingVertexV2;
    let options = super::device_options::validation_options();
    let driver = Graphic3dRenderDriver::with_raster_policy(options, policy)?;
    case_view(driver)
}

fn case_view(driver: Handle<Graphic3dRenderDriver>) -> Result<(), Box<dyn Error>> {
    let view = HatchSourceView::new(&driver)?;
    view.source_update(|view| {
        let first = Color::named(NamedColor::WHITE);
        let second = Color::named(NamedColor::BLACK);
        view.set_bg_gradient_colors(first, second, AspectGradientFillMethod::Elliptical, false);
        view.set_background_cube_map(&Handle::<dyn Graphic3dCubeMap>::default(), true, false);
        Ok(())
    })?;
    let completion = super::source_output::capture_case(driver, view, "elliptical.png")?;
    std::io::stdout().write_all(b"TEST COMPLETED\n")?;
    completion.finish_after_end()
}

fn source_heading() -> std::io::Result<()> {
    let issue = "0031039: Visualization - add elliptical gradient background style";
    writeln!(std::io::stdout(), "============\n{issue}\n============\n")
}
