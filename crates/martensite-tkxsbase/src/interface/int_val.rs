//! `InterfaceIntVal`: an integer through a handle (the shared cursor of the entity
//! iterators).
use martensite_tkernel::{
    impl_transient,
    standard::{SharedState, StandardTransient},
};
use std::sync::RwLockWriteGuard;

/// `InterfaceIntVal`: an integer shared through handles, 0 at creation.
///
/// ```
/// use martensite_tkxsbase::interface::InterfaceIntVal;
/// let val = InterfaceIntVal::new();
/// *val.c_value() += 2;
/// assert_eq!(val.value(), 2);
/// ```
#[derive(Default)]
pub struct InterfaceIntVal {
    transient: StandardTransient,
    theval: SharedState<i32>,
}
impl_transient!(
    InterfaceIntVal,
    "InterfaceIntVal",
    StandardTransient,
    transient
);

impl InterfaceIntVal {
    /// `InterfaceIntVal()`: 0.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Value`.
    pub fn value(&self) -> i32 {
        *self.theval.read()
    }
    /// `CValue`: the modifiable value (natively `int&`); drop the guard before reading
    /// it again.
    pub fn c_value(&self) -> RwLockWriteGuard<'_, i32> {
        self.theval.write()
    }
}

#[cfg(test)]
mod tests {
    use super::InterfaceIntVal;
    use martensite_tkernel::standard::Handle;

    #[test]
    fn shared_value() {
        let first = Handle::new(InterfaceIntVal::new());
        let second = first.clone();
        *second.c_value() = 5;
        assert_eq!(first.value(), 5);
        assert_eq!(first.dynamic_type().unwrap().name(), "InterfaceIntVal");
    }
}
