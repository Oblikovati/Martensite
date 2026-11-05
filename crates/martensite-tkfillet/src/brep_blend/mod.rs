//! BRepBlend: the walking of the blend functions on B-rep faces (the instances of the
//! Blend generics on `Adaptor3dSurface`, `Adaptor2dCurve2d` restrictions and
//! `Adaptor3dHVertex` vertices), the blend lines, the functions between a surface and a
//! restriction or two restrictions, and the approximation of the blend surfaces.
mod app_func;
mod app_surf;
mod app_surface;
mod arc_ends;
mod blend_tool;
mod curv_point_rad_inv;
mod extremity;
mod h_curve2d_tool;
mod h_curve_tool;
mod line;
mod marching;
mod point_on_rst;
mod rad_inv_macro;
mod rad_inv_terms;
mod rst_rst;
mod rst_rst_builder;
mod surf_curv_inv;
mod surf_point_inv;
pub(crate) mod surf_rst;
mod surf_rst_builder;
mod walking;

pub use app_func::{
    BRepBlendAppFunc, BRepBlendAppFuncRoot, BRepBlendAppFuncRst, BRepBlendAppFuncRstRst,
    SharedAppFunction,
};
pub use app_surf::{BRepBlendAppSurf, BlendSectionGenerator};
pub use app_surface::{AppSurfaceOptions, BRepBlendAppSurface};
pub use blend_tool::BRepBlendBlendTool;
pub use curv_point_rad_inv::BRepBlendCurvPointRadInv;
pub use extremity::BRepBlendExtremity;
pub use h_curve_tool::BRepBlendHCurveTool;
pub use h_curve2d_tool::BRepBlendHCurve2dTool;
pub use line::{BRepBlendLine, BRepBlendLineHandle};
pub use marching::WalkingDomain;
pub use point_on_rst::BRepBlendPointOnRst;
pub use rst_rst::{BRepBlendRstRstConstRad, BRepBlendRstRstEvolRad, RstRstInputs};
pub use rst_rst_builder::{BRepBlendRstRstLineBuilder, RstRstInverses, RstRstWalkSettings};
pub use surf_curv_inv::{BRepBlendSurfCurvConstRadInv, BRepBlendSurfCurvEvolRadInv};
pub use surf_point_inv::{BRepBlendSurfPointConstRadInv, BRepBlendSurfPointEvolRadInv};
pub use surf_rst::{BRepBlendSurfRstConstRad, BRepBlendSurfRstEvolRad, SurfRstInputs};
pub use surf_rst_builder::{BRepBlendSurfRstLineBuilder, SurfRstInverses, SurfRstWalkSettings};
pub use walking::{BRepBlendWalking, WalkingTolerances};

/// `BRepBlendChAsym`: the typedef of `BlendFuncChAsym`.
pub type BRepBlendChAsym = crate::blend_func::ChAsym;
/// `BRepBlendChAsymInv`: the typedef of `BlendFuncChAsymInv`.
pub type BRepBlendChAsymInv = crate::blend_func::ChAsymInv;
/// `BRepBlendChamfInv`: the typedef of `BlendFuncChamfInv`.
pub type BRepBlendChamfInv = crate::blend_func::ChamfInv;
/// `BRepBlendChamfer`: the typedef of `BlendFuncChamfer`.
pub type BRepBlendChamfer = crate::blend_func::Chamfer;
/// `BRepBlendConstRad`: the typedef of `BlendFuncConstRad`.
pub type BRepBlendConstRad = crate::blend_func::ConstRad;
/// `BRepBlendConstRadInv`: the typedef of `BlendFuncConstRadInv`.
pub type BRepBlendConstRadInv = crate::blend_func::ConstRadInv;
/// `BRepBlendConstThroat`: the typedef of `BlendFuncConstThroat`.
pub type BRepBlendConstThroat = crate::blend_func::ConstThroat;
/// `BRepBlendConstThroatInv`: the typedef of `BlendFuncConstThroatInv`.
pub type BRepBlendConstThroatInv = crate::blend_func::ConstThroatInv;
/// `BRepBlendConstThroatWithPenetration`: the typedef of
/// `BlendFuncConstThroatWithPenetration`.
pub type BRepBlendConstThroatWithPenetration = crate::blend_func::ConstThroatWithPenetration;
/// `BRepBlendConstThroatWithPenetrationInv`: the typedef of
/// `BlendFuncConstThroatWithPenetrationInv`.
pub type BRepBlendConstThroatWithPenetrationInv = crate::blend_func::ConstThroatWithPenetrationInv;
/// `BRepBlendEvolRad`: the typedef of `BlendFuncEvolRad`.
pub type BRepBlendEvolRad = crate::blend_func::EvolRad;
/// `BRepBlendEvolRadInv`: the typedef of `BlendFuncEvolRadInv`.
pub type BRepBlendEvolRadInv = crate::blend_func::EvolRadInv;

use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;

/// The curve behind a handle, or StandardNullObject naming `class` (a null dereference
/// natively).
fn live_curve_of<'a>(
    curve: &'a Handle<dyn Adaptor3dCurve>,
    class: &str,
) -> Result<&'a dyn Adaptor3dCurve, Failure> {
    curve.get().ok_or_else(|| {
        Failure::with_message(
            "Standard_NullObject",
            &format!("{class}: null curve; expected an Adaptor3d_Curve"),
        )
    })
}

/// The 2d curve behind a handle, or StandardNullObject naming `class`.
fn live_curve2d_of<'a>(
    curve: &'a Handle<dyn Adaptor2dCurve2d>,
    class: &str,
) -> Result<&'a dyn Adaptor2dCurve2d, Failure> {
    curve.get().ok_or_else(|| {
        Failure::with_message(
            "Standard_NullObject",
            &format!("{class}: null curve; expected an Adaptor2d_Curve2d"),
        )
    })
}
