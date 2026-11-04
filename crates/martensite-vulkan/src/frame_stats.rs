//! Frame statistics of a Vulkan view: the capability of GlFrameStats. The
//! shared Graphic3d frame logic (timers, rates, history, formatting) runs unchanged; this type
//! supplies `updateStatistics` from the retained Vulkan layers, structures and elements.
use crate::Graphic3dRenderView;
use crate::frame_stats_structures::StructureCounting;
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkservice::graphic3d::{
    Graphic3dCView, Graphic3dCViewMembers as _, Graphic3dFrameStats, Graphic3dFrameStatsBase,
    Graphic3dFrameStatsCounter as Counter, Graphic3dFrameStatsFields,
    Graphic3dFrameStatsMembers as _, Graphic3dFrameStatsTimer as Timer, Graphic3dLayer,
    Graphic3dRenderingParamsPerfCounters as Perf,
};

/// Device-local image bytes behind a view's memory counters; zero for injected backends.
/// Example: `render_view.image_memory().frame_attachment_bytes`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderImageMemory {
    /// Allocation bytes of the render-target attachments of the last prepared frame.
    pub frame_attachment_bytes: usize,
    /// Allocation bytes of the sampled texture images alive on the device.
    pub sampled_image_bytes: usize,
}

/// Rendering statistics of a Vulkan view; `view.frame_stats().counter_value(counter)`.
#[derive(Default)]
pub struct RenderFrameStats {
    transient: StandardTransient,
    base: Graphic3dFrameStatsBase,
}
martensite_tkernel::impl_transient!(
    RenderFrameStats,
    "Martensite_RenderFrameStats",
    StandardTransient,
    transient,
    interfaces: [dyn Graphic3dFrameStats]
);

impl RenderFrameStats {
    /// Empty statistics with one data frame; `RenderFrameStats::new()`.
    pub fn new() -> Handle<Self> {
        Handle::new(Self::default())
    }
    /// `IsFrameUpdated`: copy the last data frame into `previous` (created when null) and
    /// report whether the major counters changed; `stats.is_frame_updated(&mut previous)`.
    pub fn is_frame_updated(&self, previous: &mut Handle<RenderFrameStats>) -> bool {
        if previous.is_null() {
            *previous = Self::new();
        } else if previous
            .get()
            .is_some_and(|known| same_major_counters(&self.base.fields.read(), known))
        {
            return false;
        }
        let fields = self.base.fields.read();
        let mut target = previous.get().expect("created above").base.fields.write();
        target.last_frame_index = fields.last_frame_index;
        target.counters_tmp.assign_data(fields.last_data_frame());
        true
    }
    /// Run the view culling under the `CpuCulling` timer, as GlLayerList::UpdateCulling.
    /// Example: `stats.time_culling(&mut || layers_culled)`.
    pub(crate) fn time_culling<T>(&self, culling: &mut dyn FnMut() -> T) -> T {
        self.active_data_frame(&mut |tmp| tmp.change_timer(Timer::CpuCulling).start());
        let outcome = culling();
        self.active_data_frame(&mut |tmp| {
            let timer = tmp.change_timer(Timer::CpuCulling);
            timer.stop();
            let seconds = timer.user_time_cpu();
            tmp[Timer::CpuCulling] = seconds;
        });
        outcome
    }
    /// The native body without an GlView: store the active frame and reset it.
    fn store_without_view(&self) {
        let mut fields = self.base.fields.write();
        let index = usize::try_from(fields.last_frame_index).unwrap_or(0);
        let frame = (*fields.counters_tmp).clone();
        if let Some(slot) = fields.counters.get_mut(index) {
            *slot = frame;
        }
        fields.counters_tmp.reset();
    }
}

/// The couple of major counters native `IsFrameUpdated` compares (rates within 0.001).
fn same_major_counters(current: &Graphic3dFrameStatsFields, known: &RenderFrameStats) -> bool {
    let known = known.base.fields.read();
    let frame = current.last_data_frame();
    let previous = &known.counters_tmp;
    let rates = [
        (frame.frame_rate(), previous.frame_rate()),
        (frame.frame_rate_cpu(), previous.frame_rate_cpu()),
        (
            frame.immediate_frame_rate(),
            previous.immediate_frame_rate(),
        ),
        (
            frame.immediate_frame_rate_cpu(),
            previous.immediate_frame_rate_cpu(),
        ),
    ];
    let counters = [
        Counter::NbLayers,
        Counter::NbLayersNotCulled,
        Counter::NbStructs,
        Counter::NbStructsNotCulled,
    ];
    current.last_frame_index == known.last_frame_index
        && rates.iter().all(|(a, b)| (a - b).abs() <= 0.001)
        && counters.iter().all(|c| frame[*c] == previous[*c])
}

/// The requested collections, derived from the counter bits as native updateStatistics.
#[derive(Clone, Copy, Debug)]
struct CountingFlags {
    memory: bool,
    detailed: bool,
    elements: bool,
    groups: bool,
    structures: bool,
}

impl CountingFlags {
    fn new(bits: Perf) -> Self {
        let has = |flag: Perf| bits.0 & flag.0 != 0;
        let memory = has(Perf::ESTIM_MEM);
        let detailed = has(Perf::TRIANGLES) || has(Perf::LINES) || has(Perf::POINTS);
        let elements = has(Perf::GROUP_ARRAYS) || detailed || memory;
        let groups = has(Perf::GROUPS) || elements;
        Self {
            memory,
            detailed,
            elements,
            groups,
            structures: has(Perf::STRUCTURES) || has(Perf::LAYERS) || groups,
        }
    }
}

impl Graphic3dFrameStats for RenderFrameStats {
    fn frame_stats_base(&self) -> &Graphic3dFrameStatsBase {
        &self.base
    }
    fn update_statistics(&self, view: &Handle<dyn Graphic3dCView>, is_immediate_only: bool) {
        let render = view.cast::<Graphic3dRenderView>();
        let (Some(render), Some(view)) = (render.get(), view.get()) else {
            self.store_without_view();
            return;
        };
        let flags = CountingFlags::new(view.rendering_params().stats.collected_stats);
        let layers = view.layers();
        self.active_data_frame(&mut |tmp| tmp[Counter::NbLayers] = layers.len());
        if flags.structures {
            let counting = StructureCounting {
                view_id: view.identification(),
                elements: flags.elements,
                detailed: flags.detailed,
                memory: flags.memory,
            };
            for layer in &layers {
                self.count_layer(layer, &counting, flags.groups, is_immediate_only);
            }
        }
        if flags.memory {
            self.count_view_memory(render);
        }
    }
}

impl RenderFrameStats {
    fn count_layer(
        &self,
        layer: &Graphic3dLayer,
        counting: &StructureCounting,
        groups: bool,
        is_immediate_only: bool,
    ) {
        self.change_counter(Counter::NbStructs, count(layer.nb_structures()));
        if is_immediate_only && !layer.layer_settings().is_immediate() {
            return;
        }
        if !layer.is_culled() {
            self.change_counter(Counter::NbLayersNotCulled, 1);
        }
        let not_culled = count(layer.nb_structures_not_culled());
        self.change_counter(Counter::NbStructsNotCulled, not_culled);
        if !groups {
            return;
        }
        let sets = [
            layer.cullable_structures(),
            layer.cullable_trsf_pers_structures(),
            layer.non_cullable_structures(),
        ];
        for structures in &sets {
            self.active_data_frame(&mut |tmp| counting.count(structures, tmp));
        }
    }
    /// Vulkan images of the view: attachments of its last frame and resident sampled images.
    fn count_view_memory(&self, view: &Graphic3dRenderView) {
        let memory = view.image_memory();
        self.change_counter(Counter::EstimatedBytesFbos, memory.frame_attachment_bytes);
        self.change_counter(Counter::EstimatedBytesTextures, memory.sampled_image_bytes);
    }
}

fn count(value: i32) -> usize {
    usize::try_from(value).unwrap_or(0)
}

#[cfg(test)]
#[path = "frame_stats_tests.rs"]
mod tests;
