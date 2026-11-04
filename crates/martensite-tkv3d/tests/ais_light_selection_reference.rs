//! Light selection observes its caller only at the actual sensitive-entity Add boundaries.
use martensite_tkernel::standard::{Handle, HandleObserver, HandleReference};
use martensite_tkservice::graphic3d::{Graphic3dCLight, Graphic3dTypeOfLightSource};
use martensite_tkv3d::{
    ais::AisLightSource,
    select_mgr::{SelectMgrSelectableObject, SelectMgrSelection},
    select3d::{Select3DSensitivePoint, Select3DSensitiveSphere},
};
use std::cell::Cell;

struct AddBoundarySelectionReference {
    first: Handle<SelectMgrSelection>,
    second: Handle<SelectMgrSelection>,
    reads: Cell<usize>,
}

impl AddBoundarySelectionReference {
    fn new() -> Self {
        Self {
            first: Handle::new(SelectMgrSelection::new(0)),
            second: Handle::new(SelectMgrSelection::new(0)),
            reads: Cell::new(0),
        }
    }
}

impl HandleReference<SelectMgrSelection> for AddBoundarySelectionReference {
    fn observe(&self) -> HandleObserver<SelectMgrSelection> {
        let previous = self.reads.replace(self.reads.get() + 1);
        if previous == 0 {
            assert!(
                self.first.is_empty(),
                "first Add boundary; expected empty selection"
            );
            return self.first.observe();
        }
        assert_eq!(
            previous, 1,
            "selection read; expected exactly two Add sites"
        );
        assert_eq!(
            self.first.entities().len(),
            1,
            "second Add; expected inserted sphere"
        );
        self.second.observe()
    }
    fn reference_address(&self) -> *const () {
        // The named fake changes the supplied target, never this capability's identity.
        std::ptr::from_ref(self).cast()
    }
}

struct ForbiddenSelectionReference;
impl HandleReference<SelectMgrSelection> for ForbiddenSelectionReference {
    fn observe(&self) -> HandleObserver<SelectMgrSelection> {
        // A nonzero mode must return before even dereferencing the argument.
        panic!("selection observed; expected no handle access for nonzero mode")
    }
    fn reference_address(&self) -> *const () {
        // Metadata access is forbidden just like pointee access on this branch.
        panic!("selection metadata read; expected no handle access for nonzero mode")
    }
}

fn light(kind: Graphic3dTypeOfLightSource) -> Handle<AisLightSource> {
    let source = Handle::new(Graphic3dCLight::new(kind));
    AisLightSource::new(&source).unwrap()
}

#[test]
fn nonzero_light_modes_never_observe_selection_argument() {
    let object = light(Graphic3dTypeOfLightSource::Directional);
    for mode in [-1, 1, 42] {
        object
            .compute_selection(&ForbiddenSelectionReference, mode)
            .unwrap();
    }
}

#[test]
fn directional_light_rereads_selection_after_actual_sphere_add() {
    let object = light(Graphic3dTypeOfLightSource::Directional);
    let selection = AddBoundarySelectionReference::new();
    object.compute_selection(&selection, 0).unwrap();
    assert_eq!(
        selection.reads.get(),
        2,
        "expected one read at each actual Add"
    );
    require_distinct_add_targets(&selection);
}

#[test]
fn ambient_light_observes_only_its_actual_point_add() {
    let object = light(Graphic3dTypeOfLightSource::Ambient);
    let selection = AddBoundarySelectionReference::new();
    object.compute_selection(&selection, 0).unwrap();
    assert_eq!(selection.reads.get(), 1, "expected only actual point Add");
    assert_eq!(
        selection.first.entities().len(),
        1,
        "expected one actual point"
    );
    assert!(
        selection.second.is_empty(),
        "expected untouched second target"
    );
}

fn require_distinct_add_targets(selection: &AddBoundarySelectionReference) {
    let sphere = selection.first.entities()[0]
        .base_sensitive()
        .cast::<Select3DSensitiveSphere>();
    let point = selection.second.entities()[0]
        .base_sensitive()
        .cast::<Select3DSensitivePoint>();
    assert!(
        !sphere.is_null(),
        "first Add; expected real sensitive sphere"
    );
    assert!(
        !point.is_null(),
        "second Add; expected real sensitive point"
    );
}
