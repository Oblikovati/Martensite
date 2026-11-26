// SPDX-License-Identifier: GPL-2.0-only
//! Source-attributed input uncertainty and tolerance composition for profile extrusion (#494).
use super::decline::{
    DeclineCause, Decliner, OperationDecline, OperationKind, OperationStage as Stage,
};
use super::resolution::{Resolution, ToleranceCap};

const DECLINE: Decliner = Decliner {
    operation: OperationKind::Extrude,
};

/// Provenance of an input geometric or parametric uncertainty bound (#494).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UncertaintySource {
    /// Residual from the upstream 2D/3D sketch constraint solver.
    SketchSolver,
    /// Stored tolerance on an imported or reference B-rep entity.
    ImportedBrep,
    /// Numeric tolerance from a feature dimension or expression.
    FeatureParameter,
    /// Explicit modelling resolution chosen by the caller.
    UserResolution,
}

/// One source-attributed linear and angular uncertainty bound (#494).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputUncertainty {
    source: UncertaintySource,
    linear_bound: f64,
    angular_bound: f64,
}

impl InputUncertainty {
    /// Constructs a validated uncertainty entry; rejects non-finite or negative bounds.
    pub fn new(
        source: UncertaintySource,
        linear_bound: f64,
        angular_bound: f64,
    ) -> Result<Self, OperationDecline> {
        if !(linear_bound.is_finite() && linear_bound >= 0.0) {
            return Err(DECLINE.at(
                Stage::Admission,
                DeclineCause::InvalidResolution {
                    field: "uncertainty.linear_bound",
                    value: linear_bound,
                },
            ));
        }
        if !(angular_bound.is_finite() && angular_bound >= 0.0) {
            return Err(DECLINE.at(
                Stage::Admission,
                DeclineCause::InvalidResolution {
                    field: "uncertainty.angular_bound",
                    value: angular_bound,
                },
            ));
        }
        Ok(Self {
            source,
            linear_bound,
            angular_bound,
        })
    }

    /// Origin of this uncertainty bound.
    pub fn source(&self) -> UncertaintySource {
        self.source
    }

    /// Linear uncertainty bound in model units.
    pub fn linear_bound(&self) -> f64 {
        self.linear_bound
    }

    /// Angular uncertainty bound in radians.
    pub fn angular_bound(&self) -> f64 {
        self.angular_bound
    }
}

/// Composed modelling tolerance carrying every contributing uncertainty source (#494).
#[derive(Clone, Debug, PartialEq)]
pub struct ComposedTolerance {
    linear: f64,
    angular: f64,
    contributors: Vec<InputUncertainty>,
}

impl ComposedTolerance {
    /// Composes `base` resolution with `inputs`, refusing if the composed linear bound
    /// violates `ToleranceCap::Refuse` (#494).
    pub fn compose(
        base: &Resolution,
        inputs: &[InputUncertainty],
    ) -> Result<Self, OperationDecline> {
        if let Err((field, value)) = base.admit() {
            return Err(DECLINE.at(
                Stage::Admission,
                DeclineCause::InvalidResolution { field, value },
            ));
        }
        let mut linear = base.sew;
        let mut angular = base.angular;
        let mut contributors = Vec::with_capacity(inputs.len() + 1);
        contributors.push(InputUncertainty {
            source: UncertaintySource::UserResolution,
            linear_bound: base.sew,
            angular_bound: base.angular,
        });
        for input in inputs {
            linear = linear.max(input.linear_bound);
            angular = angular.max(input.angular_bound);
            contributors.push(*input);
        }
        if let ToleranceCap::Refuse(limit) = base.cap
            && linear > limit
        {
            return Err(DECLINE.at(
                Stage::Admission,
                DeclineCause::InvalidResolution {
                    field: "uncertainty.composed_linear",
                    value: linear,
                },
            ));
        }
        Ok(Self {
            linear,
            angular,
            contributors,
        })
    }

    /// Composed linear tolerance bound.
    pub fn linear(&self) -> f64 {
        self.linear
    }

    /// Composed angular tolerance bound.
    pub fn angular(&self) -> f64 {
        self.angular
    }

    /// Source-attributed contributors in deterministic order.
    pub fn contributors(&self) -> &[InputUncertainty] {
        &self.contributors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile_extrusion::DisplayTolerance;

    #[test]
    fn composes_attributed_uncertainties_and_enforces_refuse_cap() {
        let display = DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        };
        let resolution = Resolution::new(1e-6, 1e-7, 1e-9, display, ToleranceCap::Refuse(1e-5));
        let solver = InputUncertainty::new(UncertaintySource::SketchSolver, 5e-6, 1e-7)
            .expect("valid solver uncertainty");
        let composed = ComposedTolerance::compose(&resolution, &[solver]).expect("within cap");
        assert_eq!(composed.linear(), 5e-6);
        assert_eq!(composed.contributors().len(), 2);
        assert_eq!(composed.contributors()[1].source(), UncertaintySource::SketchSolver);

        let too_wide = InputUncertainty::new(UncertaintySource::ImportedBrep, 2e-4, 0.0)
            .expect("valid imported uncertainty");
        assert!(ComposedTolerance::compose(&resolution, &[too_wide]).is_err());
    }
}
