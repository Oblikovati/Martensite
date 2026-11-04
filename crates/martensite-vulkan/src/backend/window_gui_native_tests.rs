//! Explicit native gate: synchronous X resize at operation-local presentation seams.
use super::window_gui_contract::{NoPresentationProbe, WindowPresentationProbe};
use super::*;
use crate::{
    PrimitiveDraw, RenderGuiSurfaceOutcome, RenderGuiSurfacePhase, RenderGuiSurfacePolicy,
    RenderLayer, RenderVertex, identity_clip,
};
#[path = "../../examples/support/native_window.rs"]
mod native_window;
use native_window::NativeWindow;
struct NativeResizeProbe<'a> {
    window: &'a NativeWindow,
    selected: RenderGuiSurfacePhase,
    fault: bool,
    resize: bool,
    observed: Vec<RenderGuiSurfacePhase>,
}
impl<'a> NativeResizeProbe<'a> {
    fn new(
        window: &'a NativeWindow,
        selected: RenderGuiSurfacePhase,
        fault: bool,
        resize: bool,
    ) -> Self {
        Self {
            window,
            selected,
            fault,
            resize,
            observed: Vec::new(),
        }
    }
}
impl WindowPresentationProbe for NativeResizeProbe<'_> {
    fn observe(&mut self, phase: RenderGuiSurfacePhase) -> Result<(), RenderError> {
        self.observed.push(phase);
        if phase != self.selected {
            return Ok(());
        }
        if self.resize {
            self.window.resize(80, 72);
        }
        if self.fault {
            return Err(native_injected_fault());
        }
        Ok(())
    }
}
fn native_injected_fault() -> RenderError {
    // Oblikovati#3630: inject the same VK_ERROR_DEVICE_LOST shape real submissions return,
    // so a device fault observed alongside a native resize is proven to remain a fault.
    let operation = "owned native submission fault 3630";
    super::runtime::vk_error(operation, ash::vk::Result::ERROR_DEVICE_LOST)
}
fn native_gui_frame([width, height]: [u32; 2]) -> RenderFrame {
    let points = [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]];
    let vertices = points
        .map(|point| RenderVertex::new(point, [0., 1., 0., 1.]))
        .to_vec();
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], identity_clip()).unwrap();
    let layer = RenderLayer::new(vec![draw], false, false, false);
    let mut frame = RenderFrame::with_layers(width, height, [0., 0., 1., 1.], vec![layer]).unwrap();
    frame.gui_draw_start = Some(0);
    frame
}
fn assert_native_change(error: &RenderError, phase: RenderGuiSurfacePhase, with_fault: bool) {
    let change = match error.gui_surface_outcome().unwrap() {
        RenderGuiSurfaceOutcome::SurfaceChanged(change) => {
            assert!(!with_fault);
            change
        }
        RenderGuiSurfaceOutcome::FaultWithSurfaceChange { fault, change } => {
            assert!(with_fault);
            assert_eq!(fault, &native_injected_fault());
            change
        }
    };
    assert_eq!(change.phase, phase);
    assert_eq!(
        (change.accepted_extent, change.observed_extent),
        ([64, 64], Some([80, 72]))
    );
}
fn native_boundary_case(renderer: &mut VulkanRenderer, phase: RenderGuiSurfacePhase, fault: bool) {
    let native = NativeWindow::new(64, 64);
    let mut presenter = renderer.window_presenter(&native.window.cast()).unwrap();
    let mut probe = NativeResizeProbe::new(&native, phase, fault, true);
    let result = render_native_probe(renderer, &mut presenter, &mut probe, [64, 64]);
    assert_native_change(&result.unwrap_err(), phase, fault);
    assert_eq!(probe.observed.last(), Some(&phase));
    assert!(
        presenter.chain.is_none(),
        "failed operation must retire its swapchain"
    );
    if fault {
        assert_lost_then_fresh_renderer(renderer, &mut presenter, &native, [80, 72]);
    } else {
        assert_native_fresh_frame(renderer, &mut presenter, [80, 72]);
    }
    println!(
        "{{\"case\":\"native-gui-surface\",\"phase\":\"{phase:?}\",\"injected_fault\":{fault},\"status\":\"passed\"}}"
    );
}
fn assert_native_fresh_frame(
    renderer: &mut VulkanRenderer,
    presenter: &mut WindowPresenter,
    extent: [u32; 2],
) {
    let capture =
        render_native_probe(renderer, presenter, &mut NoPresentationProbe, extent).unwrap();
    let pixels = (extent[0] * extent[1]) as usize;
    assert_eq!(capture.rgba, [0, 255, 0, 255].repeat(pixels));
    assert_eq!(capture.depth, vec![1.; pixels]);
    assert!(
        renderer.validation_errors().unwrap().is_empty(),
        "{:?}",
        renderer.validation_errors().unwrap()
    );
}
/// An observed VK_ERROR_DEVICE_LOST is terminal for its logical device (Vulkan spec,
/// devsandqueues-lost-device; DeviceLifecycle keeps Lost after later successes). Only a new
/// renderer may present again into the same, still-live native window.
fn assert_lost_then_fresh_renderer(
    renderer: &mut VulkanRenderer,
    presenter: &mut WindowPresenter,
    native: &NativeWindow,
    extent: [u32; 2],
) {
    assert_eq!(renderer.device_state(), crate::RenderDeviceState::Lost);
    let refused = render_native_probe(renderer, presenter, &mut NoPresentationProbe, extent);
    let failure = refused.unwrap_err();
    let admission = failure.device_failure().unwrap();
    assert_eq!(
        (admission.class(), admission.state()),
        (
            crate::RenderDeviceFailureClass::Admission,
            Some(crate::RenderDeviceState::Lost)
        )
    );
    assert!(renderer.validation_errors().unwrap().is_empty());
    let mut fresh = native_validation_renderer();
    let mut fresh_presenter = fresh.window_presenter(&native.window.cast()).unwrap();
    assert_native_fresh_frame(&mut fresh, &mut fresh_presenter, extent);
}
fn render_native_probe(
    renderer: &mut VulkanRenderer,
    presenter: &mut WindowPresenter,
    probe: &mut dyn WindowPresentationProbe,
    extent: [u32; 2],
) -> Result<FrameCapture, RenderError> {
    let frame = native_gui_frame(extent);
    renderer.render_window_with_probe(
        &frame,
        presenter,
        false,
        RenderGuiSurfacePolicy::CheckedV2,
        probe,
    )
}
fn native_same_extent_fault(renderer: &mut VulkanRenderer, phase: RenderGuiSurfacePhase) {
    let native = NativeWindow::new(64, 64);
    let mut presenter = renderer.window_presenter(&native.window.cast()).unwrap();
    let mut probe = NativeResizeProbe::new(&native, phase, true, false);
    let result = render_native_probe(renderer, &mut presenter, &mut probe, [64, 64]);
    assert_eq!(result.unwrap_err(), native_injected_fault());
    assert!(
        presenter.chain.is_none(),
        "failed operation must retire signalled synchronization resources"
    );
    assert_lost_then_fresh_renderer(renderer, &mut presenter, &native, [64, 64]);
    println!(
        "{{\"case\":\"native-gui-same-extent-fault\",\"phase\":\"{phase:?}\",\"status\":\"passed\"}}"
    );
}

#[test]
#[ignore = "requires explicit exclusive native/GPU lease and pinned validation layer"]
fn native_gui_surface_phase_gate() {
    // Surface-change-only cases share one renderer; each injected loss is terminal for its own.
    let mut renderer = native_validation_renderer();
    for phase in [
        RenderGuiSurfacePhase::Acquire,
        RenderGuiSurfacePhase::BeforeSubmit,
        RenderGuiSurfacePhase::Present,
        RenderGuiSurfacePhase::Publication,
    ] {
        native_boundary_case(&mut renderer, phase, false);
        native_boundary_case(&mut native_validation_renderer(), phase, true);
        native_same_extent_fault(&mut native_validation_renderer(), phase);
    }
}

fn native_validation_renderer() -> VulkanRenderer {
    let renderer = VulkanRenderer::new(VulkanOptions {
        validation: true,
        device_index: None,
    })
    .unwrap();
    println!(
        "{{\"device\":{:?},\"identity\":{:?}}}",
        renderer.device_name(),
        format!("{:?}", renderer.driver_identity())
    );
    renderer
}

fn orientation_band(upper: bool) -> Vec<RenderVertex> {
    let (bottom, top, color) = if upper {
        (0., 1., [1., 0., 0., 1.])
    } else {
        (-1., 0., [0., 1., 0., 1.])
    };
    [
        [-1., bottom, 0.],
        [1., bottom, 0.],
        [1., top, 0.],
        [-1., top, 0.],
    ]
    .map(|point| RenderVertex::new(point, color))
    .to_vec()
}
fn orientation_frame(extent: [u32; 2]) -> RenderFrame {
    let vertices = orientation_band(true)
        .into_iter()
        .chain(orientation_band(false))
        .collect();
    let indices = vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7];
    let draw = PrimitiveDraw::new(vertices, indices, identity_clip()).unwrap();
    RenderFrame::new(extent[0], extent[1], [0., 0., 1., 1.], vec![draw]).unwrap()
}
fn expected_orientation_rows(extent: [u32; 2]) -> Vec<u8> {
    // Two primary-color half-planes have no sRGB rounding or edge-interpolation ambiguity.
    (0..extent[1])
        .flat_map(|row| {
            let color = if row < extent[1] / 2 {
                [255, 0, 0, 255]
            } else {
                [0, 255, 0, 255]
            };
            (0..extent[0]).flat_map(move |_| color)
        })
        .collect()
}
fn present_orientation_frame(
    renderer: &mut VulkanRenderer,
    presenter: &mut WindowPresenter,
    extent: [u32; 2],
) -> FrameCapture {
    renderer
        .render_window(
            &orientation_frame(extent),
            presenter,
            false,
            RenderGuiSurfacePolicy::CheckedV2,
        )
        .unwrap()
}
fn retain_window_orientation(capture: &FrameCapture, native: &[u8], stage: usize) {
    let Some(directory) = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR") else {
        return;
    };
    let directory = std::path::PathBuf::from(directory);
    std::fs::create_dir_all(&directory).unwrap();
    let prefix = directory.join(format!(
        "window-orientation-{stage}-{}x{}",
        capture.width, capture.height
    ));
    let depth: Vec<_> = capture
        .depth
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    std::fs::write(prefix.with_extension("rgba"), &capture.rgba).unwrap();
    std::fs::write(prefix.with_extension("window.rgba"), native).unwrap();
    std::fs::write(prefix.with_extension("depth"), depth).unwrap();
}
#[test]
#[ignore = "requires explicit exclusive native/GPU lease and pinned validation layer"]
fn native_window_and_readback_keep_asymmetric_top_left_rows_through_resize() {
    let mut renderer = native_validation_renderer();
    assert_coordinate_window(&mut renderer);
}
fn assert_coordinate_window(renderer: &mut VulkanRenderer) {
    let native = NativeWindow::new(96, 64);
    let mut presenter = renderer.window_presenter(&native.window.cast()).unwrap();
    for (stage, extent) in [[96, 64], [80, 72], [96, 64]].into_iter().enumerate() {
        native.resize(extent[0] as i32, extent[1] as i32);
        let capture = present_orientation_frame(renderer, &mut presenter, extent);
        let expected = expected_orientation_rows(extent);
        assert_eq!(capture.rgba, expected);
        assert!(capture.depth.iter().all(|depth| *depth == 0.5));
        let pixels = native.wait_for_pixels(&expected);
        retain_window_orientation(&capture, &pixels, stage);
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
#[ignore = "requires explicit exclusive native/GPU lease and pinned validation layer"]
fn four_coordinate_cells_present_and_capture_identical_native_rows_after_resize() {
    use crate::{RenderCoordinateConvention as Coordinates, RenderRasterPolicy as Raster};
    for raster in [Raster::LegacyFirstV1, Raster::LastProvokingVertexV2] {
        for coordinates in [Coordinates::LegacyBottomUpV1, Coordinates::TopLeftLogicalV2] {
            let options = VulkanOptions {
                validation: true,
                device_index: Some(0),
            };
            let mut renderer =
                VulkanRenderer::new_with_conventions(options, raster, coordinates).unwrap();
            assert_coordinate_window(&mut renderer);
        }
    }
}

#[path = "resource_lifecycle_window_gpu_tests.rs"]
mod resource_owner;
