//! Independent typed contracts cover group aspects, storage and all selection branches.
use super::*;
use crate::v3d::V3dViewer;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::{Graphic3dArrayOfPoints, Graphic3dStructure};
use martensite_tkservice::null_driver::NullGraphicDriver;

fn points() -> Handle<PrimitiveArrayPresentation> {
    let array = Handle::new(Graphic3dArrayOfPoints::new(2, 0).unwrap());
    array.add_vertex_pnt(&Point3::new(1., 2., 3.)).unwrap();
    array.add_vertex_pnt(&Point3::new(4., 5., 6.)).unwrap();
    PrimitiveArrayPresentation::from_array(array.cast())
}

fn presentation() -> Handle<Graphic3dStructure> {
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    Graphic3dStructure::new(&viewer.structure_manager(), &Handle::default()).unwrap()
}

#[test]
fn runtime_identity_and_display_mode_remain_compatible() {
    let object = points();
    assert_eq!(object.dynamic_type().unwrap().name(), "MyPArrayObject");
    for mode in [-1, 0, 1] {
        assert_eq!(object.accept_display_mode(mode), mode == 0);
    }
}

#[test]
fn null_arrays_and_unsupported_modes_create_no_groups_or_selection() {
    let object = PrimitiveArrayPresentation::from_array(Handle::default());
    let prs = presentation();
    object.compute(&Handle::default(), &prs.cast(), 0).unwrap();
    points()
        .compute(&Handle::default(), &prs.cast(), 1)
        .unwrap();
    assert!(prs.groups().is_empty());
    let selection = Handle::new(SelectMgrSelection::new(0));
    object.compute_selection(&selection, 0).unwrap();
    points().compute_selection(&selection, 1).unwrap();
    assert!(selection.entities().is_empty());
}

#[test]
fn marker_compute_retains_actual_marker_aspect_and_one_group() {
    let object = points();
    let marker = Handle::new(Graphic3dAspectMarker3d::default());
    object.set_marker_aspect(&marker);
    let prs = presentation();
    object.compute(&Handle::default(), &prs.cast(), 0).unwrap();
    assert_eq!(prs.groups().len(), 1);
    assert_eq!(
        prs.groups()[0].aspects().object_address(),
        marker.object_address()
    );
}

#[test]
fn colored_material_compute_uses_white_plastified_aspect() {
    let object = points();
    object.setup_vertex_color_shading().unwrap();
    let prs = presentation();
    object.compute(&Handle::default(), &prs.cast(), 0).unwrap();
    let aspect = prs.groups()[0].aspects();
    assert_eq!(aspect.interior_color(), Color::named(NamedColor::WHITE));
    let material = aspect.front_material();
    assert_eq!(
        material.ambient_color(),
        Color::from_vec3(Vec3::new(0.5, 0.5, 0.5)).unwrap()
    );
    assert_eq!(material.diffuse_color(), material.ambient_color());
    assert_eq!(material.specular_color(), Color::named(NamedColor::BLACK));
    assert_eq!(material.emissive_color(), Color::named(NamedColor::BLACK));
}

#[test]
fn points_selection_retains_factor_eight_and_owner_identity() {
    let object = points();
    let selection = Handle::new(SelectMgrSelection::new(0));
    object.compute_selection(&selection, 0).unwrap();
    assert_eq!(selection.entities().len(), 1);
    let sensitive = selection.entities()[0].base_sensitive();
    assert_eq!(sensitive.sensitivity_factor(), 8);
    assert_eq!(
        sensitive.owner_id().selectable().object_address(),
        object.object_address()
    );
}

fn segment_object(indexed: bool) -> Handle<PrimitiveArrayPresentation> {
    let array =
        Handle::new(Graphic3dArrayOfSegments::new(4, if indexed { 2 } else { 0 }, 0).unwrap());
    for x in [1., 2., 3., 4.] {
        array.add_vertex_pnt(&Point3::new(x, 0., 0.)).unwrap();
    }
    if indexed {
        array.add_edge(3).unwrap();
        array.add_edge(4).unwrap();
    }
    PrimitiveArrayPresentation::from_array(array.cast())
}

#[test]
fn segments_selection_retains_indexed_and_sequential_endpoints() {
    for (indexed, count, start) in [(false, 2, 1.), (true, 1, 3.)] {
        let object = segment_object(indexed);
        let selection = Handle::new(SelectMgrSelection::new(0));
        object.compute_selection(&selection, 0).unwrap();
        assert_eq!(selection.entities().len(), count);
        let segment = selection.entities()[0]
            .base_sensitive()
            .cast::<Select3DSensitiveSegment>();
        assert_eq!(segment.start_point(), Point3::new(start, 0., 0.));
        assert_eq!(segment.end_point(), Point3::new(start + 1., 0., 0.));
        assert_eq!(segment.sensitivity_factor(), 4);
    }
}
