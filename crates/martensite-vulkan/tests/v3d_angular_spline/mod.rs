#[cfg(test)]
mod cpu;
mod geometry;
mod surface;
use crate::case_view::viewer_session::{DisplayRequest, ViewParameters};
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{
    CountExpectation, MeshRequest, TriangulationExpectation, check_triangulation, mesh_shape,
};
use martensite_tkbrep::topods::Shape;
type BoxError = Box<dyn std::error::Error>;
const CAMERA: ViewParameters = ViewParameters {
    scale: Some(335.304),
    eye: Some([1.95918, 0.124681, 5.05411]),
    at: Some([0.0736354, 0.0378839, 1.46922]),
    up: Some([-0.748808, 0.542534, 0.380715]),
    proj: Some([0.4654, 0.0214236, 0.884841]),
};
const HEADING: &[&str] = &[
    "=======",
    "0030008: BRepMesh does not respect angular deflection in internal area of bspline surface",
    "=======",
    "",
];

fn case_angular_mesh_checks(shape: &Shape) -> Result<(), BoxError> {
    crate::mesh_topology_check::report_mesh_validity(shape)?;
    check_triangulation(
        shape,
        &TriangulationExpectation {
            triangles: CountExpectation::exactly(193.),
            nodes: CountExpectation::exactly(147.),
            deflection: CountExpectation::Near {
                value: 0.052300780129031083,
                tolerance_absolute: 1e-6,
                tolerance_relative: 0.,
            },
            ..TriangulationExpectation::default()
        },
    )?;
    Ok(())
}

fn original_mesh(shape: &Shape) -> Result<(), BoxError> {
    mesh_shape(shape, &MeshRequest::new(0.15).with_angle_degrees(20.))?;
    case_angular_mesh_checks(shape)
}

/// Preserve actual half-space cut, face copy/scale/conversion and all authored mesh checks.
/// Example: `angular_spline_view()?`.
pub(crate) fn angular_spline_view() -> Result<(), BoxError> {
    banner(HEADING);
    let result = geometry::actual_scaled_cut_face()?;
    original_mesh(&result)?;
    let mut run = SessionRun::new()?;
    run.session().set_auto_triangulation(false);
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    run.session().set_view_parameters(&CAMERA)?;
    run.dump("bug30008_2.png")?;
    run.finish()
}
