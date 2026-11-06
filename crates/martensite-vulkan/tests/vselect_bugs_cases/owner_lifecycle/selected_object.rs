//! OCC27893 selects the anonymous real box via the context's object overload.
use super::*;

pub(super) fn select_anonymous_box(context: &AisInteractiveContext) -> Outcome {
    let (_shape, object) = objects::anonymous_box(context)?;
    context.set_selected_object(&object, true)?;
    // The command returns before the view is fitted; the context retains its object.
    Ok(())
}

pub(crate) fn direct_selected_object() -> Outcome {
    let mut run = lifecycle_begin(
        "===========",
        "OCC27893: Visualization - AisInteractiveContext::SetSelected does not work",
    )?;
    select_anonymous_box(run.session().context())?;
    run.session().fit()?;
    check_direct_selection(&mut run)?;
    run.dump("bug27893.png")?;
    run.finish()
}

fn check_direct_selection(run: &mut SessionRun) -> Outcome {
    // The OR does not read a pixel when the count already fails.
    if run.session().selected_count() != 1 || run.session().pixel_rgba_name(204, 254)? != "GRAY80 1"
    {
        println!(
            "ERROR: The box is not selected, AisInteractiveContext::SetSelected works incorrect"
        );
    }
    Ok(())
}
