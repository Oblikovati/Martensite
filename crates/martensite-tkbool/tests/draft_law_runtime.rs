//! Draft placement keeps its direct edge-law parent and projected base state.
use martensite_tkbool::brep_fill::{BRepFillDraftLaw, BRepFillEdge3DLaw, BRepFillLocationLaw};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, Transient};
use martensite_tkgeomalgo::geom_fill::GeomFillLocationDraft;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tktopalgo::brep_lib::BRepLibMakePolygon;

fn unit_draft_placement() -> Result<BRepFillDraftLaw, Failure> {
    let path =
        BRepLibMakePolygon::from_two_points(Point3::default(), Point3::new(1., 0., 0.))?.wire()?;
    let draft = Handle::new(GeomFillLocationDraft::new(Direction3::Z, 0.2)?);
    BRepFillDraftLaw::new(&path, &draft)
}
#[test]
fn draft_class_name_and_descriptor_identity_are_stable() {
    assert_eq!(BRepFillDraftLaw::TYPE_NAME, "BRepFillDraftLaw");
    let first = BRepFillDraftLaw::static_type();
    let second = BRepFillDraftLaw::static_type();
    assert_eq!(first.name(), "BRepFillDraftLaw");
    assert!(std::ptr::eq(first, second));
    assert!(std::ptr::eq(
        unit_draft_placement().unwrap().dynamic_type(),
        first
    ));
}
#[test]
fn draft_runtime_descriptor_has_its_direct_edge_law_ancestor() {
    let descriptor = BRepFillDraftLaw::static_type();
    assert_eq!(descriptor.parent().unwrap().name(), "BRepFillEdge3DLaw");
    assert!(descriptor.is_subtype(BRepFillEdge3DLaw::static_type()));
    assert!(descriptor.is_subtype(<dyn BRepFillLocationLaw>::static_type()));
}
#[test]
fn inherited_edge_law_projection_keeps_the_same_authored_unit_segment() {
    let original = Handle::new(unit_draft_placement().unwrap());
    let base: Handle<BRepFillEdge3DLaw> = original.clone().cast();
    let projected = base
        .get()
        .expect("draft placement must project to its edge-law base");
    assert_eq!(projected.nb_law().unwrap(), 1);
    assert_eq!(projected.curvilinear_bounds(1).unwrap(), (0., 1.));
    assert!(!projected.is_closed().unwrap());
    assert_eq!(base.dynamic_type().unwrap().name(), "BRepFillDraftLaw");
    let restored: Handle<BRepFillDraftLaw> = base.cast();
    assert_eq!(restored.object_address(), original.object_address());
    assert_eq!(
        restored.get().unwrap().curvilinear_bounds(1).unwrap(),
        (0., 1.)
    );
}

#[test]
fn projected_parent_owns_the_law_after_the_derived_handle_is_dropped() {
    let base: Handle<BRepFillEdge3DLaw> = {
        let derived = Handle::new(unit_draft_placement().unwrap());
        let projected = derived.clone().cast();
        drop(derived);
        projected
    };
    assert_eq!(base.get().unwrap().curvilinear_bounds(1).unwrap(), (0., 1.));
    let restored: Handle<BRepFillDraftLaw> = base.cast();
    assert_eq!(restored.dynamic_type().unwrap().name(), "BRepFillDraftLaw");
    assert_eq!(restored.get().unwrap().nb_law().unwrap(), 1);
}
