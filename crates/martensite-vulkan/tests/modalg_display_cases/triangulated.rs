//! Dataset shapes whose triangulation, computed by the viewer or by the incremental mesher,
//! is counted before the shape is shown (OCC109, OCC263, OCC264, OCC287, OCC292).
use super::display_result_view;
use super::prelude::*;
use crate::shape_mesh::{
    CountExpectation, MeshRequest, TriangulationExpectation, check_triangulation, mesh_shape,
    remove_triangulation,
};

use martensite_geometry_raster::{TopologyDrawable, TopologyStyle};
use martensite_raster_core::{ProcessReports, RasterReports};
use std::rc::Rc;

/// One shown-mesh regression: heading, dataset input, expected triangulation and area.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ShownMeshCase {
    /// The banner lines.
    pub heading: &'static [&'static str],
    /// The dataset file restored as `result`.
    pub input: &'static str,
    /// The expected triangle count.
    pub triangles: CountExpectation,
    /// The expected node count.
    pub nodes: CountExpectation,
    /// The expected area.
    pub area: f64,
    /// The image dumped.
    pub image: &'static str,
}

impl ShownMeshCase {
    fn check_mesh_and_area(&self, result: &Shape) -> Result<(), Box<dyn Error>> {
        let expected = TriangulationExpectation {
            triangles: self.triangles,
            nodes: self.nodes,
            ..TriangulationExpectation::default()
        };
        check_triangulation(result, &expected)?;
        check_mass(result, &MassCheck::near(MassKind::Area, self.area))?;
        Ok(())
    }

    /// Retain the original named drawing object alongside its shared topology.
    /// Example: `let (shape, drawing) = case.restored_with_drawing()?;`.
    pub(super) fn restored_with_drawing(
        &self,
    ) -> Result<(Shape, TopologyDrawable), Box<dyn Error>> {
        let shape = self.restored()?;
        let drawing = retained_mesh_drawing(&shape, Rc::new(ProcessReports));
        Ok((shape, drawing))
    }

    fn restored(&self) -> Result<Shape, Box<dyn Error>> {
        banner(self.heading);
        Ok(read_dataset_shape(self.input)?)
    }
}

/// Check and clean the shape, display it as wireframe and clear the view, count its
/// triangulation and area, then show it; `cleared_wireframe_view(&case)?`.
pub(crate) fn cleared_wireframe_view(case: &ShownMeshCase) -> Outcome {
    let (result, drawing) = case.restored_with_drawing()?;
    check_shape_structure(&result)?;
    remove_triangulation(&result)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().clear()?;
    drawing.rendering().change_nb_isos(0, &ProcessReports)?;
    drawing
        .rendering()
        .set_display_triangulation(!drawing.rendering().display_triangulation());
    case.check_mesh_and_area(&result)?;
    // The result check opens another view after the earlier view, display and clear.
    run.session().open_view(409, 409)?;
    display_result_view(&mut run, &result, case.image)?;
    run.finish()
}

/// Check and clean the shape, mesh it at 0.1 deflection, count its triangulation and area,
/// check it again, then show it; `incremental_mesh_view(&case)?`.
pub(crate) fn incremental_mesh_view(case: &ShownMeshCase) -> Outcome {
    let (result, drawing) = case.restored_with_drawing()?;
    check_shape_structure(&result)?;
    remove_triangulation(&result)?;
    drawing.rendering().change_nb_isos(0, &ProcessReports)?;
    mesh_shape(&result, &MeshRequest::new(0.1))?;
    drawing
        .rendering()
        .set_display_triangulation(!drawing.rendering().display_triangulation());
    case.check_mesh_and_area(&result)?;
    check_shape_structure(&result)?;
    let mut run = SessionRun::new()?;
    display_result_view(&mut run, &result, case.image)?;
    run.finish()
}

/// Check and clean the shape, display it shaded, count its triangulation and area, then
/// show it; `shaded_object_view(&case)?`.
pub(crate) fn shaded_object_view(case: &ShownMeshCase) -> Outcome {
    let result = case.restored()?;
    check_shape_structure(&result)?;
    remove_triangulation(&result)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_object_display_mode("result", 1)?;
    case.check_mesh_and_area(&result)?;
    display_result_view(&mut run, &result, case.image)?;
    run.finish()
}

/// Check the shape, clean it, display it shaded and fitted, count its triangulation and
/// area, then dump the view as it is; `shaded_screenshot(&case)?`.
pub(crate) fn shaded_screenshot(case: &ShownMeshCase) -> Outcome {
    let result = case.restored()?;
    check_shape_structure(&result)?;
    let mut run = SessionRun::new()?;
    remove_triangulation(&result)?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_object_display_mode("result", 1)?;
    run.session().fit()?;
    case.check_mesh_and_area(&result)?;
    run.dump(case.image)?;
    run.finish()
}

/// Keep the same topology and fresh named-shape defaults without a command session.
/// Example: `let drawing = retained_mesh_drawing(&shape, reports);`.
pub(super) fn retained_mesh_drawing(
    shape: &Shape,
    reports: Rc<dyn RasterReports>,
) -> TopologyDrawable {
    TopologyDrawable::new(shape.clone(), TopologyStyle::default(), reports)
}
