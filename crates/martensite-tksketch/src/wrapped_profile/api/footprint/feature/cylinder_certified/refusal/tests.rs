// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CylindricalContactPrefixQuery, CylindricalProfileOptions, CylindricalRadialSense,
    CylindricalRadialSweepQuery, CylindricalTrimQuery, MetricBezier,
    certify_cylindrical_contact_prefix, certify_cylindrical_radial_sweep, certify_cylindrical_trim,
};

const ATTEMPT: Attempt = Attempt {
    attachment: FaceAttachmentId(19),
    revision: SupportRevision(7),
    kind: CylinderFeatureKind::Boss,
};
const STAGE: CylinderFeatureStage = CylinderFeatureStage::Validation;
const CURVE: HistorySource = HistorySource::Curve(SurfaceCurveId(71));
const VERTEX: HistorySource = HistorySource::Vertex(SurfaceVertexId(4));
const REGION: HistorySource = HistorySource::Region(SurfaceRegionId(3));

fn seam(options: CylindricalProfileOptions) -> CylindricalTrimQuery<'static> {
    CylindricalTrimQuery {
        curve: MetricBezier {
            poles: &[[4., 0., 3.], [4., 0., 4.]],
            weights: None,
        },
        pcurve: &[[0., 3.], [0., 4.]],
        radius: 4.,
        spatial_limit: 1e-5,
        options,
    }
}

fn trim_error(failure: &CylinderFeatureFailure) -> &CylindricalTrimError {
    failure
        .source()
        .unwrap()
        .downcast_ref::<CylindricalTrimError>()
        .unwrap()
}

#[test]
fn certified_cylinder_refusal_keeps_work_limit_policy_and_authored_entities() {
    let options = CylindricalProfileOptions {
        max_operations: 1,
        ..Default::default()
    };
    let error = certify_cancellable(
        ATTEMPT,
        STAGE,
        "actual corner seam point and UV lift",
        [VERTEX, CURVE, CURVE],
        &mut || false,
        |poll| certify_cylindrical_trim(seam(options), poll),
    )
    .unwrap_err();
    assert_eq!(error.stage, STAGE);
    assert_eq!(error.entities(), [CURVE, VERTEX]);
    assert_eq!(error.class(), Class::Incomplete);
    let original = trim_error(&error);
    assert_eq!(original.requested_options, options);
    assert!(
        matches!(&original.cause, CylindricalProfileFailure::Arithmetic(cause)
        if **cause == (Arithmetic::WorkLimit { completed: 1, limit: 1 }))
    );
}

#[test]
fn certified_cylinder_refusal_keeps_invalid_precision_distinct_from_incomplete_proof() {
    let options = CylindricalProfileOptions {
        fractional_bits: 1,
        ..Default::default()
    };
    let error = certify(
        ATTEMPT,
        STAGE,
        "actual corner seam point and UV lift",
        [CURVE],
        certify_cylindrical_trim(seam(options), || false),
    )
    .unwrap_err();
    assert_eq!(error.class(), Class::InvalidInput);
    assert_eq!(error.entities(), [CURVE]);
    assert_eq!(
        trim_error(&error).cause,
        CylindricalProfileFailure::Policy(options)
    );
}

#[test]
fn certified_cylinder_refusal_keeps_exact_contact_source_mismatch() {
    let seam = seam(Default::default());
    let far = MetricBezier {
        poles: &[[6., 0., 3.], [6., 0., 4.]],
        weights: None,
    };
    let sense = CylindricalRadialSense::Outward;
    let sweep = certify_cylindrical_radial_sweep(
        CylindricalRadialSweepQuery {
            rails: [seam.curve, far],
            axis_origin: [0.; 2],
            sense,
            minimum: 0.,
            options: Default::default(),
        },
        || false,
    )
    .unwrap();
    let trim = certify_cylindrical_trim(seam, || false).unwrap();
    let query = CylindricalContactPrefixQuery {
        seam,
        far_rail: MetricBezier {
            poles: &[[7., 0., 3.], [7., 0., 4.]],
            ..far
        },
        sense,
        contact_allowance: 0.2,
        max_normalized_depth: 1.,
        options: Default::default(),
    };
    let error = certify_cancellable(
        ATTEMPT,
        STAGE,
        "stored cylinder radial contact prefix",
        [CURVE],
        &mut || false,
        |poll| certify_cylindrical_contact_prefix(query, &sweep, &trim, poll),
    )
    .unwrap_err();
    assert_eq!(error.class(), Class::Incomplete);
    assert_eq!(error.entities(), [CURVE]);
    let cause = error
        .source()
        .unwrap()
        .downcast_ref::<CylindricalContactPrefixError>()
        .unwrap();
    assert_eq!(cause.cause, CylindricalContactPrefixFailure::SweepSource);
    assert_eq!(cause.contact_allowance, 0.2);
}

#[test]
fn certified_cylinder_refusal_keeps_selected_source_identity_and_chain() {
    let cause = selection::SelectionFailure::DuplicateCurve {
        curve: SurfaceCurveId(71),
        owners: [SurfaceRegionId(3), SurfaceRegionId(5)],
    };
    let failure = project(
        ATTEMPT,
        CylinderFeatureStage::Tool,
        "source-bound selected material",
        [],
        cause,
        false,
    );
    assert_eq!(failure.class(), Class::InvalidInput);
    assert_eq!(
        failure.entities(),
        [REGION, HistorySource::Region(SurfaceRegionId(5)), CURVE]
    );
    assert!(
        failure
            .source()
            .unwrap()
            .downcast_ref::<selection::SelectionFailure>()
            .is_some()
    );
}

#[test]
fn certified_cylinder_public_invalid_planar_grant_keeps_original_provider_failure() {
    use crate::surface_sketch::wrapped_profile::api::footprint::feature::tests::{
        REVISION, indirect_extruded_circle,
    };
    use crate::surface_sketch::wrapped_profile::api::footprint::tests::request;
    use martensite_tkbo::body_check::{DirectedCapCause, DirectedCapFailure};

    let stock = indirect_extruded_circle();
    let mut request = request(&stock, [-0.25, 0.25], [1., 2.]);
    request.support.normal = NormalSense::Outward;
    let outcome = stock.apply_certified_wrapped_profile(
        REVISION,
        CylinderFeatureKind::Boss,
        &request,
        PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: u64::MAX,
        },
    );
    let error = outcome
        .err()
        .expect("the directed proof must refuse an integration grant outside its policy");
    let original = error
        .source()
        .unwrap()
        .downcast_ref::<DirectedCapFailure>()
        .unwrap();
    assert_eq!(error.stage, CylinderFeatureStage::Tool);
    assert_eq!(error.class(), Class::InvalidInput);
    assert!(matches!(
        original.cause.as_ref(),
        DirectedCapCause::Policy {
            field: "max_total_subintervals",
            actual: u64::MAX,
            ..
        }
    ));
    assert!(original.observed.is_none());
}
