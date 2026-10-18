//! Folded row access publishes each sum/difference store before the next read.
pub(crate) use crate::adv_app2var::AdvApp2VarFoldedOutput as FoldedTable;
use crate::adv_app2var::real_slots::OwnedFoldedSlots;
enum FoldedStorage<'a> {
    Borrowed([&'a mut [f64]; 2]),
    Owned(OwnedFoldedSlots),
}
pub(crate) struct FoldedPieceTables<'a> {
    storage: FoldedStorage<'a>,
}
impl<'a> FoldedPieceTables<'a> {
    pub(crate) fn new(sum: &'a mut [f64], difference: &'a mut [f64]) -> Self {
        Self {
            storage: FoldedStorage::Borrowed([sum, difference]),
        }
    }
    pub(crate) fn owned(slots: OwnedFoldedSlots) -> Self {
        Self {
            storage: FoldedStorage::Owned(slots),
        }
    }
    pub(crate) fn value(&self, table: FoldedTable, index: i32) -> f64 {
        match &self.storage {
            FoldedStorage::Borrowed(values) => values[table.index()][index as usize],
            FoldedStorage::Owned(slots) => slots.value(table, index).expect(
                "AdvApp2VarFoldedOutput: invalid read; expected initialized caller storage",
            ),
        }
    }
    pub(crate) fn set_value(&mut self, table: FoldedTable, index: i32, value: f64) {
        match &mut self.storage {
            FoldedStorage::Borrowed(values) => values[table.index()][index as usize] = value,
            FoldedStorage::Owned(slots) => slots.set_value(table, index, value).expect(
                "AdvApp2VarFoldedOutput: invalid write; expected initialized caller storage",
            ),
        }
    }
    pub(crate) fn with_table_values<ResultValue>(
        &mut self,
        table: FoldedTable,
        operation: impl FnOnce(&mut [f64]) -> ResultValue,
    ) -> ResultValue {
        match &mut self.storage {
            FoldedStorage::Borrowed(values) => operation(values[table.index()]),
            FoldedStorage::Owned(slots) => slots.with_values(table, operation).expect(
                "AdvApp2VarFoldedOutput: invalid scope; expected initialized caller storage",
            ),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_folded_rows_preserve_sequential_writes_and_exact_scoped_views() {
        let (mut sum, mut difference) = ([1., 2., 7.], [3., 4., 11.]);
        let mut folded = FoldedPieceTables::new(&mut sum, &mut difference);
        folded.set_value(FoldedTable::SymmetricSum, 1, -0.);
        assert_eq!(
            folded.value(FoldedTable::SymmetricSum, 1).to_bits(),
            (-0_f64).to_bits()
        );
        assert_eq!(folded.value(FoldedTable::AntisymmetricDifference, 1), 4.);
        let length = folded.with_table_values(FoldedTable::AntisymmetricDifference, |values| {
            values[1] = 5.;
            values.len()
        });
        assert_eq!(length, 3);
        assert_eq!(folded.value(FoldedTable::AntisymmetricDifference, 1), 5.);
        assert_eq!(
            sum.map(f64::to_bits),
            [1_f64.to_bits(), (-0_f64).to_bits(), 7_f64.to_bits()]
        );
        assert_eq!(difference, [3., 5., 11.]);
    }
}
