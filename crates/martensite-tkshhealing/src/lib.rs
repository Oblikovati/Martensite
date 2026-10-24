//! Shape healing for boundary representations.
//!
//! The toolkit analyzes and repairs shapes, converts and divides their geometry,
//! and applies operator sequences to imported shapes. Its public modules separate
//! analysis, construction, repair, geometry conversion and process orchestration.
pub mod sh_message;
pub mod shape_algo;
pub mod shape_analysis;
pub mod shape_build;
pub mod shape_construct;
pub mod shape_custom;
pub mod shape_extend;
pub mod shape_fix;
pub mod shape_process;
pub mod shape_process_api;
pub mod shape_upgrade;
