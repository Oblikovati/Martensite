//! `BOPAlgoPaveFiller_3`: edge/edge intersection (`PerformEE`) and its task class.
use super::{BOPAlgoPaveFiller, missing_structure};
use crate::bop_ds::{CoupleOfPaveBlocks, InterfEE, PaveBlockHandle};
use crate::bop_tools::make_new_vertex_edges;
use crate::int_tools::{CommonPrt, Range, Tools};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topods::{Edge, Shape, Vertex};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::{IndexedDataMap, Map};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::geom_abs::CurveType;
use martensite_tkmath::gp::Point3;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(test)]
#[path = "edge_edge/census.rs"]
mod census;
#[path = "edge_edge/execution.rs"]
mod execution;
#[path = "edge_edge/forced.rs"]
mod forced;
#[path = "edge_edge/new_vertices.rs"]
mod new_vertices;
#[path = "edge_edge/perform.rs"]
mod perform;
#[path = "edge_edge/preparation.rs"]
mod preparation;
#[path = "edge_edge/progress.rs"]
mod progress;
#[path = "edge_edge/store.rs"]
mod store;
#[path = "edge_edge/task.rs"]
mod task;
pub(super) use task::EdgeEdgeTask;

/// New vertices keyed by shape identity with the pave blocks that produced them.
pub(super) type VertexCouples = IndexedDataMap<Shape, CoupleOfPaveBlocks, ShapeMapHasher>;

/// A pave block's parameter range and the box used for its intersections.
pub(super) type RangeAndBox = ((f64, f64), BoundingBox3);

/// Cache of computed pave-block boxes (`BOPAlgoDataMapOfPaveBlockBndBox`).
pub(super) type PaveBlockBoxes = HashMap<usize, BoundingBox3>;

impl BOPAlgoPaveFiller {
    fn is_line_circle_pair(&self, blocks: &EdgePairBlocks) -> Result<bool, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let first = BRepAdaptorCurve::with_edge(&Edge::unchecked(
            structure.shape(blocks.edges.0)?.clone(),
        ))?;
        let second = BRepAdaptorCurve::with_edge(&Edge::unchecked(
            structure.shape(blocks.edges.1)?.clone(),
        ))?;
        let types = (first.get_type()?, second.get_type()?);
        Ok(matches!(
            types,
            (CurveType::Line, CurveType::Circle) | (CurveType::Circle, CurveType::Line)
        ))
    }

    fn analyze_ee_vertex(
        &mut self,
        part: &CommonPrt,
        blocks: &EdgePairBlocks,
        analytical: bool,
        couples: &mut VertexCouples,
        edges: &mut Map<i32>,
    ) -> Result<(), Failure> {
        if !blocks.splittable[0] || !blocks.splittable[1] {
            return Ok(());
        }
        let (mut t1, mut t2) = (0., 0.);
        Tools::vertex_parameters(part, &mut t1, &mut t2)?;
        let tolerance = precision::confusion();
        let common1 = *part.range1();
        let common2 = *part.ranges2().first().ok_or_else(missing_range2)?;
        let ranges = &blocks.side_ranges;
        let mut on_pave = [
            Tools::is_on_pave1(t1, &ranges[0], tolerance)
                || Tools::is_on_pave1(ranges[0].first(), &common1, tolerance),
            Tools::is_on_pave1(t1, &ranges[1], tolerance)
                || Tools::is_on_pave1(ranges[1].last(), &common1, tolerance),
            Tools::is_on_pave1(t2, &ranges[2], tolerance)
                || Tools::is_on_pave1(ranges[2].first(), &common2, tolerance),
            Tools::is_on_pave1(t2, &ranges[3], tolerance)
                || Tools::is_on_pave1(ranges[3].last(), &common2, tolerance),
        ];
        if (on_pave[0] && on_pave[2])
            || (on_pave[0] && on_pave[3])
            || (on_pave[1] && on_pave[2])
            || (on_pave[1] && on_pave[3])
        {
            return Ok(());
        }
        let vertices = blocks.vertices;
        let mut exists = false;
        for (position, flag) in on_pave.iter_mut().enumerate() {
            if *flag {
                let block = if position < 2 {
                    &blocks.block2
                } else {
                    &blocks.block1
                };
                *flag = self.force_interf_ve(vertices[position], block, edges)?;
                exists |= *flag;
            }
        }
        let mut vertex = Vertex::default();
        make_new_vertex_edges(part.edge1(), t1, part.edge2(), t2, &mut vertex)?;
        let point = BRepTool::pnt(&vertex)?;
        if exists && !self.extend_existing_ee_vertices(part, (t1, t2), &on_pave, vertices, point)? {
            return Ok(());
        }
        let mut new_tolerance = BRepTool::vertex_tolerance(&vertex)?;
        if analytical {
            // A Line/Circle hit gets half the common range as tolerance, applied only later.
            let line_first =
                BRepAdaptorCurve::with_edge(part.edge1())?.get_type()? == CurveType::Line;
            let minimum = if line_first {
                (common1.last() - common1.first()) / 2.
            } else {
                (common2.last() - common2.first()) / 2.
            };
            if minimum > new_tolerance {
                new_tolerance = minimum;
            }
        }
        if self.near_shared_ee_vertex(vertices, point, new_tolerance)? {
            return Ok(());
        }
        let index = self.append_ee_record(blocks.edges, part);
        let mut couple = CoupleOfPaveBlocks::new();
        couple.set_pave_blocks(
            Some(Arc::clone(&blocks.block1)),
            Some(Arc::clone(&blocks.block2)),
        );
        couple.set_interference_index(index);
        couple.set_tolerance(new_tolerance);
        couples.add(vertex.into_shape(), couple);
        Ok(())
    }

    /// For an intersection close to existing vertices: false when it is only a touching point,
    /// otherwise grows the existing vertices to the new point and protects them from extension.
    fn extend_existing_ee_vertices(
        &mut self,
        part: &CommonPrt,
        (t1, t2): (f64, f64),
        on_pave: &[bool; 4],
        vertices: [i32; 4],
        new_point: Point3,
    ) -> Result<bool, Failure> {
        let first = BRepAdaptorCurve::with_edge(part.edge1())?.value(t1)?;
        let second = BRepAdaptorCurve::with_edge(part.edge2())?.value(t2)?;
        if first.distance(&second) > precision::intersection() {
            return Ok(false);
        }
        for (position, flag) in on_pave.iter().enumerate() {
            if !*flag {
                continue;
            }
            let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
            let vertex = Vertex::unchecked(structure.shape(vertices[position])?.clone());
            let distance = BRepTool::pnt(&vertex)?.distance(&new_point);
            self.update_vertex(vertices[position], distance)?;
            if !self
                .vertices_to_avoid_extension
                .contains(&vertices[position])
            {
                self.vertices_to_avoid_extension.push(vertices[position]);
            }
        }
        Ok(true)
    }

    /// Native "LXBR" check: a new point near a vertex shared by both pave blocks is dropped.
    fn near_shared_ee_vertex(
        &self,
        vertices: [i32; 4],
        point: Point3,
        tolerance: f64,
    ) -> Result<bool, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut first_block: Map<i32> = Map::new(1);
        first_block.add(vertices[0]);
        first_block.add(vertices[1]);
        let shared: Vec<i32> = [vertices[2], vertices[3]]
            .into_iter()
            .filter(|vertex| first_block.contains(vertex))
            .collect();
        for vertex in shared {
            let shape = Vertex::unchecked(structure.shape(vertex)?.clone());
            let vertex_tolerance = BRepTool::vertex_tolerance(&shape)?;
            let distance = point.square_distance(&BRepTool::pnt(&shape)?);
            let limit = 100. * (tolerance + vertex_tolerance) * (tolerance + vertex_tolerance);
            if distance < limit {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn append_ee_record(&mut self, (first, second): (i32, i32), part: &CommonPrt) -> i32 {
        let structure = self
            .data_structure
            .as_mut()
            .expect("PerformEE runs on an initialized DS");
        let record = structure.interf_ee().append(InterfEE::default());
        record.set_indices(first, second);
        record.set_common_part(part);
        let index = structure.interf_ee().len() as i32 - 1;
        structure.add_interference(first, second);
        index
    }
}

/// The two pave blocks of an EE task with their native bookkeeping values.
struct EdgePairBlocks {
    block1: PaveBlockHandle,
    block2: PaveBlockHandle,
    edges: (i32, i32),
    vertices: [i32; 4],
    splittable: [bool; 2],
    /// aR11, aR12, aR21, aR22: the parts of each block outside its shrunk range.
    side_ranges: [Range; 4],
}

impl EdgePairBlocks {
    fn new(block1: &PaveBlockHandle, block2: &PaveBlockHandle) -> Self {
        let (edge1, vertices1, splittable1, ranges1) = block_values(block1);
        let (edge2, vertices2, splittable2, ranges2) = block_values(block2);
        Self {
            block1: Arc::clone(block1),
            block2: Arc::clone(block2),
            edges: (edge1, edge2),
            vertices: [vertices1.0, vertices1.1, vertices2.0, vertices2.1],
            splittable: [splittable1, splittable2],
            side_ranges: [ranges1[0], ranges1[1], ranges2[0], ranges2[1]],
        }
    }
}

/// Edge index, end vertices, splittable flag and the two side ranges of a block; without
/// shrunk data the shrunk range is the whole range and the block is not splittable.
fn block_values(block: &PaveBlockHandle) -> (i32, (i32, i32), bool, [Range; 2]) {
    let locked = block.lock().expect("pave-block lock must not be poisoned");
    let (first, last) = locked.range();
    let (shrunk_first, shrunk_last, splittable) = if locked.has_shrunk_data() {
        let (shrunk_first, shrunk_last, _, splittable) = locked.shrunk_data();
        (shrunk_first, shrunk_last, splittable)
    } else {
        (first, last, false)
    };
    let ranges = [
        Range::new(first, shrunk_first),
        Range::new(shrunk_last, last),
    ];
    (locked.original_edge(), locked.indices(), splittable, ranges)
}

/// Native `GetPBBox`: the block range, and its shrunk box (or a cached curve box); `None`
/// for a block without a valid range.
pub(super) fn pave_block_box(
    edge: &Edge,
    block: &PaveBlockHandle,
    boxes: &mut PaveBlockBoxes,
) -> Result<Option<RangeAndBox>, Failure> {
    let locked = block.lock().expect("pave-block lock must not be poisoned");
    let range = locked.range();
    if range.1 - range.0 <= precision::p_confusion() {
        return Ok(None);
    }
    if locked.has_shrunk_data() {
        return Ok(Some((range, locked.shrunk_data().2.clone())));
    }
    let key = Arc::as_ptr(block) as usize;
    if let Some(bounds) = boxes.get(&key) {
        return Ok(Some((range, bounds.clone())));
    }
    let curve = BRepAdaptorCurve::with_edge(edge)?;
    let tolerance = BRepTool::edge_tolerance(edge)? + precision::confusion();
    let mut bounds = BoundingBox3::default();
    martensite_tkgeombase::bnd_lib::Add3dCurve::add_range(
        &curve,
        range.0,
        range.1,
        tolerance,
        &mut bounds,
    )?;
    boxes.insert(key, bounds.clone());
    Ok(Some((range, bounds)))
}

fn original_edge(block: &PaveBlockHandle) -> i32 {
    block
        .lock()
        .expect("pave-block lock must not be poisoned")
        .original_edge()
}

fn missing_range2() -> Failure {
    Failure::with_type(
        "Standard_OutOfRange",
        "IntToolsCommonPrt::Ranges2 is empty; expected one range for an EE vertex",
        "",
    )
}

#[cfg(test)]
#[path = "edge_edge/progress_fixture.rs"]
mod progress_fixture;
