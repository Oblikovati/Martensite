//! `MoniToolIntVal`: an integer through a handle.
use martensite_tkernel::{
    impl_transient,
    standard::{SharedState, Transient},
};
use std::sync::RwLockWriteGuard;

/// `MoniToolIntVal`: an integer shared through handles; the value is edited through
/// any of them (`CValue`).
///
/// ```
/// use martensite_tkxsbase::moni_tool::MoniToolIntVal;
/// let val = MoniToolIntVal::new(3);
/// *val.c_value() = 7;
/// assert_eq!(val.value(), 7);
/// ```
#[derive(Default)]
pub struct MoniToolIntVal {
    theval: SharedState<i32>,
}
impl_transient!(MoniToolIntVal, "MoniToolIntVal", dyn Transient);

impl MoniToolIntVal {
    /// `MoniToolIntVal(val = 0)`.
    pub fn new(val: i32) -> Self {
        Self {
            theval: SharedState::new(val),
        }
    }
    /// `Value`.
    pub fn value(&self) -> i32 {
        *self.theval.read()
    }
    /// `CValue`: the modifiable value (natively `int&`); drop the guard before reading it
    /// again.
    pub fn c_value(&self) -> RwLockWriteGuard<'_, i32> {
        self.theval.write()
    }
}

#[cfg(test)]
mod tests {
    use super::MoniToolIntVal;
    use martensite_tkernel::standard::{Handle, Transient};

    #[test]
    fn value_is_shared_through_handles() {
        let first = Handle::new(MoniToolIntVal::default());
        let second: Handle<dyn Transient> = first.cast();
        assert_eq!(first.value(), 0);
        *second.cast::<MoniToolIntVal>().c_value() = 12;
        assert_eq!(first.value(), 12);
        assert_eq!(second.dynamic_type().unwrap().name(), "MoniToolIntVal");
    }
}
