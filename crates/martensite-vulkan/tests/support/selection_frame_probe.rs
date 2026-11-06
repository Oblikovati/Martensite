//! CPU-only frame coverage compares real extraction hooks against the injected backend submission.
#[path = "recording_device.rs"]
mod recording_device;
use crate::{case_view::viewer_session::ViewerSession, tie_edges::SubmittedDraws};
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Handle,
};
use martensite_tkservice::{graphic3d::*, image::ImagePixMap};
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, PrimitiveDraw, RenderFrame};
use recording_device::RecordingDevice;
use std::sync::Arc;

pub struct SelectionFrameProbe {
    _driver: Handle<Graphic3dRenderDriver>,
    device: RecordingDevice,
    recorder: Arc<SubmittedDraws>,
    pub session: ViewerSession,
}
impl SelectionFrameProbe {
    /// Build an injected CPU recorder; `let probe = SelectionFrameProbe::new()`.
    pub fn new() -> Self {
        let device = RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(device.clone(), 4096).unwrap();
        let session = ViewerSession::with_view_size(&driver, 409, 409).unwrap();
        let view = session
            .view()
            .get()
            .unwrap()
            .view()
            .cast::<Graphic3dRenderView>();
        let recorder = SubmittedDraws::install(&view);
        Self {
            _driver: driver,
            device,
            recorder,
            session,
        }
    }
    /// Extract the actual frame and observer draws; `let (frame, seen) = probe.snapshot()`.
    pub fn snapshot(&self) -> (RenderFrame, Vec<PrimitiveDraw>) {
        let image = ImagePixMap::default();
        assert!(
            self.session
                .view()
                .to_pix_map_sized(&image, 409, 409, Graphic3dBufferType::Rgba, true,)
                .unwrap()
        );
        (self.device.last_frame(), self.recorder.draws())
    }
}

/// Match retained vertex color in linear space; `named_linear_rgba(NamedColor::YELLOW)`.
pub fn named_linear_rgba(color: NamedColor) -> [f32; 4] {
    let value = Color::named(color);
    [
        value.red() as f32,
        value.green() as f32,
        value.blue() as f32,
        1.,
    ]
}

/// Select uniformly colored real draws; `uniform_color_draws(frame.draws(), yellow)`.
pub fn uniform_color_draws(draws: &[PrimitiveDraw], color: [f32; 4]) -> Vec<&PrimitiveDraw> {
    draws
        .iter()
        .filter(|draw| {
            !draw.vertices().is_empty()
                && draw.vertices().iter().all(|vertex| vertex.color == color)
        })
        .collect()
}

/// Optionally retain CPU geometry diagnostics; `write_geometry("scene", &frame)`.
pub fn write_geometry(name: &str, frame: &RenderFrame) {
    let Some(directory) = std::env::var_os("MARTENSITE_SELECTION_FRAME_PROBE_DIR") else {
        return;
    };
    let output = std::path::PathBuf::from(directory).join(format!("{name}.json"));
    let draws: Vec<String> = frame.draws().iter().map(draw_geometry).collect();
    let json = format!(
        "{{\"kind\":\"CPU injected-backend geometry, not GPU pixels\",\"draws\":[{}]}}",
        draws.join(",")
    );
    std::fs::write(output, json).unwrap();
}

fn draw_geometry(draw: &PrimitiveDraw) -> String {
    let vertices: Vec<[[u32; 4]; 2]> = draw
        .vertices()
        .iter()
        .map(|vertex| {
            [
                [
                    vertex.position[0].to_bits(),
                    vertex.position[1].to_bits(),
                    vertex.position[2].to_bits(),
                    vertex.weight.to_bits(),
                ],
                vertex.color.map(f32::to_bits),
            ]
        })
        .collect();
    let line = draw.line_style().map_or("null".to_owned(), |style| {
        let (width, pattern, factor) = style.parameters();
        format!(
            "{{\"width\":{width},\"raster_width\":{},\"pattern\":{pattern},\"factor\":{factor}}}",
            style.raster_width()
        )
    });
    format!(
        "{{\"topology\":\"{:?}\",\"clip_bits\":{:?},\"vertices_bits\":{vertices:?},\"indices\":{:?},\"line\":{line}}}",
        draw.topology(),
        draw.clip_transform().map(f32::to_bits),
        draw.indices()
    )
}

/// Exercise order and multiplicity refusals on actual distinct draws; `assert_ordered_join_rejects_corruption(frame.draws())`.
pub fn assert_ordered_join_rejects_corruption(actual: &[PrimitiveDraw]) {
    assert!(
        actual.len() >= 2,
        "ordered join controls expected at least two real draws, got {}",
        actual.len()
    );
    assert_ne!(
        actual[0], actual[1],
        "reorder control expected distinct first two draws"
    );
    let mut reversed = actual.to_vec();
    reversed.swap(0, 1);
    assert_ne!(actual, reversed);
    let mut duplicated = actual.to_vec();
    duplicated.insert(0, actual[0].clone());
    assert_ne!(actual, duplicated);
    let mut missing = actual.to_vec();
    missing.remove(0);
    assert_ne!(actual, missing);
    let mut replaced = actual.to_vec();
    replaced[1] = actual[0].clone();
    assert_ne!(actual, replaced);
}
