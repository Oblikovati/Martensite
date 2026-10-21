//! Original `BRepGraph_Fuzz_Test`.
//! Inputs preserve libstdc++ distributions and GCC argument evaluation; see graph_fuzz/inputs.
#![allow(non_snake_case)]
use martensite_tkbrep::brep_graph::{
    BRepGraph, EdgeId, FaceId, VertexId,
    editor::{EdgeOps, FaceOps, VertexOps},
    validate::{self, Options},
};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
include!("graph_fuzz/replay.rs");
include!("graph_fuzz/cases.rs");
