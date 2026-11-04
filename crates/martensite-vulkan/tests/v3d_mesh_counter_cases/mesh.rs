//! Preserve the authored mesh across display only when auto-triangulation is disabled.
use super::*;
use crate::primitive_shapes::sphere;
use crate::shape_mesh::{
    CountExpectation, MeshRequest, TriangulationExpectation, TriangulationSummary,
    check_triangulation, mesh_shape, remove_triangulation,
};
use martensite_tkbrep::topods::Shape;

const MESH_BANNER: [&str; 7] = [
    "============",
    "CR23200",
    "Check that the shape is automatic re-triangulated with deflection-check logic",
    "in case of enabled auto triangulation feature (its own triangulation can be lost).",
    "And it keeps its already computed triangulation",
    "in case of disabled auto triangulation feature.",
    "============",
];

/// Apply the authored shaded defaults; `prepare_auto_mesh(&session)?`.
pub(super) fn prepare_auto_mesh(session: &mut ViewerSession) -> Result<(), Failure> {
    session.clear()?;
    session.set_axonometric()?;
    session.set_display_mode(1)?;
    session.set_auto_triangulation(true);
    Ok(())
}

/// Clean and incrementally mesh the actual sphere; `rebuild_source_mesh(&shape)?`.
pub(super) fn rebuild_source_mesh(shape: &Shape) -> Result<i32, Failure> {
    remove_triangulation(shape)?;
    mesh_shape(shape, &MeshRequest::new(0.1).with_angle_degrees(45.))?;
    Ok(TriangulationSummary::of(shape)?.triangles)
}

/// Display the owned shape and measure its current triangles; `show_sphere(&mut session, &shape)?`.
pub(super) fn show_sphere(session: &mut ViewerSession, shape: &Shape) -> Result<i32, Failure> {
    session.display("s", shape, &Default::default())?;
    Ok(TriangulationSummary::of(shape)?.triangles)
}

fn count_expectation(observed: i32, automatic: bool) -> Result<CountExpectation, Failure> {
    if automatic {
        return Ok(CountExpectation::Not(f64::from(observed)));
    }
    CountExpectation::from_checktrinfo_reference(&observed.to_string(), 0., 0.)
}

fn fit_dump(run: &mut SessionRun, image: &'static str) -> Outcome {
    run.session().fit()?;
    run.dump(image)?;
    let observation = super::unlit::observe(run.session(), image);
    run.retain_observation(observation);
    Ok(())
}

const STAGE_IMAGES: [&str; 2] = [
    "bug23200_shape_not_auto_triangulation.png",
    "bug23200_shape_auto_triangulation.png",
];

fn check_mesh_stage(run: &mut SessionRun, shape: &Shape, automatic: bool) -> Outcome {
    run.session().clear()?;
    if !automatic {
        run.session().set_auto_triangulation(false);
    }
    let observed = rebuild_source_mesh(shape)?;
    show_sphere(run.session(), shape)?;
    let expected = TriangulationExpectation {
        triangles: count_expectation(observed, automatic)?,
        ..Default::default()
    };
    check_triangulation(shape, &expected)?;
    fit_dump(run, STAGE_IMAGES[usize::from(automatic)])
}

/// Run both real mesh-count transitions and all three dumps; `automatic_mesh()?`.
pub(crate) fn automatic_mesh() -> Outcome {
    banner(&MESH_BANNER);
    println!();
    let mut run = SessionRun::new()?;
    prepare_auto_mesh(run.session())?;
    let shape = sphere(0.5)?;
    show_sphere(run.session(), &shape)?;
    fit_dump(&mut run, "bug23200_shape.png")?;
    check_mesh_stage(&mut run, &shape, true)?;
    check_mesh_stage(&mut run, &shape, false)?;
    run.finish()
}

#[cfg(test)]
mod source_count_tests {
    use super::*;
    #[test]
    fn disabled_auto_mesh_retains_boolean_reference_dispatch_only_without_bang() {
        assert_eq!(count_expectation(0, false).unwrap(), CountExpectation::Any);
        assert_eq!(
            count_expectation(1, false).unwrap(),
            CountExpectation::Present
        );
        assert_eq!(
            count_expectation(1, true).unwrap(),
            CountExpectation::Not(1.)
        );
        assert_eq!(
            count_expectation(2, false).unwrap(),
            CountExpectation::exactly(2.)
        );
    }
}
