//! Label positions and arrow orientations of lengths, diameters, radii and angles with 3d
//! text (`bug24351_1` to `_4`): four layouts in a first viewer, then a grid of every label
//! position with external arrows in a second viewer, which is dumped.
use super::prelude::*;
use martensite_tkv3d::prs3d::{
    Prs3dDimensionArrowOrientation as Arrow, Prs3dDimensionTextHorizontalPosition as HPos,
    Prs3dDimensionTextVerticalPosition as VPos,
};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
#[path = "text_layout_inputs.rs"]
mod inputs;
use inputs::{Layout, THREE, arc, first_circle_layouts, grid};

/// Display the layout's points (and circle), then its dimension, then display it `again`.
fn show(run: &mut SessionRun, kind: DimensionKind, spec: &Layout, again: bool) -> Outcome {
    layout_points(run, spec)?;
    show_layout_dimension(run, kind, spec)?;
    if again {
        run.session().display_again(&spec.name)?;
    }
    Ok(())
}

/// Show the first viewer's layouts, then the grid in `Viewer2`, fit each and dump.
fn layouts(
    kind: DimensionKind,
    title: &str,
    first: &[Layout],
    second: &[Layout],
    image: &'static str,
) -> Outcome {
    dimension_banner(&[title]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    for (index, specs) in [first, second].into_iter().enumerate() {
        show_viewer_layouts(&mut run, kind, specs, index)?;
    }
    run.dump(image)?;
    run.finish()
}

/// `bug24351_1`: lengths between points.
/// Example: `length_layouts()?`.
pub(crate) fn length_layouts() -> Outcome {
    let first = inputs::first_length_layouts();
    let cell = |r: usize, c: usize| {
        let (x, z) = (50. * (2 * c) as f64, 50. * r as f64);
        vec![[x, 0., z], [x + 50., 0., z]]
    };
    let title = "CR24351: Test AIS Length dimensions: label position and arrow orientation";
    layouts(
        DimensionKind::Length,
        title,
        &first,
        &grid("d_", &THREE, cell, 10.),
        "bug24351_1.png",
    )
}

/// `bug24351_2` (diameters) and `bug24351_3` (radii) of circles.
/// Example: `circle_layouts(DimensionKind::Radius)?`.
pub(crate) fn circle_layouts(kind: DimensionKind) -> Outcome {
    let diameter = kind == DimensionKind::Diameter;
    let first = first_circle_layouts(diameter);
    let (columns, step, size, what, image) = if diameter {
        (&THREE[..], (80., 60.), 22., "Diameter", "bug24351_2.png")
    } else {
        (&THREE[..2], (90., 80.), 33., "Radius", "bug24351_3.png")
    };
    let cell = |r: usize, c: usize| arc(step.0 * c as f64, step.1 * r as f64, size).to_vec();
    let title =
        format!("CR24351: Test AIS {what} dimensions: label position and arrow orientation");
    let prefix = if diameter { "d_" } else { "r_" };
    layouts(
        kind,
        &title,
        &first,
        &grid(prefix, columns, cell, 0.),
        image,
    )
}

/// `bug24351_4`: angles at the middle of three points.
/// Example: `angle_layouts()?`.
pub(crate) fn angle_layouts() -> Outcome {
    let first = inputs::first_angle_layouts();
    let cell =
        |r: usize, c: usize| inputs::angle_corner(60. * c as f64, 60. * r as f64, 40.).to_vec();
    let title = "CR24351: Test AIS angle dimensions: label position and arrow orientation";
    layouts(
        DimensionKind::Angle,
        title,
        &first,
        &grid("r_", &THREE, cell, 40.),
        "bug24351_4.png",
    )
}

fn show_viewer_layouts(
    run: &mut SessionRun,
    kind: DimensionKind,
    specs: &[Layout],
    index: usize,
) -> Outcome {
    if index == 1 {
        run.session().open_viewer()?;
    }
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)?;
    show_layout_group(run, kind, specs, index == 1)?;
    run.session().fit()?;
    Ok(())
}

fn layout_points(run: &mut SessionRun, spec: &Layout) -> Outcome {
    if spec.point_names.len() != spec.points.len() {
        return Err(format!(
            "layout {:?} has {} names for {} points; expected equal counts",
            spec.name,
            spec.point_names.len(),
            spec.points.len()
        )
        .into());
    }
    for (name, point) in spec.point_names.iter().zip(&spec.points) {
        display_point(run, name, *point)?;
    }
    Ok(())
}

fn layout_operands(
    run: &mut SessionRun,
    kind: DimensionKind,
    spec: &Layout,
) -> Result<Vec<String>, Box<dyn Error>> {
    if !matches!(kind, DimensionKind::Radius | DimensionKind::Diameter) {
        return Ok(spec.point_names.clone());
    }
    let circle = &spec.circle_name;
    let [a, b, c] = spec.points[..] else {
        return Err("three circle points".into());
    };
    display_circle(run, circle, [a, b, c])?;
    Ok(vec![circle.clone()])
}

fn layout_request<'a>(
    kind: DimensionKind,
    spec: &Layout,
    operands: &[Measured<'a>],
) -> DimensionRequest<'a> {
    DimensionRequest {
        font: Some("SansFont"),
        text_3d: Some(true),
        plane: Some(WorkingPlane::Zox),
        flyout: Some(spec.flyout),
        style: DimensionStyle {
            label: Some(spec.label.0),
            label_vertical: spec.label.1,
            arrow: Some(spec.arrow),
            ..Default::default()
        },
        ..DimensionRequest::new(kind, operands)
    }
}

fn show_layout_dimension(run: &mut SessionRun, kind: DimensionKind, spec: &Layout) -> Outcome {
    let shapes = layout_operands(run, kind, spec)?;
    let names: Vec<&str> = shapes.iter().map(String::as_str).collect();
    let operands = objects(&names);
    display_dimension(
        run.session(),
        &spec.name,
        &layout_request(kind, spec, &operands),
    )
}

fn show_layout_group(
    run: &mut SessionRun,
    kind: DimensionKind,
    specs: &[Layout],
    second: bool,
) -> Outcome {
    if !second && kind == DimensionKind::Length {
        return show_initial_lengths(run, specs);
    }
    for spec in specs {
        show(run, kind, spec, second)?;
    }
    if !second && kind == DimensionKind::Angle {
        display_first_angles(run, specs)?;
    }
    Ok(())
}

fn show_initial_lengths(run: &mut SessionRun, specs: &[Layout]) -> Outcome {
    for spec in specs {
        layout_points(run, spec)?;
    }
    for spec in specs {
        show_layout_dimension(run, DimensionKind::Length, spec)?;
    }
    Ok(())
}

fn display_first_angles(run: &mut SessionRun, specs: &[Layout]) -> Outcome {
    let session = run.session();
    session.update_context(|_| {
        for spec in specs {
            session.display_again(&spec.name)?;
        }
        Ok(())
    })?;
    Ok(())
}
