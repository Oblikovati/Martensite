//! `BOPAlgoEdgeEdge`, the edge/edge task class of `BOPAlgoPaveFiller_3`.
#[path = "task_progress.rs"]
mod progress;
use crate::bop_algo::tools::{TRSF_TO_POINT_CRITERIA, trsf_to_point};
use crate::bop_ds::PaveBlockHandle;
use crate::int_tools::{CommonPrt, EdgeEdge};
use martensite_tkbrep::topods::Edge;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::toploc::Location;

/// `BOPAlgoEdgeEdge`: an IntToolsEdgeEdge between two pave blocks, run near the origin
/// when the boxes lie far away.
pub(in crate::bop_algo::pave_filler) struct EdgeEdgeTask {
    pub(in crate::bop_algo::pave_filler) block1: PaveBlockHandle,
    pub(in crate::bop_algo::pave_filler) block2: PaveBlockHandle,
    edges: [Edge; 2],
    ranges: [(f64, f64); 2],
    boxes: [BoundingBox3; 2],
    fuzzy: f64,
    quick: bool,
    done: bool,
    failed: bool,
    failure: Option<Failure>,
    pub(in crate::bop_algo::pave_filler) parts: Vec<CommonPrt>,
}

impl EdgeEdgeTask {
    pub(in crate::bop_algo::pave_filler) fn new(
        blocks: [PaveBlockHandle; 2],
        edges: [Edge; 2],
        ranges: [(f64, f64); 2],
    ) -> Self {
        let [block1, block2] = blocks;
        Self {
            block1,
            block2,
            edges,
            ranges,
            boxes: [BoundingBox3::default(), BoundingBox3::default()],
            fuzzy: precision::confusion(),
            quick: false,
            done: false,
            failed: false,
            failure: None,
            parts: Vec::new(),
        }
    }

    pub(in crate::bop_algo::pave_filler) fn configure(
        &mut self,
        boxes: [BoundingBox3; 2],
        fuzzy: f64,
        quick: bool,
    ) {
        self.boxes = boxes;
        self.fuzzy = fuzzy;
        self.quick = quick;
    }

    /// Native `BOPAlgoEdgeEdge::Perform`; exceptions become the task's error.
    pub(in crate::bop_algo::pave_filler) fn perform(&mut self) {
        if let Err(cause) = self.run() {
            self.failed = true;
            self.failure = Some(cause);
        }
    }

    /// `!IsDone() || HasErrors()`.
    pub(in crate::bop_algo::pave_filler) fn failed(&self) -> bool {
        // Native result storage also rejects a solver that returned without IsDone.
        !self.done || self.failed
    }

    fn run(&mut self) -> Result<(), Failure> {
        let (edges, translated) = self.translated_edge_inputs()?;
        let mut intersector = self.configured_edge_intersector(&edges);
        let result = intersector.perform();
        self.done = intersector.is_done();
        self.parts = intersector.common_parts().to_vec();
        if translated {
            self.restore_common_edge_identities();
        }
        result
    }

    fn translated_edge_inputs(&self) -> Result<([Edge; 2], bool), Failure> {
        let translation = trsf_to_point(
            &self.boxes[0],
            &self.boxes[1],
            Point3::default(),
            TRSF_TO_POINT_CRITERIA,
        )?;
        let mut edges = self.edges.clone();
        if let Some(transform) = translation {
            let location = Location::new(transform);
            for edge in &mut edges {
                *edge = Edge::unchecked(edge.moved(&location, false)?);
            }
        }
        Ok((edges, translation.is_some()))
    }

    fn configured_edge_intersector(&self, edges: &[Edge; 2]) -> EdgeEdge {
        let mut intersector = EdgeEdge::new();
        intersector.set_edge1_range(&edges[0], self.ranges[0].0, self.ranges[0].1);
        intersector.set_edge2_range(&edges[1], self.ranges[1].0, self.ranges[1].1);
        intersector.set_fuzzy_value(self.fuzzy);
        intersector.use_quick_coincidence_check(self.quick);
        intersector
    }

    fn restore_common_edge_identities(&mut self) {
        for part in &mut self.parts {
            part.set_edge1(&self.edges[0]);
            part.set_edge2(&self.edges[1]);
        }
    }
}
