//! Named resource fake reaches actual capacity/lookup failures in the provided registry.
use crate::adv_app2var::sys_base::numeric_workspace::{HeapNumericWorkspace, NumericWorkspace};
use crate::adv_app2var::sys_base::{AdvApp2VarSysBase, MAX_ALLOC_NB};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ResourceMode {
    None,
    Capacity,
    Retired,
}
pub(super) struct RegistryWorkspace {
    pub(super) mode: ResourceMode,
    pub(super) events: Vec<(usize, i32)>,
    registry: AdvApp2VarSysBase,
    offset: isize,
    length: usize,
}
impl RegistryWorkspace {
    pub(super) fn new(mode: ResourceMode) -> Self {
        Self {
            mode,
            events: Vec::new(),
            registry: AdvApp2VarSysBase::new(),
            offset: 0,
            length: 0,
        }
    }
    fn capacity_failure(&mut self, length: usize) -> i32 {
        let mut offsets = [0; MAX_ALLOC_NB];
        for offset in &mut offsets {
            let mut code = -7;
            self.registry.mcrrqst_(8, 1, 0, offset, &mut code);
            assert_eq!(code, 0);
        }
        let mut code = -7;
        self.registry
            .mcrrqst_(8, length as i32, 0, &mut self.offset, &mut code);
        for offset in &mut offsets {
            let mut cleanup = -7;
            self.registry.mcrdelt_(8, 1, 0, offset, &mut cleanup);
            assert_eq!(cleanup, 0);
        }
        code
    }
    fn retire(&mut self) {
        let mut offset = self.offset;
        let mut code = -7;
        self.registry
            .mcrdelt_(8, self.length as i32, 0, &mut offset, &mut code);
        assert_eq!(code, 0);
    }
}
impl NumericWorkspace for RegistryWorkspace {
    fn allocate(&mut self, length: usize) -> Result<Vec<f64>, ()> {
        self.length = length;
        let mut code = -7;
        if self.mode == ResourceMode::Capacity {
            code = self.capacity_failure(length);
        } else {
            self.registry
                .mcrrqst_(8, length as i32, 0, &mut self.offset, &mut code);
        }
        self.events.push((length, code));
        if code != 0 {
            return Err(());
        }
        HeapNumericWorkspace.allocate(length)
    }
    fn release(&mut self, buffer: Vec<f64>) -> Result<(), ()> {
        assert_eq!(buffer.len(), self.length);
        if self.mode == ResourceMode::Retired {
            self.retire();
        }
        let mut code = -7;
        self.registry
            .mcrdelt_(8, self.length as i32, 0, &mut self.offset, &mut code);
        self.events.push((self.length, code));
        assert_eq!(self.registry.ncore(), 0);
        drop(buffer);
        if code == 0 { Ok(()) } else { Err(()) }
    }
}
