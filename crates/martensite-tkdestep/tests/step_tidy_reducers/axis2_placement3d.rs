//! `StepTidyAxis2Placement3dReducer`.
use crate::step_tidy_fixture::{Fixture, ReplacedEntities, name, one_item, placement_select};
use martensite_tkdestep::step_data::StepDataSelectType;
use martensite_tkdestep::step_geom::{
    StepGeomAxis2Placement3d, StepGeomCircle, StepGeomConicalSurface, StepGeomCylindricalSurface,
    StepGeomEllipse, StepGeomPlane, StepGeomSphericalSurface, StepGeomToroidalSurface,
};
use martensite_tkdestep::step_repr::{
    StepReprConstructiveGeometryRepresentation, StepReprItemDefinedTransformation,
    StepReprRepresentationContext, StepReprRepresentationItem,
};
use martensite_tkdestep::step_shape::{
    StepShapeAdvancedBrepShapeRepresentation, StepShapeShapeRepresentation,
};
use martensite_tkdestep::step_tidy::StepTidyAxis2Placement3dReducer;
use martensite_tkdestep::step_visual::{
    StepVisualLayeredItem, StepVisualPresentationLayerAssignment,
    StepVisualPresentationStyleAssignment, StepVisualStyledItem,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::Handle;

type Placement = Handle<StepGeomAxis2Placement3d>;

fn replaced(fixture: &Fixture) -> ReplacedEntities {
    fixture.replaced_by(&mut StepTidyAxis2Placement3dReducer::new(&fixture.ws))
}

/// Two equal placements, each held by its own sharing entity: one is replaced.
fn assert_merged_in(share: impl Fn(&Fixture, &Placement)) {
    let fixture = Fixture::new();
    let (first, second) = (fixture.add_placement(""), fixture.add_placement(""));
    share(&fixture, &first);
    share(&fixture, &second);
    replaced(&fixture).assert_one_of(&first, &second);
}

fn representation_items(
    placement: &Placement,
) -> Handle<HArray1<Handle<StepReprRepresentationItem>>> {
    one_item(placement.cast())
}

fn new_context() -> Handle<StepReprRepresentationContext> {
    Handle::new(StepReprRepresentationContext::new())
}

fn add_plane_on(fixture: &Fixture, placement: &Placement) {
    let plane = Handle::new(StepGeomPlane::new());
    plane.init(&name(""), placement);
    fixture.add(&plane);
}

#[test]
fn different_names() {
    let fixture = Fixture::new();
    let (first, second) = (
        fixture.add_placement("Axis1"),
        fixture.add_placement("Axis2"),
    );
    add_plane_on(&fixture, &first);
    add_plane_on(&fixture, &second);
    replaced(&fixture).assert_empty();
}

#[test]
fn step_geom_plane() {
    assert_merged_in(add_plane_on);
}

/// The second transformation stays empty and the first is initialized twice, as written does: the
/// first placement ends with no sharing entity at all, and still counts as a class of duplicates.
#[test]
fn step_repr_item_defined_transformation() {
    let fixture = Fixture::new();
    let (first, second) = (fixture.add_placement(""), fixture.add_placement(""));
    let third = fixture.add_placement_at("", [1.0, 1.0, 1.0]);
    let held = Handle::new(StepReprItemDefinedTransformation::new());
    held.init(&name(""), &name(""), &first.cast(), &third.cast());
    fixture.add(&held);
    let empty = Handle::new(StepReprItemDefinedTransformation::new());
    held.init(&name(""), &name(""), &second.cast(), &third.cast());
    fixture.add(&empty);
    replaced(&fixture).assert_one_of(&first, &second);
}

#[test]
fn step_geom_cylindrical_surface() {
    assert_merged_in(|fixture, placement| {
        let surface = Handle::new(StepGeomCylindricalSurface::new());
        surface.init(&name(""), placement, 1.0);
        fixture.add(&surface);
    });
}

#[test]
fn step_shape_shape_representation() {
    assert_merged_in(|fixture, placement| {
        let representation = Handle::new(StepShapeShapeRepresentation::new());
        representation.init(&name(""), &representation_items(placement), &new_context());
        fixture.add(&representation);
    });
}

#[test]
fn step_repr_constructive_geometry_representation() {
    assert_merged_in(|fixture, placement| {
        let representation = Handle::new(StepReprConstructiveGeometryRepresentation::new());
        representation.init(&name(""), &representation_items(placement), &new_context());
        fixture.add(&representation);
    });
}

#[test]
fn step_geom_circle() {
    assert_merged_in(|fixture, placement| {
        let circle = Handle::new(StepGeomCircle::new());
        circle.init(&name(""), &placement_select(placement), 1.0);
        fixture.add(&circle);
    });
}

#[test]
fn step_visual_presentation_layer_assignment() {
    assert_merged_in(|fixture, placement| {
        let items = HArray1::with_bounds(1, 1).expect("bounds");
        let mut item = StepVisualLayeredItem::default();
        item.set_value(&placement.cast()).expect("a layered item");
        items.change_array1().set_value(1, item).expect("set");
        let assignment = Handle::new(StepVisualPresentationLayerAssignment::new());
        assignment.init(&name(""), &name(""), &Handle::new(items));
        fixture.add(&assignment);
    });
}

#[test]
fn step_visual_styled_item() {
    assert_merged_in(|fixture, placement| {
        let styles = HArray1::<Handle<StepVisualPresentationStyleAssignment>>::with_bounds(1, 1);
        let styled = Handle::new(StepVisualStyledItem::new());
        styled.init(
            &name(""),
            &Handle::new(styles.expect("bounds")),
            &placement.cast(),
        );
        fixture.add(&styled);
    });
}

#[test]
fn step_geom_ellipse() {
    assert_merged_in(|fixture, placement| {
        let ellipse = Handle::new(StepGeomEllipse::new());
        ellipse.init(&name(""), &placement_select(placement), 1.0, 2.0);
        fixture.add(&ellipse);
    });
}

#[test]
fn step_geom_conical_surface() {
    assert_merged_in(|fixture, placement| {
        let surface = Handle::new(StepGeomConicalSurface::new());
        surface.init(&name(""), placement, 1.0, 1.0);
        fixture.add(&surface);
    });
}

#[test]
fn step_geom_toroidal_surface() {
    assert_merged_in(|fixture, placement| {
        let surface = Handle::new(StepGeomToroidalSurface::new());
        surface.init(&name(""), placement, 1.0, 1.0);
        fixture.add(&surface);
    });
}

#[test]
fn step_shape_advanced_brep_shape_representation() {
    assert_merged_in(|fixture, placement| {
        let representation = Handle::new(StepShapeAdvancedBrepShapeRepresentation::new());
        representation.init(&name(""), &representation_items(placement), &new_context());
        fixture.add(&representation);
    });
}

#[test]
fn step_geom_spherical_surface() {
    assert_merged_in(|fixture, placement| {
        let surface = Handle::new(StepGeomSphericalSurface::new());
        surface.init(&name(""), placement, 1.0);
        fixture.add(&surface);
    });
}
