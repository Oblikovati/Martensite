//! Legacy `BRepGraph_Deduplicate_Test`.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_graph::layer_history::LayerHistory;
use martensite_tkbrep::brep_graph::shapes_view::Options as ShapeOptions;
use martensite_tkbrep::brep_graph::{
    compact,
    deduplicate::{self, DeduplicateResult, Options},
    validate, *,
};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Compound, Shape, Wire};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer as Analyzer;
include!("brep_graph_deduplicate_legacy/fixtures.rs");
include!("brep_graph_deduplicate_legacy/geometry.rs");
include!("brep_graph_deduplicate_legacy/basic_1.rs");
include!("brep_graph_deduplicate_legacy/basic_2.rs");
include!("brep_graph_deduplicate_legacy/basic_3.rs");
include!("brep_graph_deduplicate_legacy/history.rs");
include!("brep_graph_deduplicate_legacy/topology.rs");
include!("brep_graph_deduplicate_legacy/pipeline.rs");
include!("brep_graph_deduplicate_legacy/checks.rs");
