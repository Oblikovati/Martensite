//! Custom dimension values and texts (`bug27688`) and arrows of a fixed screen size under
//! zoom and relocation (`bug27919`).
use super::prelude::*;
use crate::case_view::viewer_session::LocationChange;
use martensite_tkv3d::prs3d::{
    Prs3dDimensionArrowOrientation as Arrow, Prs3dDimensionTextHorizontalPosition as HPos,
};
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// A length dimension in the XOY plane between two new points.
fn xoy_length(run: &mut SessionRun, name: &str, points: [(&str, [f64; 3]); 2]) -> Outcome {
    for (point, at) in points {
        display_point(run, point, at)?;
    }
    let operands = objects(&[points[0].0, points[1].0]);
    let request = DimensionRequest {
        plane: Some(WorkingPlane::Xoy),
        ..DimensionRequest::new(DimensionKind::Length, &operands)
    };
    display_dimension(run.session(), name, &request)
}

/// Flyout -15 and arrow length 4, with the units shown or hidden.
fn short_arrows(show_units: bool) -> DimensionParams {
    let style = DimensionStyle {
        arrow_length: Some(4.),
        show_units: Some(show_units),
        ..DimensionStyle::default()
    };
    DimensionParams {
        flyout: Some(-15.),
        style,
        ..DimensionParams::default()
    }
}

/// `bug27688`: a real custom value, a custom text, and a custom text reset to the measure.
/// Example: `custom_text_values()?`.
pub(crate) fn custom_text_values() -> Outcome {
    custom_value_banner();
    let mut run = SessionRun::new()?;
    numeric_custom_length(&mut run)?;
    textual_custom_length(&mut run)?;
    run.session().fit()?;
    resettable_custom_length(&mut run)?;
    set_dimension_params(
        run.session(),
        "dim3",
        &DimensionParams {
            auto_value: true,
            ..DimensionParams::default()
        },
    )?;
    run.session().fit()?;
    run.dump("bug27688.png")?;
    run.finish()
}

/// Print the original's error unless pixel (x, y) is black.
fn require_black(run: &mut SessionRun, (x, y): (i32, i32), what: &str) -> Outcome {
    if run.session().pixel_color_name(x, y)? != "BLACK" {
        println!("ERROR: the arrow of the {what} should not be zoomable");
    }
    Ok(())
}

/// Each compound source location command has its own explicit X translation.
fn turn_angle(run: &mut SessionRun, offset_x: f64) -> Outcome {
    let session = run.session();
    session.update_context(|_| {
        session.change_location("angle", LocationChange::Reset)?;
        session.set_location_translation("angle", [offset_x, 0., 0.])?;
        let rotate = LocationChange::Rotate {
            origin: [0.; 3],
            axis: [1., 0., 0.],
            degrees: 40.,
        };
        session.change_location("angle", rotate)
    })?;
    Ok(())
}

/// Dump `image`, then the selection-owner image `owners`.
fn dump_with_owners(run: &mut SessionRun, image: &'static str, owners: &str) -> Outcome {
    run.dump(image)?;
    run.selection_dump(owners, StdSelectTypeOfSelectionImage::COLORED_OWNER)
}

/// `bug27919`: a length and an angle with 30-unit arrows that do not zoom.
/// Example: `fixed_size_arrows()?`.
pub(crate) fn fixed_size_arrows() -> Outcome {
    banner(&[
        "=============",
        "0027919: Visualization - support multiple transformation persistence groups within single presentation",
        "=============",
    ]);
    let mut run = SessionRun::new()?;
    run.session().display_trihedron("t1")?;
    fixed_arrow_length(&mut run)?;
    fixed_arrow_angle(&mut run)?;
    inspect_fixed_arrows(&mut run)?;
    turn_angle(&mut run, 10.)?;
    dump_with_owners(&mut run, "bug27919_rot1.png", "bug27919_rot1_selowner.png")?;
    run.session().zoom(0.25)?;
    turn_angle(&mut run, 30.)?;
    dump_with_owners(&mut run, "bug27919_rot2.png", "bug27919_rot2_selowner.png")?;
    run.finish()
}

fn numeric_custom_length(run: &mut SessionRun) -> Outcome {
    xoy_length(run, "dim1", [("p1", [0., 0., 0.]), ("p2", [50., 50., 0.])])?;
    set_dimension_params(
        run.session(),
        "dim1",
        &DimensionParams {
            value: Some(10.),
            ..short_arrows(true)
        },
    )?;
    Ok(())
}

fn textual_custom_length(run: &mut SessionRun) -> Outcome {
    xoy_length(
        run,
        "dim2",
        [("p3", [100., 100., 0.]), ("p4", [150., 150., 0.])],
    )?;
    set_dimension_params(
        run.session(),
        "dim2",
        &DimensionParams {
            text_value: Some("w_1"),
            ..short_arrows(false)
        },
    )?;
    Ok(())
}

fn resettable_custom_length(run: &mut SessionRun) -> Outcome {
    xoy_length(
        run,
        "dim3",
        [("p5", [200., 200., 0.]), ("p6", [250., 250., 0.])],
    )?;
    set_dimension_params(
        run.session(),
        "dim3",
        &DimensionParams {
            text_value: Some("w_1"),
            ..short_arrows(true)
        },
    )?;
    Ok(())
}

fn fixed_arrow_length(run: &mut SessionRun) -> Outcome {
    xoy_length(
        run,
        "dim",
        [("p11", [-25., 50., 0.]), ("p12", [25., 50., 0.])],
    )?;
    set_fixed_length_parameters(run)?;
    Ok(())
}

fn fixed_arrow_angle(run: &mut SessionRun) -> Outcome {
    for (name, x) in [("p21", -100.), ("p22", 0.), ("p23", 100.)] {
        display_point(run, name, [x, 0., 0.])?;
    }
    let operands = objects(&["p21", "p22", "p23"]);
    let style = DimensionStyle {
        arrow_length: Some(30.),
        zoomable_arrow: Some(false),
        ..DimensionStyle::default()
    };
    display_dimension(
        run.session(),
        "angle",
        &DimensionRequest {
            style,
            ..DimensionRequest::new(DimensionKind::Angle, &operands)
        },
    )?;
    Ok(())
}

fn inspect_fixed_arrows(run: &mut SessionRun) -> Outcome {
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    run.session().fit()?;
    run.session().zoom(1.5)?;
    require_black(run, (182, 119), "dimension")?;
    require_black(run, (149, 195), "angle dimension")?;
    dump_with_owners(run, "bug27919_def.png", "bug27919_def_selowner.png")?;
    Ok(())
}

fn custom_value_banner() {
    banner(&[
        "========",
        "Visualization, AisDimension - add possibility to set custom text value",
        "Test case creates three length dimensions where value is set as real,",
        "as text and is reset to auto value state.",
        "========",
    ]);
}

fn set_fixed_length_parameters(run: &mut SessionRun) -> Outcome {
    let style = DimensionStyle {
        arrow_length: Some(30.),
        arrow: Some(Arrow::Internal),
        label: Some(HPos::Center),
        zoomable_arrow: Some(false),
        ..DimensionStyle::default()
    };
    set_dimension_params(
        run.session(),
        "dim",
        &DimensionParams {
            flyout: Some(1.),
            style,
            ..DimensionParams::default()
        },
    )?;
    Ok(())
}
