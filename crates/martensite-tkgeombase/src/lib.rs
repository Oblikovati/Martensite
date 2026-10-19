//! Geometric construction, approximation, bounds and analysis.
//!
//! These low-level builders and evaluators expose operation-specific results and
//! diagnostics. They do not perform full modelling-result validation.
pub mod adv_app2var;
pub mod app_cont;
pub mod app_def;
pub mod app_par_curves;
pub mod approx;
mod array_access;
pub mod bnd_lib;
pub mod cpnts;
mod curve_approx_eval;
pub mod extrema;
pub mod extrema_pc;
pub mod extrema_pc2d;
pub mod extrema_pc_core;
pub mod failure_capture;
pub mod fem_tool;
pub mod gc;
pub mod gce;
pub mod gce2d;
pub mod gcpnts;
pub mod geom2d_convert;
pub mod geom_bnd_lib;
pub mod geom_convert;
pub mod geom_lib;
pub mod geom_lprop;
pub mod geom_proj_lib;
pub mod geom_tools;
pub mod gprop;
pub mod hermit;
pub mod int_ana;
pub mod int_ana2d;
pub mod lprop;
mod native_failure;
mod native_text;
#[doc(hidden)]
pub mod probe_support;
pub mod proj_lib;
pub mod scan_step;
