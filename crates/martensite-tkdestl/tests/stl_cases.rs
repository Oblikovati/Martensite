//! STL regressions on real files: transfers, the checks made after them and the real views of
//! their results. A case that opens a view runs alone in a child process, as a viewer owns
//! process state.
#![cfg(target_os = "linux")]
mod support;

#[path = "stl_cases/checks.rs"]
mod checks;
#[path = "stl_cases/file_cases.rs"]
mod file_cases;
#[path = "stl_cases/mesh_view.rs"]
mod mesh_view;
#[path = "stl_cases/mesh_view_cases.rs"]
mod mesh_view_cases;
#[path = "stl_cases/raster_view.rs"]
mod raster_view;
#[path = "stl_cases/real_view.rs"]
mod real_view;
#[path = "stl_cases/shape_view_cases.rs"]
mod shape_view_cases;
#[path = "stl_cases/stl_data_source.rs"]
mod stl_data_source;
