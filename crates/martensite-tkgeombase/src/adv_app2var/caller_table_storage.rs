//! Checked table access reuses operation-local real ownership for each role.
use super::*;
pub(crate) trait CallerRealTableRole {
    const LABEL: &'static str;
    const POISON_MESSAGE: &'static str;
    fn index(self) -> usize;
    fn slot_failure(failure: Failure, _origin: usize, _index: i32, _capacity: usize) -> Failure {
        failure
    }
}
pub(crate) struct OwnedCallerTableSlots<Role, const COUNT: usize, Scalar = f64> {
    pub(super) values: Arc<Mutex<Vec<Scalar>>>,
    origins: [usize; COUNT],
    offset: i32,
    role: std::marker::PhantomData<Role>,
}
impl<Role: CallerRealTableRole, const COUNT: usize, Scalar: Copy>
    OwnedCallerTableSlots<Role, COUNT, Scalar>
{
    pub(in crate::adv_app2var) fn new(
        values: Arc<Mutex<Vec<Scalar>>>,
        origins: [usize; COUNT],
        offset: i32,
    ) -> Self {
        Self {
            values,
            origins,
            offset,
            role: std::marker::PhantomData,
        }
    }
    fn logical_index(&self, index: i32) -> Result<i32, Failure> {
        self.offset.checked_add(index).ok_or_else(|| {
            Failure::with_message(
                "Standard_OutOfRange",
                &format!(
                    "{}: piece offset={}, index={index}; expected logical sum inside i32 range",
                    Role::LABEL,
                    self.offset
                ),
            )
        })
    }
    fn checked_slot(&self, table: Role, index: i32, capacity: usize) -> Result<usize, Failure> {
        let origin = self.origins[table.index()];
        let logical = self.logical_index(index)?;
        crate::adv_app2var::real_slots::checked_real_slot(origin, logical, capacity)
            .map_err(|failure| Role::slot_failure(failure, origin, logical, capacity))
    }
    pub(crate) fn value(&self, table: Role, index: i32) -> Result<Scalar, Failure> {
        let values = self.values.lock().expect(Role::POISON_MESSAGE);
        Ok(values[self.checked_slot(table, index, values.len())?])
    }
    pub(crate) fn set_value(
        &mut self,
        table: Role,
        index: i32,
        value: Scalar,
    ) -> Result<(), Failure> {
        let mut values = self.values.lock().expect(Role::POISON_MESSAGE);
        let slot = self.checked_slot(table, index, values.len())?;
        values[slot] = value;
        Ok(())
    }
    pub(crate) fn with_values<ResultValue>(
        &mut self,
        table: Role,
        operation: impl FnOnce(&mut [Scalar]) -> ResultValue,
    ) -> Result<ResultValue, Failure> {
        let mut values = self.values.lock().expect(Role::POISON_MESSAGE);
        let start = crate::adv_app2var::real_slots::checked_real_range(
            self.origins[table.index()],
            self.offset,
            0,
            values.len(),
        )?
        .start;
        // Only pure numerical kernels execute here; evaluator dispatch holds no owner guard.
        Ok(operation(&mut values[start..]))
    }
}

#[cfg(test)]
#[path = "caller_table_storage_tests.rs"]
mod scalar_tests;
