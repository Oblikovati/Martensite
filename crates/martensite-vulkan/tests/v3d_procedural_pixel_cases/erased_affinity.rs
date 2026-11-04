//! Loading an erased object must register its later presentation with the retained affinity.
use crate::primitive_shapes::box_sized;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::Handle;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::{PrsMgrDisplayStatus, PrsMgrPresentableObjectMembers as _};
use martensite_tkv3d::v3d::V3dViewSource;
use std::error::Error;

pub(crate) fn erased_two_view_affinity() -> Result<(), Box<dyn Error>> {
    banner(ERASED_LOAD_BANNER);
    // A fresh run starts with nothing to clear before the two views are opened.
    let mut run = SessionRun::new()?;
    load_erased_in_second_view(&mut run)?;
    run.session().fit()?;
    expect_named_pixel(
        &mut run,
        "DARKGOLDENROD",
        "Error: object is not displayed in Shaded mode",
    )?;
    run.dump("bug29262_view2.png")?;
    run.session().activate_view(0)?;
    expect_named_pixel(&mut run, "BLACK", "Error: object view affinity is ignored")?;
    run.dump("bug29262_view1.png")?;
    run.finish()
}

fn load_erased_in_second_view(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let session = run.session();
    let first = session.view().clone();
    session.open_view(409, 409)?;
    let second = session.view().clone();
    let object: Handle<dyn AisInteractiveObject> = AisShape::new(&box_sized([1., 2., 3.])?).cast();
    object.set_display_mode(1);
    session.bind_object("b", object.clone())?;
    session.update_context(|context| erased_load_affinity(context, &object, &first, &second))?;
    session.update_context(|context| context.display(&object, false))?;
    Ok(())
}

fn expect_named_pixel(
    run: &mut SessionRun,
    expected: &str,
    diagnostic: &str,
) -> Result<(), Box<dyn Error>> {
    if run.session().pixel_color_name(200, 200)? != expected {
        println!("{diagnostic}");
    }
    Ok(())
}

fn erased_load_affinity(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
    first: &Handle<dyn V3dViewSource>,
    second: &Handle<dyn V3dViewSource>,
) -> Result<(), martensite_tkernel::standard::Failure> {
    context.display_with_modes(object, 1, 0, false, PrsMgrDisplayStatus::Erased)?;
    context.set_view_affinity(object, first, false)?;
    context.set_view_affinity(object, second, false)?;
    context.set_view_affinity(object, second, true)
}

#[cfg(test)]
#[path = "erased_affinity_tests.rs"]
mod tests;

const ERASED_LOAD_BANNER: &[&str] = &[
    "========",
    "0029262: Visualization - AisInteractiveContext::Load() does not register Object in the Viewer",
    "========",
    "",
];
