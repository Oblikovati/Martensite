//! Read leases preserve topology identity while assignments retire the previous value outside the lock.
use martensite_tkbrep::topods::{Compound, Shape, TShape};
use martensite_tkernel::standard::{Failure, SharedState, WeakHandle, catch_native_failure, exceptions};
use std::sync::Arc;

enum ColoredCompoundState {
    Live(Arc<Shape>),
    Retiring(WeakHandle<dyn TShape>),
    Removed,
}

impl ColoredCompoundState {
    fn from_compound(compound: &Compound) -> Self {
        Self::Live(Arc::new(compound.to_shape()))
    }

    fn update_unchanged_topology(&mut self, compound: &Compound) -> Result<bool, Failure> {
        let Self::Live(value) = self else {
            return Ok(false);
        };
        if value.tshape() != compound.tshape() {
            return Ok(false);
        }
        if let Some(shape) = Arc::get_mut(value) {
            shape.set_location(compound.location(), false)?;
            shape.set_orientation(compound.orientation());
        } else {
            *value = Arc::new(compound.to_shape());
        }
        Ok(true)
    }

    fn begin_retirement(&mut self) -> Result<Self, Failure> {
        match self {
            Self::Live(shape) => {
                let pointer = shape.tshape().downgrade();
                Ok(std::mem::replace(self, Self::Retiring(pointer)))
            }
            Self::Removed => Ok(Self::Removed),
            Self::Retiring(pointer) => Err(exceptions::program_error(&format!(
                "compound topology={:p} is retiring; expected an idle value",
                pointer.object_address()
            ))),
        }
    }

    fn live(&self) -> Result<&Arc<Shape>, Failure> {
        match self {
            Self::Live(shape) => Ok(shape),
            Self::Retiring(pointer) => Err(exceptions::null_object(&format!(
                "pointer={:p}, strong count={}; expected a projectable live pointee",
                pointer.object_address(),
                pointer.ref_count()
            ))),
            Self::Removed => Err(super::colored_compound_node::colored_compound_removed()),
        }
    }
}

pub(super) struct ColoredCompoundValue {
    state: SharedState<ColoredCompoundState>,
}

impl ColoredCompoundValue {
    pub(super) fn new(compound: &Compound) -> Self {
        Self {
            state: SharedState::new(ColoredCompoundState::from_compound(compound)),
        }
    }

    pub(super) fn assign(&self, compound: &Compound) -> Result<(), Failure> {
        let mut state = self.state.write();
        if state.update_unchanged_topology(compound)? {
            return Ok(());
        }
        if matches!(*state, ColoredCompoundState::Removed) {
            return Err(super::colored_compound_node::colored_compound_removed());
        }
        let retired = state.begin_retirement()?;
        drop(state);
        // Without an active read, the old destructor observes its pointer at count zero
        // before this field acquires the incoming topology.
        drop(retired);
        *self.state.write() = ColoredCompoundState::from_compound(compound);
        Ok(())
    }

    pub(super) fn clone_compound(&self) -> Result<Compound, Failure> {
        let state = self.state.read();
        Ok(Compound::unchecked(state.live()?.to_shape()))
    }

    pub(super) fn with_shape<R>(
        &self,
        project: impl FnOnce(&Shape) -> Result<R, Failure>,
    ) -> Result<R, Failure> {
        let shape = Arc::clone(self.state.read().live()?);
        catch_native_failure(move || project(&shape))
    }

    #[cfg(test)]
    pub(super) fn object_address(&self) -> *const () {
        match &*self.state.read() {
            ColoredCompoundState::Live(shape) => shape.tshape().object_address(),
            ColoredCompoundState::Retiring(pointer) => pointer.object_address(),
            ColoredCompoundState::Removed => std::ptr::null(),
        }
    }

    pub(super) fn ref_count(&self) -> i32 {
        let count = match &*self.state.read() {
            ColoredCompoundState::Live(shape) => shape.tshape().ref_count(),
            ColoredCompoundState::Retiring(pointer) => pointer.ref_count(),
            ColoredCompoundState::Removed => 0,
        };
        i32::try_from(count).unwrap_or(i32::MAX)
    }

    pub(super) fn release(&self) {
        let retired = self
            .state
            .write()
            .begin_retirement()
            .expect("compound topology retirement failed; expected a single owning-map release");
        drop(retired);
        *self.state.write() = ColoredCompoundState::Removed;
    }
}
