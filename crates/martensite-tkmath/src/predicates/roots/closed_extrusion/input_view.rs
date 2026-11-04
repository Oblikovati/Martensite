//! A private borrowed view lets one traversal retain the original owning input types.
#[derive(Clone, Copy)]
pub(super) struct QuadraticSplineProofView<'a> {
    pub poles: &'a [[f64; 3]],
    pub weights: &'a [f64],
    pub knots: &'a [f64],
    pub multiplicities: &'a [u32],
    pub direction: [f64; 3],
    pub interval: [f64; 2],
}
impl QuadraticSplineProofView<'_> {
    pub fn curve_interval(&self) -> [f64; 2] {
        [self.knots[0], self.knots[self.knots.len() - 1]]
    }
    pub fn flat_knots(&self) -> Vec<f64> {
        self.knots
            .iter()
            .zip(self.multiplicities)
            .flat_map(|(k, n)| std::iter::repeat_n(*k, *n as usize))
            .collect()
    }
}
