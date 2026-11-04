//! Complete source case bodies; stronger observation requirements live only in controls.
use super::*;
use super::{state_text, topology::TopologyOperands};
use crate::primitive_shapes::{box_at, box_sized};
use crate::session_run::{SessionRun, banner};
use martensite_tkv3d::{ais::AisSelectionScheme::Xor, v3d::V3dTypeOfOrientation};

/// Run the complete topology roundtrip with its weak source checks; `selected_topology_roundtrip()?`.
pub(crate) fn selected_topology_roundtrip() -> Outcome {
    let (mut run, mut operands) = roundtrip_start()?;
    run.session().select_point(100, 100)?;
    super::wire::roundtrip_checks(run.session(), &mut operands, false)?;
    run.session().select_point(0, 0)?;
    super::topology::remove_named(run.session(), "aSelected")?;
    run.session().set_object_selection_mode("aBox", 6, true)?;
    run.session().select_point(100, 100)?;
    super::wire::roundtrip_checks(run.session(), &mut operands, true)?;
    run.dump("bug25627.png")?;
    run.finish()
}

fn roundtrip_start() -> Result<(SessionRun, TopologyOperands), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25627: SelectedShape() and HasSelectedShape() of AisInteractiveContext class do not work as expected.",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let shape = box_sized([5.; 3])?;
    let mut operands = TopologyOperands::default();
    operands.bind("aBox", shape.clone());
    run.session()
        .display("aBox", &shape, &DisplayRequest::default())?;
    run.session().fit()?;
    Ok((run, operands))
}

/// Run both source state-query phases and one dump; `local_and_neutral_rows()?`.
pub(crate) fn local_and_neutral_rows() -> Outcome {
    let (mut run, b2) = local_start()?;
    super::wire::report(
        &[],
        &super::predicates::first_face_errors(&state_text(run.session(), &["b1"], false)?),
    );
    mixed(&mut run, &b2)?;
    run.dump("bug26779.png")?;
    run.finish()
}

fn local_start() -> Result<(SessionRun, Shape), Box<dyn Error>> {
    banner(&[
        "============",
        "CR26779: Test Commands, ViewerCommands - vstate output is incorrect for sub-shapes selection without opening local context",
        "============",
        "",
    ]);
    let b1 = box_sized([1., 2., 3.])?;
    let b2 = box_at([5.; 3], [1.; 3])?;
    // VClear/VClose ALL precede VInit: this isolated fresh session has no prior namespace/view.
    let mut run = SessionRun::new()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    run.session()
        .display("b1", &b1, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b1", 4, true)?;
    run.session().select_point(300, 300)?;
    Ok((run, b2))
}

fn mixed(run: &mut SessionRun, b2: &Shape) -> Outcome {
    run.session()
        .display("b2", b2, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().select_polyline(&[(350, 120)], Xor)?;
    let text = state_text(run.session(), &["b1", "b2"], true)?;
    let (lines, errors) = super::predicates::mixed_report(&text);
    super::wire::report(&lines, &errors);
    Ok(())
}

/// Run the original line discriminator without adding an image; `line_shape_discrimination()?`.
pub(crate) fn line_shape_discrimination() -> Outcome {
    banner(&[
        "=============",
        "0027848: Visualization - sensitivity of lines is too high",
        "=============",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    super::geometry::display_line_scene(run.session())?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().fit()?;
    run.session().select_point(100, 305)?;
    if super::predicates::line_was_reported(&state_text(run.session(), &["b", "l", "p"], false)?) {
        println!("Error: AisShape should be detected");
    }
    run.finish_without_image()
}
