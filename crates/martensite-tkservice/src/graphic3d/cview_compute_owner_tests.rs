//! Compute snapshots retain the actual receiver during virtual calls (#592).
use super::*;
use crate::aspect::{AspectNeutralWindow, AspectWindow};
use crate::null_driver::{NullGraphicDriver, NullView};
use martensite_tkernel::standard::{Handle, SharedState, StandardTransient, WeakHandle};
use std::sync::OnceLock;

/// A structure whose RecomputeTransformation override records its own owner count.
struct OwnerCountingStructure {
    base: Graphic3dStructureBase,
    owners_seen: SharedState<Vec<usize>>,
    identity: OnceLock<WeakHandle<OwnerCountingStructure>>,
}
martensite_tkernel::impl_transient!(OwnerCountingStructure, "Martensite_OwnerCountingStructure",
    StandardTransient, interfaces: [dyn Graphic3dStructureSource]);
impl Graphic3dStructureSource for OwnerCountingStructure {
    fn structure_base(&self) -> &Graphic3dStructureBase {
        &self.base
    }
    fn recompute_transformation(&self, _projector: &Handle<Graphic3dCamera>) {
        self.owners_seen
            .write()
            .push(self.identity.get().unwrap().ref_count());
    }
}

#[test]
fn compute_retains_one_read_snapshot_while_recomputing_displayed_structures() {
    let driver = NullGraphicDriver::new_handle();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver).unwrap());
    let view = driver.create_view(&manager).unwrap();
    let window = AspectNeutralWindow::default();
    window.set_size(64, 64);
    let window: Handle<dyn AspectWindow> = Handle::new(window).cast();
    view.cast::<NullView>().set_null_window(&window);
    view.activate();
    let structure = Handle::new(OwnerCountingStructure {
        base: Graphic3dStructureBase::new(&manager, &Handle::default()).unwrap(),
        owners_seen: SharedState::new(Vec::new()),
        identity: OnceLock::new(),
    });
    structure.identity.set(structure.downgrade()).unwrap();
    let source: Handle<dyn Graphic3dStructureSource> = structure.clone().cast();
    graphic3d_structure_bind(&source);
    drop(source);
    view.display(&structure.clone().cast());
    assert!(view.is_displayed(&structure.clone().cast()));
    let outside = structure.ref_count();
    structure.owners_seen.write().clear();
    view.compute();
    // #592: a safe Arc receiver snapshot contributes exactly one owner during the call.
    assert_eq!(*structure.owners_seen.read(), vec![outside + 1]);
    assert_eq!(structure.ref_count(), outside);
}
