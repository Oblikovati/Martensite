//! IGES exchange: the data structures, the file parser and writer, the configuration values.
pub mod brep_to_iges;
pub mod brep_to_iges_brep;
pub mod de_iges;
pub mod geom2d_to_iges;
pub mod geom_to_iges;
pub mod iges_appli;
pub mod iges_basic;
pub mod iges_control;
pub mod iges_conv_geom;
pub mod iges_data;
pub mod iges_defs;
pub mod iges_dimen;
pub mod iges_draw;
pub mod iges_file;
pub mod iges_geom;
pub mod iges_graph;
pub mod iges_package_tables;
pub mod iges_select;
pub(crate) mod iges_shared_blocks;
pub mod iges_solid;
#[cfg(test)]
mod iges_test_samples;
pub mod iges_to_brep;
pub mod iges_tool_support;
pub mod igescaf_control;
