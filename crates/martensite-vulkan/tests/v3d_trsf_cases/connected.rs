//! Instances and assemblies of shapes at their own locations.
use super::prelude::*;
use crate::case_view::viewer_session::LocationChange::*;

/// `bug24358`: an assembly displays the same in a view reopened on the kept context.
pub(crate) fn assembly_after_reopened_view() -> Outcome {
    banner(&[
        "===========",
        "0024358: TKV3d - connected structures are not re-computed on device lost",
        "===========",
        "",
    ]);
    let b1 = box_at([0., 0., 0.], [1., 2., 3.])?;
    let b2 = box_at([3., 0., 0.], [3., 2., 1.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    let parts = [
        ConnectedPart::shape("b1", &b1),
        ConnectedPart::shape("b2", &b2),
    ];
    run.session()
        .connect_assembly("b1c", [-3., 0., 0.], &parts)?;
    run.session().fit()?;
    run.dump("bug24358_1.png")?;
    run.session().reopen_view()?;
    run.session().display_again("b1c")?;
    run.session().fit()?;
    run.dump("bug24358_2.png")?;
    run.finish()
}

/// `bug24756`: an instance of an instance follows its parent's moved location.
pub(crate) fn moved_instance_chain() -> Outcome {
    banner(&[
        "============",
        "CR24756: AisConnectedInteractive does not support nested AisConnectedInteractive",
        "============",
        "",
    ]);
    let b = box_sized([1., 1., 1.])?;
    let mut run = SessionRun::new()?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session()
        .connect_instance("b1", [2., 0., 0.], ConnectedPart::object("b"))?;
    run.session()
        .connect_instance("b2", [2., 0., 0.], ConnectedPart::object("b1"))?;
    run.session().set_translation("b1", [2., 1., 0.])?;
    run.session().fit()?;
    run.dump("bug24756.png")?;
    run.finish()
}

/// `bug27285`: an assembly of an object is selected together with the object.
pub(crate) fn selected_assembly() -> Outcome {
    banner(&[
        "========",
        "OCC27285: Visualization - selection of AisMultipleConnectedInteractive is broken",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    let b = box_sized([100., 100., 100.])?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session()
        .connect_assembly("bc", [0., 0., 300.], &[ConnectedPart::object("b")])?;
    run.session().fit()?;
    run.session().toggle_selected("b")?;
    run.session().toggle_selected("bc")?;
    if run.session().selected_count() != 2 {
        println!("ERROR: only one object is selected");
    }
    if !run.session().is_selected("bc") {
        println!("ERROR: multiple connected interactive object is not selected");
    }
    run.dump("bug27285.png")?;
    run.finish()
}

/// `bug25276`: shading stays correct on assemblies translated, rotated, scaled and mirrored.
pub(crate) fn transformed_assemblies() -> Outcome {
    banner(&[
        "============",
        "0025276: Visualization - Lighting is broken if some kinds of transformation applied to a shape",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let b1 = box_sized([1., 6., 1.])?;
    run.session().set_display_mode(1)?;
    run.session()
        .display("b1", &b1, &DisplayRequest::default())?;
    run.session()
        .connect_instance("b2", [6., 0., 0.], ConnectedPart::object("b1"))?;
    let b3 = box_sized([7., 1., 1.])?;
    run.session()
        .display("b3", &b3, &DisplayRequest::default())?;
    let session = run.session();
    session.change_location("b3", Translate([0., 4., 0.]))?;
    let parts = ["b1", "b2", "b3"].map(ConnectedPart::object);
    session.connect_assembly("z", [0., 0., 0.], &parts)?;
    let z = [ConnectedPart::object("z")];
    session.connect_assembly("z1", [0., 0., 0.], &z)?;
    session.change_location("z1", Translate([10., 0., 0.]))?;
    session.connect_assembly("z2", [0., 10., 0.], &z)?;
    let (origin, axis) = ([0.; 3], [1., 0., 0.]);
    session.change_location(
        "z2",
        PreRotate {
            origin,
            axis,
            degrees: 90.,
        },
    )?;
    session.connect_assembly("z3", [-10., 0., 0.], &z)?;
    session.change_location(
        "z3",
        PreScale {
            center: [0.; 3],
            factor: 0.5,
        },
    )?;
    session.connect_assembly("z4", [0., 0., 0.], &z)?;
    session.change_location("z4", CopyFrom("z3"))?;
    let sp = sphere(3.)?;
    session.display("sp", &sp, &DisplayRequest::default())?;
    session.change_location("sp", CopyFrom("z3"))?;
    session.change_location("sp", Reset)?;
    let (origin, normal) = ([0., -0.5, 0.], [0., 1., 0.]);
    session.change_location("z", Mirror { origin, normal })?;
    session.fit()?;
    run.dump("bug25276.png")?;
    run.finish()
}

/// A sphere of `radius` about the origin; `sphere(3.)?`.
fn sphere(radius: f64) -> Result<martensite_tkbrep::topods::Shape, Box<dyn Error>> {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(radius)?;
    Ok(maker.shape()?.clone())
}
