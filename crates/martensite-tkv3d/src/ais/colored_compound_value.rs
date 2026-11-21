//! Read leases preserve topology identity while assignments retire the previous value outside the lock.
use martensite_tkbrep::topods::{Compound, Shape, ShapeView, TShape};
use martensite_tkernel::standard::{Failure, Handle, SharedState, WeakHandle, exceptions};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::toploc::Location;
use std::sync::Arc;

#[derive(Clone)]
struct ColoredCompoundPlacement {
    location: Location,
    orientation: Orientation,
}

impl ColoredCompoundPlacement {
    fn of(compound: &Compound) -> Self {
        Self {
            location: compound.location().clone(),
            orientation: compound.orientation(),
        }
    }
}

enum ColoredCompoundState {
    Live {
        topology: Arc<Handle<dyn TShape>>,
        placement: ColoredCompoundPlacement,
    },
    Retiring(WeakHandle<dyn TShape>),
    Removed,
}

impl ColoredCompoundState {
    fn from_compound(compound: &Compound) -> Self {
        Self::Live {
            topology: Arc::new(compound.tshape().clone()),
            placement: ColoredCompoundPlacement::of(compound),
        }
    }

    fn update_unchanged_topology(&mut self, compound: &Compound) -> bool {
        let Self::Live {
            topology,
            placement,
        } = self
        else {
            return false;
        };
        if topology.object_address() != compound.tshape().object_address() {
            return false;
        }
        *placement = ColoredCompoundPlacement::of(compound);
        true
    }

    fn begin_retirement(&mut self) -> Result<Self, Failure> {
        match self {
            Self::Live { topology, .. } => {
                let pointer = topology.downgrade();
                Ok(std::mem::replace(self, Self::Retiring(pointer)))
            }
            Self::Removed => Ok(Self::Removed),
            Self::Retiring(pointer) => Err(exceptions::program_error(&format!(
                "compound topology={:p} is retiring; expected an idle value",
                pointer.object_address()
            ))),
        }
    }

    fn live(&self) -> Result<(&Arc<Handle<dyn TShape>>, &ColoredCompoundPlacement), Failure> {
        match self {
            Self::Live {
                topology,
                placement,
            } => Ok((topology, placement)),
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
        if state.update_unchanged_topology(compound) {
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
        let (topology, placement) = state.live()?;
        Ok(Compound::unchecked(Shape::from_parts(
            Handle::clone(topology),
            placement.location.clone(),
            placement.orientation,
        )))
    }

    pub(super) fn with_shape<R>(
        &self,
        project: impl FnOnce(&ShapeView<'_>) -> Result<R, Failure>,
    ) -> Result<R, Failure> {
        let state = self.state.read();
        let (topology, placement) = state.live()?;
        let topology = Arc::clone(topology);
        let placement = placement.clone();
        drop(state);
        Shape::with_reference_parts(
            topology.as_ref(),
            &placement.location,
            placement.orientation,
            project,
        )
    }

    #[cfg(test)]
    pub(super) fn object_address(&self) -> *const () {
        match &*self.state.read() {
            ColoredCompoundState::Live { topology, .. } => topology.object_address(),
            ColoredCompoundState::Retiring(pointer) => pointer.object_address(),
            ColoredCompoundState::Removed => std::ptr::null(),
        }
    }

    pub(super) fn ref_count(&self) -> i32 {
        let count = match &*self.state.read() {
            ColoredCompoundState::Live { topology, .. } => topology.ref_count(),
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
