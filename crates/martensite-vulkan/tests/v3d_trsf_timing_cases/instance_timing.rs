//! Time to connect many instances of one meshed sphere, and to clear them again.
use super::prelude::*;
use crate::case_view::viewer_session::{ConnectedPart, ViewerSession};
use crate::shape_mesh::{MeshRequest, mesh_shape};
use martensite_tkbrep::topods::Shape;
use martensite_tkservice::aspect::AspectTypeOfDeflection;
use std::time::Instant;

/// A shaded axonometric view of a sphere of radius 0.5 meshed at 0.1, with automatic
/// triangulation off (deflection 1.0, no automatic triangulation), displayed as `s`.
fn meshed_sphere_view() -> Result<(SessionRun, Shape), Box<dyn Error>> {
    let s = crate::primitive_shapes::sphere(0.5)?;
    crate::shape_mesh::remove_triangulation(&s)?;
    mesh_shape(&s, &MeshRequest::new(0.1))?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    if let Some(drawer) = run.session().context().default_drawer().get() {
        drawer.set_type_of_deflection(AspectTypeOfDeflection::Absolute);
        drawer.set_maximal_chordial_deviation(1.0);
    }
    run.session().set_auto_triangulation(false);
    run.session().display("s", &s, &DisplayRequest::default())?;
    Ok((run, s))
}

/// Connect `count` x `count` instances of `s` on a unit grid without updates; microseconds.
fn connect_grid(
    session: &mut ViewerSession,
    sphere: &Shape,
    count: usize,
) -> Result<f64, Box<dyn Error>> {
    println!("Creating {} instances...", count * count);
    let start = Instant::now();
    for i in 0..count {
        for j in 0..count {
            let name = format!("s_{i}_{j}");
            session.connect_instance_quietly(
                &name,
                [i as f64, j as f64, 0.],
                ConnectedPart::shape("s", sphere),
            )?;
        }
    }
    Ok(start.elapsed().as_secs_f64() * 1e6)
}

/// Clear the context; microseconds.
fn timed_clear(session: &mut ViewerSession) -> Result<f64, Box<dyn Error>> {
    let start = Instant::now();
    session.clear()?;
    Ok(start.elapsed().as_secs_f64() * 1e6)
}

/// `bug26029`: connecting 40000 instances takes at most 1.5 times four times 10000.
pub(crate) fn linear_instance_time() -> Outcome {
    banner(&[
        "============",
        "0026029: Visualization - Poor performance of connected objects",
        "Time spent on computation of large number of connected objects",
        "should grow linearly with the number of objects",
        "============",
        "",
    ]);
    let (mut run, s) = meshed_sphere_view()?;
    let d1 = connect_grid(run.session(), &s, 100)?;
    println!("Done in {d1} microseconds!\n");
    run.session().clear()?;
    let d2 = connect_grid(run.session(), &s, 200)?;
    println!("Done in {d2} microseconds!\n");
    run.session().clear()?;
    let (expected, actual) = (40000. / 10000., d2 / d1);
    println!(
        "Comparing CPU time for the two test cases...\n============================================="
    );
    println!(
        "Expected ratio: {expected}\nActual ratio:   {actual}\n============================================="
    );
    println!(
        "{}",
        if actual / expected > 1.5 {
            "Error: non-linear time growth detected!"
        } else {
            "Test passed!"
        }
    );
    run.finish_without_image()
}

/// `bug26199`: 10000 connected instances are created and cleared within a second each.
pub(crate) fn instance_clear_time() -> Outcome {
    banner(&[
        "==========",
        "OCC26199: Visualization - use NCollection_IndexedMap instead of NCollection_Sequence in GlBVHClipPrimitiveSet to improve performance",
        "==========",
        "",
    ]);
    let (mut run, s) = meshed_sphere_view()?;
    let d1 = connect_grid(run.session(), &s, 100)?;
    let d2 = timed_clear(run.session())?;
    println!("vconnectto done in {d1} microseconds!\n");
    println!("vclear done in {d2} microseconds!\n");
    for (what, micros) in [("vconnectto", d1), ("vclear", d2)] {
        if micros * 1e-6 > 1. {
            println!("Elapsed time of {what} is more than 1 seconds - Error");
        } else {
            println!("Elapsed time of {what} is less than 1 seconds - OK");
        }
    }
    run.finish_without_image()
}
