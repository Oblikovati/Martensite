use super::*;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

pub(super) const STEPS: [(usize, &str); 7] = [
    (0, "ERROR: incorrect highlighting of box b0"),
    (
        1,
        "ERROR: incorrect highlighting of box b1 after vselnext call",
    ),
    (
        2,
        "ERROR: incorrect highlighting of box b2 after vselnext call",
    ),
    (
        0,
        "ERROR: incorrect highlighting of box b0 after vselnext call",
    ),
    (
        2,
        "ERROR: incorrect highlighting of box b2 after vselprev call",
    ),
    (
        1,
        "ERROR: incorrect highlighting of box b1 after vselprev call",
    ),
    (
        0,
        "ERROR: incorrect highlighting of box b0 after vselprev call",
    ),
];

pub(super) fn boxes() -> Result<[Shape; 3], Failure> {
    Ok([
        box_sized([3., 3., 3.])?,
        box_at([1., 1., 1.], [3., 2., 1.])?,
        box_at([1.5, 1.5, 1.5], [4., 4., 4.])?,
    ])
}

pub(super) fn scene(session: &mut ViewerSession, boxes: &[Shape; 3]) -> Result<(), Failure> {
    session.clear()?;
    session.display_many(
        &[("b0", &boxes[0]), ("b1", &boxes[1]), ("b2", &boxes[2])],
        &DisplayRequest::default(),
    )?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_LEFT)?;
    session.fit()
}

pub(super) fn advance(session: &ViewerSession, stage: usize) -> Result<(), Failure> {
    if stage <= 3 {
        return session.highlight_next_detected();
    }
    session.highlight_previous_detected()
}

pub(super) fn check(session: &ViewerSession, stage: usize) -> Result<(), Failure> {
    let mut replies = pixel_contract::RenderedHoverPixels(session);
    let mut output = pixel_contract::PrintedHoverLines;
    pixel_contract::check_stage(&mut replies, &mut output, stage)
}
