// Extrema (GFuncExtPC, GGenExtPC, GenLocateExtPC,
// GGExtPC, GLocateExtPC, GCurveLocator and their
// ExtPC*, LocateExtPC*, EPCOf*, ELPCOf*,
// LocEPCOf*, PCFOf* and PCLocFOf* aliases) at
// .

//! The point-curve extrema: the class templates, generic over [`ExtremaCurveTool`],
//! and the retained aliases of their 3D and 2D instantiations.
mod curve_locator;
mod ext_pc;
mod func_ext_pc;
mod gen_ext_pc;
mod gen_locate_ext_pc;
mod locate_ext_pc;

pub use curve_locator::GCurveLocator;
pub use ext_pc::GGExtPC;
pub use func_ext_pc::GFuncExtPC;
pub use gen_ext_pc::GGenExtPC;
pub use gen_locate_ext_pc::GenLocateExtPC;
pub use locate_ext_pc::GLocateExtPC;

use super::curve_tools::{Curve2dTool, CurveTool};

/// `ExtPC`: `GGExtPC` over Adaptor3d_Curve.
pub type ExtPC<'a> = GGExtPC<'a, CurveTool>;
/// `ExtPC2d`: `GGExtPC` over Adaptor2d_Curve2d.
pub type ExtPC2d<'a> = GGExtPC<'a, Curve2dTool>;
/// `ELPCOfLocateExtPC`: the closed-form search of LocateExtPC.
pub type ELPCOfLocateExtPC<'a> = GGExtPC<'a, CurveTool>;
/// `ELPCOfLocateExtPC2d`.
pub type ELPCOfLocateExtPC2d<'a> = GGExtPC<'a, Curve2dTool>;
/// `EPCOfExtPC`: `GGenExtPC` over Adaptor3d_Curve.
pub type EPCOfExtPC<'a> = GGenExtPC<'a, CurveTool>;
/// `EPCOfExtPC2d`.
pub type EPCOfExtPC2d<'a> = GGenExtPC<'a, Curve2dTool>;
/// `EPCOfELPCOfLocateExtPC`.
pub type EPCOfELPCOfLocateExtPC<'a> = GGenExtPC<'a, CurveTool>;
/// `EPCOfELPCOfLocateExtPC2d`.
pub type EPCOfELPCOfLocateExtPC2d<'a> = GGenExtPC<'a, Curve2dTool>;
/// `PCFOfEPCOfExtPC`: `GFuncExtPC` over Adaptor3d_Curve.
pub type PCFOfEPCOfExtPC<'a> = GFuncExtPC<'a, CurveTool>;
/// `PCFOfEPCOfExtPC2d`.
pub type PCFOfEPCOfExtPC2d<'a> = GFuncExtPC<'a, Curve2dTool>;
/// `PCFOfEPCOfELPCOfLocateExtPC`.
pub type PCFOfEPCOfELPCOfLocateExtPC<'a> = GFuncExtPC<'a, CurveTool>;
/// `PCFOfEPCOfELPCOfLocateExtPC2d`.
pub type PCFOfEPCOfELPCOfLocateExtPC2d<'a> = GFuncExtPC<'a, Curve2dTool>;
/// `PCLocFOfLocEPCOfLocateExtPC`.
pub type PCLocFOfLocEPCOfLocateExtPC<'a> = GFuncExtPC<'a, CurveTool>;
/// `PCLocFOfLocEPCOfLocateExtPC2d`.
pub type PCLocFOfLocEPCOfLocateExtPC2d<'a> = GFuncExtPC<'a, Curve2dTool>;
/// `LocEPCOfLocateExtPC`: `GenLocateExtPC` over Adaptor3d_Curve.
pub type LocEPCOfLocateExtPC<'a> = GenLocateExtPC<'a, CurveTool>;
/// `LocEPCOfLocateExtPC2d`.
pub type LocEPCOfLocateExtPC2d<'a> = GenLocateExtPC<'a, Curve2dTool>;
/// `LocateExtPC`: `GLocateExtPC` over Adaptor3d_Curve.
pub type LocateExtPC<'a> = GLocateExtPC<'a, CurveTool>;
/// `LocateExtPC2d`.
pub type LocateExtPC2d<'a> = GLocateExtPC<'a, Curve2dTool>;
