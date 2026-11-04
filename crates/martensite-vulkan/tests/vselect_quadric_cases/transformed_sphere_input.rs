//! Isolate the source's explicit display mesh without changing its analytic sphere branch.
use super::transformed_input::scaled_location;
use crate::{
    primitive_shapes::{rotated_about, sphere, translated},
    shape_mesh::{MeshRequest, mesh_shape, remove_geometry_only},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};

/// Preserve source `fscale`, rotation and translation as locations; `transformed_sphere_source()?`.
pub(super) fn transformed_sphere_source() -> Result<Shape, Failure> {
    let scaled = scaled_location(&sphere(10.)?, 0.1)?;
    let rotated = rotated_about(&scaled, [0.; 3], [1., 1., 0.], 25.)?;
    translated(&rotated, [2500., 3500., 1000.])
}

/// Match tcopy's new topology/geometry without copying mesh; `isolated_sphere_copy(&source)?`.
fn isolated_sphere_copy(source: &Shape) -> Result<Shape, Failure> {
    let mut copier = BRepBuilderAPICopy::with_shape(source, true, false)?;
    let copy = copier.shape()?.clone();
    Ok(copy)
}

/// Only the isolated copy loses geometry after the original incmesh1 request.
/// Example: `let display_mesh = triangulated_sphere_copy(&source)?;`.
pub(super) fn triangulated_sphere_copy(source: &Shape) -> Result<Shape, Failure> {
    let copy = isolated_sphere_copy(source)?;
    mesh_shape(&copy, &MeshRequest::new(1.))?;
    remove_geometry_only(&copy)?;
    Ok(copy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape_mesh::TriangulationSummary;
    use martensite_tkbrep::{
        brep::BRepTool, brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods,
    };
    use martensite_tkernel::standard::Handle;
    use martensite_tkg3d::{
        adaptor3d::Adaptor3dSurface, geom::GeomSphericalSurface, top_abs::ShapeEnum,
    };
    use martensite_tkmath::{
        gp::Point3,
        poly::{Triangulation, mesh_purpose},
    };
    use martensite_tkmesh::imesh_tools::MeshParameters;
    use martensite_tkv3d::{
        select_mgr::{SelectMgrEntityOwner, SelectMgrEntityOwnerSource},
        select3d::Select3DSensitiveEntity,
        std_select::StdSelectBrepSelectionTool,
    };

    fn only_face(shape: &Shape) -> topods::Face {
        let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
        assert!(faces.more());
        let face = topods::face(faces.current()).unwrap();
        faces.next().unwrap();
        assert!(!faces.more());
        face
    }

    fn real_face_entities(shape: &Shape) -> Vec<Handle<dyn Select3DSensitiveEntity>> {
        let owner: Handle<dyn SelectMgrEntityOwnerSource> = SelectMgrEntityOwner::new(0).cast();
        let mut entities: Vec<Handle<dyn Select3DSensitiveEntity>> = Vec::new();
        assert!(
            StdSelectBrepSelectionTool::get_sensitive_for_face(
                &only_face(shape),
                &owner,
                &mut entities,
                false,
                9,
                500.,
                true,
            )
            .unwrap()
        );
        entities
    }

    fn selected_face_entity_type(entity: &Handle<dyn Select3DSensitiveEntity>) -> &'static str {
        let actual = entity
            .get()
            .expect("selected face entity=null; expected real selector entity");
        actual.dynamic_type().name()
    }

    #[test]
    fn transformed_source_keeps_geometry_radius_and_forced_location() {
        let source = transformed_sphere_source().unwrap();
        let face = only_face(&source);
        let (surface, location) = BRepTool::surface_with_location(&face).unwrap();
        assert_eq!(surface.cast::<GeomSphericalSurface>().radius(), 10.);
        assert!((location.transformation().scale_factor() - 0.1).abs() < 1e-14);
        let center = Point3::new(0., 0., 0.).transformed(location.transformation());
        assert!(center.distance(&Point3::new(2500., 3500., 1000.)) < 1e-9);
        let effective = BRepAdaptorSurface::with_face(&face, true)
            .unwrap()
            .sphere()
            .unwrap();
        assert!((effective.radius() - 1.).abs() < 1e-12);
    }

    #[test]
    fn copy_owns_distinct_topology_and_surface_without_a_mesh() {
        let source = transformed_sphere_source().unwrap();
        let copy = isolated_sphere_copy(&source).unwrap();
        let (original, _) = BRepTool::surface_with_location(&only_face(&source)).unwrap();
        let (copied, _) = BRepTool::surface_with_location(&only_face(&copy)).unwrap();
        assert!(!source.is_partner(&copy));
        assert!(!only_face(&source).is_partner(&only_face(&copy)));
        assert_ne!(original, copied);
        assert!(!original.is_null() && !copied.is_null());
        assert_eq!(TriangulationSummary::of(&source).unwrap().triangles, 0);
        assert_eq!(TriangulationSummary::of(&copy).unwrap().triangles, 0);
    }

    #[test]
    fn cleanup_retains_actual_mesh_and_does_not_erase_source_surface() {
        let source = transformed_sphere_source().unwrap();
        let copy = isolated_sphere_copy(&source).unwrap();
        mesh_shape(&copy, &MeshRequest::new(1.)).unwrap();
        let before = TriangulationSummary::of(&copy).unwrap();
        let (mesh, _) = BRepTool::triangulation(&only_face(&copy), mesh_purpose::NONE).unwrap();
        assert!(before.triangles > 0 && before.nodes > 0 && before.empty_faces == 0);
        remove_geometry_only(&copy).unwrap();
        assert_cleaned_copy_mesh(&copy, before, mesh);
        assert!(
            !BRepTool::surface_with_location(&only_face(&source))
                .unwrap()
                .0
                .is_null()
        );
        assert_eq!(TriangulationSummary::of(&source).unwrap().triangles, 0);
    }

    fn assert_cleaned_copy_mesh(
        copy: &Shape,
        before: TriangulationSummary,
        mesh: Handle<Triangulation>,
    ) {
        assert_eq!(before, TriangulationSummary::of(copy).unwrap());
        assert_eq!(
            mesh,
            BRepTool::triangulation(&only_face(copy), mesh_purpose::NONE)
                .unwrap()
                .0
        );
        assert!(
            BRepTool::surface_with_location(&only_face(copy))
                .unwrap()
                .0
                .is_null()
        );
    }

    #[test]
    fn actual_face_selection_distinguishes_cleaned_copy_and_analytic_source() {
        let source = transformed_sphere_source().unwrap();
        let copy = triangulated_sphere_copy(&source).unwrap();
        let triangulated = real_face_entities(&copy);
        let analytic = real_face_entities(&source);
        assert_eq!(triangulated.len(), 1);
        assert_eq!(analytic.len(), 1);
        assert_eq!(
            selected_face_entity_type(&triangulated[0]),
            "Select3DSensitiveTriangulation"
        );
        assert_eq!(
            selected_face_entity_type(&analytic[0]),
            "Select3DSensitiveSphere"
        );
    }

    #[test]
    fn source_mesh_request_keeps_native_absolute_serial_defaults() {
        let request = MeshRequest::new(1.);
        let defaults = MeshParameters::default();
        assert_eq!(request.deflection, 1.);
        assert_eq!(request.angle_degrees, None);
        assert!(!request.relative && !request.parallel && !request.decrease);
        assert_eq!(defaults.angle, 0.5);
        assert!(!defaults.relative && !defaults.in_parallel && !defaults.allow_quality_decrease);
    }
}
