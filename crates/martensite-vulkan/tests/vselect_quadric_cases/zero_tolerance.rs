//! A3 keeps every zero-pixel-tolerance/sensitivity transition and all eight images.
use super::{datum_objects, source_checks::check_miss};
use crate::{
    case_view::viewer_session::DisplayRequest,
    primitive_shapes::box_sized,
    session_run::{SessionRun, banner, check_point},
};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::aspect::AspectTypeOfMarker;
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorMembers as _;
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;

/// The full A3 body with original point values and tolerances; `axis_a3()`.
pub(crate) fn axis_a3() -> Outcome {
    banner(&[
        "========",
        "0032388: Visualization - provide ray-picking interface with zero tolerance for rectangular frustum",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    box_scene(&mut run)?;
    box_probes(&mut run)?;
    point_probes(&mut run)?;
    run.finish()
}

fn box_scene(run: &mut SessionRun) -> Outcome {
    println!("Display box");
    run.session().display(
        "b",
        &box_sized([100., 200., 300.])?,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    run.session().fit()?;
    run.session()
        .context()
        .main_selector()
        .set_pixel_tolerance(0);
    Ok(())
}

fn sensitivity(run: &mut SessionRun, name: &str, value: i32) -> Result<(), Failure> {
    let session = run.session();
    let object = session
        .object(name)
        .expect("sensitivity: expected source binding")
        .cast();
    session
        .context()
        .selection_manager()
        .set_selection_sensitivity(&object, 0, value)
}

fn checkpoint(run: &mut SessionRun, name: &str, pixel: [i32; 2], expected: [f64; 3]) -> Outcome {
    run.session().move_cursor(pixel[0], pixel[1])?;
    let point = run.session().detected_point()?;
    run.dump(image_name(name))?;
    check_point(name, point, expected, 0.001);
    Ok(())
}

fn miss(run: &mut SessionRun, name: &str, label: &str, pixel: [i32; 2]) -> Outcome {
    run.session().move_cursor(pixel[0], pixel[1])?;
    let point = run.session().detected_point()?;
    run.dump(image_name(name))?;
    check_miss(point, label);
    Ok(())
}

fn box_probes(run: &mut SessionRun) -> Outcome {
    sensitivity(run, "b", 4)?;
    checkpoint(run, "box_sens4", [321, 319], [100., 200., 0.])?;
    sensitivity(run, "b", 0)?;
    miss(run, "box_sens0_out", "box", [321, 319])?;
    checkpoint(
        run,
        "box_sens0_in",
        [319, 317],
        [100.00000000000003, 196.92152736525867, 0.4127082000112523],
    )
}

fn point_probes(run: &mut SessionRun) -> Outcome {
    println!("\nDisplay 2d point");
    datum_objects::point_2d(run.session(), "p", [320., 318.])?;
    run.session()
        .set_marker("p", AspectTypeOfMarker::Point, 1.)?;
    let expected = [104.7974449617403, -102.98278630143884, -346.41016151377545];
    checkpoint(run, "point_sens12_out", [321, 319], expected)?;
    checkpoint(run, "point_sens12_in", [319, 317], expected)?;
    sensitivity(run, "p", 0)?;
    miss(run, "point_sens0_out", "point", [321, 319])?;
    miss(run, "point_sens0_eq", "point", [320, 318])?;
    checkpoint(
        run,
        "point_sens0_in",
        [319, 317],
        [100.00000000000003, 196.92152736525867, 0.41270820001116704],
    )
}

fn image_name(name: &str) -> &'static str {
    match name {
        "box_sens4" => "A3_box_sens4.png",
        "box_sens0_out" => "A3_box_sens0_out.png",
        "box_sens0_in" => "A3_box_sens0_in.png",
        "point_sens12_out" => "A3_point_sens12_out.png",
        "point_sens12_in" => "A3_point_sens12_in.png",
        "point_sens0_out" => "A3_point_sens0_out.png",
        "point_sens0_eq" => "A3_point_sens0_eq.png",
        "point_sens0_in" => "A3_point_sens0_in.png",
        value => panic!("image label {value:?}; expected an original A3 checkpoint"),
    }
}
