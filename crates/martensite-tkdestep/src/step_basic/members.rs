//! The select members of `StepBasic` and the selects that hold immediate values through them.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_data::{
    StepDataSelectMember, StepDataSelectNamed, StepDataSelectReal, StepDataSelectType,
    StepDataSelectValue,
};
use crate::step_entity::*;
use crate::{impl_step_select_storage, step_select_case};
use martensite_tkernel::impl_transient;

/// The measure names by case number, from 1, in the order.
const MEASURE_NAMES: [&str; 16] = [
    "LENGTH_MEASURE",
    "TIME_MEASURE",
    "PLANE_ANGLE_MEASURE",
    "SOLID_ANGLE_MEASURE",
    "RATIO_MEASURE",
    "PARAMETER_VALUE",
    "CONTEXT_DEPENDANT_MEASURE",
    "POSITIVE_LENGTH_MEASURE",
    "POSITIVE_PLANE_ANGLE_MEASURE",
    "POSITIVE_RATIO_MEASURE",
    "AREA_MEASURE",
    "VOLUME_MEASURE",
    "MASS_MEASURE",
    "THERMODYNAMIC_TEMPERATURE_MEASURE",
    "COUNT_MEASURE",
    "NUMERIC_MEASURE",
];

/// `StepBasicMeasureValueMember`: a real named by one of the measure names.
///
/// The `set_name` prefilters on one letter of the name before comparing it whole, so it is
/// the exact match this table makes.
#[derive(Default)]
pub struct StepBasicMeasureValueMember {
    base: StepDataSelectReal,
    thecase: SharedState<usize>,
}
impl_transient!(
    StepBasicMeasureValueMember,
    "StepBasicMeasureValueMember",
    StepDataSelectReal,
    base,
    interfaces: [dyn StepDataSelectMember]
);

impl StepDataSelectMember for StepBasicMeasureValueMember {
    fn has_name(&self) -> bool {
        *self.thecase.read() > 0
    }
    fn name(&self) -> String {
        let case = *self.thecase.read();
        case.checked_sub(1)
            .and_then(|rank| MEASURE_NAMES.get(rank))
            .map_or_else(String::new, ToString::to_string)
    }
    fn set_name(&self, name: &str) -> bool {
        if name.is_empty() {
            *self.thecase.write() = 0;
            return true;
        }
        let Some(rank) = MEASURE_NAMES.iter().position(|known| *known == name) else {
            return false;
        };
        *self.thecase.write() = rank + 1;
        true
    }
    step_real_member_values!(base);
}

impl StepBasicMeasureValueMember {
    /// `StepBasicMeasureValueMember()`: no name yet.
    pub fn new() -> Self {
        Self::default()
    }
}

step_fixed_name_member!(
    /// `StepBasicSizeMember`: a real named `POSITIVE_LENGTH_MEASURE`.
    StepBasicSizeMember,
    "StepBasicSizeMember",
    "POSITIVE_LENGTH_MEASURE"
);

/// `StepBasicSizeSelect`: a size member or an immediate real named `POSITIVE_LENGTH_MEASURE`.
#[derive(Clone, Default)]
pub struct StepBasicSizeSelect {
    value: StepDataSelectValue,
}

impl StepDataSelectType for StepBasicSizeSelect {
    impl_step_select_storage!(value);
    fn case_num(&self, ent: &Handle<StandardTransient>) -> i32 {
        step_select_case!(ent; StepBasicSizeMember)
    }
    fn new_member(&self) -> Handle<dyn StepDataSelectMember> {
        Handle::new(StepBasicSizeMember::new()).cast()
    }
    fn case_mem(&self, ent: &Handle<dyn StepDataSelectMember>) -> i32 {
        ent.get()
            .is_some_and(|member| member.matches("POSITIVE_LENGTH_MEASURE")) as i32
    }
}

impl StepBasicSizeSelect {
    /// `StepBasicSizeSelect()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `set_real_value`.
    ///
    /// # Errors
    /// a type mismatch failure from the select's member.
    pub fn set_real_value(&mut self, a_real_value: f64) -> Result<(), Failure> {
        self.set_real(a_real_value, "POSITIVE_LENGTH_MEASURE")
    }
    /// `real_value`.
    pub fn real_value(&self) -> f64 {
        self.real()
    }
}

/// `StepBasicSourceItem`: an identifier held as a named select member.
#[derive(Clone, Default)]
pub struct StepBasicSourceItem {
    value: StepDataSelectValue,
}

impl StepDataSelectType for StepBasicSourceItem {
    impl_step_select_storage!(value);
    fn case_num(&self, ent: &Handle<StandardTransient>) -> i32 {
        step_select_case!(ent; StepDataSelectNamed)
    }
    fn new_member(&self) -> Handle<dyn StepDataSelectMember> {
        Handle::new(StepDataSelectNamed::default()).cast()
    }
}

impl StepBasicSourceItem {
    /// `StepBasicSourceItem()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `identifier`: the stored value as a string, null when it holds a member.
    pub fn identifier(&self) -> Handle<HAsciiString> {
        self.value().cast::<HAsciiString>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_measure_member_knows_only_the_listed_names() {
        let member = StepBasicMeasureValueMember::new();
        assert!(!member.has_name());
        assert_eq!(member.name(), "");
        assert!(member.set_name("AREA_MEASURE"));
        assert_eq!(member.name(), "AREA_MEASURE");
        assert!(!member.set_name("AREA"));
        assert_eq!(member.name(), "AREA_MEASURE");
        assert!(member.set_name(""));
        assert!(!member.has_name());
        member.set_real(2.5);
        assert_eq!(member.real(), 2.5);
    }

    #[test]
    fn a_size_member_keeps_its_name() {
        let member = StepBasicSizeMember::new();
        assert!(member.set_name("OTHER"));
        assert_eq!(member.name(), "POSITIVE_LENGTH_MEASURE");
        assert!(member.has_name());
    }

    #[test]
    fn a_size_select_holds_a_real_through_its_member() {
        let mut select = StepBasicSizeSelect::new();
        select.set_real_value(1.5).unwrap();
        assert_eq!(select.real_value(), 1.5);
        assert_eq!(select.case_member(), 1);
        assert_eq!(select.select_name(), "POSITIVE_LENGTH_MEASURE");
    }

    #[test]
    fn a_source_item_makes_named_members() {
        let item = StepBasicSourceItem::new();
        assert!(item.new_member().is_kind_name("StepDataSelectNamed"));
        assert!(item.identifier().is_null());
    }
}
