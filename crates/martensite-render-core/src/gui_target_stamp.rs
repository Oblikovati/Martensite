//! Packet acceptance, composition replacement and native target lifetime are distinct.
#![allow(missing_docs)] // Backend-plumbing accessors promoted for render-core split (#633).
use crate::view_gui_layer::GuiLayerSlot;
use crate::{
    RenderGuiFreshPacket, RenderGuiSurfaceCause, RenderGuiSurfaceChange, RenderGuiSurfacePhase,
};
use std::sync::Arc;
#[derive(Clone)]
pub struct GuiTargetStamp {
    pub extent: Option<[u32; 2]>,
    target: Arc<()>,
    composition: Arc<()>,
}
impl GuiLayerSlot {
    pub fn retain_observed_change(&mut self, stamp: &GuiTargetStamp, changed: bool) {
        if !changed || self.stale || !Arc::ptr_eq(&stamp.composition, &self.composition_epoch) {
            return;
        }
        self.stale = self.packet.is_some();
        self.target_epoch = Arc::new(());
    }
    pub fn capture_stamp(&self) -> GuiTargetStamp {
        let latest = self.last_draw.clone();
        latest.unwrap_or_else(|| self.target_stamp())
    }
    pub fn target_stamp(&self) -> GuiTargetStamp {
        GuiTargetStamp {
            extent: self.packet.as_ref().map(|packet| packet.extent),
            target: self.packet_target.clone(),
            composition: self.composition_epoch.clone(),
        }
    }
    pub fn fresh_packet_state(&self, extent: Option<[u32; 2]>) -> RenderGuiFreshPacket {
        let Some(packet) = &self.packet else {
            return RenderGuiFreshPacket::Absent;
        };
        if self.stale || extent.is_some_and(|actual| actual != packet.extent) {
            return RenderGuiFreshPacket::Required;
        }
        RenderGuiFreshPacket::AlreadyCurrent
    }
}
impl GuiTargetStamp {
    pub fn observe(
        &self,
        slot: &GuiLayerSlot,
        extent: Option<[u32; 2]>,
        phase: RenderGuiSurfacePhase,
    ) -> Option<RenderGuiSurfaceChange> {
        let target_revision_changed = !Arc::ptr_eq(&self.target, &slot.target_epoch);
        let composition_revision_changed = !Arc::ptr_eq(&self.composition, &slot.composition_epoch);
        let mut change = revision_change(
            self.extent?,
            extent,
            phase,
            target_revision_changed,
            composition_revision_changed,
        )?;
        change.fresh_packet = slot.fresh_packet_state(extent);
        Some(change)
    }
}
fn observed_cause(
    accepted: [u32; 2],
    observed: Option<[u32; 2]>,
    target_changed: bool,
    composition_changed: bool,
) -> Option<RenderGuiSurfaceCause> {
    if observed.is_some_and(|extent| extent != accepted) {
        return Some(RenderGuiSurfaceCause::ExtentMismatch);
    }
    if target_changed {
        return Some(RenderGuiSurfaceCause::TargetRevision);
    }
    composition_changed.then_some(RenderGuiSurfaceCause::CompositionRevision)
}

fn revision_change(
    accepted: [u32; 2],
    observed: Option<[u32; 2]>,
    phase: RenderGuiSurfacePhase,
    target_changed: bool,
    composition_changed: bool,
) -> Option<RenderGuiSurfaceChange> {
    let cause = observed_cause(accepted, observed, target_changed, composition_changed)?;
    Some(RenderGuiSurfaceChange {
        phase,
        cause,
        accepted_extent: accepted,
        observed_extent: observed,
        target_revision_changed: target_changed,
        composition_revision_changed: composition_changed,
        fresh_packet: RenderGuiFreshPacket::Absent,
    })
}
