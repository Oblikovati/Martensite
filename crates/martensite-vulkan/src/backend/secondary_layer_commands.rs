//! Independent ordinary draw ranges record off the primary submission thread (#594).
use super::{frame::FrameResources, runtime::VulkanRuntime, scene_color_scope::SceneColorScope};
use crate::{RenderFrame, RenderLayerSpan};
use ash::vk;
use rayon::prelude::*;
use std::sync::{Arc, Mutex, PoisonError};

#[path = "secondary_command_pool.rs"]
mod pool;
use pool::SecondaryCommand;
pub(super) use pool::SecondaryCommandPools;

const MIN_BUCKET_DRAWS: usize = 64;
const MAX_BUCKETS: usize = 4;

pub(super) struct FrameSecondaryLayers {
    runtime: Arc<VulkanRuntime>,
    // The primary submission's frame owner releases these after its fence.
    commands: Mutex<Vec<SecondaryCommand>>,
}

impl FrameSecondaryLayers {
    pub(super) fn new(runtime: &Arc<VulkanRuntime>) -> Self {
        Self {
            runtime: runtime.clone(),
            commands: Mutex::default(),
        }
    }

    pub(super) fn prepare(
        &self,
        resources: &FrameResources,
        frame: &RenderFrame,
        start: usize,
    ) -> Vec<LayerCommands> {
        if self.runtime.secondary_pools.disabled() {
            return Vec::new();
        }
        let jobs = recording_jobs(resources, frame, start);
        if jobs.len() < 2 || rayon::current_num_threads() < 2 {
            return Vec::new();
        }
        let recording = jobs
            .into_par_iter()
            .map(|job| {
                self.runtime
                    .lifecycle
                    .observe(self.record_job(resources, job))
            })
            .collect::<Result<Vec<_>, _>>();
        let Ok(recorded) = recording else {
            return Vec::new();
        };
        self.keep(recorded)
    }

    fn record_job(
        &self,
        resources: &FrameResources,
        job: RecordingJob,
    ) -> Result<(usize, SecondaryCommand), crate::RenderError> {
        let runtime = &self.runtime;
        let command = SecondaryCommand::begin(runtime, resources)?;
        super::render_target_record::set_viewport(runtime, command.handle(), resources.extent);
        super::record::set_depth_policy(runtime, command.handle(), &job.span);
        super::record::record_draws_selected(
            runtime,
            command.handle(),
            resources,
            &job.span,
            None,
            SceneColorScope::initialized_srgb(),
        );
        if job.finish_layer {
            super::ordered_stencil_record::finish_layer(
                runtime,
                command.handle(),
                resources,
                job.layer,
            );
        }
        Ok((job.layer, command.finish()?))
    }

    fn keep(&self, recorded: Vec<(usize, SecondaryCommand)>) -> Vec<LayerCommands> {
        let mut layers = Vec::<LayerCommands>::new();
        let mut retained = self.commands.lock().unwrap_or_else(PoisonError::into_inner);
        for (layer, command) in recorded {
            if let Some(last) = layers.last_mut().filter(|last| last.index == layer) {
                last.commands.push(command.handle());
            } else {
                layers.push(LayerCommands {
                    index: layer,
                    commands: vec![command.handle()],
                });
            }
            retained.push(command);
        }
        layers
    }
}

pub(super) struct LayerCommands {
    pub(super) index: usize,
    pub(super) commands: Vec<vk::CommandBuffer>,
}

struct RecordingJob {
    layer: usize,
    span: RenderLayerSpan,
    finish_layer: bool,
}

fn recording_jobs(
    resources: &FrameResources,
    frame: &RenderFrame,
    start: usize,
) -> Vec<RecordingJob> {
    if !ordinary_frame(resources, frame) {
        return Vec::new();
    }
    let mut jobs: Vec<_> = frame
        .layers
        .iter()
        .enumerate()
        .skip(start)
        .filter(|(index, span)| ordinary_layer(resources, *index, span))
        .take(MAX_BUCKETS)
        .map(|(layer, span)| RecordingJob {
            layer,
            span: span.clone(),
            finish_layer: true,
        })
        .collect();
    if jobs.len() != 1 {
        return jobs;
    }
    let job = jobs.remove(0);
    split_job(job, resources)
}

fn ordinary_frame(resources: &FrameResources, frame: &RenderFrame) -> bool {
    !cfg!(any(
        feature = "command-state-diagnostics",
        feature = "lit-only-command-diagnostics",
        feature = "vertex-output-diagnostics",
        feature = "raytrace-input-diagnostics",
        feature = "raytrace-hit-diagnostics",
    )) && resources.oit.is_none()
        && resources.immediate_source.is_none()
        && resources.main_snapshot.is_none()
        && resources.background.is_none()
        && resources.grid.is_none()
        && resources.raytrace.is_none()
        && frame.gui_draw_start.is_none()
}

fn ordinary_layer(resources: &FrameResources, index: usize, span: &RenderLayerSpan) -> bool {
    span.draw_range.len() >= MIN_BUCKET_DRAWS
        && span.draw_range.start >= resources.first_draw
        && span.draw_range.end - resources.first_draw <= resources.draws.len()
        && !resources.stencil.clear_after_layer[index]
        && !resources.stencil.after_layer[index].enabled
        && span.draw_range.clone().all(|draw| {
            !resources.stencil.clear_before[draw]
                && !resources.stencil.during[draw].enabled
                && ordinary_draw(&resources.draws[draw - resources.first_draw])
        })
}

fn ordinary_draw(draw: &super::frame::UploadedDraw) -> bool {
    let presentation = &draw.presentation;
    draw.user_program.is_none()
        && draw.native_alpha.is_none()
        && !draw.window_depth_expected
        && presentation.polygon
        && !presentation.surface_blend
        && presentation.capping == crate::RenderCappingPass::None
        && SceneColorScope::for_pipeline(
            presentation.text_logic_xor || presentation.encoded_surface,
            draw.table_run
                .as_ref()
                .map_or(draw.pipeline.kind, |run| run.pipeline.kind),
            true,
        ) == SceneColorScope::initialized_srgb()
}

fn split_job(job: RecordingJob, resources: &FrameResources) -> Vec<RecordingJob> {
    let range = &job.span.draw_range;
    let count = (range.len() / MIN_BUCKET_DRAWS).min(MAX_BUCKETS);
    if count < 2 {
        return Vec::new();
    }
    let independent = range.clone().all(|index| {
        let draw = &resources.draws[index - resources.first_draw];
        draw.table_run.is_none()
            && draw.presentation.polygon_offset.is_none()
            && !draw.presentation.polygon_edge
            && !draw.presentation.text_shadow_bias
    });
    if !independent {
        return Vec::new();
    }
    bucket_spans(job, count)
}

fn bucket_spans(job: RecordingJob, count: usize) -> Vec<RecordingJob> {
    let first = job.span.draw_range.start;
    let length = job.span.draw_range.len();
    (0..count)
        .map(|bucket| {
            let mut span = job.span.clone();
            span.draw_range =
                first + length * bucket / count..first + length * (bucket + 1) / count;
            RecordingJob {
                layer: job.layer,
                span,
                finish_layer: bucket + 1 == count,
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "secondary_layer_commands_tests.rs"]
mod tests;
