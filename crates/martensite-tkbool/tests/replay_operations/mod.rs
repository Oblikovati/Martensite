//! The martensite-tkbool probe operations, one module per recorded replay file under the recording tool
pub mod brep_algo;
pub mod brep_fill_advanced_evolved;
pub mod brep_fill_compatible;
pub mod brep_fill_draft;
pub mod brep_fill_filling;
pub mod brep_fill_generator;
pub mod brep_fill_location_law;
pub mod brep_fill_multi_line;
pub mod brep_fill_offset_wire;
pub mod brep_fill_package;
pub mod brep_fill_pipe;
pub mod brep_fill_pipe_shell;
pub mod brep_fill_section;
pub mod brep_fill_section_law;
pub mod brep_fill_sweep;
pub mod brep_proj;
pub mod curve_explorer;
pub mod interference_tool;
pub mod point_explorer;
pub mod top_ope_brep_build;
pub mod top_ope_brep_build_tools;
pub mod top_ope_brep_ds;
pub mod top_ope_brep_ds_build;
pub mod top_ope_brep_ds_ops;
pub mod top_ope_brep_tool_classifier;
pub mod top_ope_brep_tool_curve_tool;
pub mod top_ope_brep_tool_regu;
pub mod top_ope_brep_tool_shapes;
pub mod top_ope_brep_tool_tool;
pub mod top_ope_brep_tool_topology;

// Package-owned copies preserve the established numeric protocol. Their immutable
// original hashes are retained in the package closure evidence; they must no longer
// depend on sibling toolkit example trees during packaging.
#[allow(dead_code)]
pub mod brep_lib_common;
#[allow(dead_code, unused_imports)]
pub mod common;
#[allow(dead_code)]
pub mod print;
#[allow(dead_code)]
pub mod shape;

// The face and shape program readers of the martensite-tktopalgo probe (tktopalgo_probe_face and
// tktopalgo_probe_shapes natively), which build the sections.
#[allow(dead_code)]
pub mod face;
#[allow(dead_code)]
pub mod shapes;
#[allow(dead_code)]
pub mod shapes_geometry;

pub mod brep_fill_evolved;

pub mod evolved_digest;

pub mod buffered_tokens;

pub mod cases;
