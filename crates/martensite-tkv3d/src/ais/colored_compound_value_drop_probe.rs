//! Actual compound subclass reads authoritative old Value fields from its native destructor callback.
use super::AisColoredDrawerCompounds;
use martensite_tkbrep::topods::{TCompound, TShape, TShapeBase};
use martensite_tkernel::standard::{Failure, Handle, WeakHandle};
use std::{
    fmt::Write,
    sync::{Arc, Mutex, Weak},
};
pub(super) struct ColoredCompoundValueDropFake {
    topology: TCompound,
    owner: Weak<AisColoredDrawerCompounds>,
    incoming: WeakHandle<dyn TShape>,
    rows: Arc<Mutex<String>>,
}
martensite_tkernel::impl_transient!(ColoredCompoundValueDropFake,"CompoundValueDestructorProbe",TCompound,interfaces:[dyn TShape]);
impl ColoredCompoundValueDropFake {
    pub(super) fn new(
        owner: &Arc<AisColoredDrawerCompounds>,
        incoming: &Handle<dyn TShape>,
        rows: &Arc<Mutex<String>>,
    ) -> Handle<Self> {
        Handle::new(Self {
            topology: TCompound::new(),
            owner: Arc::downgrade(owner),
            incoming: incoming.downgrade(),
            rows: Arc::clone(rows),
        })
    }
}
impl TShape for ColoredCompoundValueDropFake {
    fn tshape_base(&self) -> &TShapeBase {
        self.topology.tshape_base()
    }
    fn empty_copy(&self) -> Result<Handle<dyn TShape>, Failure> {
        self.topology.empty_copy()
    }
}
impl Drop for ColoredCompoundValueDropFake {
    fn drop(&mut self) {
        let owner = self
            .owner
            .upgrade()
            .expect("map expired; expected live assignment owner");
        let node = owner.node_ref(1).unwrap().live_node().unwrap();
        let old_address = node.compound.object_address();
        let same = old_address == std::ptr::from_ref(self).cast();
        writeln!(
            self.rows.lock().unwrap(),
            "value-destructor {} {} {} {}",
            owner.len(),
            i32::from(same),
            node.compound.ref_count(),
            self.incoming.ref_count()
        )
        .unwrap();
    }
}
