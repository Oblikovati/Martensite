//! One owned admission path serves retained leases and immutable face inputs.
use super::StoredSplineInput;
use crate::geom::{
    ClosedSplineExtrusionError as Error, ClosedSplineExtrusionInconclusive as Inconclusive,
    ClosedSplineExtrusionInput as Admission, ClosedSplineExtrusionLayout as Layout,
};
use martensite_tkmath::predicates::roots::closed_extrusion::{
    ClosedExtrusionInputError as InputError, ClosedQuadraticExtrusionInput,
};
impl StoredSplineInput {
    /// Admit the actual owned support without live lookup; `input.into_closed_extrusion_input([0.,0.,1.],[-1.,1.])?`.
    pub fn into_closed_extrusion_input(
        self,
        direction: [f64; 3],
        interval: [f64; 2],
    ) -> Result<Admission, Error> {
        let layout = self.layout(direction, interval);
        if layout.basis_override {
            return Ok(Admission::Inconclusive(
                Inconclusive::BasisEvaluationOverride(layout),
            ));
        }
        if !supported(&layout) {
            return Ok(Admission::Inconclusive(Inconclusive::UnsupportedLayout(
                layout,
            )));
        }
        admitted(self, layout)
    }
    pub(super) fn layout(&self, direction: [f64; 3], interval: [f64; 2]) -> Layout {
        Layout {
            degree: self.degree,
            periodic: self.periodic,
            rational: self.rational,
            basis_override: self.evaluation_override,
            lower_bounds: self.lower_bounds,
            flat_knots: self.flat_knots.clone(),
            multiplicities: self.multiplicities.clone(),
            direction,
            extrusion_interval: interval,
        }
    }
    fn numeric(
        self,
        direction: [f64; 3],
        interval: [f64; 2],
    ) -> Result<ClosedQuadraticExtrusionInput, Error> {
        let mults = checked_multiplicities(&self.multiplicities)?;
        ClosedQuadraticExtrusionInput::capture(
            self.poles,
            self.weights,
            self.knots,
            mults,
            direction,
            interval,
        )
        .map_err(Error::Input)
    }
}
pub(super) fn supported(layout: &Layout) -> bool {
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
fn admitted(input: StoredSplineInput, layout: Layout) -> Result<Admission, Error> {
    let numeric = match input.numeric(layout.direction, layout.extrusion_interval) {
        Ok(input) => input,
        Err(Error::Input(InputError::OpenEndpoints { first, last })) => {
            return Ok(Admission::Inconclusive(Inconclusive::NotExactlyClosed {
                layout,
                first,
                last,
            }));
        }
        Err(error) => return Err(error),
    };
    flat_sequence(&numeric, &layout.flat_knots)?;
    Ok(Admission::Captured(numeric, layout))
}
fn flat_sequence(input: &ClosedQuadraticExtrusionInput, actual: &[f64]) -> Result<(), Error> {
    validate_flat_sequence(input.knots(), input.multiplicities(), actual)
}
pub(super) fn validate_flat_sequence(
    knots: &[f64],
    counts: &[u32],
    actual: &[f64],
) -> Result<(), Error> {
    let expected = expanded_knots(knots, counts);
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

pub(super) fn checked_multiplicities(values: &[i32]) -> Result<Vec<u32>, Error> {
    values
        .iter()
        .enumerate()
        .map(|(index, n)| u32::try_from(*n).map_err(|_| Error::Multiplicity { index, value: *n }))
        .collect::<Result<Vec<_>, _>>()
}

fn expanded_knots(knots: &[f64], counts: &[u32]) -> Vec<f64> {
    knots
        .iter()
        .zip(counts)
        .flat_map(|(k, n)| std::iter::repeat_n(*k, *n as usize))
        .collect()
}
