//! Compare the exact 17-digit RGB results, as RasterInterpretor::Append(double) does.
use super::*;
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use martensite_tkernel::{quantity::Color, storage::format_general};

pub(super) fn rgb_result(color: Color) -> String {
    [color.red(), color.green(), color.blue()]
        .map(|value| format_general(value, 17))
        .join(" ")
}

pub(super) fn distinct_observations(colors: &[String; 3]) -> bool {
    colors[0] != colors[1] && colors[0] != colors[2] && colors[1] != colors[2]
}

fn observed_colors(run: &mut SessionRun) -> Result<[String; 3], Failure> {
    let initial = rgb_result(run.session().pixel_color(102, 199)?);
    run.session().move_cursor(102, 199)?;
    let detected = rgb_result(run.session().pixel_color(102, 199)?);
    run.session().select_point(102, 199)?;
    let selected = rgb_result(run.session().pixel_color(102, 199)?);
    Ok([initial, detected, selected])
}

/// Preserve shaded-mode-before-display and all three independent pixel observations.
/// Example: `detection_pixel_regression()?`.
pub(crate) fn detection_pixel_regression() -> Outcome {
    banner(&["================", "OCC22039", "================", ""]);
    let result = read_dataset_shape("OCC12121-CrankArm.brep")?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().fit()?;
    if distinct_observations(&observed_colors(&mut run)?) {
        println!("OK OCC22039");
    } else {
        println!("Faulty OCC22039");
    }
    check_mass(&result, &MassCheck::near(MassKind::Area, 24859.6))?;
    run.dump_and_finish("bug22039.png", false)
}
