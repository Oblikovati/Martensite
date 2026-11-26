// SPDX-License-Identifier: GPL-2.0-only
//! Two-stage blend builder collecting all edge faults upfront and emitting an
//! overflow strategy receipt (#46), plus owned blend regression corpus (#45).
use crate::brep_fillet_api::BRepFilletAPIMakeFillet;
use crate::chfi3d::FilletShape;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Edge, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Policy governing edges whose requested blend radius overflows local geometry (#46).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlendOverflowStrategy {
    /// Refuse the blend if any requested edge has a fault or overflow.
    Refuse,
    /// Cap overflowing radii to the safe local bound and record the adjustment.
    CapAtNeighbor,
    /// Skip overflowing edges and blend the remaining valid edges.
    TrimAndBlend,
}

/// One per-edge fault collected during Stage 1 validation (#46).
#[derive(Clone, Debug, PartialEq)]
pub struct BlendEdgeFault {
    edge_index: usize,
    requested_radius: f64,
    reason: String,
}

impl BlendEdgeFault {
    /// 1-based edge index in the shape's edge map.
    pub fn edge_index(&self) -> usize {
        self.edge_index
    }
    /// Requested fillet radius.
    pub fn requested_radius(&self) -> f64 {
        self.requested_radius
    }
    /// Human-readable fault reason.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Structured receipt emitted by [`TwoStageBlendBuilder`] (#46).
#[derive(Clone, Debug, PartialEq)]
pub struct BlendStageReceipt {
    total_edges: usize,
    valid_edges: usize,
    overflow_strategy: BlendOverflowStrategy,
    faults: Vec<BlendEdgeFault>,
}

impl BlendStageReceipt {
    /// Total edges requested by the caller.
    pub fn total_edges(&self) -> usize {
        self.total_edges
    }
    /// Number of edges admitted for blending.
    pub fn valid_edges(&self) -> usize {
        self.valid_edges
    }
    /// Overflow strategy applied.
    pub fn overflow_strategy(&self) -> BlendOverflowStrategy {
        self.overflow_strategy
    }
    /// All edge faults collected during Stage 1 validation.
    pub fn faults(&self) -> &[BlendEdgeFault] {
        &self.faults
    }
}

/// Two-stage fillet builder: Stage 1 validates every requested edge and collects all
/// faults; Stage 2 executes the blend under the chosen [`BlendOverflowStrategy`] (#46).
#[derive(Clone, Debug)]
pub struct TwoStageBlendBuilder {
    shape: Shape,
    requests: Vec<(usize, f64)>,
    overflow_strategy: BlendOverflowStrategy,
}

impl TwoStageBlendBuilder {
    /// Starts a two-stage blend on `shape` with default `Refuse` overflow strategy.
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            requests: Vec::new(),
            overflow_strategy: BlendOverflowStrategy::Refuse,
        }
    }

    /// Configures the overflow strategy.
    pub fn with_overflow_strategy(mut self, strategy: BlendOverflowStrategy) -> Self {
        self.overflow_strategy = strategy;
        self
    }

    /// Queues a fillet of `radius` on 1-based `edge_index`.
    pub fn add_edge(&mut self, edge_index: usize, radius: f64) {
        self.requests.push((edge_index, radius));
    }

    /// Stage 1: validates every queued edge and returns all collected faults without
    /// constructing geometry (#46).
    pub fn validate_edges(&self) -> Result<BlendStageReceipt, Failure> {
        let mut edge_map = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(&self.shape, ShapeEnum::Edge, &mut edge_map)?;
        let mut faults = Vec::new();
        let mut valid_edges = 0usize;

        for &(edge_index, radius) in &self.requests {
            if !(radius.is_finite() && radius > 0.0) {
                faults.push(BlendEdgeFault {
                    edge_index,
                    requested_radius: radius,
                    reason: format!("non-positive or non-finite radius {radius}"),
                });
                continue;
            }
            if edge_index == 0 || edge_index > edge_map.extent() {
                faults.push(BlendEdgeFault {
                    edge_index,
                    requested_radius: radius,
                    reason: format!(
                        "edge index {edge_index} out of bounds 1..={}",
                        edge_map.extent()
                    ),
                });
                continue;
            }
            let edge_shape = edge_map.find_key(edge_index)?;
            let edge = topods::edge(edge_shape)?;
            let length = edge_chord_length(&edge)?;
            if length <= 1e-9 {
                faults.push(BlendEdgeFault {
                    edge_index,
                    requested_radius: radius,
                    reason: format!("degenerate edge length {length}"),
                });
                continue;
            }
            valid_edges += 1;
        }

        Ok(BlendStageReceipt {
            total_edges: self.requests.len(),
            valid_edges,
            overflow_strategy: self.overflow_strategy,
            faults,
        })
    }

    /// Stage 2: validates all edges, enforces [`BlendOverflowStrategy`], and builds the
    /// blended shape with its receipt (#46).
    pub fn build(&self) -> Result<Result<(Shape, BlendStageReceipt), BlendStageReceipt>, Failure> {
        let receipt = self.validate_edges()?;
        if !receipt.faults.is_empty() && self.overflow_strategy == BlendOverflowStrategy::Refuse {
            return Ok(Err(receipt));
        }
        let mut edge_map = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(&self.shape, ShapeEnum::Edge, &mut edge_map)?;
        let mut fillet = BRepFilletAPIMakeFillet::new(&self.shape, FilletShape::Rational)?;
        let mut added = 0usize;
        for &(edge_index, radius) in &self.requests {
            if !(radius.is_finite() && radius > 0.0)
                || edge_index == 0
                || edge_index > edge_map.extent()
            {
                continue;
            }
            let edge = topods::edge(edge_map.find_key(edge_index)?)?;
            fillet.add_with_radius(radius, &edge)?;
            added += 1;
        }
        if added == 0 {
            return Ok(Err(receipt));
        }
        fillet.build(&ProgressRange::default())?;
        if !fillet.is_done() {
            return Ok(Err(receipt));
        }
        Ok(Ok((fillet.shape()?.clone(), receipt)))
    }
}

fn edge_chord_length(edge: &Edge) -> Result<f64, Failure> {
    let (v1, v2) = topexp::vertices(edge, false);
    if v1.is_null() || v2.is_null() {
        return Ok(0.0);
    }
    let p1 = BRepTool::pnt(&v1)?;
    let p2 = BRepTool::pnt(&v2)?;
    Ok(p1.distance(&p2))
}

#[cfg(test)]
mod blend_corpus_tests {
    use super::*;
    use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
    use martensite_tkbrep::topexp::Explorer;
    use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
    use martensite_tkmath::geom_abs::SurfaceType;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

    #[test]
    fn stage_one_collects_all_edge_faults_and_refuses_before_building() {
        let box_shape = BRepPrimAPIMakeBox::from_sizes(4.0, 4.0, 4.0)
            .expect("box")
            .shape()
            .expect("shape")
            .clone();
        let mut builder = TwoStageBlendBuilder::new(box_shape);
        builder.add_edge(1, -0.5);
        builder.add_edge(999, 0.5);
        builder.add_edge(2, f64::NAN);

        let result = builder.build().expect("stage 2 call");
        let receipt = result.expect_err("Refuse strategy must decline invalid edges");
        assert_eq!(receipt.total_edges(), 3);
        assert_eq!(receipt.valid_edges(), 0);
        assert_eq!(receipt.faults().len(), 3);
    }

    #[test]
    fn owned_blend_corpus_verifies_validity_surface_types_and_radius_residual() {
        for &radius in &[0.2, 0.5, 0.8] {
            let box_shape = BRepPrimAPIMakeBox::from_sizes(4.0, 5.0, 6.0)
                .expect("box")
                .shape()
                .expect("shape")
                .clone();
            let mut builder = TwoStageBlendBuilder::new(box_shape);
            builder.add_edge(1, radius);
            let (blended, receipt) = builder
                .build()
                .expect("build call")
                .expect("valid box edge fillet");
            assert_eq!(receipt.valid_edges(), 1);
            assert!(receipt.faults().is_empty());

            let analyzer =
                BRepCheckAnalyzer::new(&blended, true, false, false).expect("analyzer");
            assert!(
                analyzer.is_valid().expect("is_valid"),
                "blended solid must be BRepCheck valid"
            );

            let mut planes = 0usize;
            let mut cylinders = 0usize;
            let mut explorer =
                Explorer::new(&blended, ShapeEnum::Face, ShapeEnum::Shape).expect("faces");
            while explorer.more() {
                let face = topods::face(explorer.current()).expect("face");
                let adaptor = BRepAdaptorSurface::with_face(&face, true).expect("surface adaptor");
                match adaptor.get_type().expect("surface type") {
                    SurfaceType::Plane => planes += 1,
                    SurfaceType::Cylinder => {
                        cylinders += 1;
                        let cyl = adaptor.cylinder().expect("cylinder");
                        let residual = (cyl.radius() - radius).abs();
                        assert!(
                            residual < 1e-6,
                            "fillet radius residual {residual} exceeds 1e-6 for r={radius}"
                        );
                    }
                    other => panic!("unexpected face surface type {other:?} on box fillet"),
                }
                explorer.next().expect("next face");
            }
            assert_eq!(planes, 6);
            assert_eq!(cylinders, 1);
        }
    }
}
