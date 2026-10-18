//! The classic Extrema package: the extrema of the distance between points, curves
//! and surfaces.
//!
//! Each class drops the `Extrema_` prefix (`ExtPElC` is [`ExtPElC`]). The
//! retained class templates (`GGExtPC`, `GFuncExtPC`, ...) are generic
//! Rust types over the [`ExtremaCurveTool`] trait, which bundles their curve, point,
//! vector and tool parameters; the retained aliases (`ExtPC`,
//! `ExtPC2d`, `EPCOfExtPC`, ...) are type aliases over [`CurveTool`]
//! or [`Curve2dTool`]. An object that keeps a curve (`TheCurve*` natively) borrows it
//! for its lifetime. docs/tkgeombase-progress.md gives the full mapping;
//! `ExtPElC::with_line(point, line, 1e-7, -10.0, 10.0).nb_ext()? == 1`.
mod curve_curve;
mod curve_reference;
mod curve_surface;
mod curve_tool;
mod curve_tools;
mod elementary_point_curve;
mod ext_el_c;
mod ext_el_c2d;
mod ext_el_cs;
mod ext_el_ss;
mod ext_p_el_c;
mod ext_p_el_c2d;
mod ext_p_el_s;
mod failures;
mod gp_space;
mod pair_result;
mod point_curve;
mod point_surface;
mod pon;
mod singular;
mod surface_ref;
mod surface_surface;

pub use curve_curve::{
    CCLocFOfLocECC, ECC, ECC2d, ExtCC, ExtCC2d, GFuncExtCC, GGenExtCC, GenLocateExtCC,
    GlobOptFuncCCC0, GlobOptFuncCCC1, GlobOptFuncCCC2, LocECC, LocateExtCC,
};
pub use curve_surface::{
    ExtCS, FuncExtCS, GenExtCS, GlobOptFuncCQuadric, GlobOptFuncCS, GlobOptFuncConicS,
};
pub use curve_tool::{
    AccessD1, AccessD2, ExtremaCurveAccess, ExtremaCurveTool, SplineKnots, ToolD1, ToolD2, ToolD3,
};
pub use curve_tools::{Curve2dTool, CurveTool};
pub use elementary_point_curve::{ElementaryPointCurve, PointCurveExtrema};
pub use ext_el_c::ExtElC;
pub use ext_el_c2d::ExtElC2d;
pub use ext_el_cs::ExtElCS;
pub use ext_el_ss::ExtElSS;
pub use ext_p_el_c::ExtPElC;
pub use ext_p_el_c2d::ExtPElC2d;
pub use ext_p_el_s::ExtPElS;
pub use gp_space::{GpPoint, GpVector};
pub use pair_result::CurvePairExtrema;
pub use point_curve::{
    ELPCOfLocateExtPC, ELPCOfLocateExtPC2d, EPCOfELPCOfLocateExtPC, EPCOfELPCOfLocateExtPC2d,
    EPCOfExtPC, EPCOfExtPC2d, ExtPC, ExtPC2d, GCurveLocator, GFuncExtPC, GGExtPC, GGenExtPC,
    GLocateExtPC, GenLocateExtPC, LocEPCOfLocateExtPC, LocEPCOfLocateExtPC2d, LocateExtPC,
    LocateExtPC2d, PCFOfEPCOfELPCOfLocateExtPC, PCFOfEPCOfELPCOfLocateExtPC2d, PCFOfEPCOfExtPC,
    PCFOfEPCOfExtPC2d, PCLocFOfLocEPCOfLocateExtPC, PCLocFOfLocEPCOfLocateExtPC2d,
};
pub use point_surface::{
    ExtPExtS, ExtPRevS, ExtPS, FuncPSDist, FuncPSNorm, GenExtPS, GenLocateExtPS, Sampling,
};
pub use pon::{POnCurv, POnCurv2d, POnCurve, POnSurf, POnSurfParams};
pub use surface_surface::{ExtSS, FuncExtSS, GenExtSS};

/// `ElementType`: the grid element that holds a point on surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ElementType {
    /// `Node`: a grid node.
    #[default]
    Node,
    /// `UIsoEdge`: an edge along U.
    UIsoEdge,
    /// `VIsoEdge`: an edge along V.
    VIsoEdge,
    /// `Face`: a grid cell.
    Face,
}

/// `ExtAlgo`: the point-surface search algorithm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ExtAlgo {
    /// `ExtAlgo::Grad`: grid sampling with a gradient refinement.
    #[default]
    Grad,
    /// `ExtAlgo::Tree`: a bounding-sphere tree over the grid.
    Tree,
}

/// `ExtFlag`: which extrema a search returns.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ExtFlag {
    /// `ExtFlag::Min`: the minima.
    Min,
    /// `ExtFlag::Max`: the maxima.
    Max,
    /// `ExtFlag::MinMax`: both.
    #[default]
    MinMax,
}
