use super::*;
use martensite_tkmath::gp::{Axis3, Cone3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

pub(super) fn cone_face() -> Result<Shape, Failure> {
    let cone = Cone3::new(Axis3::default(), std::f64::consts::FRAC_PI_4, 0.)?;
    let mut maker =
        BRepBuilderAPIMakeFace::from_cone_bounds(cone, (0., std::f64::consts::TAU, 0., 10.))?;
    Ok(maker.shape()?.clone())
}

fn unchanged_mesh(shape: &Shape, baseline: &TriangulationSummary) -> Result<(), Failure> {
    let expected = TriangulationExpectation {
        deflection: CountExpectation::Near {
            value: baseline.max_deflection,
            tolerance_absolute: 1e-7,
            tolerance_relative: 0.,
        },
        ..count_expectation(baseline)
    };
    check_triangulation(shape, &expected)
}

/// The fine original mesh and its copied mesh survive later coarser requests.
/// Example: `retained_conical_mesh()?`.
pub(crate) fn retained_conical_mesh() -> Outcome {
    banner(&["============", "OCC26897", "============", ""]);
    let original = cone_face()?;
    mesh_shape(&original, &MeshRequest::new(0.1))?;
    mesh_information::print_information(&original)?;
    let baseline = TriangulationSummary::of(&original)?;
    let copy = copied_shape(&original, true)?;
    mesh_shape(&original, &MeshRequest::new(1.))?;
    unchanged_mesh(&original, &baseline)?;
    unchanged_mesh(&copy, &baseline)?;
    mesh_shape(&copy, &MeshRequest::new(1.))?;
    unchanged_mesh(&copy, &baseline)?;
    crate::shape_view_checks::checkview_3d(&copy, "bug26897.png")
}
