//! Translation of IGES entities into shapes.
mod actor;
mod basic_curve;
mod basic_curve_arcs;
mod basic_curve_conics;
mod basic_curve_conics2d;
mod basic_curve_copious;
mod basic_curve_splines;
mod basic_surface;
mod basic_surface_bspline;
mod brep_entity;
mod brep_entity_solid;
mod containers;
mod curve_and_surface;
mod curve_and_surface_transfer;
mod iges_boundary;
mod iges_boundary_reverse;
mod knot_gathering;
mod package;
mod reader;
mod topo_curve;
mod topo_curve_basic;
mod topo_curve_composite;
mod topo_curve_offset;
mod topo_curve_on_surface;
mod topo_surface;
mod topo_surface_bounded;
mod topo_surface_param;
mod topo_surface_plane;
mod topo_surface_swept;

pub use actor::IgesToBRepActor;
pub use basic_curve::IgesToBRepBasicCurve;
pub use basic_surface::IgesToBRepBasicSurface;
pub use brep_entity::IgesToBRepBRepEntity;
pub use containers::{IgesToBRepAlgoContainer, IgesToBRepToolContainer, IgesToBRepToolKind};
pub use curve_and_surface::IgesToBRepCurveAndSurface;
pub use iges_boundary::{
    BoundaryCurves, BoundaryFlags, IgesToBRepBoundaryOps, IgesToBRepCurves2d,
    IgesToBRepIgesBoundary,
};
pub(crate) use iges_boundary::{
    BoundaryPlace, BoundarySequences, local_2d_curve, local_3d_curve, transfer_2d_only,
    transfer_3d_only, wire_fix_on_face,
};
pub use iges_boundary_reverse::{reverse_curves_2d, reverse_curves_3d};
pub use package::IgesToBRep;
pub(crate) use package::one_transfer_at_a_time;
pub use reader::IgesToBRepReader;
pub use topo_curve::IgesToBRepTopoCurve;
pub use topo_surface::IgesToBRepTopoSurface;
pub use topo_surface_plane::PlaneParts;
