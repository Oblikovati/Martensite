//! Retained placement frames (#594, instancing.md step 4). A frame of a view whose scene is only
//! placements of a few plain lit draw templates (the parts of an assembly) can stand for all
//! of them with one draw: its templates, and per placement in draw order the template and the
//! world and normal matrices. The backend writes one draw-table entry per placement, with the
//! clip matrix of the frame's camera, and records the run in segments of equal geometry
//! buffers, so the draw order and the captures stay those of the separate draws.
use crate::PrimitiveDraw;
use martensite_tkernel::ncollection::Mat4;
use std::sync::Arc;

/// One placement: its template and the matrices its separate draw would carry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InstancePlacement {
    pub template: u32,
    pub world: [f32; 16],
    pub normal: [f32; 16],
}

/// Object-space bounds used only by the conservative GPU clipping test.
#[derive(Debug, Clone, Copy)]
pub struct InstanceBounds {
    pub minimum: [f32; 3],
    pub maximum: [f32; 3],
}

impl InstanceBounds {
    pub fn of(draw: &PrimitiveDraw) -> Option<Self> {
        let mut minimum = [f32::INFINITY; 3];
        let mut maximum = [f32::NEG_INFINITY; 3];
        for vertex in draw.vertices.iter() {
            if vertex.weight != 1.
                || !vertex
                    .position
                    .iter()
                    .all(|coordinate| coordinate.is_finite())
            {
                return None;
            }
            for axis in 0..3 {
                minimum[axis] = minimum[axis].min(vertex.position[axis]);
                maximum[axis] = maximum[axis].max(vertex.position[axis]);
            }
        }
        (!draw.vertices.is_empty()).then_some(Self { minimum, maximum })
    }
}

#[derive(Debug)]
pub struct InstanceVisibility {
    pub visible: Vec<bool>,
    pub bounds: Arc<[Option<InstanceBounds>]>,
}

/// The templates and placements of one frame; `placements` is shared by the frames of an
/// orbit, the templates carry this frame's camera fields.
#[derive(Debug)]
pub struct DrawInstances {
    pub templates: Vec<PrimitiveDraw>,
    pub placements: Arc<[InstancePlacement]>,
    /// `projection * view` of this frame in binary32, as `RecordCameraParts` composes it.
    pub projection_view: [f32; 16],
    pub visibility: Option<InstanceVisibility>,
    pub hiz_history: Option<Arc<dyn crate::device_backend::RenderHiZHistory>>,
}

impl DrawInstances {
    /// The clip matrix of a placement, the product a replayed group record patches;
    /// `instances.clip(&placement)`.
    pub fn clip(&self, placement: &InstancePlacement) -> [f32; 16] {
        (Mat4::from_data(self.projection_view) * Mat4::from_data(placement.world)).data()
    }
}

/// A shared `DrawInstances` that compares by identity, so that a draw that carries it stays
/// `PartialEq` without comparing 30,000 placements.
#[derive(Clone, Debug)]
pub struct DrawInstancesRef(pub Arc<DrawInstances>);

impl PartialEq for DrawInstancesRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl std::ops::Deref for DrawInstancesRef {
    type Target = DrawInstances;
    fn deref(&self) -> &DrawInstances {
        &self.0
    }
}

#[cfg(test)]
#[path = "draw_instances_tests.rs"]
mod tests;
