use super::*;

/// Entire bug27530, including all six cycle operations; `overlapping_box_cycle()`.
pub(crate) fn overlapping_box_cycle() -> Outcome {
    banner(&[
        "===========",
        "OCC27530: Visualization - AisInteractiveContext::HilightNextDetected() doesn't work in Neutral Point",
        "===========",
        "",
    ]);
    let boxes = cycle::boxes()?;
    let mut run = SessionRun::new()?;
    cycle::scene(run.session(), &boxes)?;
    run.session().move_cursor(210, 280)?;
    cycle::check(run.session(), 0)?;
    for stage in 1..7 {
        cycle::advance(run.session(), stage)?;
        cycle::check(run.session(), stage)?;
    }
    run.dump("bug27530.png")?;
    run.finish()
}

/// Entire bug31412, preserving the implicit display initialization; `occluded_box_projections()`.
pub(crate) fn occluded_box_projections() -> Outcome {
    banner(&[
        "=============",
        "0031412: Visualization - entity behind is returned as topmost at the edges",
        "=============",
    ]);
    let boxes = occlusion::boxes()?;
    // Native VDisplay2 initializes a null context here, after both box constructions.
    let mut run = SessionRun::new()?;
    occlusion::scene(run.session(), &boxes)?;
    for stage in occlusion::STAGES {
        occlusion::probe(run.session(), stage)?;
        marker::display_no_selection(run.session(), stage.marker, stage.cursor)?;
        run.dump(stage.image)?;
        run.session().remove(&[stage.marker])?;
    }
    run.dump("bug31412.png")?;
    run.finish()
}

/// Entire bug31425, including the unshown one-edge wire; `free_edge_camera_pick()`.
pub(crate) fn free_edge_camera_pick() -> Outcome {
    banner(&[
        "=============",
        "0031425: Visualization - free Edge has selection sensitivity inconsistent to presentation",
        "=============",
    ]);
    let inputs = circle::inputs()?;
    // The source VClear precedes VInit; this fresh worker owns no earlier viewer.
    let mut run = SessionRun::new()?;
    circle::scene(run.session(), &inputs)?;
    run.session().move_cursor(275, 175)?;
    let point = run.session().detected_point()?;
    check_point("pick", point, [150., 31., 294.], 1.0);
    run.dump("bug31425.png")?;
    run.finish()
}
