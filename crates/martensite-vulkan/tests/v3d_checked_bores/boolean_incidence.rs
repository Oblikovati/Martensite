//! Curve-on-face checks for each candidate face/edge incidence.
use super::boolean_coverage::{CoverageStage, OperationCoverage};
use super::boolean_outcome::{BooleanIssue, BooleanRequest, kernel_issue};
use super::boolean_validation::{LocalCheckMode, entities};
use martensite_tkbo::bop_tools;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::{geom2d::Geom2dCurve, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn tolerances(
    request: BooleanRequest,
    shape: &Shape,
    coverage: &mut OperationCoverage,
    mode: LocalCheckMode,
) -> Result<(usize, usize), BooleanIssue> {
    tolerance_metadata(shape).map_err(|cause| kernel_issue(request, "Tolerance", cause))?;
    let (mut measured, mut degenerated) = (0, 0);
    for face in entities(shape, ShapeEnum::Face)
        .map_err(|cause| kernel_issue(request, "Tolerance", cause))?
    {
        for edge in entities(&face, ShapeEnum::Edge)
            .map_err(|cause| kernel_issue(request, "Tolerance", cause))?
        {
            match measure_incidence(&face, &edge, mode)
                .map_err(|cause| kernel_issue(request, "Tolerance", cause))?
            {
                IncidenceMeasurement::Measured => measured += 1,
                IncidenceMeasurement::Degenerated => degenerated += 1,
            }
            coverage.observed(CoverageStage::Tolerance);
        }
    }
    Ok((measured, degenerated))
}

fn tolerance_metadata(shape: &Shape) -> Result<(), Failure> {
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        for entity in entities(shape, kind)? {
            let tolerance = match kind {
                ShapeEnum::Vertex => BRepTool::vertex_tolerance(&topods::vertex(&entity)?)?,
                ShapeEnum::Edge => BRepTool::edge_tolerance(&topods::edge(&entity)?)?,
                _ => BRepTool::face_tolerance(&topods::face(&entity)?)?,
            };
            if !tolerance.is_finite() || tolerance < 0. {
                return Err(Failure::new(&format!(
                    "kind={kind:?} tolerance={tolerance}; expected finite nonnegative tolerance"
                )));
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum IncidenceMeasurement {
    Measured,
    Degenerated,
}

fn classify_incidence(
    mode: LocalCheckMode,
    degenerated: bool,
    check_done: bool,
    pcurve_stored: bool,
) -> Result<IncidenceMeasurement, Failure> {
    if mode == LocalCheckMode::ExistingOrderedChecksV1 {
        return legacy_measurement(check_done);
    }
    if !degenerated && check_done {
        return Ok(IncidenceMeasurement::Measured);
    }
    // The curve checker intentionally declines degenerate edges before creating adaptors.
    if degenerated && !check_done && pcurve_stored {
        return Ok(IncidenceMeasurement::Degenerated);
    }
    v2_measurement_refusal(degenerated, check_done, pcurve_stored)
}

fn legacy_measurement(check_done: bool) -> Result<IncidenceMeasurement, Failure> {
    if check_done {
        return Ok(IncidenceMeasurement::Measured);
    }
    Err(incomplete_curve_measurement())
}

fn v2_measurement_refusal(
    degenerated: bool,
    check_done: bool,
    pcurve_stored: bool,
) -> Result<IncidenceMeasurement, Failure> {
    if !degenerated && !check_done {
        return Err(incomplete_curve_measurement());
    }
    Err(Failure::with_type(
        "BOP4_IncompleteCoverage",
        &format!(
            "degenerated={degenerated} measurement_done={check_done} pcurve_stored={pcurve_stored}; expected completed measurement or stored-pcurve degenerate incidence"
        ),
        "",
    ))
}

fn incomplete_curve_measurement() -> Failure {
    Failure::with_type(
        "BOP4_IncompleteCoverage",
        "curve/surface measurement=NotDone; expected actual completed incidence measurement",
        "",
    )
}

pub(super) fn measure_incidence(
    face: &Shape,
    edge: &Shape,
    mode: LocalCheckMode,
) -> Result<IncidenceMeasurement, Failure> {
    let face = topods::face(face)?;
    let edge = topods::edge(edge)?;
    let degenerated = BRepTool::degenerated(&edge)?;
    let pcurve_usable = if mode == LocalCheckMode::DegeneratePcurveExclusionV2 && degenerated {
        validate_stored_pcurve(&edge, &face)?;
        true
    } else {
        false
    };
    let (mut distance, mut parameter) = (f64::NAN, f64::NAN);
    let done = bop_tools::compute_curve_tolerance(&face, &edge, &mut distance, &mut parameter)?;
    let result = classify_incidence(mode, degenerated, done, pcurve_usable)?;
    if result == IncidenceMeasurement::Degenerated {
        return Ok(result);
    }
    let limit = BRepTool::edge_tolerance(&edge)?;
    if !distance.is_finite() || !parameter.is_finite() || distance > limit {
        return Err(Failure::new(&format!(
            "distance={distance} parameter={parameter} limit={limit}; expected finite discrepancy within stored edge tolerance"
        )));
    }
    Ok(IncidenceMeasurement::Measured)
}

fn validate_stored_pcurve(edge: &topods::Edge, face: &topods::Face) -> Result<(), Failure> {
    let representation = BRepTool::curve_on_face(edge, face)
        .map_err(|cause| invalid_pcurve(edge, face, &format!("lookup={cause}")))?;
    if !representation.is_stored || representation.pcurve.is_null() {
        return Err(invalid_pcurve(edge, face, "stored=null or missing"));
    }
    validate_pcurve_range(edge, face, &representation)?;
    Geom2dAdaptorCurve::with_range(
        &representation.pcurve,
        representation.first,
        representation.last,
    )
    .map_err(|cause| invalid_pcurve(edge, face, &format!("adaptor={cause}")))?;
    Ok(())
}

fn validate_pcurve_range(
    edge: &topods::Edge,
    face: &topods::Face,
    representation: &martensite_tkbrep::brep::EdgePCurve,
) -> Result<(), Failure> {
    let Some(curve) = representation.pcurve.get() else {
        return Err(invalid_pcurve(edge, face, "curve=null"));
    };
    let (first, last) = (representation.first, representation.last);
    let (domain_first, domain_last) = (curve.first_parameter(), curve.last_parameter());
    if !first.is_finite() || !last.is_finite() || first >= last {
        return Err(invalid_pcurve(
            edge,
            face,
            &format!("range=[{first},{last}]"),
        ));
    }
    if domain_first.is_nan() || domain_last.is_nan() || domain_first > domain_last {
        return Err(invalid_pcurve(
            edge,
            face,
            &format!("curve_domain=[{domain_first},{domain_last}]"),
        ));
    }
    validate_pcurve_domain(edge, face, curve, first, last, domain_first, domain_last)
}

pub(super) fn validate_pcurve_domain(
    edge: &topods::Edge,
    face: &topods::Face,
    curve: &dyn Geom2dCurve,
    first: f64,
    last: f64,
    domain_first: f64,
    domain_last: f64,
) -> Result<(), Failure> {
    if curve.is_periodic() {
        let period = curve
            .period()
            .map_err(|cause| invalid_pcurve(edge, face, &format!("period={cause}")))?;
        let domain_period = domain_last - domain_first;
        let roundoff = f64::EPSILON * period.abs().max(domain_period.abs()).max(1.) * 8.;
        if period.is_finite()
            && period > 0.
            && domain_period.is_finite()
            && domain_period > 0.
            && (period - domain_period).abs() <= roundoff
        {
            return Ok(());
        }
        return Err(invalid_pcurve(
            edge,
            face,
            &format!("period={period} domain_period={domain_period}"),
        ));
    }
    if first >= domain_first && last <= domain_last {
        return Ok(());
    }
    Err(invalid_pcurve(
        edge,
        face,
        &format!("range=[{first},{last}] curve_domain=[{domain_first},{domain_last}]"),
    ))
}

fn invalid_pcurve(edge: &topods::Edge, face: &topods::Face, detail: &str) -> Failure {
    Failure::with_type(
        "BOP4_IncompleteCoverage",
        &format!(
            "edge={edge:?} face={face:?} pcurve={detail}; expected stored, nonnull usable curve and range"
        ),
        "",
    )
}

#[cfg(test)]
#[path = "boolean_incidence_tests.rs"]
mod tests;
