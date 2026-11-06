//! Indices are the authored explode aliases, never inferred geometry identities.
use super::dimension_params::{DimensionParams, DimensionStyle, set_dimension_params};
use super::dimension_request::{
    DimensionKind, DimensionRequest, Measured, WorkingPlane, display_dimension,
};
use super::*;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::prs3d::Prs3dDimensionTextHorizontalPosition;

pub(super) const LENGTHS: [(&str, usize, usize, f64, bool); 10] = [
    ("ld_1", 1, 27, -15., false),
    ("ld_2", 1, 14, 15., false),
    ("ld_3", 1, 28, -27., true),
    ("ld_4", 0, 7, -20., false),
    ("ld_5", 0, 28, -15., false),
    ("ld_6", 0, 18, 30., false),
    ("ld_7", 1, 20, -20., false),
    ("ld_8", 0, 42, 55., true),
    ("ld_9", 0, 12, 35., true),
    ("ld_10", 0, 40, 15., false),
];
pub(super) const RADII: [(&str, usize, bool); 2] = [("rd_1", 45, true), ("rd_2", 63, false)];
pub(super) const ANGLES: [(&str, usize, usize, usize); 3] = [
    ("ad_1", 0, 38, 49),
    ("ad_2", 1, 24, 25),
    ("ad_3", 0, 48, 42),
];

pub(super) fn create_all(session: &mut ViewerSession, sections: &[Shape; 2]) -> Outcome {
    let edges = sections
        .each_ref()
        .map(|shape| crate::boolean_typed_explode::all_distinct(shape, ShapeEnum::Edge));
    for spec in LENGTHS {
        create_length(session, &edges, spec)?;
    }
    create_radii(session, &edges)?;
    create_angles(session, &edges)
}
fn create_radii(session: &mut ViewerSession, edges: &[Vec<Shape>; 2]) -> Outcome {
    for (name, index, right) in RADII {
        let shape = edge(edges, 1, index)?;
        let mut request = DimensionRequest::new(DimensionKind::Radius, &[Measured::Shape(shape)]);
        request.style = black_style(right);
        display_dimension(session, name, &request)?;
    }
    Ok(())
}
fn create_angles(session: &mut ViewerSession, edges: &[Vec<Shape>; 2]) -> Outcome {
    for (name, side, first, second) in ANGLES {
        let mut request = DimensionRequest::new(
            DimensionKind::Angle,
            &[
                Measured::Shape(edge(edges, side, first)?),
                Measured::Shape(edge(edges, side, second)?),
            ],
        );
        request.style = black_style(false);
        display_dimension(session, name, &request)?;
    }
    Ok(())
}
fn create_length(
    session: &mut ViewerSession,
    edges: &[Vec<Shape>; 2],
    (name, side, index, flyout, right): (&str, usize, usize, f64, bool),
) -> Outcome {
    let mut request = DimensionRequest::new(
        DimensionKind::Length,
        &[Measured::Shape(edge(edges, side, index)?)],
    );
    request.plane = Some(WorkingPlane::Xoy);
    request.flyout = Some(flyout);
    request.style = black_style(right);
    display_dimension(session, name, &request)
}
fn black_style(right: bool) -> DimensionStyle {
    DimensionStyle {
        color: Some(Color::named(NamedColor::BLACK)),
        label: right.then_some(Prs3dDimensionTextHorizontalPosition::Right),
        ..Default::default()
    }
}
fn edge(edges: &[Vec<Shape>; 2], side: usize, index: usize) -> Result<&Shape, Failure> {
    edges[side].get(index - 1).ok_or_else(|| {
        Failure::with_message(
            "Standard_OutOfRange",
            &format!(
                "section {side} edge{index}, count={}; expected authored one-based edge alias",
                edges[side].len()
            ),
        )
    })
}
pub(super) fn arrow_angle() -> f64 {
    3.14 * 12.0 / 180.0
}
pub(super) fn update_all(session: &ViewerSession) -> Outcome {
    let params = DimensionParams {
        style: DimensionStyle {
            text_3d: Some(true),
            text_shaded: Some(true),
            text_height: Some(6.),
            arrow_length: Some(4.),
            ..Default::default()
        },
        arrow_angle: Some(arrow_angle()),
        ..Default::default()
    };
    for name in ["rd_1", "rd_2", "ad_1", "ad_2", "ad_3"] {
        set_dimension_params(session, name, &params)?;
    }
    for (name, ..) in LENGTHS {
        set_dimension_params(session, name, &params)?;
    }
    Ok(())
}
