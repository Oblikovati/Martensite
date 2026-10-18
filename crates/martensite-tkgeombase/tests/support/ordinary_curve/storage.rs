//! Full original owner comparisons include prefixes, spare rows, and unconsumed pieces.
use super::*;
pub struct OriginalOutputs {
    counts: Vec<i32>,
    tables: [Vec<f64>; 8],
}
pub fn lengths(spec: &CaseSpec) -> [usize; 8] {
    [
        spec.limit * 2 * spec.maximum,
        (spec.roots / 2 + 1) * 2 * spec.maximum,
        (spec.roots / 2 + 1) * 2 * spec.maximum,
        2 * (spec.order + 2) as usize * spec.maximum,
        2 * (spec.order + 2) as usize * spec.maximum,
        spec.maximum + 1,
        2 * spec.maximum,
        2 * spec.maximum,
    ]
}
impl OriginalOutputs {
    pub fn new(spec: &CaseSpec) -> Self {
        Self {
            counts: vec![-7; PREFIX + spec.maximum + SUFFIX],
            tables: lengths(spec).map(|length| vec![7.; PREFIX + length + SUFFIX]),
        }
    }
    pub fn output(&mut self) -> IsoApproxOutput<'_> {
        let [curve, sums, differences, lower, upper, cuts, maximum, mean] = &mut self.tables;
        IsoApproxOutput {
            ncoeff: &mut self.counts[PREFIX..],
            courbe: &mut curve[PREFIX..],
            nbcrbe: -7,
            somtab: &mut sums[PREFIX..],
            diftab: &mut differences[PREFIX..],
            contr1: &mut lower[PREFIX..],
            contr2: &mut upper[PREFIX..],
            tabdec: &mut cuts[PREFIX..],
            errmax: &mut maximum[PREFIX..],
            errmoy: &mut mean[PREFIX..],
        }
    }
    pub fn check_complete_owner(&self, spec: &CaseSpec, expected: &RetainedObservation) {
        super::independent::check(spec, &self.tables);
        assert_eq!(self.counts, expected.counts, "{} counts", spec.name);
        for (index, values) in self.tables.iter().enumerate() {
            assert_eq!(values.len(), expected.tables[index].len());
            for (slot, (actual, retained)) in values.iter().zip(&expected.tables[index]).enumerate()
            {
                assert_eq!(
                    actual.to_bits(),
                    retained.to_bits(),
                    "{} {} slot={slot}, actual={actual:.17e}, retained={retained:.17e}",
                    spec.name,
                    TABLE_NAMES[index]
                );
            }
        }
    }
}
