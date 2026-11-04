//! Injected renderers can inspect effective widths while retaining requested presentation styles.
use martensite_vulkan::{
    LineStyle, PrimitiveDraw, RenderFrame, RenderLineWidthPolicy, RenderVertex, identity_clip,
};

#[test]
fn public_effective_width_preserves_requested_aspect_and_replay() {
    let vertices = vec![RenderVertex::new([0.; 3], [1.; 4]); 2];
    let draw = PrimitiveDraw::segments(
        vertices,
        vec![0, 1],
        identity_clip(),
        LineStyle::new(3., 0xfff0, 2).unwrap(),
    )
    .unwrap();
    let frame = RenderFrame::new(8, 8, [0.; 4], vec![draw])
        .unwrap()
        .with_line_width_policy(RenderLineWidthPolicy::SinglePixelV1);
    let style = frame.draws()[0].line_style().unwrap();
    assert_eq!(style.parameters(), (3., 0xfff0, 2));
    assert_eq!(style.raster_width(), 1.);
    let replay = frame.with_line_width_policy(RenderLineWidthPolicy::AspectWidthsV1);
    assert_eq!(replay.draws()[0].line_style().unwrap().raster_width(), 3.);
}
