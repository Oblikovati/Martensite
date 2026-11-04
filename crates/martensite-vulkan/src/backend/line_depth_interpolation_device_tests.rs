//! Isolate the witnessed section/flyout depth comparison without scene construction.
use super::*;
#[path = "../projected_segments/capture_tests.rs"]
mod capture_directory;
use capture_directory::DashCaptureDirectory;

const CLIP_BITS: [u32; 16] = [
    1007664318, 3125951603, 3138944347, 0, 781612300, 3155147966, 991927915, 0, 993122884,
    992602298, 1006008980, 0, 1054837568, 3213795254, 1020718016, 1065353216,
];

const SECTION_BITS: [[u32; 3]; 2] = [
    [3240671222, 3266402228, 2818932736],
    [3240671222, 3268302772, 2820935680],
];
const FLYOUT_BITS: [[u32; 3]; 2] = [[3240671222, 3268302772, 0], [3240671222, 3263715252, 0]];

#[test]
#[ignore = "diagnostic capture: requires an explicit GPU lease and artifact directory"]
fn capture_witnessed_coincident_line_depth_comparison() {
    let directory = DashCaptureDirectory::requested().expect("expected artifact directory");
    capture_line_pair(
        &directory,
        "positive-viewport",
        RenderCoordinateConvention::LegacyBottomUpV1,
    );
    capture_line_pair(
        &directory,
        "negative-viewport",
        RenderCoordinateConvention::TopLeftLogicalV2,
    );
}

fn capture_line_pair(
    directory: &DashCaptureDirectory,
    name: &str,
    coordinates: RenderCoordinateConvention,
) {
    let mut renderer = convention_renderer(RenderRasterPolicy::LastProvokingVertexV2, coordinates);
    let frame = witnessed_line_pair();
    let plan = super::super::frame_raster_plan::frame_raster_plan(&renderer.runtime, &frame, false);
    println!("{name}: effective samples={:?}", plan.samples);
    let capture = renderer.render(&frame).unwrap();
    directory.save(name, &capture.rgba, &capture.depth);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

fn witnessed_line_pair() -> RenderFrame {
    // Preserve actual input bits; the red section and black flyout use distinct endpoints.
    let red = line_pair_draw(SECTION_BITS, [1., 0., 0., 1.]);
    let black = line_pair_draw(FLYOUT_BITS, [0., 0., 0., 1.]);
    let mut frame = RenderFrame::new(768, 768, [1.; 4], vec![red, black]).unwrap();
    frame.raster.requested_samples = 8;
    frame
}

fn line_pair_draw(points: [[u32; 3]; 2], color: [f32; 4]) -> PrimitiveDraw {
    let vertices = points
        .map(|point| RenderVertex::new(point.map(f32::from_bits), color))
        .to_vec();
    PrimitiveDraw::segments(
        vertices,
        vec![0, 1],
        CLIP_BITS.map(f32::from_bits),
        crate::LineStyle::new(1., 65535, 1).unwrap(),
    )
    .unwrap()
}
