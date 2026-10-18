//! Sampling and basis zones retain their initial operation-local boundaries.
use super::curve::IsoApproxRequest;
use super::curve_errors::{PieceErrors, mma1fer_};
use super::curve_integers::CurveIntegerOutputs;
use super::curve_steps::{PieceTables, mma1cdi_, mma1cnt_, mma1jak_};
use crate::adv_app2var::math_base::{
    PolynomialCoefficientSlots, mmapcmp_slots, mmjacan_slots, mmmpocur_,
};

pub(super) struct CurveProjectionFrame {
    values: Vec<f64>,
    trailing_flag: f64,
    sample_start: usize,
    hermite_start: usize,
    gauss_start: usize,
    coefficient_start: usize,
}
impl CurveProjectionFrame {
    #[cfg(test)]
    pub(super) fn new(gauss_length: usize, coefficient_length: usize) -> Self {
        Self::with_allocated_values(
            [0, 0, 0, gauss_length, coefficient_length],
            vec![0.; gauss_length + coefficient_length],
        )
    }
    #[cfg(test)]
    pub(super) fn with_prefix(
        samples: usize,
        hermite: usize,
        gauss: usize,
        coefficients: usize,
    ) -> Self {
        let mut frame = Self::new(samples + hermite + gauss, coefficients);
        frame.hermite_start = samples;
        frame.gauss_start = samples + hermite;
        frame
    }
    #[cfg(test)]
    pub(super) fn with_parameters(lengths: [usize; 5]) -> Self {
        let [parameters, samples, hermite, gauss, coefficients] = lengths;
        let mut frame = Self::with_prefix(parameters + samples, hermite, gauss, coefficients);
        frame.sample_start = parameters;
        frame
    }
    #[cfg(test)]
    pub(super) fn with_allocated_values(lengths: [usize; 5], values: Vec<f64>) -> Self {
        let [parameters, samples, hermite, gauss, _] = lengths;
        Self::with_zone_starts(
            [
                parameters,
                parameters + samples,
                parameters + samples + hermite,
                parameters + samples + hermite + gauss,
            ],
            values,
        )
    }
    pub(super) fn with_zone_starts(starts: [usize; 4], values: Vec<f64>) -> Self {
        Self {
            values,
            trailing_flag: -134744073.,
            sample_start: starts[0],
            hermite_start: starts[1],
            gauss_start: starts[2],
            coefficient_start: starts[3],
        }
    }
    pub(super) fn into_values(self) -> Vec<f64> {
        self.values
    }
    #[cfg(test)]
    pub(super) fn parameter_samples_mut(&mut self) -> (&mut [f64], &mut [f64]) {
        let (parameters, remainder) = self.values.split_at_mut(self.sample_start);
        let samples = self.hermite_start - self.sample_start;
        (parameters, &mut remainder[..samples])
    }
    pub(super) fn endpoint_parameter_storage_mut(&mut self) -> (&mut [f64], usize) {
        (&mut self.values, self.sample_start)
    }
    pub(super) fn parameter_storage_mut(&mut self) -> &mut [f64] {
        &mut self.values
    }
    pub(super) fn sample_subscript(&self, index: i64) -> f64 {
        // retained  decrements the sample pointer within its original owner.
        self.original_value(self.sample_start, index - 1)
    }
    fn original_value(&self, origin: usize, offset: i64) -> f64 {
        self.values[self.original_slot(origin, offset)]
    }
    fn original_slot(&self, origin: usize, offset: i64) -> usize {
        isize::try_from(offset)
            .ok()
            .and_then(|offset| origin.checked_add_signed(offset))
            .filter(|slot| *slot < self.values.len())
            .unwrap_or_else(|| panic!("MMA2fnc: origin={origin}, offset={offset}; expected initialized original-owner element in 0..{}", self.values.len()))
    }
    pub(super) fn coefficient_offset(&self, offset: i64) -> f64 {
        self.original_value(self.coefficient_start, offset)
    }
    pub(super) fn truncation_bound(&self, index: i32) -> f64 {
        self.original_value(self.sample_start, i64::from(index))
    }
    pub(super) fn set_truncation_bound(&mut self, index: i32, value: f64) {
        let slot = self.original_slot(self.sample_start, i64::from(index));
        self.values[slot] = value;
    }
    pub(super) fn truncation_bounds(&self) -> &[f64] {
        &self.values[self.sample_start..]
    }
    pub(super) fn callback_buffers(
        &mut self,
        first: usize,
        stride: usize,
    ) -> crate::adv_app2var::AdvApp2VarEvaluationBuffers<'_> {
        crate::adv_app2var::AdvApp2VarEvaluationBuffers::shared(
            &mut self.values,
            first,
            self.sample_start,
            self.sample_start + first * stride,
            self.hermite_start,
        )
    }
    #[cfg(test)]
    pub(super) fn samples_mut(&mut self) -> &mut [f64] {
        &mut self.values[self.sample_start..self.hermite_start]
    }
    pub(super) fn remove_constraints(
        &mut self,
        request: &IsoApproxRequest<'_>,
        piece: &mut PieceTables<'_>,
        error: &mut i32,
    ) {
        mma1cdi_(request, piece, self, error);
    }
    pub(super) fn hermite_storage_mut(&mut self) -> &mut [f64] {
        &mut self.values[self.hermite_start..]
    }
    pub(super) fn hermite_value(&self, index: usize) -> f64 {
        self.values[self.hermite_start + index]
    }
    pub(super) fn sample_value(&self, root: i32, column: i32, root_count: i32) -> f64 {
        self.values[self.sample_start + ((root - 1) + (column - 1) * root_count) as usize]
    }
    pub(super) fn sample_hermite_column(
        &mut self,
        coefficient_count: usize,
        column: usize,
        parameters: &[f64],
    ) {
        for (root, parameter) in parameters.iter().enumerate() {
            let start = self.hermite_start + (column - 1) * coefficient_count;
            // The validated Hermite orders 0..=2 use at most six coefficients.
            let mut coefficients = [0.; 6];
            coefficients[..coefficient_count]
                .copy_from_slice(&self.values[start..start + coefficient_count]);
            let output = self.sample_start + (column - 1) * parameters.len() + root;
            // One scalar polynomial reads every coefficient before its final write; the next call rereads live slots.
            mmmpocur_(
                coefficient_count as i32,
                1,
                coefficient_count as i32,
                &coefficients,
                *parameter,
                &mut self.values[output..],
            );
        }
    }
    pub(super) fn add_constraints(
        &mut self,
        request: &IsoApproxRequest<'_>,
        piece: &PieceTables<'_>,
    ) {
        mma1cnt_(request, piece, self);
    }
    pub(super) fn truncate<Integers: CurveIntegerOutputs>(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        errors: &mut PieceErrors<'_, Integers>,
        error: &mut i32,
    ) {
        mma1fer_(request, self, errors, error);
    }
    pub(super) fn compress(&mut self, dimension: i32, maximum: i32, kept: i32) {
        let mut slots = CurvePolynomialSlots {
            values: &mut self.values,
            input_start: self.coefficient_start,
            output_start: self.sample_start,
        };
        mmapcmp_slots(dimension, maximum, kept, &mut slots);
    }
    pub(super) fn canonical_component(&mut self, order: i32, degree: i32, component: i32) {
        let packed = (((component - 1) << 1) * (degree / 2 + 1)) as usize;
        let result = ((component - 1) * (degree + 1)) as usize;
        let mut slots = CurvePolynomialSlots {
            values: &mut self.values,
            input_start: self.sample_start + packed,
            output_start: self.coefficient_start + result,
        };
        mmjacan_slots(order, degree, &mut slots);
    }
    pub(super) fn coefficients(&self) -> &[f64] {
        &self.values[self.coefficient_start..]
    }
    #[cfg(test)]
    pub(super) fn coefficients_mut(&mut self) -> &mut [f64] {
        &mut self.values[self.coefficient_start..]
    }
    pub(super) fn gauss_storage_mut(&mut self) -> &mut [f64] {
        &mut self.values[self.gauss_start..]
    }
    pub(super) fn gauss_value(&self, index: usize) -> f64 {
        self.values[self.gauss_start + index]
    }
    pub(super) fn coefficient(&self, index: usize) -> f64 {
        self.values[self.coefficient_start + index]
    }
    pub(super) fn set_coefficient(&mut self, index: usize, value: f64) {
        self.values[self.coefficient_start + index] = value;
    }
    pub(super) fn project(
        &mut self,
        request: &IsoApproxRequest<'_>,
        piece: &PieceTables<'_>,
        error: &mut i32,
    ) {
        mma1jak_(request, piece, self, error);
    }
}
struct CurvePolynomialSlots<'a> {
    values: &'a mut [f64],
    input_start: usize,
    output_start: usize,
}
impl PolynomialCoefficientSlots for CurvePolynomialSlots<'_> {
    fn input_coefficient(&self, index: usize) -> f64 {
        self.values[self.input_start + index]
    }
    fn write_output_coefficient(&mut self, index: usize, value: f64) {
        self.values[self.output_start + index] = value;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn truncation_bound_writes_change_the_coefficient_cell_immediately() {
        let mut frame = CurveProjectionFrame::with_zone_starts([2, 4, 4, 5], vec![0.; 10]);
        frame.set_truncation_bound(3, 7.);
        assert_eq!(frame.coefficient_offset(0), 7.);
        assert_eq!(frame.truncation_bound(3), 7.);
        assert_eq!(&frame.truncation_bounds()[..4], &[0., 0., 0., 7.]);
        frame.set_coefficient(0, 9.);
        assert_eq!(frame.truncation_bound(3), 9.);
        assert_eq!(frame.coefficient_offset(-1), 0.);
    }
    #[test]
    fn sample_subscripts_retain_initialized_prefix_and_physical_bounds() {
        let mut frame = CurveProjectionFrame::with_parameters([2, 2, 0, 0, 0]);
        frame.values.copy_from_slice(&[9., 8., 7., 6.]);
        assert_eq!(frame.sample_subscript(-1), 9.);
        assert_eq!(frame.sample_subscript(0), 8.);
        assert_eq!(frame.sample_subscript(1), 7.);
        assert_eq!(frame.sample_subscript(2), 6.);
        for index in [-2, 3, i32::MIN as i64, i32::MAX as i64] {
            assert!(std::panic::catch_unwind(|| frame.sample_subscript(index)).is_err());
        }
    }
    #[test]
    fn polynomial_slot_writes_are_visible_through_the_same_input_slot() {
        let mut values = [0., 0., 2., 4.];
        let mut slots = CurvePolynomialSlots {
            values: &mut values,
            input_start: 2,
            output_start: 0,
        };
        assert_eq!(slots.input_coefficient(0), 2.);
        slots.write_output_coefficient(2, 9.);
        assert_eq!(slots.input_coefficient(0), 9.);
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CurvePolynomialSlots<'static>>();
    }
    #[test]
    fn projection_frame_preserves_initial_zone_lengths_and_zero_initialization() {
        let frame = CurveProjectionFrame::new(5, 10);
        assert_eq!(frame.values, [0.; 15]);
        assert_eq!(frame.coefficients(), [0.; 10]);
    }
    #[test]
    fn gauss_and_coefficient_views_observe_the_same_owned_slot_immediately() {
        let mut frame = CurveProjectionFrame::new(5, 10);
        frame.gauss_storage_mut()[7] = 3.;
        assert_eq!(frame.coefficient(2), 3.);
        frame.set_coefficient(2, 4.);
        assert_eq!(frame.gauss_value(7), 4.);
        assert_eq!(frame.values.len(), 15);
    }
    #[test]
    fn prefixed_frame_retains_all_four_initial_zone_boundaries() {
        let mut frame = CurveProjectionFrame::with_prefix(20, 4, 15, 10);
        assert_eq!(frame.values.len(), 49);
        assert_eq!(frame.samples_mut().len(), 20);
        assert_eq!(frame.gauss_storage_mut().len(), 25);
        frame.gauss_storage_mut()[15] = 2.;
        assert_eq!(frame.coefficients()[0], 2.);
        assert_eq!(
            (
                frame.hermite_start,
                frame.gauss_start,
                frame.coefficient_start
            ),
            (20, 24, 39)
        );
    }
    #[test]
    fn each_polynomial_call_observes_prior_sample_writes_into_the_live_basis() {
        let mut frame = CurveProjectionFrame::with_prefix(2, 4, 0, 2);
        frame.hermite_storage_mut()[..2].copy_from_slice(&[1., 1.]);
        frame.sample_hermite_column(2, 1, &[0., 1., 2., 3.]);
        assert_eq!(&frame.values[..4], &[1., 2., 3., 6.]);
        assert_eq!(frame.sample_value(4, 1, 4), 6.);
        assert_eq!(&frame.hermite_storage_mut()[..2], &[3., 6.]);
    }
    #[test]
    fn constraint_addition_reads_basis_updates_from_earlier_components() {
        let request = constraint_addition_request();
        let mut frame = CurveProjectionFrame::with_prefix(0, 2, 0, 4);
        frame.hermite_storage_mut()[..4].copy_from_slice(&[1., 2., 3., 4.]);
        let (mut left, mut right) = ([1.; 2], [1.; 2]);
        let piece = PieceTables {
            folded: crate::adv_app2var::approx_f2var::curve_folded_storage::FoldedPieceTables::new(
                &mut [],
                &mut [],
            ),
            endpoints:
                crate::adv_app2var::approx_f2var::curve_endpoint_storage::EndpointPieceTables::new(
                    &mut left, &mut right,
                ),
        };
        frame.add_constraints(&request, &piece);
        assert_eq!(frame.coefficients(), [4., 6., 5., 8.]);
        assert_eq!(frame.hermite_value(2), 4.);
    }
    fn constraint_addition_request() -> IsoApproxRequest<'static> {
        IsoApproxRequest {
            ndimen: 2,
            nbsesp: 1,
            ndimse: &[2],
            uvfonc: [0., 1., 0., 1.],
            tconst: 0.5,
            isofav: 1,
            nbroot: 0,
            rootlg: &[],
            iordre: 0,
            ideriv: 0,
            ndgjac: 1,
            nbcrmx: 1,
            ncflim: 2,
            epsapr: &[0.],
        }
    }
    #[test]
    fn complete_frame_keeps_parameter_prefix_out_of_projection_scratch() {
        let mut frame = CurveProjectionFrame::with_parameters([10, 20, 4, 5, 6]);
        let (parameters, samples) = frame.parameter_samples_mut();
        parameters.fill(9.);
        samples[0] = 2.;
        assert_eq!(frame.values.len(), 45);
        assert_eq!(frame.sample_value(1, 1, 8), 2.);
        assert_eq!(
            (
                frame.hermite_start,
                frame.gauss_start,
                frame.coefficient_start
            ),
            (30, 34, 39)
        );
        frame.coefficients_mut()[..2].copy_from_slice(&[3., 4.]);
        frame.compress(1, 2, 1);
        assert_eq!(&frame.values[..10], &[9.; 10]);
        assert_eq!(frame.sample_value(1, 1, 8), 3.);
    }
    #[test]
    fn externally_allocated_frame_returns_the_same_owned_buffer_without_copying() {
        let mut values = vec![7.; 45];
        let pointer = values.as_mut_ptr();
        let mut frame = CurveProjectionFrame::with_allocated_values([10, 20, 4, 5, 6], values);
        frame.coefficients_mut()[0] = 3.;
        let returned = frame.into_values();
        assert_eq!(returned.as_ptr(), pointer);
        assert_eq!(returned[39], 3.);
        assert_eq!(&returned[..39], &[7.; 39]);
    }
    #[test]
    fn nonmonotone_zone_origins_retain_owner_without_touching_unused_origin() {
        let mut frame = CurveProjectionFrame::with_zone_starts([10, 42, 58, 43], vec![7.; 44]);
        assert_eq!(frame.values.len(), 44);
        assert_eq!(frame.gauss_start, 58);
        frame.coefficients_mut()[0] = 3.;
        assert_eq!(frame.coefficient(0), 3.);
        assert_eq!(&frame.values[..43], &[7.; 43]);
    }
    #[test]
    fn mutable_coefficient_view_updates_only_the_initial_coefficient_zone() {
        let mut frame = CurveProjectionFrame::new(5, 10);
        frame.coefficients_mut()[0] = 2.;
        assert_eq!(frame.coefficients()[0], 2.);
        assert_eq!(&frame.values[..5], &[0.; 5]);
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CurveProjectionFrame>();
    }
}

#[path = "curve_workspace_clear.rs"]
mod coefficient_clear;
