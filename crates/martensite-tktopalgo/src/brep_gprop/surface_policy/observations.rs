use super::{
    SurfaceErrorEvidence, SurfaceIntegration, SurfacePropertyOptions, SurfacePropertyResult,
};
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkgeombase::gprop::GProps;

#[derive(Default)]
pub(crate) struct SurfaceObservations {
    geometric_faces: usize,
    triangulated_faces: usize,
    unknown_estimate: bool,
    missing_source: bool,
    incomplete_estimate: Option<f64>,
    invalid_estimate: Option<f64>,
    refinement_exhausted: bool,
}

impl SurfaceObservations {
    pub(crate) fn mesh(&mut self) {
        self.triangulated_faces += 1;
    }

    pub(crate) fn missing_source(&mut self) {
        self.missing_source = true;
    }

    pub(crate) fn geometry(&mut self, error: f64, mass: f64, exhausted: bool) {
        self.geometric_faces += 1;
        self.refinement_exhausted |= exhausted;
        if !error.is_finite() || error < 0.0 {
            self.invalid_estimate = Some(error);
        }
        if error == precision::infinite() {
            self.incomplete_estimate = Some(error);
        }
        self.unknown_estimate |=
            !error.is_finite() || error < 0.0 || error >= precision::infinite() || mass == 0.0;
    }

    fn evidence(&self, options: SurfacePropertyOptions, error: f64) -> SurfaceErrorEvidence {
        if self.unknown_estimate
            || self.geometric_faces == 0
            || self.triangulated_faces > 0
            || options.integration == SurfaceIntegration::FixedOrder
        {
            return SurfaceErrorEvidence::Unknown;
        }
        SurfaceErrorEvidence::EstimatedRelative {
            max_face_relative_error: error,
        }
    }

    fn validate_sources(&self) -> Result<(), Failure> {
        if self.missing_source {
            return Err(Failure::with_type(
                "Standard_DomainError",
                "surface_properties_with_options: face has 0 usable representations; expected geometry or a nonempty triangulation",
                "",
            ));
        }
        if self.refinement_exhausted {
            return Err(incomplete_measurement(
                "refinement_exhausted=true for a required nonzero face interval",
            ));
        }
        self.validate_estimates()
    }

    fn validate_estimates(&self) -> Result<(), Failure> {
        if let Some(error) = self.invalid_estimate {
            return Err(incomplete_measurement(&format!(
                "invalid adaptive error estimate={error}; expected a finite nonnegative estimate"
            )));
        }
        if let Some(error) = self.incomplete_estimate {
            return Err(incomplete_measurement(&format!(
                "incomplete face estimate={error}"
            )));
        }
        Ok(())
    }

    pub(crate) fn finish(
        self,
        properties: GProps,
        options: SurfacePropertyOptions,
        error: f64,
    ) -> Result<SurfacePropertyResult, Failure> {
        self.validate_sources()?;
        finite_properties(&properties)?;
        Ok(SurfacePropertyResult {
            properties,
            options,
            evidence: self.evidence(options, error),
            // The engine may raise absolute refinement budgets; it publishes no final global target.
            effective_relative_target: None,
            geometric_faces: self.geometric_faces,
            triangulated_faces: self.triangulated_faces,
        })
    }
}

fn incomplete_measurement(reason: &str) -> Failure {
    Failure::with_type(
        "StdFail_NotDone",
        &format!(
            "surface_properties_with_options: {reason}; expected a completed geometric measurement"
        ),
        "",
    )
}

fn finite_properties(props: &GProps) -> Result<(), Failure> {
    finite_value(props.dim)?;
    for point in [props.loc, props.g] {
        for value in [point.x(), point.y(), point.z()] {
            finite_value(value)?;
        }
    }
    for row in 1..=3 {
        for column in 1..=3 {
            finite_value(props.inertia.value(row, column)?)?;
        }
    }
    Ok(())
}

fn finite_value(value: f64) -> Result<(), Failure> {
    if value.is_finite() {
        return Ok(());
    }
    Err(Failure::with_type(
        "Standard_DomainError",
        &format!(
            "surface_properties_with_options: property value={value}; expected finite measurement values"
        ),
        "",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_incomplete_face_signal_does_not_publish_previous_face_properties() {
        let mut observations = SurfaceObservations::default();
        observations.geometry(0.0, 4.0, false);
        observations.geometry(precision::infinite(), 4.0, false);
        let failure = observations
            .finish(
                GProps::new(),
                SurfacePropertyOptions::default(),
                precision::infinite(),
            )
            .unwrap_err();
        assert_eq!(failure.exception_type(), "StdFail_NotDone");
        assert!(
            failure
                .message()
                .contains(&precision::infinite().to_string())
        );
    }
    #[test]
    fn invalid_adaptive_estimates_do_not_publish_otherwise_finite_properties() {
        for error in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
            let mut observations = SurfaceObservations::default();
            observations.geometry(error, 4.0, false);
            let mut props = GProps::new();
            props.dim = 4.0;
            let failure = observations
                .finish(props, SurfacePropertyOptions::default(), error)
                .unwrap_err();
            assert_eq!(failure.exception_type(), "StdFail_NotDone");
            assert!(failure.message().contains(&error.to_string()));
        }
    }

    #[test]
    fn a_completed_zero_mass_face_is_nonempty_with_unknown_error() {
        fn send_sync<T: Send + Sync>() {}
        send_sync::<SurfaceObservations>();
        let mut observations = SurfaceObservations::default();
        observations.geometry(0.0, 0.0, false);
        let result = observations
            .finish(GProps::new(), SurfacePropertyOptions::default(), 0.0)
            .unwrap();
        assert!(!result.is_empty());
        assert_eq!(result.error_evidence(), SurfaceErrorEvidence::Unknown);
    }
}
