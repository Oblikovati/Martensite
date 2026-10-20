//! Original `BRepGraph_Validate_Test`.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_graph::{
        BRepGraph, CoEdgeId, EdgeCurve3DRepId, EdgeId, FaceId, NodeId, NodeKind, OccurrenceId,
        ProductId, VertexId, VertexRefId, WireId, deduplicate,
        iterator::EdgeIterator,
        validate::{self, Options, Severity, ValidationResult},
    },
    brep_graph_inc::ParityOrientation,
    topexp::Explorer,
    topods::{Compound, Shape},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::{
    geom::{
        GeomCircle, GeomCurve, GeomCylindricalSurface, GeomLine, GeomPlane, GeomSphericalSurface,
        GeomSurface,
    },
    top_abs::ShapeEnum,
};
use martensite_tkmath::{
    gp::{Axis1, Axis2, Axis3, Direction3, Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
include!("graph_validate/fixtures.rs");
include!("graph_validate/clean.rs");
include!("graph_validate/corruption.rs");
include!("graph_validate/assembly.rs");
include!("graph_validate/connectivity.rs");
include!("graph_validate/synthetic_helpers.rs");
include!("graph_validate/synthetic_box.rs");
include!("graph_validate/synthetic_curved.rs");
include!("graph_validate/synthetic_assembly.rs");
