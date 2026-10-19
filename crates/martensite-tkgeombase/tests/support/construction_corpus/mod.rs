//! Mod behavior and supporting operations.
#[allow(dead_code, unused_imports)]
pub mod common;
mod dispatch;
#[path = "elementary_construction/mod.rs"]
pub mod gce;
#[allow(dead_code)]
pub mod print;
pub use dispatch::run_values;
pub mod app_par_containers;
pub mod bndlib;
pub mod constraint_line_fixtures;
pub mod continuity_function;
pub mod continuity_least_squares;
pub mod continuity_matrix;
pub mod continuity_tables;
pub mod extrema_el;
pub mod extrema_el_surf;
pub mod extrema_pc;
pub mod finite_element_curve;
pub mod finite_element_driver;
pub mod fit_constraints_line;
pub mod fit_constraints_points;
pub mod fixture_input;
#[path = "geometric_construction/mod.rs"]
pub mod gc;
#[path = "curve_sampling/mod.rs"]
pub mod gcpnts;
#[path = "planar_conversion.rs"]
pub mod geom2d_convert;
#[path = "spatial_conversion/mod.rs"]
pub mod geom_convert;
pub mod geometry_fixtures;
pub mod geometry_record_fixtures;
pub mod geometry_utilities_elementary;
#[path = "mass_properties.rs"]
pub mod gprop;
#[path = "hermite.rs"]
pub mod hermit;
#[path = "analytic_intersection/mod.rs"]
pub mod int_ana;
#[path = "planar_analytic_intersection.rs"]
pub mod int_ana2d;
pub mod linear_tokens;
#[path = "differential_properties.rs"]
pub mod lprop;
#[path = "polynomial_arithmetic.rs"]
pub mod math_base;
pub mod matrix_tokens;
pub mod multi_curve_tokens;
pub mod numeric_protocol;
pub mod pc2d;
pub mod pc3d;
#[cfg(test)]
pub mod policy;
pub mod polynomial_arithmetic_system;
#[cfg(test)]
pub mod replay;

pub mod extrema_cc;

pub mod extrema_ps;
pub mod extrema_swept;

pub mod extrema_cs;
pub mod extrema_ss;

pub mod geometry_utilities_statics;

pub mod geometry_utilities_classes;

pub mod app_par_fit;
pub mod curve_projection_elementary;

pub mod approx_compute_shared;

pub mod adv_class;
pub mod adv_eval;
pub mod adv_f2var;
pub mod adv_f2var2;
pub mod adv_mutable_eval;

pub mod approx_compute_cases;
pub mod approx_fitting;
pub mod approx_line_fixtures;
pub mod curve_projection_comp;
pub mod curve_projection_curves;
pub mod curve_projection_on_plane;
pub mod fit_constraints_compute;
pub mod fit_constraints_criteria;
pub mod fit_constraints_variational;
#[path = "geometry_text/mod.rs"]
pub mod geom_tools;
pub mod geometry_adaptor_fixtures;

pub mod approx_adaptors;
pub mod approx_sweep;

#[path = "surface_projection.rs"]
pub mod geom_proj_lib;
