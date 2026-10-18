//! Mod behavior and supporting operations.
use martensite_tkgeombase::adv_app2var::approx_f2var::{IsoApproxOutput, IsoApproxRequest};
#[path = "../bivariate_zero_owners/padded.rs"]
mod padded;
pub(super) use padded::bits;
use padded::{Padded, expect_real_owner};
#[derive(Clone)]
pub(super) struct LimitInputs {
    partitions: Padded<i32>,
    roots: Padded<f64>,
    tolerance: Padded<f64>,
}
impl LimitInputs {
    pub(super) fn new() -> Self {
        let mut partitions = Padded::new(1, -17);
        partitions.payload_mut()[0] = 1;
        let mut roots = Padded::new(8, 13.);
        roots.payload_mut().copy_from_slice(&[
            -0.9602898564975363,
            -0.7966664774136267,
            -0.525532409916329,
            -0.1834346424956498,
            0.1834346424956498,
            0.525532409916329,
            0.7966664774136267,
            0.9602898564975363,
        ]);
        let mut tolerance = Padded::new(1, 13.);
        tolerance.payload_mut()[0] = 1e-9;
        Self {
            partitions,
            roots,
            tolerance,
        }
    }
    pub(super) fn request(&self, fail_evaluator: bool) -> IsoApproxRequest<'_> {
        IsoApproxRequest {
            ndimen: 1,
            nbsesp: 1,
            ndimse: self.partitions.payload(),
            uvfonc: [0., if fail_evaluator { 1. } else { 0. }, 0., 1.],
            tconst: if fail_evaluator { 0.5 } else { 0. },
            isofav: 1,
            nbroot: 8,
            rootlg: self.roots.payload(),
            iordre: -1,
            ideriv: 0,
            ndgjac: 0,
            nbcrmx: i32::from(fail_evaluator),
            ncflim: i32::from(!fail_evaluator),
            epsapr: self.tolerance.payload(),
        }
    }
    pub(super) fn assert_unchanged(&self, original: &Self, request: &IsoApproxRequest<'_>) {
        assert_eq!(self.partitions, original.partitions);
        assert_eq!(bits(&self.roots.values), bits(&original.roots.values));
        assert_eq!(
            bits(&self.tolerance.values),
            bits(&original.tolerance.values)
        );
        assert!(std::ptr::eq(request.ndimse, self.partitions.payload()));
        assert!(std::ptr::eq(request.rootlg, self.roots.payload()));
        assert!(std::ptr::eq(request.epsapr, self.tolerance.payload()));
    }
}
pub(super) struct LimitOutputs {
    counts: Padded<i32>,
    tables: [Padded<f64>; 8],
}
impl LimitOutputs {
    pub(super) fn new(fail_evaluator: bool) -> Self {
        let maximum = usize::from(fail_evaluator);
        let lengths = [0, 5, 5, 1, 1, maximum + 1, maximum, maximum];
        Self {
            counts: Padded::new(maximum, -7),
            tables: lengths.map(|length| Padded::new(length, 7.)),
        }
    }

    pub(super) fn borrow(&mut self) -> IsoApproxOutput<'_> {
        let [curve, sums, differences, lower, upper, cuts, maximum, mean] = &mut self.tables;
        IsoApproxOutput {
            ncoeff: self.counts.payload_mut(),
            courbe: curve.payload_mut(),
            nbcrbe: -7,
            somtab: sums.payload_mut(),
            diftab: differences.payload_mut(),
            contr1: lower.payload_mut(),
            contr2: upper.payload_mut(),
            tabdec: cuts.payload_mut(),
            errmax: maximum.payload_mut(),
            errmoy: mean.payload_mut(),
        }
    }
    pub(super) fn assert_expected(&self, fail_evaluator: bool) {
        assert_eq!(self.counts, Padded::new(usize::from(fail_evaluator), -7));
        for (index, owner) in self.tables.iter().enumerate() {
            let expected = if index == 5 && fail_evaluator {
                vec![0., 1.]
            } else {
                vec![7.; owner.payload().len()]
            };
            expect_real_owner(owner, &expected, 7.);
        }
    }
    pub(super) fn assert_warning(&self) {
        let mut counts = Padded::new(1, -7);
        counts.payload_mut()[0] = 0;
        assert_eq!(self.counts, counts);
        for (index, owner) in self.tables.iter().enumerate() {
            let expected = match index {
                5 => vec![0., 1.],
                1 | 2 | 6 | 7 => vec![0.; owner.payload().len()],
                _ => vec![7.; owner.payload().len()],
            };
            expect_real_owner(owner, &expected, 7.);
        }
    }
}
