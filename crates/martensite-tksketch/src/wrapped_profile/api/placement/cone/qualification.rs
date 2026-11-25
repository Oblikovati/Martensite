//! Place the native graph once and qualify each member with the requested search or certificate.
use super::*;
use martensite_tkbo::body_check::{
    CertifiedConicalBody, CheckMode, ConicalNativeMember, ValidationReport,
    validate_body_with_transported_conical,
};

pub(super) struct PlacedCertificate<'a> {
    proof: &'a CertifiedConicalBody,
    members: BTreeMap<SurfaceRegionId, ConicalNativeMember>,
}

pub(super) fn capture<'a>(
    base: &'a WrappedConeNormalResult,
    physical: &freeze::Physical,
) -> Checked<Option<PlacedCertificate<'a>>> {
    if !base.execution_selection().requires_transported_placement() {
        return Ok(None);
    }
    let proof = base
        .family
        .budget
        .actual_certificate
        .as_deref()
        .ok_or_else(|| {
            fail(PlacementFailure::Native {
                step: "transported certificate",
                cause: Failure::new("expected complete canonical cone family certificate"),
            })
        })?;
    let members = operation_cone::placed_members(base, &physical.bindings)?;
    Ok(Some(PlacedCertificate { proof, members }))
}

pub(super) fn validate(
    certificate: Option<&PlacedCertificate<'_>>,
    region: SurfaceRegionId,
    solid: &Shape,
    plan: &ConePlan,
) -> Checked<ValidationReport> {
    let tolerance = plan.budget.input().spatial_limit;
    let Some(certificate) = certificate else {
        return publication::physical_validation(region, solid, tolerance);
    };
    let member = certificate
        .members
        .get(&region)
        .filter(|member| member.shape.is_equal(solid))
        .ok_or_else(|| fail(PlacementFailure::Member(region)))?;
    certificate.validate(member, region, tolerance, plan.budget.frame())
}

impl PlacedCertificate<'_> {
    fn validate(
        &self,
        member: &ConicalNativeMember,
        region: SurfaceRegionId,
        tolerance: f64,
        frame: &CertifiedRotatedFrameTransport,
    ) -> Checked<ValidationReport> {
        let report = validate_body_with_transported_conical(
            member,
            &pipeline::resolution(tolerance),
            self.proof,
            frame,
        )
        .map_err(|cause| {
            fail(PlacementFailure::Native {
                step: "placed cone resolution",
                cause: Failure::new(&format!(
                    "{}={}; expected finite positive resolution",
                    cause.field, cause.value
                )),
            })
        })?;
        if publication_budget::certify_native_with_mode(
            region,
            &report,
            tolerance,
            CheckMode::Exact,
        )
        .is_err()
        {
            return Err(fail(PlacementFailure::TransportValidation {
                region,
                report: Box::new(report),
            }));
        }
        Ok(report)
    }
}
