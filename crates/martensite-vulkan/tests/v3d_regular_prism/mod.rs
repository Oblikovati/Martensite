#[cfg(test)]
mod cpu;
mod geometry;
use crate::case_view::viewer_session::DisplayRequest;
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{CountExpectation, TriangulationExpectation, check_triangulation};
use martensite_tkbrep::topods::Shape;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
type BoxError = Box<dyn std::error::Error>;
const HEADING: &[&str] = &[
    "========",
    "0032422: Mesh - Weird rendering",
    "========",
    "",
];

fn authored_counts(shape: &Shape, triangles: f64, nodes: f64) -> Result<(), BoxError> {
    check_triangulation(
        shape,
        &TriangulationExpectation {
            triangles: CountExpectation::exactly(triangles),
            nodes: CountExpectation::exactly(nodes),
            ..TriangulationExpectation::default()
        },
    )?;
    Ok(())
}

fn show_regular_face(run: &mut SessionRun, shape: &Shape) -> Result<(), BoxError> {
    run.session()
        .display("r", shape, &DisplayRequest::default().with_display_mode(1))?;
    run.session()
        .set_material("r", Graphic3dNameOfMaterial::Stone)?;
    run.session().set_color(
        "r",
        martensite_tkernel::quantity::Color::named(martensite_tkernel::quantity::NamedColor::GREEN),
    )?;
    run.session().set_location("r", [0., 0., -100.])?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_LEFT)?;
    run.session().fit()?;
    Ok(())
}

/// Preserve auto-meshed source prism and rebuilt regular50×50 face with all counts.
/// Example: `regular_prism_view()?`.
pub(crate) fn regular_prism_view() -> Result<(), BoxError> {
    banner(HEADING);
    let result = geometry::actual_prism()?;
    crate::shape_checks::check_shape_valid(&result)?;
    let mut run = SessionRun::new()?;
    run.session().display(
        "result",
        &result,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    run.session().fit()?;
    authored_counts(&result, 1286., 863.)?;
    let regular = geometry::tessellated_first_face(&result)?;
    show_regular_face(&mut run, &regular)?;
    authored_counts(&regular, 5000., 2601.)?;
    run.dump("bug32422.png")?;
    run.finish()
}
