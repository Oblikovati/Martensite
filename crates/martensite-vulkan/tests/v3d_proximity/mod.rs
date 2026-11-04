#[cfg(test)]
mod cpu;
use crate::case_view::viewer_session::DisplayRequest;
use crate::primitive_shapes::{compound_of, cone, sphere};
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{MeshRequest, mesh_shape, remove_triangulation};
use crate::shape_report_checks::{ShapeCounts, check_shape_counts};
use martensite_tkbrep::topods::Shape;
use martensite_tktopalgo::brep_extrema::BRepExtremaShapeProximity;
type BoxError = Box<dyn std::error::Error>;
const COUNTS: [usize; 9] = [2, 3, 1, 1, 0, 0, 0, 1, 8];

fn source_mesh(shape: &Shape) -> Result<(), BoxError> {
    remove_triangulation(shape)?;
    mesh_shape(shape, &MeshRequest::new(0.001))?;
    Ok(())
}

fn overlap_compounds(tool: &BRepExtremaShapeProximity) -> Result<[Shape; 2], BoxError> {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for (index, _) in tool.overlap_sub_shapes1().items() {
        first.push(tool.get_sub_shape1(*index)?);
        println!("s_{} ", index + 1);
    }
    for (index, _) in tool.overlap_sub_shapes2().items() {
        second.push(tool.get_sub_shape2(*index)?);
        println!("c_{} ", index + 1);
    }
    Ok([compound_of(&first)?, compound_of(&second)?])
}

fn actual_overlapping_faces() -> Result<[Shape; 2], BoxError> {
    let sphere = sphere(10.)?;
    source_mesh(&sphere)?;
    let cone = cone(5., 20., 30.)?;
    source_mesh(&cone)?;
    let start = std::time::Instant::now();
    let mut tool = BRepExtremaShapeProximity::with_shapes(&sphere, &cone, 0.01)?;
    let initialization = start.elapsed();
    let start = std::time::Instant::now();
    tool.perform()?;
    let work = start.elapsed();
    report_proximity_profile(&tool, initialization, work)?;
    overlap_compounds(&tool)
}

fn report_proximity_profile(
    tool: &BRepExtremaShapeProximity,
    initialization: std::time::Duration,
    work: std::time::Duration,
) -> Result<(), BoxError> {
    if !tool.is_done() {
        return Err("proximity done=false; expected completed sphere/cone overlap".into());
    }
    println!(
        "Number of primitives in shape 1: {}",
        tool.element_set1().size()
    );
    println!(
        "Number of primitives in shape 2: {}",
        tool.element_set2().size()
    );
    println!("Building data structures: {}", initialization.as_secs_f64());
    println!("Executing proximity test: {}", work.as_secs_f64());
    Ok(())
}

/// Preserve both actual meshes, overlapping-face bindings and authored topology checks.
/// Example: `sphere_cone_overlap()?`.
pub(crate) fn sphere_cone_overlap() -> Result<(), BoxError> {
    banner(&["========", "OCC25398", "========", ""]);
    let [sphere, cone] = actual_overlapping_faces()?;
    check_shape_counts(&sphere, "s_overlapped", &authored_overlap_expectation())?;
    check_shape_counts(&cone, "c_overlapped", &authored_overlap_expectation())?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("s_overlapped", &sphere, &DisplayRequest::default())?;
    run.session()
        .display("c_overlapped", &cone, &DisplayRequest::default())?;
    run.session().fit()?;
    run.dump("bug25398.png")?;
    run.finish()
}

/// Retain both original overlap commands' located counts and literal report subject.
/// Example: `check_shape_counts(&shape, "s_overlapped", &authored_overlap_expectation())?`.
fn authored_overlap_expectation() -> ShapeCounts {
    ShapeCounts {
        per_location: true,
        subject: "Overlapped part of shere",
        ..ShapeCounts::all(COUNTS)
    }
}
