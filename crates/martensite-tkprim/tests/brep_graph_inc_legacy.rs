//! Original `BRepGraphInc_Test`.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_graph::{
        layer_topo_supplement::{AttachmentKind, LayerTopoSupplement},
        tool::{edge, face},
        *,
    },
    brep_graph_inc::{ParityOrientation, Storage, populate, reconstruct},
    topexp::Explorer,
    topods::{self, CompSolid, Compound, Edge, Face, Shape, Shell, Solid, Wire},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::{
    geom::{GeomPlane, GeomSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{
    gp::{Direction2, Plane3, Point2, Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere};
use martensite_tktopalgo::{brep_builder_api::*, brep_gprop::BRepGProp};
include!("graph_inc/fixtures.rs");
include!("graph_inc/primitives.rs");
include!("graph_inc/storage.rs");
include!("graph_inc/relations.rs");
include!("graph_inc/mutations.rs");
include!("graph_inc/cleanup.rs");
include!("graph_inc/supplements.rs");
include!("graph_inc/locations.rs");

include!("graph_inc/rebinding.rs");
