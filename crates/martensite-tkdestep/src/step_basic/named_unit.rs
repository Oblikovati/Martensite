//! A named unit and the SI unit that redefines its dimensions: `Dimensions` is virtual
//! and the SI unit always answers null, so the base keeps a flag that makes the same answer when
//! reached through a `Handle<StepBasicNamedUnit>`.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_entity::*;

step_entity! {
    /// Named unit.
    StepBasicNamedUnit, StepBasicNamedUnitState, "StepBasicNamedUnit", StandardTransient, {
        dimensions: Handle<StepBasicDimensionalExponents>,
        dimensions_redefined: bool,
    }
}
impl StepBasicNamedUnit {
    /// `init`.
    pub fn init(&self, a_dimensions: &Handle<StepBasicDimensionalExponents>) {
        self.state.write().dimensions = a_dimensions.clone();
    }
    /// `set_dimensions`; an SI unit ignores it.
    pub fn set_dimensions(&self, a_dimensions: &Handle<StepBasicDimensionalExponents>) {
        let mut state = self.state.write();
        if !state.dimensions_redefined {
            state.dimensions = a_dimensions.clone();
        }
    }
    /// `Dimensions`; an SI unit answers null.
    pub fn dimensions(&self) -> Handle<StepBasicDimensionalExponents> {
        let state = self.state.read();
        if state.dimensions_redefined {
            return Handle::default();
        }
        state.dimensions.clone()
    }
}

step_entity! {
    /// Si unit.
    custom_default StepBasicSiUnit, StepBasicSiUnitState, "StepBasicSiUnit", StepBasicNamedUnit, {
        has_prefix: bool,
        prefix: StepBasicSiPrefix,
        name: StepBasicSiUnitName,
    }
}
impl Default for StepBasicSiUnit {
    fn default() -> Self {
        let object = Self {
            parent: StepBasicNamedUnit::default(),
            state: SharedState::default(),
        };
        object.parent.state.write().dimensions_redefined = true;
        object
    }
}
impl StepBasicSiUnit {
    /// `StepBasicSiUnit()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `init`.
    pub fn init(
        &self,
        has_aprefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        let mut state = self.state.write();
        state.has_prefix = has_aprefix;
        state.prefix = a_prefix;
        state.name = a_name;
    }
    step_methods! {
        set_copy set_prefix = prefix flag has_prefix: StepBasicSiPrefix;
        clear un_set_prefix = has_prefix;
        get prefix = prefix: StepBasicSiPrefix;
        get has_prefix = has_prefix: bool;
        set_copy set_name = name: StepBasicSiUnitName;
        get name = name: StepBasicSiUnitName;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_si_unit_answers_null_dimensions_through_its_base_handle() {
        let unit = Handle::new(StepBasicSiUnit::new());
        let exponents = Handle::new(StepBasicDimensionalExponents::new());
        let base = unit.cast::<StepBasicNamedUnit>();
        base.set_dimensions(&exponents);
        assert!(base.dimensions().is_null());
        let plain = StepBasicNamedUnit::new();
        plain.set_dimensions(&exponents);
        assert!(plain.dimensions().is_some());
    }
}
