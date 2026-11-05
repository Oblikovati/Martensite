//! Meshing performance regressions: the meshing or the shaded display completes within the
//! group's CPU limit and keeps its counts. The group's end hook dumps the open view as
//! `<case>.png`.
use super::prelude::*;
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeRevol;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;

/// `bug23650`: a slow-to-mesh shape cleaned, meshed at 0.2 and shown shaded.
pub(crate) fn meshed_shaded() -> Result<(), Box<dyn Error>> {
    banner(&["==========", "OCC23650", "==========", ""]);
    let result = read_dataset_shape("bug23650_slowmesh.brep")?;
    remeshed(&result, MeshRequest::new(0.2))?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    run.dump("bug23650.png")?;
    run.finish()
}

/// `bug23795`: a shape meshed at 0.1 keeps its counts and deflection; shown shaded from its
/// own triangulation.
pub(crate) fn meshed_shaded_counts() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=========",
        "0023795: Performance regression of meshing",
        "=========",
        "",
    ]);
    let result = read_dataset_shape("bug23795_s.brep")?;
    mesh_shape(&result, &MeshRequest::new(0.1))?;
    check_triangulation(
        &result,
        &counts_deflection(10992., 11016., 0.9990000181414841, 1e-6),
    )?;
    let mut run = shaded_view(&result, false)?;
    run.dump("bug23795.png")?;
    run.finish()
}

pub(crate) fn revolved(wire: &Shape, axis_y: f64, axis_z: f64) -> Result<Shape, Box<dyn Error>> {
    let axis = Axis1::new(
        Point3::new(20583.283203125, axis_y, axis_z),
        Direction3::new(0., 0., 1.)?,
    );
    let mut maker =
        BRepPrimAPIMakeRevol::new(wire, axis, 360.0 * (std::f64::consts::PI / 180.0), false)?;
    Ok(maker.shape()?.clone())
}

/// Measure the authored process-user CPU counter around the actual mesh call.
/// Example: `timed_revolution_mesh(&shape, "incmesh_1")?`.
pub(crate) fn timed_revolution_mesh(shape: &Shape, label: &str) -> Result<(), Box<dyn Error>> {
    let mut timer = martensite_tkernel::osd::Timer::new(false);
    timer.restart();
    mesh_shape(shape, &MeshRequest::new(0.1))?;
    timer.stop();
    println!(
        "COUNTER {label}: {}",
        martensite_tkernel::storage::format_general(timer.show_values().3, 17)
    );
    Ok(())
}

/// `bug25264`: a wire revolved about two nearly equal axes meshes at 0.1; both results are
/// shown in the check view, and the end hook dumps the last.
pub(crate) fn revolved_meshes() -> Result<(), Box<dyn Error>> {
    banner(&["================", "OCC25264", "================", ""]);
    let w = read_dataset_shape("bug25264_wire.brep")?;
    let result1 = revolved(&w, 14039.0208237378, 28443.2585934755)?;
    timed_revolution_mesh(&result1, "incmesh_1")?;
    let result2 = revolved(&w, 14039.0208217527, 28443.25860033352)?;
    timed_revolution_mesh(&result2, "incmesh_2")?;
    let mut run = SessionRun::new()?;
    crate::shape_view_checks::checkview_in(&mut run, &result1, "bug25264_1.png")?;
    // Each checkview -display invokes an unnamed vinit in the existing context.
    run.session().open_view(409, 409)?;
    crate::shape_view_checks::checkview_in(&mut run, &result2, "bug25264_2.png")?;
    run.dump("bug25264.png")?;
    run.finish()
}

/// `bug26965`: a shape whose shaded display used to hang shows with known counts.
pub(crate) fn display_mesh_counts() -> Result<(), Box<dyn Error>> {
    banner(&[
        "==========",
        "0026965: BRepMesh_IncrementalMesh hangs, and consumes infinite memory",
        "==========",
        "",
    ]);
    let a = read_dataset_shape("bug26965_mesh_hang.brep")?;
    remove_triangulation(&a)?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session().display("a", &a, &DisplayRequest::default())?;
    run.session().fit()?;
    run.dump("bug26965.png")?;
    check_triangulation(
        &a,
        &counts_deflection(15564., 7987., 0.25696012112765304, 1e-6),
    )?;
    run.finish()
}

/// `bug30511`: an assembly of one shared solid meshes at 0.1 with known counts.
pub(crate) fn assembly_mesh_counts() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=======",
        "0030511: Mesh - too long meshing of assembly containing single solid (shared)",
        "=======",
        "",
    ]);
    let result = read_dataset_shape("bug30511_hugeassembly0.brep")?;
    remeshed(&result, MeshRequest::new(0.1))?;
    check_triangulation(&result, &counts(1119744., 2239488.))?;
    println!("TEST COMPLETED");
    Ok(())
}
