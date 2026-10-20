//! One-to-one port of `BRepGraph_ShapesViewImport_Test` at
//!, using production primitive/copy builders.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::brep_graph::child_explorer::ChildExplorer;
use martensite_tkbrep::brep_graph::defs_iterator::DefsVertexOfEdge;
use martensite_tkbrep::brep_graph::iterator::{EdgeIterator, FaceIterator, VertexIterator};
use martensite_tkbrep::brep_graph::layer_history::LayerHistory;
use martensite_tkbrep::brep_graph::related_iterator::{RelatedIterator, RelationKind};
use martensite_tkbrep::brep_graph::shapes_view::{Options, ShapeNodeMap};
use martensite_tkbrep::brep_graph::*;
use martensite_tkbrep::brep_graph_inc::ParityOrientation;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis1, Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
use support::ref_test_tools::*;
const FORWARD: ParityOrientation = ParityOrientation::FORWARD;
include!("brep_graph_shapes_view_import_legacy/fixtures.rs");
include!("brep_graph_shapes_view_import_legacy/construction.rs");
include!("brep_graph_shapes_view_import_legacy/mutation.rs");
include!("brep_graph_shapes_view_import_legacy/adjacency.rs");
