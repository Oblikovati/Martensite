//! Source input programs; the original values are expectations, never conversion results.
#[cfg(test)]
mod cpu;
mod grid;
mod grid_regular_control;
mod selection;
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
pub(crate) use grid::{pixel_grid, shifted_grid};
use martensite_tkernel::standard::Failure;
pub(crate) use selection::selected_walk;
type Outcome = Result<(), Box<dyn std::error::Error>>;

fn report_grid(label: &str, actual: (f64, f64, f64), expected: [f64; 3], named_axes: bool) {
    for ((name, value), wanted) in ["X", "Y", "Z"]
        .into_iter()
        .zip([actual.0, actual.1, actual.2])
        .zip(expected)
    {
        if !value.is_finite() {
            println!("Error: {label}, {name} = {value}; expected finite {wanted}");
            continue;
        }
        let actual_label = grid_check_name(label, name, named_axes);
        crate::shape_mesh::check_real(&actual_label, value, wanted, 1e-7, 0.);
    }
}

fn grid_check_name(label: &str, axis: &str, named_axes: bool) -> String {
    if named_axes {
        return format!("{label}, {axis}");
    }
    label.to_owned()
}
