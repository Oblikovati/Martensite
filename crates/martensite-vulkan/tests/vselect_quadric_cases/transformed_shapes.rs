//! Original forced-location scaled cone/cylinder, shell and solid picking and all four dumps.
use super::{
    datum_objects,
    ray_command::pick_axis,
    transformed_input::{shell_compound, transformed_quadric},
};
use crate::{
    case_view::viewer_session::DisplayRequest,
    session_run::{SessionRun, banner, check_point},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkservice::aspect::AspectTypeOfMarker;
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage;
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;
struct TransformedCase {
    name: &'static str,
    is_cone: bool,
    expected: [f64; 3],
}

/// Full original trsf_cone; `transformed_cone()`.
pub(crate) fn transformed_cone() -> Outcome {
    let case = TransformedCase {
        name: "trsf_cone",
        is_cone: true,
        expected: [2500.42, 3499.54, 1000.81],
    };
    transformed_source(&case)
}

/// Full original trsf_cyl; `transformed_cylinder()`.
pub(crate) fn transformed_cylinder() -> Outcome {
    let case = TransformedCase {
        name: "trsf_cyl",
        is_cone: false,
        expected: [2500.9, 3499.0, 1001.6],
    };
    transformed_source(&case)
}

fn transformed_source(case: &TransformedCase) -> Outcome {
    banner(&[
        "=================================",
        "0032652: Visualization - Select3DSensitiveCylinder returns wrong 3D point on transformed shape",
    ]);
    let kind = if case.is_cone { "cone" } else { "cylinder" };
    println!("Check picking of transformed {kind}\n=================================");
    let source = transformed_quadric(case.is_cone)?;
    let shells = shell_compound(&source)?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    transformed_probe(&mut run, &shells, case, "tris")?;
    run.session().clear()?;
    transformed_probe(&mut run, &source, case, "cyl")?;
    run.finish()
}

fn transformed_probe(
    run: &mut SessionRun,
    shape: &Shape,
    case: &TransformedCase,
    suffix: &str,
) -> Outcome {
    let picked = detect_transformed(run, shape, suffix)?;
    let (label, entity) = probe_labels(case.is_cone, suffix);
    check_point(label, picked, case.expected, 0.1);
    check_entity(run, entity)?;
    run.session().fit()?;
    let images = image_names(case.name, suffix);
    run.dump(images.0)?;
    run.selection_dump(images.1, StdSelectTypeOfSelectionImage::SURFACE_NORMAL)
}

fn detect_transformed(
    run: &mut SessionRun,
    shape: &Shape,
    suffix: &str,
) -> Result<[f64; 3], Box<dyn Error>> {
    let name = if suffix == "tris" { "cc" } else { "c" };
    run.session()
        .display(name, shape, &DisplayRequest::default().with_display_mode(1))?;
    run.session().fit()?;
    pick_axis(run, [2500., 3498., 1001.], [0., 1., 0.], "a", true)?;
    run.session().move_cursor(200, 200)?;
    let picked = run.session().detected_point()?;
    datum_objects::point(run.session(), "pp", picked, AspectTypeOfMarker::Plus, None)?;
    Ok(picked)
}

fn probe_labels(is_cone: bool, suffix: &str) -> (&'static str, &'static str) {
    if suffix == "tris" {
        return ("aPntTris_p", "Select3DSensitiveCircle");
    }
    let label = if is_cone { "aPntCone_p" } else { "aPntCyl_p" };
    (label, "Select3DSensitiveCylinder")
}

fn check_entity(run: &mut SessionRun, entity: &str) -> Outcome {
    if !run
        .session()
        .picked_entity_state_types()?
        .iter()
        .any(|name| name.contains(entity))
    {
        let label = if entity.ends_with("Circle") {
            "circle"
        } else {
            "cylinder"
        };
        println!("Error: {label} should be detected");
    }
    Ok(())
}

fn image_names(case: &str, suffix: &str) -> (&'static str, &'static str) {
    match (case, suffix) {
        ("trsf_cone", "tris") => ("trsf_cone_prs_tris.png", "trsf_cone_selnorm_tris.png"),
        ("trsf_cone", "cyl") => ("trsf_cone_prs_cyl.png", "trsf_cone_selnorm_cyl.png"),
        ("trsf_cyl", "tris") => ("trsf_cyl_prs_tris.png", "trsf_cyl_selnorm_tris.png"),
        ("trsf_cyl", "cyl") => ("trsf_cyl_prs_cyl.png", "trsf_cyl_selnorm_cyl.png"),
        value => panic!("image source {value:?}; expected an original transformed shape dump"),
    }
}
