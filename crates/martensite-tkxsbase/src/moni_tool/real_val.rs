//! `MoniToolRealVal`: a real through a handle.
use martensite_tkernel::{
    impl_transient,
    standard::{SharedState, StandardTransient},
};
use std::sync::RwLockWriteGuard;

/// `MoniToolRealVal`: a real shared through handles; the value is edited through any of
/// them (`CValue`).
///
/// ```
/// use martensite_tkxsbase::moni_tool::MoniToolRealVal;
/// let val = MoniToolRealVal::new(1.5);
/// *val.c_value() += 1.0;
/// assert_eq!(val.value(), 2.5);
/// ```
#[derive(Default)]
pub struct MoniToolRealVal {
    transient: StandardTransient,
    theval: SharedState<f64>,
}
impl_transient!(
    MoniToolRealVal,
    "MoniToolRealVal",
    StandardTransient,
    transient
);

impl MoniToolRealVal {
    /// `MoniToolRealVal(val = 0.)`.
    pub fn new(val: f64) -> Self {
        Self {
            transient: StandardTransient::default(),
            theval: SharedState::new(val),
        }
    }
    /// `Value`.
    pub fn value(&self) -> f64 {
        *self.theval.read()
    }
    /// `CValue`: the modifiable value (natively `double&`); drop the guard before reading
    /// it again.
    pub fn c_value(&self) -> RwLockWriteGuard<'_, f64> {
        self.theval.write()
    }
}

#[cfg(test)]
mod tests {
    use super::MoniToolRealVal;
    use martensite_tkernel::standard::Handle;

    #[test]
    fn value_is_shared_through_handles() {
        let first = Handle::new(MoniToolRealVal::default());
        let second = first.clone();
        assert_eq!(first.value(), 0.0);
        *second.c_value() = -4.25;
        assert_eq!(first.value(), -4.25);
        assert_eq!(first.dynamic_type().unwrap().name(), "MoniToolRealVal");
    }
}
