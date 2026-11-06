#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwWeekOfYearAndDayDate,
    "RwStepBasicRwWeekOfYearAndDayDate"
);
impl RwStepBasicRwWeekOfYearAndDayDate {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicWeekOfYearAndDayDate>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "week_of_year_and_day_date")? {
            return Ok(());
        }
        let mut a_year_component: i32 = Default::default();
        data.read_integer(num, 1, "year_component", ach, &mut a_year_component)?;
        let mut a_week_component: i32 = Default::default();
        data.read_integer(num, 2, "week_component", ach, &mut a_week_component)?;
        let mut a_day_component: i32 = Default::default();
        let mut has_aday_component: bool = true;
        if data.is_param_defined(num, 3)? {
            data.read_integer(num, 3, "day_component", ach, &mut a_day_component)?;
        } else {
            has_aday_component = false;
            a_day_component = 0;
        }
        ent.init(
            a_year_component,
            a_week_component,
            has_aday_component,
            a_day_component,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicWeekOfYearAndDayDate>,
    ) -> Result<(), Failure> {
        sw.send_integer(ent.year_component())?;
        sw.send_integer(ent.week_component())?;
        let has_aday_component: bool = ent.has_day_component();
        if has_aday_component {
            sw.send_integer(ent.day_component())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
}
