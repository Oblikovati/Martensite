//! Meshing regressions of the bugs and perf mesh groups: shapes meshed incrementally or by
//! their shaded presentation, measured, checked for mesh topology and dumped.
pub(crate) mod meshed;
pub(crate) mod perf;
mod shown;
pub(crate) use crate::session_run::banner;
pub(crate) use meshed::{
    cone_relative_mesh, cut_cone_face_wireframe, fixed_face_front, hollow_cut_sphere,
    loopback_face_mesh, remeshed_sphere_time, spline_cavity_mesh, terrain_free_mesh,
};
pub(crate) use perf::{
    assembly_mesh_counts, display_mesh_counts, meshed_shaded, meshed_shaded_counts, revolved_meshes,
};
pub(crate) use prelude::counts_deflection;
pub(crate) use shown::{
    CleanedMesh, cleaned_mesh_shaded, coarse_edges_remeshed, deviation_coefficient_top,
    mesh_faulties, shaded_after_tricheck, side_views_of_thin_cone,
};

/// What every mesh view program uses.
pub(crate) mod prelude {
    pub(super) use crate::case_view::viewer_session::{DisplayRequest, ViewParameters};
    pub(super) use crate::dataset_shape::read_dataset_shape;
    pub(super) use crate::mesh_topology_check::report_mesh_validity;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use crate::shape_mesh::{
        CountExpectation, MeshRequest, TriangulationExpectation, check_triangulation, mesh_shape,
        remove_triangulation,
    };
    pub(super) use crate::shape_view_checks::checkview_3d;
    pub(super) use martensite_tkbrep::topods::Shape;
    pub(super) use std::error::Error;

    /// Triangle and node counts and a deflection within an absolute tolerance
    /// (`checktrinfo s -tri T -nod N -defl D -tol_abs_defl tol`).
    pub(crate) fn counts_deflection(
        triangles: f64,
        nodes: f64,
        deflection: f64,
        tolerance: f64,
    ) -> TriangulationExpectation {
        TriangulationExpectation {
            triangles: CountExpectation::exactly(triangles),
            nodes: CountExpectation::exactly(nodes),
            deflection: CountExpectation::Near {
                value: deflection,
                tolerance_absolute: tolerance,
                tolerance_relative: 0.,
            },
            ..TriangulationExpectation::default()
        }
    }

    /// Exact triangle and node counts (`checktrinfo s -tri T -nod N`).
    pub(super) fn counts(triangles: f64, nodes: f64) -> TriangulationExpectation {
        TriangulationExpectation {
            triangles: CountExpectation::exactly(triangles),
            nodes: CountExpectation::exactly(nodes),
            ..TriangulationExpectation::default()
        }
    }

    /// `tclean` then `incmesh` with `request`; `remeshed(&s, MeshRequest::new(0.1))?`.
    pub(super) fn remeshed(shape: &Shape, request: MeshRequest) -> Result<(), Box<dyn Error>> {
        remove_triangulation(shape)?;
        mesh_shape(shape, &request)?;
        Ok(())
    }

    /// A new view showing `shape` shaded (global display mode 1), optionally only from its
    /// stored triangulation, fitted; `shaded_view(&s, false)?`.
    pub(super) fn shaded_view(
        shape: &Shape,
        auto_triangulation: bool,
    ) -> Result<SessionRun, Box<dyn Error>> {
        let mut run = SessionRun::new()?;
        run.session().set_display_mode(1)?;
        run.session().set_auto_triangulation(auto_triangulation);
        run.session()
            .display("result", shape, &DisplayRequest::default())?;
        run.session().fit()?;
        Ok(run)
    }
}
