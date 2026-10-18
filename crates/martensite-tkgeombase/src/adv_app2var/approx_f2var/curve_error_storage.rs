//! Error output access keeps the scalar publication boundary explicit.
pub(crate) use crate::adv_app2var::AdvApp2VarErrorOutput as CurveErrorOutput;
use crate::adv_app2var::real_slots::OwnedErrorSlots;
enum ErrorStorage<'a> {
    Borrowed([&'a mut [f64]; 2]),
    Owned(OwnedErrorSlots),
}
pub(super) struct PieceErrorTables<'a> {
    storage: ErrorStorage<'a>,
}
impl<'a> PieceErrorTables<'a> {
    pub(super) fn new(maximum: &'a mut [f64], mean: &'a mut [f64]) -> Self {
        Self {
            storage: ErrorStorage::Borrowed([maximum, mean]),
        }
    }
    pub(super) fn owned(slots: OwnedErrorSlots) -> Self {
        Self {
            storage: ErrorStorage::Owned(slots),
        }
    }
    pub(super) fn set_error(&mut self, role: CurveErrorOutput, index: usize, value: f64) {
        match &mut self.storage {
            ErrorStorage::Borrowed(values)=>values[role.index()][index]=value,
            ErrorStorage::Owned(slots)=>slots.set_value(role,index as i32,value).expect("AdvApp2VarErrorOutput: invalid publication; expected initialized caller error slot"),
        }
    }
    pub(super) fn with_error<ResultValue>(
        &mut self,
        role: CurveErrorOutput,
        index: usize,
        operation: impl FnOnce(&mut f64) -> ResultValue,
    ) -> ResultValue {
        match &mut self.storage {
            ErrorStorage::Borrowed(values) => operation(&mut values[role.index()][index]),
            ErrorStorage::Owned(slots) => slots
                .with_values(role, |values| operation(&mut values[index]))
                .expect(
                    "AdvApp2VarErrorOutput: invalid scope; expected initialized caller error slot",
                ),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_error_scalars_preserve_roles_bits_and_ordered_publication() {
        let (mut maximum, mut mean) = ([7.; 2], [11.; 2]);
        let mut errors = PieceErrorTables::new(&mut maximum, &mut mean);
        assert_eq!(
            errors.with_error(CurveErrorOutput::Maximum, 1, |value| {
                let old = *value;
                *value = -0.;
                old
            }),
            7.
        );
        errors.with_error(CurveErrorOutput::RootMeanSquare, 0, |value| *value = 3.);
        assert_eq!(
            maximum.map(f64::to_bits),
            [7_f64.to_bits(), (-0_f64).to_bits()]
        );
        assert_eq!(mean, [3., 11.]);
    }
}
