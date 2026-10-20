//! `Graphic3d_CView::Compute` walks myStructsDisplayed itself: the virtual
//! RecomputeTransformation of a displayed structure sees no owner that the native loop lacks.
use super::*;
use crate::aspect::{AspectNeutralWindow, AspectWindow};
use crate::null_driver::{NullGraphicDriver, NullView};
use martensite_tkernel::standard::{Handle, SharedState, StandardTransient, Transient};

/// A structure whose RecomputeTransformation override records its own owner count.
struct OwnerCountingStructure {
    transient: StandardTransient,
    base: Graphic3dStructureBase,
    owners_seen: SharedState<Vec<i32>>,
}
martensite_tkernel::impl_transient!(OwnerCountingStructure, "Martensite_OwnerCountingStructure",
    StandardTransient, transient, interfaces: [dyn Graphic3dStructureSource]);
impl Graphic3dStructureSource for OwnerCountingStructure {
    fn structure_base(&self) -> &Graphic3dStructureBase {
        &self.base
    }
    fn recompute_transformation(&self, _projector: &Handle<Graphic3dCamera>) {
        self.owners_seen
            .write()
            .push(self.transient_base().ref_count());
    }
}

#[test]
fn compute_adds_no_owner_while_recomputing_displayed_structures() {
    let driver = NullGraphicDriver::new_handle();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver).unwrap());
    let view = driver.create_view(&manager).unwrap();
    let window = AspectNeutralWindow::default();
    window.set_size(64, 64);
    let window: Handle<dyn AspectWindow> = Handle::new(window).cast();
    view.cast::<NullView>().set_null_window(&window);
    view.activate();
    let structure = Handle::new(OwnerCountingStructure {
        transient: StandardTransient::default(),
        base: Graphic3dStructureBase::new(&manager, &Handle::default()).unwrap(),
        owners_seen: SharedState::new(Vec::new()),
    });
    let source: Handle<dyn Graphic3dStructureSource> = structure.clone().cast();
    graphic3d_structure_bind(&source);
    drop(source);
    view.display(&structure.clone().cast());
    assert!(view.is_displayed(&structure.clone().cast()));
    let outside = structure.ref_count();
    structure.owners_seen.write().clear();
    view.compute();
    // The native loop sees the same owners as outside it: no temporary copy of the set.
    assert_eq!(*structure.owners_seen.read(), vec![outside]);
}
