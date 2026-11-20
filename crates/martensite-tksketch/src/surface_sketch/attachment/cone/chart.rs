//! Decode the selected face once for interactive hit development.
use super::*;
use crate::surface_sketch::{ConeSupport, NormalSense};
use martensite_tkbo::{
    body_check::CheckMode,
    measured::{IntegrationMethod, Measured},
};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::State};
use martensite_tkmath::{
    gp::Cone3,
    predicates::roots::metric_bounds::{ConicalDevelopment, ConicalDevelopmentFailure},
};

/// Reusable native chart for one revision and seam of a frozen conical face.
pub struct ConeAuthoringChart {
    attempt: Attempt,
    face: Face,
    cone: Cone3,
    metric: ConicalDevelopment,
    seam: f64,
    direct: bool,
    tolerance: f64,
}

/// Native parameters lifted into the selected face's own angular chart.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConeParameterRequest {
    /// Native binary64 parameters `(u,v)` in the chosen unwrapped turn.
    pub uv: [f64; 2],
    /// Treatment of points outside the native trim.
    pub outside: OutOfSupportPolicy,
}

/// Native parameters and bounded developed Cartesian coordinates for authoring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DevelopedConeHit {
    /// Original unwrapped UV, including the caller's angular lift.
    pub uv: [f64; 2],
    /// Apex-relative `(x,y)`. Bounds cover chart arithmetic on exact input UV.
    /// They exclude intersector uncertainty and the native stored frame's metric deviation.
    pub coordinates: [Measured; 2],
    /// Classifier's actual result on this selected face.
    pub trim_state: State,
    /// Trim coverage, reported as `NativeSearch`, separately from the coordinate bounds.
    pub trim_check: CheckMode,
}

impl ConeFaceAttachment {
    /// Open a reusable authoring chart at an explicit unwrapped native seam.
    /// Example: `let chart = attachment.authoring_chart(revision, 0.0)?;`.
    pub fn authoring_chart(
        &self,
        revision: SupportRevision,
        seam: f64,
    ) -> ConeChecked<ConeAuthoringChart> {
        let attempt = self.attempt(AttachmentOperation::Develop);
        let DecodedFace { face, .. } = self.resolve(&attempt, revision)?;
        ConeAuthoringChart::capture(attempt, face, seam, self.record().resolution.sew)
    }
}

impl ConeAuthoringChart {
    fn capture(attempt: Attempt, face: Face, seam: f64, tolerance: f64) -> ConeChecked<Self> {
        let adaptor = attempt.native(
            "selected cone adaptor",
            BRepAdaptorSurface::with_face(&face, false),
        )?;
        let cone = attempt.native("selected cone geometry", adaptor.cone())?;
        let direct = attempt.native("selected cone handedness", cone.direct())?;
        let metric = ConicalDevelopment::new(cone.semi_angle(), cone.ref_radius(), seam, direct)
            .map_err(|cause| development(&attempt, cause))?;
        Ok(Self {
            attempt,
            face,
            cone,
            metric,
            seam,
            direct,
            tolerance,
        })
    }

    /// Develop a UV hit without decoding the support body again.
    /// Example: `let hit = chart.develop_parameters(revision, request)?;`.
    pub fn develop_parameters(
        &self,
        revision: SupportRevision,
        request: ConeParameterRequest,
    ) -> ConeChecked<DevelopedConeHit> {
        self.attempt.revision(revision)?;
        let bounds = self
            .metric
            .bounds(request.uv)
            .map_err(|cause| development(&self.attempt, cause))?;
        let trim_state = super::super::chart::classify(
            &self.attempt,
            &self.face,
            self.tolerance,
            request.uv,
            request.outside,
        )?;
        let [x, y] = bounds;
        let coordinates = [measured(&self.attempt, x)?, measured(&self.attempt, y)?];
        Ok(DevelopedConeHit {
            uv: request.uv,
            coordinates,
            trim_state,
            trim_check: CheckMode::NativeSearch,
        })
    }

    /// Estimate the ideal extrusion frame for authoring on the same seam and nappe.
    /// The converted apex uses the native binary64 estimate. This frame can drive free-standing
    /// extrusion and carries no certificate of equivalence to the selected trimmed carrier.
    /// Example: `let support = chart.extrusion_support(revision, NormalSense::Outward)?;`.
    pub fn extrusion_support(
        &self,
        revision: SupportRevision,
        normal: NormalSense,
    ) -> ConeChecked<ConeSupport> {
        self.attempt.revision(revision)?;
        let apex = *self.cone.apex().coordinates();
        finite_apex(&self.attempt, apex)?;
        let growing = self.cone.semi_angle() > 0.;
        let axis = *self.cone.axis().direction().coordinates();
        Ok(ConeSupport {
            apex,
            axis: if growing {
                axis
            } else {
                axis.map(|value| -value)
            },
            radial: *self.cone.position().x_direction().coordinates(),
            semi_angle: self.cone.semi_angle().abs(),
            chart_origin: if growing == self.direct {
                self.seam
            } else {
                -self.seam
            },
            normal,
        })
    }
}

fn finite_apex(attempt: &Attempt, apex: [f64; 3]) -> ConeChecked<()> {
    if let Some(value) = apex.into_iter().find(|value| !value.is_finite()) {
        return Err(attempt
            .reject(AttachmentFailureCause::ChartValue {
                field: "native cone apex",
                value,
            })
            .into());
    }
    Ok(())
}

fn development(attempt: &Attempt, cause: ConicalDevelopmentFailure) -> ConeAttachmentFailure {
    ConeAttachmentFailure::Development {
        attachment: attempt.id,
        revision: attempt.revision,
        cause: Box::new(cause),
    }
}

fn measured(attempt: &Attempt, [lower, upper]: [f64; 2]) -> ConeChecked<Measured> {
    let value = lower.midpoint(upper);
    let radius = (value - lower).max(upper - value);
    let error_bound = if radius == 0. { 0. } else { radius.next_up() };
    if !error_bound.is_finite() {
        return Err(attempt
            .reject(AttachmentFailureCause::ChartValue {
                field: "developed cone coordinate error",
                value: error_bound,
            })
            .into());
    }
    Ok(Measured {
        value,
        error_bound,
        method: IntegrationMethod::CertifiedEnclosure,
    })
}
