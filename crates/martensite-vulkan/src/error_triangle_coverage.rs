use super::{RenderError, RenderFailureCause};

impl RenderError {
    /// Inspect triangle coverage refusal; `error.triangle_coverage_failure()`.
    pub fn triangle_coverage_failure(&self) -> Option<&crate::RenderTriangleCoverageFailure> {
        match &self.1 {
            Some(RenderFailureCause::TriangleCoverage(failure)) => Some(failure),
            _ => None,
        }
    }
}

impl From<crate::RenderTriangleCoverageFailure> for RenderError {
    fn from(failure: crate::RenderTriangleCoverageFailure) -> Self {
        Self(
            failure.to_string(),
            Some(RenderFailureCause::TriangleCoverage(failure)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refusal_remains_typed_and_reports_the_invalid_property_value() {
        let failure = crate::RenderTriangleCoverageFailure::InvalidOverestimateSize {
            size_bits: f32::NAN.to_bits(),
        };
        let error = RenderError::from(failure);
        assert!(error.triangle_coverage_failure().is_some());
        assert!(
            error
                .to_string()
                .contains("primitiveOverestimationSize bits")
        );
        assert!(
            std::error::Error::source(&error)
                .unwrap()
                .is::<crate::RenderTriangleCoverageFailure>()
        );
    }
}
