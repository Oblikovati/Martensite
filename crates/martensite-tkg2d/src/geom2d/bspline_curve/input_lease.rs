//! Owned planar spline fields copied while the actual curve state is leased.
use super::{BSplineState, Geom2dBSplineCurve};
use martensite_tkernel::standard::SharedReadFailure;
use std::sync::RwLockReadGuard;

/// A planar B-spline read lease; obtain it with `curve.try_input_lease()?`.
pub struct PlanarSplineInputLease<'a> {
    fields: RwLockReadGuard<'a, BSplineState>,
}

/// Immutable stored planar spline fields, including evaluator override and array layout.
#[derive(Clone, Debug, PartialEq)]
pub struct StoredPlanarSplineInput {
    poles: Vec<[f64; 2]>,
    weights: Vec<f64>,
    knots: Vec<f64>,
    multiplicities: Vec<i32>,
    flat_knots: Vec<f64>,
    domain: [f64; 2],
    degree: i32,
    periodic: bool,
    rational: bool,
    evaluation_override: bool,
    lower_bounds: [i32; 5],
}

impl Geom2dBSplineCurve {
    /// Borrow stored evaluator state without waiting; `curve.try_input_lease()?.stored_input()`.
    pub fn try_input_lease(&self) -> Result<PlanarSplineInputLease<'_>, SharedReadFailure> {
        Ok(PlanarSplineInputLease {
            fields: self.state.try_read()?,
        })
    }
}

impl PlanarSplineInputLease<'_> {
    /// Read allocation sizes in poles/weights/knots/multiplicities/flat-knots order;
    /// `lease.stored_sizes()`.
    pub fn stored_sizes(&self) -> [usize; 5] {
        [
            self.fields.poles.len(),
            self.fields.weights.len(),
            self.fields.knots.len(),
            self.fields.mults.len(),
            self.fields.flat_knots.len(),
        ]
    }

    /// Copy actual stored fields without evaluation or normalization; `lease.stored_input()`.
    pub fn stored_input(&self) -> StoredPlanarSplineInput {
        copy_stored_input(&self.fields)
    }
}

impl StoredPlanarSplineInput {
    /// Read the captured poles; `input.poles()`.
    pub fn poles(&self) -> &[[f64; 2]] {
        &self.poles
    }
    /// Read stored weights, including generated unit weights; `input.weights()`.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    /// Read distinct stored knots; `input.knots()`.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }
    /// Read stored knot multiplicities; `input.multiplicities()`.
    pub fn multiplicities(&self) -> &[i32] {
        &self.multiplicities
    }
    /// Read the actual expanded knot sequence; `input.flat_knots()`.
    pub fn flat_knots(&self) -> &[f64] {
        &self.flat_knots
    }
    /// Read the evaluator domain; `input.domain()`.
    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }
    /// Read degree; `input.degree()`.
    pub fn degree(&self) -> i32 {
        self.degree
    }
    /// Read periodic state; `input.periodic()`.
    pub fn periodic(&self) -> bool {
        self.periodic
    }
    /// Read rational state; `input.rational()`.
    pub fn rational(&self) -> bool {
        self.rational
    }
    /// Read whether an evaluation override is installed; `input.evaluation_override()`.
    pub fn evaluation_override(&self) -> bool {
        self.evaluation_override
    }
    /// Read all original array lower bounds; `input.lower_bounds()`.
    pub fn lower_bounds(&self) -> [i32; 5] {
        self.lower_bounds
    }
}

fn copy_stored_input(fields: &BSplineState) -> StoredPlanarSplineInput {
    StoredPlanarSplineInput {
        poles: fields
            .poles
            .iter()
            .map(|point| [point.x(), point.y()])
            .collect(),
        weights: fields.weights.as_slice().to_vec(),
        knots: fields.knots.as_slice().to_vec(),
        multiplicities: fields.mults.as_slice().to_vec(),
        flat_knots: fields.flat_knots.as_slice().to_vec(),
        domain: [fields.first_parameter(), fields.last_parameter()],
        degree: fields.deg,
        periodic: fields.periodic,
        rational: fields.rational,
        evaluation_override: !fields.eval_rep.is_null(),
        lower_bounds: [
            fields.poles.lower(),
            fields.weights.lower(),
            fields.knots.lower(),
            fields.mults.lower(),
            fields.flat_knots.lower(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::ncollection::Array1;
    use martensite_tkmath::gp::Point2;

    fn curve() -> Geom2dBSplineCurve {
        let pole_values = [
            Point2::new(1., 2.),
            Point2::new(3., 4.),
            Point2::new(5., 6.),
        ];
        let poles = Array1::from_shared(&pole_values, 4);
        let weights = Array1::from_shared(&[1., 2., 1.], 4);
        let knots = Array1::from_shared(&[0., 1.], 7);
        let mults = Array1::from_shared(&[3, 3], 7);
        Geom2dBSplineCurve::with_weights(&poles, &weights, &knots, &mults, 2, false).unwrap()
    }

    #[test]
    fn planar_spline_lease_copies_every_stored_field_and_holds_writer() {
        let curve = curve();
        let lease = curve.try_input_lease().unwrap();
        assert_eq!(lease.stored_sizes(), [3, 3, 2, 2, 6]);
        let input = lease.stored_input();
        assert_eq!(input.poles(), &[[1., 2.], [3., 4.], [5., 6.]]);
        assert_eq!(input.weights(), &[1., 2., 1.]);
        assert_eq!(input.knots(), &[0., 1.]);
        assert_eq!(input.multiplicities(), &[3, 3]);
        assert_eq!(input.flat_knots(), &[0., 0., 0., 1., 1., 1.]);
        assert_eq!(input.domain(), [0., 1.]);
        assert_eq!(input.degree(), 2);
        assert!(!input.periodic());
        assert!(input.rational());
        assert!(!input.evaluation_override());
        assert_eq!(input.lower_bounds(), [1, 1, 1, 1, 1]);
        assert!(curve.state.try_write().is_none());
    }

    #[test]
    fn planar_spline_lease_refuses_busy_and_poisoned_state() {
        let curve = curve();
        drop(curve.try_input_lease().unwrap());
        let writer = curve.state.write();
        assert!(matches!(
            curve.try_input_lease(),
            Err(SharedReadFailure::Busy)
        ));
        drop(writer);
        let fault = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _writer = curve.state.write();
            panic!("NamedPlanarSplineWriterPoison: expected typed refusal");
        }));
        assert!(fault.is_err());
        assert!(matches!(
            curve.try_input_lease(),
            Err(SharedReadFailure::Poisoned)
        ));
    }

    #[test]
    fn copied_planar_spline_fields_survive_mutation_and_are_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<StoredPlanarSplineInput>();
        let curve = curve();
        let captured = curve.try_input_lease().unwrap().stored_input();
        curve.set_pole(1, Point2::new(9., 8.)).unwrap();
        assert_eq!(captured.poles()[0], [1., 2.]);
        assert_eq!(curve.pole(1).unwrap(), Point2::new(9., 8.));
    }
}
