//! The complete shared sample precedes each wrapper’s actual lighting and capture commands.
mod capping;
#[cfg(test)]
mod controls;
#[path = "../support/dimension_params.rs"]
mod dimension_params;
#[path = "../support/dimension_request.rs"]
mod dimension_request;
mod dimensions;
mod lighting_tails;
mod section_inputs;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::session_run::SessionRun;
#[allow(unused_imports)]
pub(crate) use lighting_tails::{
    dimensions_fixed_function, dimensions_physically_based, dimensions_programmable,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle},
};
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial as Material;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
type Outcome = Result<(), Box<dyn std::error::Error>>;

struct DimensionsScene {
    run: SessionRun,
    _inputs: [Shape; 2],
}
/// Execute the entire scene before the wrapper’s actual tail; `dimensions_sample("dimensions.png", tail)?`.
fn dimensions_sample(
    image: &'static str,
    after_scene: impl FnOnce(&mut SessionRun) -> Outcome,
) -> Outcome {
    let mut scene = build_dimensions_scene()?;
    after_scene(&mut scene.run)?;
    scene.run.dump(image)?;
    scene.run.finish()
}
fn build_dimensions_scene() -> Result<DimensionsScene, Box<dyn std::error::Error>> {
    let (shapes, hatch) = section_inputs::read_case_inputs()?;
    let mut run = SessionRun::with_view_size(768, 768)?;
    prepare_view(run.session())?;
    println!("Getting cut projection...");
    let sections = section_inputs::cut_sections(&shapes)?;
    show_sections(run.session(), &sections)?;
    create_and_update_dimensions(run.session(), &sections)?;
    finish_scene(&mut run, &shapes, &hatch)?;
    Ok(DimensionsScene {
        run,
        _inputs: shapes,
    })
}
fn create_and_update_dimensions(session: &mut ViewerSession, sections: &[Shape; 2]) -> Outcome {
    println!("Creating dimensions...");
    dimensions::create_all(session, sections)?;
    println!("Changing text and arrow parameters of dimensions...");
    dimensions::update_all(session)?;
    session.fit()?;
    Ok(())
}
fn prepare_view(session: &mut ViewerSession) -> Result<(), Failure> {
    session.clear()?;
    session.set_display_mode(0)?;
    session.set_background_color(Color::named(NamedColor::WHITE))?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_BOTTOM)
}
fn show_sections(session: &mut ViewerSession, sections: &[Shape; 2]) -> Result<(), Failure> {
    session.display("bs", &sections[1], &DisplayRequest::default())?;
    session.display("as", &sections[0], &DisplayRequest::default())?;
    session.fit()
}
fn finish_scene(run: &mut SessionRun, shapes: &[Shape; 2], hatch: &std::path::Path) -> Outcome {
    println!("Displaying exported shapes...");
    let session = run.session();
    show_solids(session, shapes)?;
    println!("Clipping shapes for better view...");
    capping::attach_planes(session, hatch)?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_BOTTOM)?;
    session.rotate(-0.3, -0.3, 0.)?;
    session.fit()?;
    session.zoom(1.2)?;
    session.set_sample_count(8)?;
    println!("Done.");
    Ok(())
}

fn show_solids(session: &mut ViewerSession, shapes: &[Shape; 2]) -> Outcome {
    session.display(
        "a",
        &shapes[0],
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.display(
        "b",
        &shapes[1],
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.set_material("a", Material::Steel)?;
    session.set_material("b", Material::Bronze)?;
    Ok(())
}
