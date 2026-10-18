//! Shared operation implementation for in-process fixtures and development CLI.
macro_rules! replay_print {
    ($output:expr, $($token:tt)*) => { $output.emit(format_args!($($token)*)) };
}
macro_rules! replay_println {
    ($output:expr) => { $output.emit(format_args!("\n")) };
    ($output:expr, $($token:tt)*) => { $output.emit(format_args!("{}\n", format_args!($($token)*))) };
}
pub mod adaptor3d;
pub mod adaptor3d_inputs;
pub mod advapprox;
pub mod bezier_surface;
pub mod bspl;
pub mod bspline_surface;
pub mod common;
pub mod conics;
pub mod curves;
pub mod eval_bezier_surfaces;
pub mod eval_curves;
pub mod eval_surfaces;
pub mod geom_adaptor;
pub mod geom_adaptor_cases;
pub mod grid;
pub mod grid_surfaces;
pub mod hash;
pub mod offset_surface;
pub mod print;
pub mod shapes;
pub mod simple;
pub mod surfaces;
pub mod swept;
pub mod top_abs;
pub mod topology;

pub mod dispatch;
pub mod output;
pub mod reader;
