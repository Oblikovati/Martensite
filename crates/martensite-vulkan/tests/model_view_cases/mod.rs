//! Regressions that restore or build a shape, mesh, heal or measure it, and show it in a view.
mod booleans;
mod checked;
mod datum_objects;
mod geometry_faces;
mod meshed;
mod presented;
mod spline_faces;
mod unified;
mod v3d_presented;
pub(crate) use booleans::{
    BooleanPath, Occ3721, fused_vertex_position, occ3721_view, prepared_face_fuse,
    surface_face_meshed, surface_faces_section,
};
pub(crate) use checked::{
    area_with_empty_view, bad_triangulation_shaded, green_box_color, healed_shape_area,
    same_parameter_edge_length, scaled_face_mesh, self_intersecting_open_wire,
};
pub(crate) use datum_objects::{
    erased_points_circle, erased_points_plane, erased_points_segment, erased_points_triangle,
};
pub(crate) use geometry_faces::rail_pipe_faces;
pub(crate) use meshed::{
    bspline_face_bounded, bspline_face_deflection, fillet_shape_shaded, forced_coarser_mesh,
    incmesh_shaded_counts, long_bspline_face_bounded, planar_face_zero_deflection,
    seam_edges_split,
};
pub(crate) use presented::{
    invisible_face_shaded, own_shaded_not_faulty, own_shaded_present, parabola_face_shaded,
    restored_shaded_counts, shading_face_143, shading_face_144, shifted_wire_shaded,
    unfitted_own_shaded,
};
pub(crate) use unified::{UnifyInput, unified_face_history, unified_view};
pub(crate) use v3d_presented::{
    bspline_sphere_shaded, cleaned_after_display, face_counts_shaded, notched_face_qualities,
    thin_ring_top, v3d_tcleaned_shaded,
};

/// What every model view program uses.
mod prelude {
    pub(super) use super::spline_faces::{LONG_SPLINE, NARROW_SPLINE, SplineSurface, bspline_face};
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::dataset_shape::read_dataset_shape;
    pub(super) use crate::primitive_shapes::sphere;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use crate::shape_checks::{MassCheck, MassKind, check_mass, check_shape_valid};
    pub(super) use crate::shape_mesh::{
        CountExpectation, MeshRequest, TriangulationExpectation, check_triangulation, mesh_shape,
        remove_geometry_only, remove_triangulation,
    };
    pub(super) use crate::shape_view_checks::{
        check_sub_shape_counts, checkview_3d, checkview_in, finish_without_view,
        report_alone_vertices,
    };
    pub(super) use martensite_tkbrep::topods::Shape;
    pub(super) use martensite_tkv3d::v3d::V3dTypeOfOrientation;
    pub(super) use std::error::Error;

    /// The top view orientation (+Z towards the eye).
    pub(super) const TOP: V3dTypeOfOrientation = V3dTypeOfOrientation::ZUP_TOP;

    /// Triangles that must be present (`checktrinfo s -tri`).
    pub(super) const PRESENT_TRIANGLES: TriangulationExpectation = TriangulationExpectation {
        faces: CountExpectation::Any,
        triangles: CountExpectation::Present,
        nodes: CountExpectation::Any,
        deflection: CountExpectation::Any,
        max_deflection: None,
    };

    /// Triangle and node counts that must both be present (`checktrinfo s -tri -nod`).
    pub(super) const PRESENT_TRIANGLES_NODES: TriangulationExpectation = TriangulationExpectation {
        nodes: CountExpectation::Present,
        ..PRESENT_TRIANGLES
    };

    /// A value within `absolute + relative * |value|`; `near(0.7, 1e-3, 0.01)`.
    pub(super) fn near(value: f64, absolute: f64, relative: f64) -> CountExpectation {
        CountExpectation::Near {
            value,
            tolerance_absolute: absolute,
            tolerance_relative: relative,
        }
    }

    /// Exact triangle and node counts (`checktrinfo s -tri 10 -nod 12`).
    pub(super) fn exact_counts(triangles: f64, nodes: f64) -> TriangulationExpectation {
        TriangulationExpectation {
            triangles: CountExpectation::exactly(triangles),
            nodes: CountExpectation::exactly(nodes),
            ..TriangulationExpectation::default()
        }
    }

    /// Display `shape` as `name` with the context's display mode; `show(&mut run, "a", &a)?`.
    pub(super) fn show(
        run: &mut SessionRun,
        name: &str,
        shape: &Shape,
    ) -> Result<(), Box<dyn Error>> {
        run.session()
            .display(name, shape, &DisplayRequest::default())?;
        Ok(())
    }
}
