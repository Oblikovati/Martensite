//! Reuse complete checked view programs; expected failures remain mandatory diagnostics.
#[path = "../modalg_display_cases/result_view.rs"]
mod result_view;
#[allow(dead_code)]
#[path = "../modalg_display_cases/triangulated.rs"]
mod triangulated;
use result_view::display_result_view;
mod prelude {
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::dataset_shape::read_dataset_shape;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use crate::shape_checks::{MassCheck, MassKind, check_mass};
    pub(super) use crate::shape_report_checks::check_shape_structure;
    pub(super) use martensite_tkbrep::topods::Shape;
    pub(super) use std::error::Error;
    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}
use crate::shape_mesh::CountExpectation;
use prelude::*;
use triangulated::ShownMeshCase;
#[cfg(test)]
mod cpu;

const fn exact_count(value: f64) -> CountExpectation {
    CountExpectation::Near {
        value,
        tolerance_absolute: 0.,
        tolerance_relative: 0.,
    }
}

pub(super) const SHADED: ShownMeshCase = ShownMeshCase {
    heading: &[
        "TODO OCC12345 ALL: Error: Number of triangles",
        "TODO OCC12345 ALL: Error: Number of nodes",
        "================",
        "BUC61057",
        "OCC109",
        "================",
        "",
    ],
    input: "OCC109.brep",
    triangles: exact_count(294.),
    nodes: exact_count(300.),
    area: 33.8757,
    image: "bug109.png",
};
pub(super) const CLEARED: ShownMeshCase = ShownMeshCase {
    heading: &[
        "TODO OCC12345 ALL: Error : The area of result shape is",
        "TODO OCC12345 ALL: Error: Number of triangles",
        "TODO OCC12345 ALL: Error: Number of nodes",
        "========",
        "OCC264",
        " (case 1)",
        "========",
    ],
    input: "OCC264_01.brep",
    triangles: exact_count(26.),
    nodes: exact_count(28.),
    area: 0.,
    image: "bug264_1.png",
};
pub(super) const INCREMENTAL: ShownMeshCase = ShownMeshCase {
    heading: &[
        "TODO OCC12345 ALL: Error : The area of result shape is",
        "========================",
        " OCC287 ",
        "========================",
    ],
    input: "shading_170.brep",
    triangles: CountExpectation::Not(1315.),
    nodes: CountExpectation::Not(784.),
    area: 0.,
    image: "bug287.png",
};

/// Preserve shaded display, fit, source counts/area and unchanged screenshot.
/// Example: `shaded_count_regression()?`.
pub(crate) fn shaded_count_regression() -> Outcome {
    triangulated::shaded_screenshot(&SHADED)
}

/// Keep display/clear, authored count failures, zero-area failure and the view check.
/// Example: `cleared_count_regression()?`.
pub(crate) fn cleared_count_regression() -> Outcome {
    triangulated::cleared_wireframe_view(&CLEARED)
}

/// Preserve actual incremental mesh, negated counts, area failure and second validity check.
/// Example: `incremental_count_regression()?`.
pub(crate) fn incremental_count_regression() -> Outcome {
    triangulated::incremental_mesh_view(&INCREMENTAL)
}
