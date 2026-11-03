//! Keep one basis read guard through numeric and evaluator admission.
use super::{BSplineState, GeomBSplineCurve};
use crate::geom::surface_of_linear_extrusion::self_exclusion::{
    BasisAdmission, ClosedSplineExtrusionError as Error,
    ClosedSplineExtrusionInconclusive as Inconclusive, ClosedSplineExtrusionLayout as Layout,
};
use martensite_tkmath::predicates::roots::closed_extrusion::{
    ClosedExtrusionInputError as InputError, ClosedQuadraticExtrusionInput,
};

impl GeomBSplineCurve {
    pub(in crate::geom) fn closed_extrusion_basis_capture(
        &self,
        direction: [f64; 3],
        interval: [f64; 2],
    ) -> Result<BasisAdmission, Error> {
        let state = self.state.read();
        capture_leased_basis(&state, direction, interval)
    }
}

pub(super) fn capture_leased_basis(
    state: &BSplineState,
    direction: [f64; 3],
    interval: [f64; 2],
) -> Result<BasisAdmission, Error> {
    let layout = layout(state, direction, interval);
    if layout.basis_override {
        return Ok(BasisAdmission::Inconclusive(
            Inconclusive::BasisEvaluationOverride(layout),
        ));
    }
    if !supported(&layout) {
        return Ok(BasisAdmission::Inconclusive(
            Inconclusive::UnsupportedLayout(layout),
        ));
    }
    admitted_snapshot(state, layout)
}

fn admitted_snapshot(state: &BSplineState, layout: Layout) -> Result<BasisAdmission, Error> {
    let input = match copied_input(state, layout.direction, layout.extrusion_interval) {
        Ok(input) => input,
        Err(Error::Input(InputError::OpenEndpoints { first, last })) => {
            return Ok(BasisAdmission::Inconclusive(
                Inconclusive::NotExactlyClosed {
                    layout,
                    first,
                    last,
                },
            ));
        }
        Err(error) => return Err(error),
    };
    flat_sequence(&input, &layout.flat_knots)?;
    Ok(BasisAdmission::Captured(input, layout))
}

fn supported(layout: &Layout) -> bool {
    let m = &layout.multiplicities;
    layout.degree == 2
        && !layout.periodic
        && layout.lower_bounds == [1; 5]
        && m.len() >= 2
        && m.first() == Some(&3)
        && m.last() == Some(&3)
        && m.iter()
            .skip(1)
            .take(m.len().saturating_sub(2))
            .all(|n| (1..=2).contains(n))
}

fn layout(state: &BSplineState, direction: [f64; 3], interval: [f64; 2]) -> Layout {
    Layout {
        degree: state.deg,
        periodic: state.periodic,
        rational: state.rational,
        basis_override: !state.eval_rep.is_null(),
        lower_bounds: [
            state.poles.lower(),
            state.weights.lower(),
            state.knots.lower(),
            state.mults.lower(),
            state.flat_knots.lower(),
        ],
        flat_knots: state.flat_knots.as_slice().to_vec(),
        multiplicities: state.mults.as_slice().to_vec(),
        direction,
        extrusion_interval: interval,
    }
}

fn copied_input(
    state: &BSplineState,
    direction: [f64; 3],
    interval: [f64; 2],
) -> Result<ClosedQuadraticExtrusionInput, Error> {
    let poles = state.poles.iter().map(|p| [p.x(), p.y(), p.z()]).collect();
    let weights = state.weights.as_slice().to_vec();
    let knots = state.knots.as_slice().to_vec();
    let mults = state
        .mults
        .iter()
        .enumerate()
        .map(|(index, n)| u32::try_from(*n).map_err(|_| Error::Multiplicity { index, value: *n }))
        .collect::<Result<Vec<_>, _>>()?;
    ClosedQuadraticExtrusionInput::capture(poles, weights, knots, mults, direction, interval)
        .map_err(Error::Input)
}

fn flat_sequence(input: &ClosedQuadraticExtrusionInput, actual: &[f64]) -> Result<(), Error> {
    let expected: Vec<f64> = input
        .knots()
        .iter()
        .zip(input.multiplicities())
        .flat_map(|(k, n)| std::iter::repeat_n(*k, *n as usize))
        .collect();
    if expected
        .iter()
        .map(|v| v.to_bits())
        .eq(actual.iter().map(|v| v.to_bits()))
    {
        return Ok(());
    }
    Err(Error::FlatSequence {
        actual: actual.to_vec(),
        expected,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::{GeomSurfaceOfLinearExtrusion, GeomSweptSurface};
    use martensite_tkernel::{ncollection::Array1, standard::Handle};
    use martensite_tkmath::{
        gp::{Direction3, Point3},
        predicates::roots::closed_extrusion::ClosedExtrusionPolicy,
    };
    #[test]
    fn actual_inconsistent_flat_storage_refuses_before_numerical_execution() {
        let surface = backtracking_surface();
        let basis = surface.basis_curve().cast::<GeomBSplineCurve>();
        basis
            .get()
            .unwrap()
            .state
            .write()
            .flat_knots
            .set_value(4, 0.75)
            .unwrap();
        let result = surface.closed_quadratic_self_exclusion(
            [-1., 1.],
            ClosedExtrusionPolicy::bounded(),
            || false,
        );
        assert!(matches!(result, Err(Error::FlatSequence { .. })));
    }
    fn backtracking_surface() -> GeomSurfaceOfLinearExtrusion {
        let points = [
            Point3::default(),
            Point3::new(1., 0., 0.),
            Point3::default(),
        ];
        let poles = Array1::from_shared(&points, 1);
        let knots = Array1::from_shared(&[0., 1.], 1);
        let mults = Array1::from_shared(&[3, 3], 1);
        let curve = GeomBSplineCurve::new(&poles, &knots, &mults, 2, false).unwrap();
        GeomSurfaceOfLinearExtrusion::new(
            &Handle::new(curve).cast(),
            Direction3::new(0., 0., 1.).unwrap(),
        )
        .unwrap()
    }
}
