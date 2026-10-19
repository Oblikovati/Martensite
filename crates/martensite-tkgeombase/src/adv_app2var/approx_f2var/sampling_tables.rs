//! Sequential table access permits one physical owner without simultaneous aliases.
use super::SquareTables;
use crate::adv_app2var::AdvApp2VarEvaluationBuffers;
#[derive(Clone, Copy)]
pub(crate) enum SquareTableRole {
    Soso,
    Diso,
    Sodi,
    Didi,
}
pub(crate) trait SquareTableStorage {
    #[cfg(test)]
    fn table<'a>(
        &'a self,
        role: SquareTableRole,
        buffers: &'a AdvApp2VarEvaluationBuffers<'_>,
    ) -> &'a [f64] {
        self.table_at(role, 0, buffers)
    }
    #[cfg(test)]
    fn table_mut<'a>(
        &'a mut self,
        role: SquareTableRole,
        buffers: &'a mut AdvApp2VarEvaluationBuffers<'_>,
    ) -> &'a mut [f64] {
        self.table_at_mut(role, 0, buffers)
    }
    fn table_at<'a>(
        &'a self,
        role: SquareTableRole,
        offset: isize,
        buffers: &'a AdvApp2VarEvaluationBuffers<'_>,
    ) -> &'a [f64];
    fn table_at_mut<'a>(
        &'a mut self,
        role: SquareTableRole,
        offset: isize,
        buffers: &'a mut AdvApp2VarEvaluationBuffers<'_>,
    ) -> &'a mut [f64];
}
pub(crate) struct BorrowedSquareTables<'a, 'b>(pub(crate) &'a mut SquareTables<'b>);
impl SquareTableStorage for BorrowedSquareTables<'_, '_> {
    fn table_at<'a>(
        &'a self,
        role: SquareTableRole,
        offset: isize,
        _: &'a AdvApp2VarEvaluationBuffers<'_>,
    ) -> &'a [f64] {
        let table: &[f64] = match role {
            SquareTableRole::Soso => self.0.sosotb,
            SquareTableRole::Diso => self.0.disotb,
            SquareTableRole::Sodi => self.0.soditb,
            SquareTableRole::Didi => self.0.diditb,
        };
        let start = borrowed_table_start(offset);
        &table[start..]
    }
    fn table_at_mut<'a>(
        &'a mut self,
        role: SquareTableRole,
        offset: isize,
        _: &'a mut AdvApp2VarEvaluationBuffers<'_>,
    ) -> &'a mut [f64] {
        let table: &mut [f64] = match role {
            SquareTableRole::Soso => self.0.sosotb,
            SquareTableRole::Diso => self.0.disotb,
            SquareTableRole::Sodi => self.0.soditb,
            SquareTableRole::Didi => self.0.diditb,
        };
        let start = borrowed_table_start(offset);
        &mut table[start..]
    }
}
fn borrowed_table_start(offset: isize) -> usize {
    usize::try_from(offset).unwrap_or_else(|_| panic!("borrowed square table offset={offset}; expected a nonnegative offset or an owned original allocation for prefix access"))
}
pub(super) struct SamplingTableLayout {
    full: [i32; 2],
    half: [i32; 2],
}
impl SamplingTableLayout {
    pub(super) fn new(counts: [i32; 2]) -> Self {
        Self {
            full: [counts[0] / 2 + 1, counts[1] / 2 + 1],
            half: [counts[0] / 2, counts[1] / 2],
        }
    }
    pub(super) fn f(&self, i: i32, j: i32, nd: i32) -> isize {
        let [u, v] = self.full.map(i64::from);
        let native = i64::from(i) + (i64::from(j) + i64::from(nd) * v) * u;
        (native - u * v) as isize
    }
    pub(super) fn h(&self, i: i32, j: i32, nd: i32) -> isize {
        let [u, v] = self.half.map(i64::from);
        let native = i64::from(i) + (i64::from(j) + i64::from(nd) * v) * u;
        (native - (u * (v + 1) + 1)) as isize
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_offsets_preserve_valid_values_and_reject_negative_origins() {
        assert_eq!(borrowed_table_start(0), 0);
        assert_eq!(borrowed_table_start(2), 2);
        for offset in [-1, isize::MIN] {
            assert!(std::panic::catch_unwind(|| borrowed_table_start(offset)).is_err());
        }
    }
    #[test]
    fn table_layout_matches_physical_full_and_half_origins() {
        let layout = SamplingTableLayout::new([5, 7]);
        assert_eq!(layout.f(0, 0, 1), 0);
        assert_eq!(layout.f(2, 3, 2), 23);
        assert_eq!(layout.h(1, 1, 1), 0);
        assert_eq!(layout.h(2, 3, 2), 11);
    }
    #[test]
    fn signed_layout_preserves_full_and_half_retained_prefix_offsets() {
        let layout = SamplingTableLayout::new([-5, 3]);
        assert_eq!(layout.f(0, 1, 1), -1);
        assert_eq!(layout.f(0, 0, 2), -2);
        assert_eq!(layout.h(1, 1, 1), 0);
        assert_eq!(layout.h(1, 1, 2), -2);
        let extreme = SamplingTableLayout::new([i32::MIN, 0]);
        assert_eq!(extreme.f(0, 0, 2), -1_073_741_823);
        assert_eq!(extreme.h(1, 1, 1), 0);
    }
    #[test]
    fn borrowed_roles_return_actual_storage_and_preserve_other_tables() {
        let (mut a, mut b, mut c, mut d) = ([1.; 4], [2.; 4], [3.; 4], [4.; 4]);
        let mut tables = SquareTables {
            sosotb: &mut a,
            disotb: &mut b,
            soditb: &mut c,
            diditb: &mut d,
        };
        let mut storage = BorrowedSquareTables(&mut tables);
        let mut parameters = [];
        let mut values = [];
        let mut buffers = AdvApp2VarEvaluationBuffers::separate(&mut parameters, &mut values, 0);
        for (role, value) in [
            (SquareTableRole::Soso, 1.),
            (SquareTableRole::Diso, 2.),
            (SquareTableRole::Sodi, 3.),
            (SquareTableRole::Didi, 4.),
        ] {
            assert_eq!(storage.table(role, &buffers), &[value; 4]);
            storage.table_mut(role, &mut buffers)[1] = value + 10.;
            assert_eq!(storage.table(role, &buffers)[1], value + 10.);
        }
    }
}
