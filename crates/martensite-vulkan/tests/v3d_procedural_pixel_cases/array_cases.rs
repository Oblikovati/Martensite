//! Actual point and mutable triangle presentations drive the original pixel predicates.
use super::arrays::{mutable_triangles, patch_triangles, point_presentation};
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::Handle;
use martensite_tkv3d::ais::PrimitiveArrayPresentation;
use std::error::Error;

pub(crate) fn four_point_pixels() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC23186: Unable to display Graphic3d_ArrayOfPoints after migrating from version 6.5.2",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    display_array(&mut run, point_presentation()?)?;
    run.session().fit()?;
    for (x, y) in [(321, 2), (321, 271), (87, 137), (87, 406)] {
        run.session().check_color(x, y, [1., 1., 0.])?;
    }
    run.dump("bug23186.png")?;
    run.finish()
}

pub(crate) fn patch_vertex_colors() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=============",
        "0030076: Visualization, TKV3d - API to update certain vertex attribute(s) without recomputing a presentation",
        "=============",
    ]);
    let mut run = SessionRun::new()?;
    let object = mutable_triangles()?;
    display_array(&mut run, object.clone())?;
    run.session().fit()?;
    run.dump("bug30076_1.png")?;
    if !patch_triangles(&object)? {
        println!("Syntax error: array cannot be patched");
    }
    run.session().update_view(|_| Ok(()))?;
    check_patched_pixels(&mut run)?;
    run.dump("bug30076_2.png")?;
    run.finish()
}

fn display_array(
    run: &mut SessionRun,
    object: Handle<PrimitiveArrayPresentation>,
) -> Result<(), Box<dyn Error>> {
    run.session().bind_object("a", object.cast())?;
    run.session()
        .update_context(|context| context.display(&object.cast(), false))?;
    Ok(())
}

fn check_patched_pixels(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    for (y, expected) in [(150, "RED2"), (250, "BLUE2")] {
        if run.session().pixel_color_name(200, y)? != expected {
            println!("Error: array was not updated");
        }
    }
    Ok(())
}
