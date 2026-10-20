//! Original `BRepGraph_Reconstruct_Test`.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool, EdgeCurve3D},
    brep_graph::{
        BRepGraph, EdgeId, FaceId, NodeId, NodeKind, ShellId, SolidId, TypedId, VertexId,
        editor::{EdgeOps, VertexOps},
        iterator::{DefIterator, NodeTraits},
        tool::{edge, vertex},
    },
    topexp::Explorer,
    topods::{self, Compound, Shape},
};
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere};
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp};
use support::ref_test_tools;
include!("graph_reconstruct/fixtures.rs");
include!("graph_reconstruct/geometry.rs");
include!("graph_reconstruct/originals.rs");
include!("graph_reconstruct/mutations.rs");
