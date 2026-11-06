//! Color and material programs share real renderer operations, never an interpreter.
mod color_table;
mod colors;
#[cfg(test)]
mod controls;
mod material_labels;
mod material_objects;
mod materials_scene;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Failure,
};
use martensite_tkservice::graphic3d::{Graphic3dMaterialAspect, Graphic3dTypeOfShadingModel};
type Outcome = Result<(), Box<dyn std::error::Error>>;
pub(crate) use colors::colors;
const MATERIALS: [&str; 20] = [
    "brass",
    "bronze",
    "copper",
    "gold",
    "jade",
    "neon_phc",
    "pewter",
    "obsidian",
    "plaster",
    "plastic",
    "satin",
    "silver",
    "steel",
    "stone",
    "chrome",
    "aluminium",
    "water",
    "glass",
    "diamond",
    "charcoal",
];
const COLUMN_COLORS: [&str; 4] = ["default", "red", "green", "blue"];

pub(crate) fn phong() -> Outcome {
    banner(&[
        "=========",
        "0024872: Support of emission color in Phong shader and ray-tracing",
        "Test emission material within Phong rasterization model",
        "=========",
        "",
    ]);
    let mut run = materials_scene::materials_scene()?;
    run.session()
        .set_shading_model(Graphic3dTypeOfShadingModel::Phong)?;
    run.session().set_sample_count(0)?;
    run.dump("phong1.png")?;
    run.finish()
}
pub(crate) fn raytrace() -> Outcome {
    banner(&[
        "=========",
        "0024872: Support of emission color in Phong shader and ray-tracing",
        "Test emission material within Ray-Tracing",
        "=========",
        "",
    ]);
    let mut run = materials_scene::materials_scene()?;
    run.session().select_ray_tracing()?;
    run.session().set_sample_count(0)?;
    run.dump("raytrace1.png")?;
    run.finish()
}
fn named_color(name: &str) -> Result<Color, Failure> {
    NamedColor::from_name(name)
        .map(Color::named)
        .ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "color {name:?}; expected a predefined source color"
            ))
        })
}

/// The demo sample retains its eight-sample request instead of the Phong overrides.
pub(crate) fn demo_materials() -> Outcome {
    banner(&[
        "Test visual properties of materials in 3d viewer",
        "Workaround instability on GeForce cards with MSAA at semitransparent sphere",
        "IMAGE_COLOR_TOLERANCE: 0.008",
    ]);
    let mut run = materials_scene::materials_scene()?;
    run.dump("materials.png")?;
    run.finish()
}
