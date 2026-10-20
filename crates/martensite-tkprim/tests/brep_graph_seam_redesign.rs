//! Original `BRepGraph_SeamRedesign_Test`.
#![allow(non_snake_case)]
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_graph::{
        BRepGraph, CoEdgeId, NodeId, NodeKind, WireId,
        iterator::{EdgeIterator, WireIterator},
        refs_iterator::CoEdgesOfWire,
        tool::{co_edge, edge, wire},
        validate,
    },
    topexp::Explorer,
    topods::{Shape, ShapeIterator, Vertex},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
include!("graph_seams/fixtures.rs");
include!("graph_seams/queries.rs");
include!("graph_seams/round_trip.rs");
