//! A trim point: a cartesian point, or an immediate real named `PARAMETER_VALUE`.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_data::{StepDataSelectMember, StepDataSelectType, StepDataSelectValue};
use crate::step_entity::*;
use crate::{impl_step_select_storage, step_select_case};
use martensite_tkxsbase::interface::InterfaceParamType;

step_fixed_name_member!(
    /// `StepGeomTrimmingMember`: a real named `PARAMETER_VALUE`.
    StepGeomTrimmingMember,
    "StepGeomTrimmingMember",
    "PARAMETER_VALUE"
);

/// Trimming select.
#[derive(Clone, Default)]
pub struct StepGeomTrimmingSelect {
    value: StepDataSelectValue,
}

impl StepDataSelectType for StepGeomTrimmingSelect {
    impl_step_select_storage!(value);
    fn case_num(&self, ent: &Handle<StandardTransient>) -> i32 {
        step_select_case!(ent; StepGeomCartesianPoint)
    }
    fn new_member(&self) -> Handle<dyn StepDataSelectMember> {
        Handle::new(StepGeomTrimmingMember::new()).cast()
    }
    fn case_mem(&self, ent: &Handle<dyn StepDataSelectMember>) -> i32 {
        let Some(member) = ent.get() else {
            return 0;
        };
        let kind = member.param_type();
        if kind != InterfaceParamType::ParamVoid && kind != InterfaceParamType::ParamReal {
            return 0;
        }
        member.matches("PARAMETER_VALUE") as i32
    }
}

impl StepGeomTrimmingSelect {
    /// `StepGeomTrimmingSelect()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `cartesian_point`: null when a parameter value is held.
    pub fn cartesian_point(&self) -> Handle<StepGeomCartesianPoint> {
        self.value().cast::<StepGeomCartesianPoint>()
    }
    /// `set_parameter_value`.
    ///
    /// # Errors
    /// a type mismatch failure from the select's member.
    pub fn set_parameter_value(&mut self, a_parameter_value: f64) -> Result<(), Failure> {
        self.set_real(a_parameter_value, "PARAMETER_VALUE")
    }
    /// `parameter_value`.
    pub fn parameter_value(&self) -> f64 {
        self.real()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_trimming_select_holds_a_point_or_a_parameter() {
        let mut select = StepGeomTrimmingSelect::new();
        select.set_parameter_value(0.25).unwrap();
        assert_eq!(select.parameter_value(), 0.25);
        assert!(select.cartesian_point().is_null());
        let point = Handle::new(StepGeomCartesianPoint::new());
        select.set_value(&point.cast()).unwrap();
        assert_eq!(select.cartesian_point(), point);
        assert_eq!(select.case_number(), 1);
    }
}
