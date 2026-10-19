//! Mod behavior and supporting operations.
use martensite_tkgeombase::adv_app2var::approx_f2var::{IsoApproxOutput, IsoApproxRequest};
mod evaluator;
mod padded;
pub(super) use evaluator::ZeroEvaluator;
pub(super) use padded::bits;
use padded::{Padded, expect_real_owner};
pub(super) const ROOTS: [f64; 8] = [
    -0.9602898564975363,
    -0.7966664774136267,
    -0.525532409916329,
    -0.1834346424956498,
    0.1834346424956498,
    0.525532409916329,
    0.7966664774136267,
    0.9602898564975363,
];
#[derive(Clone, Copy)]
pub(super) struct ZeroCase {
    pub(super) dimension: i32,
    pub(super) order: i32,
    pub(super) degree: i32,
    pub(super) limit: i32,
    pub(super) partitions: &'static [i32],
    pub(super) retained: i32,
}
#[derive(Clone)]
pub(super) struct ZeroInputs {
    partitions: Padded<i32>,
    roots: Padded<f64>,
    tolerance: Padded<f64>,
}
impl ZeroInputs {
    pub(super) fn new(case: ZeroCase) -> Self {
        let mut partitions = Padded::new(case.partitions.len(), -17);
        partitions.payload_mut().copy_from_slice(case.partitions);
        let mut roots = Padded::new(8, 13.);
        roots.payload_mut().copy_from_slice(&ROOTS);
        let mut tolerance = Padded::new(case.partitions.len(), 13.);
        tolerance.payload_mut().fill(1e-9);
        Self {
            partitions,
            roots,
            tolerance,
        }
    }
    pub(super) fn request(&self, case: ZeroCase) -> IsoApproxRequest<'_> {
        IsoApproxRequest {
            ndimen: case.dimension,
            nbsesp: case.partitions.len() as i32,
            ndimse: self.partitions.payload(),
            uvfonc: [0., 1., 0., 1.],
            tconst: 0.5,
            isofav: 1,
            nbroot: 8,
            rootlg: self.roots.payload(),
            iordre: case.order,
            ideriv: 0,
            ndgjac: case.degree,
            nbcrmx: 1,
            ncflim: case.limit,
            epsapr: self.tolerance.payload(),
        }
    }
    pub(super) fn assert_borrowed_inputs(&self, request: &IsoApproxRequest<'_>) {
        assert!(std::ptr::eq(request.ndimse, self.partitions.payload()));
        assert!(std::ptr::eq(request.rootlg, self.roots.payload()));
        assert!(std::ptr::eq(request.epsapr, self.tolerance.payload()));
    }
    pub(super) fn assert_unchanged(&self, original: &Self) {
        assert_eq!(self.partitions, original.partitions);
        assert_eq!(bits(&self.roots.values), bits(&original.roots.values));
        assert_eq!(
            bits(&self.tolerance.values),
            bits(&original.tolerance.values)
        );
    }
}
pub(super) struct ZeroOutputs {
    counts: Padded<i32>,
    curve: Padded<f64>,
    sums: Padded<f64>,
    differences: Padded<f64>,
    lower: Padded<f64>,
    upper: Padded<f64>,
    cuts: Padded<f64>,
    maximum: Padded<f64>,
    mean: Padded<f64>,
}
impl ZeroOutputs {
    pub(super) fn new(case: ZeroCase) -> Self {
        let dimension = case.dimension as usize;
        Self {
            counts: Padded::new(1, -7),
            curve: Padded::new(dimension * case.limit as usize, 7.),
            sums: Padded::new(5 * dimension, 7.),
            differences: Padded::new(5 * dimension, 7.),
            lower: Padded::new(dimension * (case.order + 2) as usize, 7.),
            upper: Padded::new(dimension * (case.order + 2) as usize, 7.),
            cuts: Padded::new(2, 7.),
            maximum: Padded::new(case.partitions.len(), 7.),
            mean: Padded::new(case.partitions.len(), 7.),
        }
    }
    pub(super) fn borrow(&mut self) -> IsoApproxOutput<'_> {
        IsoApproxOutput {
            ncoeff: self.counts.payload_mut(),
            courbe: self.curve.payload_mut(),
            nbcrbe: -7,
            somtab: self.sums.payload_mut(),
            diftab: self.differences.payload_mut(),
            contr1: self.lower.payload_mut(),
            contr2: self.upper.payload_mut(),
            tabdec: self.cuts.payload_mut(),
            errmax: self.maximum.payload_mut(),
            errmoy: self.mean.payload_mut(),
        }
    }
    pub(super) fn assert_expected(&self, case: ZeroCase) {
        let mut counts = Padded::new(1, -7);
        counts.payload_mut()[0] = case.retained;
        assert_eq!(self.counts, counts);
        for owner in [
            &self.curve,
            &self.sums,
            &self.differences,
            &self.maximum,
            &self.mean,
        ] {
            expect_real_owner(owner, &vec![0.; owner.payload().len()], 7.);
        }
        for owner in [&self.lower, &self.upper] {
            expect_real_owner(owner, &vec![7.; owner.payload().len()], 7.);
        }
        expect_real_owner(&self.cuts, &[0., 1.], 7.);
    }
}
