//! Original `BRepGraph_Assembly_Test`.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_graph::{
        editor::OccurrenceOps,
        iterator::{DefIterator, RootProductIterator},
        shapes_view::Options,
        *,
    },
    topexp::Explorer,
    topods::{Compound, Shape, ShapeIterator},
};
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::{Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::{brep_bnd_lib::BRepBndLib, brep_builder_api::BRepBuilderAPIMakeShape};
include!("graph_assembly/fixtures.rs");
include!("graph_assembly/basics.rs");
include!("graph_assembly/links.rs");
include!("graph_assembly/roots.rs");
include!("graph_assembly/removals.rs");
include!("graph_assembly/placements.rs");
include!("graph_assembly/reconstruction.rs");
include!("graph_assembly/imports.rs");
