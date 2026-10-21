//! Original `BRepGraph_Build_Test`.
#![allow(non_snake_case)]
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_graph::{
        BRepGraph, EdgeId, FaceId, NodeId, NodeKind, VertexId, WireId,
        iterator::{EdgeIterator, FaceIterator, VertexIterator},
        shapes_view::{AddStatus, Options},
        tool::{edge, face, vertex},
    },
    topexp,
    topods::{self, Compound, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tkprim::brep_prim_api::*;
use martensite_tktopalgo::brep_builder_api::*;
include!("graph_build/fixtures.rs");
include!("graph_build/primitives.rs");
include!("graph_build/compounds.rs");
include!("graph_build/minimal.rs");
include!("graph_build/box_queries.rs");
include!("graph_build/flatten.rs");
include!("graph_build/natural.rs");
