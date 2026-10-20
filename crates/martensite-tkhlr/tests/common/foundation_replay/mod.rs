//! Owned interval/interference operation dispatch shared by direct tests and CLI.
pub(crate) mod arc_fixture;
mod arc_function;
mod boundary_arc_rank;
mod brep_algo;
mod brep_algo_report;
mod brep_cinter;
mod brep_curve;
pub(crate) mod brep_curve_fixture;
pub(crate) mod brep_curve_print;
mod brep_data;
pub(crate) mod brep_data_bounds;
mod brep_data_report;
mod brep_data_walk;
mod brep_edge_record;
mod brep_edgeface;
pub(crate) mod brep_face_fixture;
mod brep_face_record;
mod brep_hider;
mod brep_intersector;
mod brep_polygonal;
pub(crate) mod brep_record_fixture;
pub(crate) mod brep_record_print;
mod brep_reflection;
pub(crate) mod brep_shape_fixture;
mod brep_surface;
pub(crate) mod brep_surface_print;
mod common;
mod contana;
mod contap;
mod contour_frame;
mod frame;
mod function;
#[path = "../geometry_records/mod.rs"]
pub(crate) mod geometry_records;
mod hcont_arc;
mod hcurve;
mod hlr_algo;
mod hlr_algo_internal;
mod hlr_algo_poly;
mod hlr_algo_projector;
mod intrv;
mod iwline;
#[path = "../../support/grid_mesh.rs"]
pub(crate) mod polygon_mesh_fixture;
mod reader;
mod sampling;
mod sampling_fixture;
mod search_boundary;
mod search_inside;
mod search_records;
mod sink;
mod surface;
mod surface_setup;
mod top_cnx;
mod topo_outliner;
mod walking;

/// Execute one selected foundation request without process I/O.
pub fn execute_request(line: &str) -> Option<String> {
    let (operation, values) = request_fields(line)?;
    dispatch(operation, &values)
}

fn request_fields(line: &str) -> Option<(&str, Vec<f64>)> {
    let mut fields = line.split_whitespace();
    let operation = fields.next()?;
    let values = fields
        .map(|value| value.parse().expect("numeric record field"))
        .collect();
    Some((operation, values))
}

/// Execute a sampling or function request with owned bounds; other families remain unsupported.
/// `execute_request_with_context(line, context)`.
pub fn execute_request_with_context(
    line: &str,
    context: &mut martensite_tkhlr::contap::ContapSamplingContext,
) -> Option<String> {
    let (operation, values) = request_fields(line)?;
    let mut tokens = sink::Sink::default();
    match operation {
        "ct_sampling" => sampling::case_with_context(&mut tokens, &values, context),
        "ct_function" => function::case_with_context(&mut tokens, &values, context),
        "ct_arc_function" => arc_function::case_with_context(&mut tokens, &values, context),
        _ => return None,
    }
    Some(tokens.finish())
}

type ReplayOperation = fn(&mut sink::Sink, &[f64]);
const REPLAY_OPERATIONS: &[(&str, ReplayOperation)] = &[
    ("iv_interval", intrv::interval_case),
    ("iv_edit", intrv::edit_case),
    ("iv_relate", intrv::relate_case),
    ("iv_set", intrv::set_case),
    ("iv_value", intrv::value_case),
    ("tb_interference", top_cnx::interference_case),
    ("tc_transition", top_cnx::transition_case),
    ("hb_algo", brep_algo::algo_case),
    ("hb_hider", brep_hider::hider_case),
    ("hb_data", brep_data::data_case),
    ("hb_poly", brep_polygonal::poly_case),
    ("ht_outliner", topo_outliner::outliner_case),
    ("hb_reflect", brep_reflection::reflect_case),
    ("hb_edgeface", brep_edgeface::edgeface_case),
    ("hb_intersector", brep_intersector::intersector_case),
    ("hb_edgedata", brep_edge_record::edgedata_case),
    ("hb_facedata", brep_face_record::facedata_case),
    ("hb_surface", brep_surface::surface_case),
    ("hb_cinter", brep_cinter::cinter_case),
    ("hb_curve", brep_curve::curve_case),
    ("ct_hcont_arc", hcont_arc::arc_case),
    ("ct_sampling", sampling::sampling_case),
    ("ct_function", function::function_case),
    ("ct_arc_function", arc_function::arc_function_case),
    ("ct_surface", surface::properties_case),
    ("ct_hcurve", hcurve::hc2d_case),
    ("ct_search_records", search_records::records_case),
    ("ct_inside", search_inside::inside_case),
    ("ct_boundary", search_boundary::boundary_case),
    ("ct_walking", walking::walking_case),
    ("ct_iwline", iwline::iwline_case),
    ("ct_point", contap::point_case),
    ("ct_line", contap::line_case),
    ("ct_analytic", contana::analytic_case),
    ("hl_frame", frame::frame_case),
    ("ha_minmax", hlr_algo::minmax_case),
    ("ha_encode", hlr_algo::encode_case),
    ("ha_bipoint", hlr_algo::bipoint_case),
    ("ha_edges", hlr_algo::edges_case),
    ("ha_wires", hlr_algo::wires_case),
    ("ha_status", hlr_algo::status_case),
    ("ha_records", hlr_algo::records_case),
    ("ha_projector", hlr_algo_projector::projector_case),
    ("ha_polydata", hlr_algo_poly::polydata_case),
    ("ha_polyalgo", hlr_algo_poly::polyalgo_case),
    ("ha_internal", hlr_algo_internal::internal_case),
    ("ha_grow", hlr_algo_internal::grow_case),
];

fn dispatch(operation: &str, values: &[f64]) -> Option<String> {
    let (_, execute) = REPLAY_OPERATIONS
        .iter()
        .find(|(name, _)| *name == operation)?;
    let mut tokens = sink::Sink::default();
    execute(&mut tokens, values);
    Some(tokens.finish())
}

// The old CLI table keeps its established function shape; algorithms only live above.
macro_rules! cli_adapter {
    ($($name:ident => $operation:literal),* $(,)?) => {$ (
        pub fn $name(values: &[f64]) {
            use std::io::Write;
            let output = dispatch($operation, values).expect("registered foundation operation");
            std::io::stdout().write_all(output.as_bytes()).expect("probe output");
        }
    )*};
}
cli_adapter! {
    interval_case => "iv_interval",
    edit_case => "iv_edit",
    relate_case => "iv_relate",
    set_case => "iv_set",
    value_case => "iv_value",
    interference_case => "tb_interference",
    transition_case => "tc_transition",
    algo_case => "hb_algo",
    hider_case => "hb_hider",
    data_case => "hb_data",
    poly_case => "hb_poly",
    outliner_case => "ht_outliner",
    reflect_case => "hb_reflect",
    brep_edgeface_case => "hb_edgeface",
    brep_intersector_case => "hb_intersector",
    brep_edgedata_case => "hb_edgedata",
    brep_facedata_case => "hb_facedata",
    brep_surface_case => "hb_surface",
    brep_cinter_case => "hb_cinter",
    brep_curve_case => "hb_curve",
    hcont_arc_case => "ct_hcont_arc",
    sampling_case => "ct_sampling",
    function_case => "ct_function",
    arc_function_case => "ct_arc_function",
    properties_case => "ct_surface",
    hc2d_case => "ct_hcurve",
    search_records_case => "ct_search_records",
    inside_case => "ct_inside",
    boundary_case => "ct_boundary",
    walking_case => "ct_walking",
    iwline_case => "ct_iwline",
    point_case => "ct_point",
    line_case => "ct_line",
    analytic_case => "ct_analytic",
    frame_case => "hl_frame",
    minmax_case => "ha_minmax",
    encode_case => "ha_encode",
    bipoint_case => "ha_bipoint",
    edges_case => "ha_edges",
    wires_case => "ha_wires",
    status_case => "ha_status",
    records_case => "ha_records",
    projector_case => "ha_projector",
    polydata_case => "ha_polydata",
    polyalgo_case => "ha_polyalgo",
    internal_case => "ha_internal",
    grow_case => "ha_grow",
}

/// Retain the legacy walking report's exact bytes through the owned snapshot.
pub fn walking_line_snapshot(
    line: &martensite_tkhlr::contap::ContapTheIWLineOfTheIWalking,
) -> String {
    let mut tokens = sink::Sink::default();
    iwline::line_report(&mut tokens, line);
    tokens.finish()
}
