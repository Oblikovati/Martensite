//! Owned API input evidence for bug28890; no ray, hit, or post-vertex authority.
use crate::{FrameCapture, RenderError, error::rejected};
use std::sync::{Arc, Mutex};
#[path = "raytrace_input_encoding.rs"]
mod encoding;
#[path = "raytrace_input_publication.rs"]
mod publication;
pub(crate) use publication::ViewRayInputs;

pub(crate) const SELECTOR: &str = "MARTENSITE_RAYTRACE_INPUT_OBSERVATION";
pub(crate) fn requested() -> Result<bool, RenderError> {
    selected(std::env::var_os(SELECTOR).as_deref())
}
fn selected(value: Option<&std::ffi::OsStr>) -> Result<bool, RenderError> {
    match value {
        None => Ok(false),
        Some(value) if value == "1" => Ok(true),
        actual => Err(rejected(
            actual,
            "unset or literal 1 raytrace input selector",
        )),
    }
}

pub(crate) fn supported_scope(
    ray: Option<(bool, bool)>,
    scaled: bool,
    stereo: bool,
    subviews: bool,
) -> bool {
    ray == Some((false, false)) && !scaled && !stereo && !subviews
}

#[derive(Clone, Copy)]
pub(crate) enum RayInputPhase {
    Frame,
    Dispatch,
}

#[derive(Clone, Debug)]
pub(crate) struct RayPipelineInputs {
    pub pipeline: u64,
    pub layout: u64,
    pub modules: Vec<u64>,
    pub scene_layout: u64,
    pub specialization_entries: Vec<[u64; 3]>,
    pub program: String,
    pub specialization: Vec<u8>,
    pub vertex: Vec<u8>,
    pub fragment: Vec<u8>,
}
#[derive(Clone, Debug)]
pub(crate) struct RayUploadInputs {
    pub buffer: u64,
    pub set: u64,
    pub layout: u64,
    pub bytes: Vec<u8>,
}
#[derive(Clone, Debug)]
pub(crate) struct RayDispatchInputs {
    pub command: u64,
    pub sets: Vec<u64>,
    pub pipeline: RayPipelineInputs,
}
#[derive(Clone, Debug)]
pub(crate) struct RayCommandInputs {
    pub command: u64,
    pub logical: [u32; 2],
    pub attachment: [u32; 2],
}
#[derive(Debug, Default)]
pub(crate) struct RayInputRows {
    upload: Option<RayUploadInputs>,
    dispatch: Option<RayDispatchInputs>,
    command: Option<RayCommandInputs>,
    refusal: Option<&'static str>,
}
#[derive(Debug)]
pub(crate) struct RayInputObservation {
    pub owner: usize,
    pub view_id: i64,
    pub generation: u64,
    rows: Mutex<RayInputRows>,
}
impl RayInputObservation {
    pub fn new(owner: usize, view_id: i64, generation: u64) -> Arc<Self> {
        Arc::new(Self {
            owner,
            view_id,
            generation,
            rows: Mutex::default(),
        })
    }
    pub fn refuse(&self, reason: &'static str) {
        if let Ok(mut rows) = self.rows.lock() {
            rows.refusal.get_or_insert(reason);
        }
    }
    pub fn uploaded(&self, input: RayUploadInputs) {
        let Ok(mut rows) = self.rows.lock() else {
            return;
        };
        if rows.upload.is_some() || input.bytes.len() != 416 || input.buffer == 0 || input.set == 0
        {
            rows.refusal
                .get_or_insert("duplicate or malformed actual uniform upload");
            return;
        }
        rows.upload = Some(input);
    }
    pub fn dispatched(&self, input: RayDispatchInputs) {
        let Ok(mut rows) = self.rows.lock() else {
            return;
        };
        if rows.dispatch.is_some() {
            rows.refusal
                .get_or_insert("multiple actual raytrace dispatches");
            return;
        }
        rows.dispatch = Some(input);
    }
    pub fn command(&self, input: RayCommandInputs) {
        let Ok(mut rows) = self.rows.lock() else {
            return;
        };
        if rows.command.is_some() {
            rows.refusal
                .get_or_insert("multiple actual frame command buffers");
            return;
        }
        rows.command = Some(input);
    }
    pub fn marker(&self, phase: RayInputPhase, command: u64) -> String {
        let kind = match phase {
            RayInputPhase::Frame => "frame",
            RayInputPhase::Dispatch => "render-dispatch",
        };
        format!(
            "{{\"kind\":\"raytrace-input-{kind}\",\"view_address\":{},\"view_id\":{},\"frame_generation\":{},\"command\":{command},\"post_vertex_observed\":false}}",
            self.owner, self.view_id, self.generation
        )
    }
    pub fn encode(&self, publication: u64) -> Result<String, RenderError> {
        let rows = self
            .rows
            .lock()
            .map_err(|e| rejected(e, "unpoisoned ray input evidence"))?;
        encoding::encode(self, &rows, publication)
    }
}

pub(crate) fn same_capture(left: &FrameCapture, right: &FrameCapture) -> bool {
    left.width == right.width
        && left.height == right.height
        && left.rgba == right.rgba
        && left.depth.len() == right.depth.len()
        && left
            .depth
            .iter()
            .zip(&right.depth)
            .all(|(a, b)| a.to_bits() == b.to_bits())
}

#[cfg(test)]
#[path = "raytrace_input_observation_tests.rs"]
mod tests;
