//! Original `BRepGraph_Geometry_Test`.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_graph::{
        cache_derived_state::CacheDerivedState,
        iterator::DefIterator,
        parent_explorer::ParentExplorer,
        tool::{co_edge, edge, face},
        *,
    },
    topods::{Compound, Shape},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::{
    adaptor2d::Adaptor2dCurve2d,
    geom2d::{Geom2dCurve, Geom2dLine},
};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::{
    gp::{Direction2, Plane3, Point2, Point3, Transform3, Vector3},
    poly::{Polygon2D, Polygon3D, PolygonOnTriangulation, Triangulation},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::collections::HashMap;
include!("graph_geometry/fixtures.rs");
include!("graph_geometry/navigation.rs");
include!("graph_geometry/traversal.rs");
include!("graph_geometry/pcurves.rs");
include!("graph_geometry/representations.rs");
include!("graph_geometry/reuse_access.rs");
include!("graph_geometry/reuse.rs");
