use super::*;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::Graphic3dTypeOfShadingModel;

#[derive(Clone, Copy)]
pub(super) struct OcclusionStage {
    pub projection: [f64; 3],
    pub cursor: [i32; 2],
    pub pixel: [i32; 2],
    pub marker: &'static str,
    pub image: &'static str,
}

pub(super) const STAGES: [OcclusionStage; 3] = [
    OcclusionStage {
        projection: [0., 0., 1.],
        cursor: [325, 100],
        pixel: [325, 400],
        marker: "p1",
        image: "bug31412_proj_Z.png",
    },
    OcclusionStage {
        projection: [0., 1., 0.],
        cursor: [200, 58],
        pixel: [400, 58],
        marker: "p2",
        image: "bug31412_proj_Y.png",
    },
    OcclusionStage {
        projection: [1., 0., 0.],
        cursor: [200, 58],
        pixel: [50, 58],
        marker: "p3",
        image: "bug31412_proj_X.png",
    },
];

pub(super) fn boxes() -> Result<[Shape; 2], Failure> {
    let back = box_at([250., 250., 900.], [1000., 1000., 200.])?;
    let front = box_sized([1000., 1000., 1000.])?;
    Ok([back, front])
}

pub(super) fn scene(session: &mut ViewerSession, boxes: &[Shape; 2]) -> Result<(), Failure> {
    let shaded = DisplayRequest::default().with_display_mode(1);
    session.display_many(&[("b1", &boxes[0]), ("b2", &boxes[1])], &shaded)?;
    session.set_line_width("b1", 3.)?;
    session.set_color("b1", Color::named(NamedColor::ORANGE))?;
    session.set_line_width("b2", 3.)?;
    session.set_color("b2", Color::named(NamedColor::MATRABLUE))?;
    session.set_shading_model(Graphic3dTypeOfShadingModel::Unlit)
}

pub(super) fn camera(session: &ViewerSession, stage: OcclusionStage) -> Result<(), Failure> {
    session.set_view_parameters(&ViewParameters {
        proj: Some(stage.projection),
        ..Default::default()
    })?;
    session.fit()
}

pub(super) fn probe(session: &mut ViewerSession, stage: OcclusionStage) -> Result<(), Failure> {
    camera(session, stage)?;
    session.move_cursor(stage.cursor[0], stage.cursor[1])?;
    let color = session.pixel_color_name(stage.pixel[0], stage.pixel[1])?;
    check_color(&color, &mut pixel_contract::PrintedHoverLines);
    Ok(())
}

pub(super) fn check_color(color: &str, output: &mut impl pixel_contract::HoverOutputLines) {
    if color == "CYAN" {
        output.line("ERROR: back box is misdetected");
    }
}
