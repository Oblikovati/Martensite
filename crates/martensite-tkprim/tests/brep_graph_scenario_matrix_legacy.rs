//! Original `BRepGraph_ScenarioMatrix_Test`.
#![allow(non_snake_case)]
mod support;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_graph::{
        editor::{EdgeOps, FaceOps, VertexOps},
        tool::{co_edge, edge, vertex},
        validate::{self, Options, Severity},
        *,
    },
    brep_graph_inc::{ParityOrientation, populate},
    topexp::{self, Explorer},
    topods::{self, CompSolid, Compound, Shape, ShapeIterator, Vertex, Wire},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::{
    geom::{GeomPlane, GeomSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{
    gp::{Direction2, Direction3, Point2, Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape},
    brep_gprop::BRepGProp,
};
include!("graph_scenarios/fixtures.rs");
include!("graph_scenarios/mutations.rs");
include!("graph_scenarios/containers.rs");
include!("graph_scenarios/assemblies.rs");
include!("graph_scenarios/seams.rs");
include!("graph_scenarios/editor.rs");
