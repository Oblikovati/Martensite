//! Original `BRepGraph_Views_Test`.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::brep_graph_inc::ParityOrientation;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_graph::{
        defs_iterator::DefsVertexOfEdge,
        layer_history::LayerHistory,
        layer_topo_supplement::LayerTopoSupplement,
        refs_iterator::RefsVertexOfEdge,
        related_iterator::{RelatedIterator, RelationKind},
        tool::{co_edge, edge, face, vertex},
        *,
    },
    topods::{Compound, Edge, Face, Shape, Shell, Vertex, Wire},
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::{
    gp::{Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
include!("graph_views/fixtures.rs");
include!("graph_views/topology.rs");
include!("graph_views/uids.rs");
include!("graph_views/refs.rs");
include!("graph_views/shapes.rs");
include!("graph_views/cache.rs");
include!("graph_views/editor.rs");
include!("graph_views/regressions.rs");
