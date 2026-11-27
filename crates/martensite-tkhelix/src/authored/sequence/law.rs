// SPDX-License-Identifier: GPL-2.0-only
//! Ordinary-radian evaluation shares the finite helix's radial and frame mathematics.
use super::{
    FlatEndHelixRequest, HelixParameterSide as Side, HelixPitchRow, HelixSequence,
    HelixSequenceDefinition, HelixSequenceError as Failure, HelixSequencePlacement,
    HelixSequenceSpan,
    stations::{self, Form, Span},
};
use crate::{
    authored::{
        HelixEvaluation, HelixFailure, HelixQuantity, HelixRotation,
        law::{checked_frame, place, positive},
    },
    helix_geom::local::LocalHelix,
};
use martensite_tkmath::gp::{Point3, Vector3};

impl HelixSequence {
    /// Admit cylindrical flat ends in one immutable law.
    /// Use `HelixSequence::flat_ends(request)?` with zone sizes in revolutions.
    pub fn flat_ends(mut request: FlatEndHelixRequest) -> Result<Self, Failure> {
        request.placement = placement(request.placement)?;
        let spans = stations::flat(request)?;
        Ok(Self {
            placement: request.placement,
            definition: HelixSequenceDefinition::FlatEnds(request),
            spans,
        })
    }

    /// Capture positive ordered pitch rows and their caller-issued identities.
    /// Use `HelixSequence::pitch_table(frame, &rows)?` before reusing the input buffer.
    pub fn pitch_table(
        frame: HelixSequencePlacement,
        rows: &[HelixPitchRow],
    ) -> Result<Self, Failure> {
        let placement = placement(frame)?;
        let spans = stations::table(rows)?;
        Ok(Self {
            placement,
            definition: HelixSequenceDefinition::PitchTable(rows.into()),
            spans,
        })
    }

    /// Return the cylindrical model placement used for evaluation.
    /// Use `sequence.placement().radius` to recover the common radius.
    pub fn placement(&self) -> HelixSequencePlacement {
        self.placement
    }

    /// Return the original immutable defining choices.
    /// Use `sequence.definition()` when restoring a recipe.
    pub fn definition(&self) -> &HelixSequenceDefinition {
        &self.definition
    }

    /// Enumerate disjoint smooth spans in source order with their shared stations.
    /// Use `sequence.spans()` to enumerate the native parameter candidates.
    pub fn spans(
        &self,
    ) -> impl DoubleEndedIterator<Item = HelixSequenceSpan> + ExactSizeIterator + '_ {
        self.spans.iter().map(Span::summary)
    }

    /// Evaluate position and the one-sided first and second ordinary-radian derivatives.
    /// At a shared station use `sequence.evaluate(parameter, Side::Before)?` on the earlier row.
    pub fn evaluate(&self, parameter: f64, side: Side) -> Result<HelixEvaluation, Failure> {
        let span = self.locate(parameter, side)?;
        let [p, v, a] = self.components(span, parameter)?;
        Ok(HelixEvaluation {
            position: Point3::new(p[0], p[1], p[2]),
            first: Vector3::new(v[0], v[1], v[2]),
            second: Vector3::new(a[0], a[1], a[2]),
        })
    }

    fn locate(&self, parameter: f64, side: Side) -> Result<&Span, Failure> {
        let index = self.spans.partition_point(|span| {
            let last = span.stations[1].parameter;
            match side {
                Side::Before => last < parameter,
                Side::After => last <= parameter,
            }
        });
        self.spans
            .get(index)
            .filter(|span| {
                parameter >= span.stations[0].parameter
                    && !(side == Side::Before && parameter == 0.)
                    && parameter.is_finite()
            })
            .ok_or(Failure::ParameterSide { parameter, side })
    }

    pub(super) fn components(
        &self,
        span: &Span,
        parameter: f64,
    ) -> Result<[[f64; 3]; 3], HelixFailure> {
        let local = LocalHelix {
            opening_radius: self.placement.radius,
            radial_rate: 0.,
            axial_rate: 0.,
            positive: self.placement.rotation == HelixRotation::Positive,
        };
        let (sin, cos) = martensite_cmath::sincos(parameter);
        let xy = [
            local.point(parameter, cos, sin),
            local.first(parameter, cos, sin),
            local.second(parameter, cos, sin),
        ];
        let axial = span.axial(parameter);
        let world = std::array::from_fn(|order| {
            let [x, y, _] = xy[order];
            place(self.placement.frame, [x, y, axial[order]], order == 0)
        });
        finite(world, parameter)
    }
}

impl Span {
    fn axial(&self, parameter: f64) -> [f64; 3] {
        let [first, last] = self.stations.each_ref();
        let u = (parameter - first.parameter) / (last.parameter - first.parameter);
        let [offset, rate, acceleration] = self.axial_at(u);
        let z = if parameter == last.parameter {
            last.elevation
        } else {
            first.elevation + self.rise * offset
        };
        [z, rate, acceleration]
    }

    fn axial_at(&self, u: f64) -> [f64; 3] {
        let rate = self.natural_rate;
        match self.form {
            Form::Flat => [0.; 3],
            Form::Constant => [u, rate, 0.],
            Form::Accelerating => {
                let [progress, speed, acceleration] = ramp(u);
                [
                    progress,
                    rate * speed,
                    rate * acceleration / self.natural_parameter_width,
                ]
            }
            Form::Decelerating => {
                let [progress, speed, acceleration] = ramp(1. - u);
                [
                    1. - progress,
                    rate * speed,
                    -rate * acceleration / self.natural_parameter_width,
                ]
            }
        }
    }
}

fn ramp(u: f64) -> [f64; 3] {
    let u2 = u * u;
    // The integral of 3u²-2u³ over [0, 1] is 1/2 of the natural rise.
    [u2 * u * (2. - u), u2 * (3. - 2. * u), 6. * u * (1. - u)]
}

fn placement(mut placement: HelixSequencePlacement) -> Result<HelixSequencePlacement, Failure> {
    placement.frame = checked_frame(placement.frame)?;
    positive(placement.radius, HelixQuantity::OpeningRadius)?;
    Ok(placement)
}

fn finite(values: [[f64; 3]; 3], parameter: f64) -> Result<[[f64; 3]; 3], HelixFailure> {
    for (order, coordinates) in values.iter().enumerate() {
        if coordinates.iter().any(|value| !value.is_finite()) {
            return Err(HelixFailure::Evaluation {
                parameter,
                order: order as u8,
            });
        }
    }
    Ok(values)
}
