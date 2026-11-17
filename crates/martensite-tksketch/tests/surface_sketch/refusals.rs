//! Admission and report/refuse policy controls for the cylinder operation.
use martensite_tkbo::profile_extrusion::*;
use martensite_tksketch::surface_sketch::*;

fn support() -> CylinderSupport {
    CylinderSupport {
        origin: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        radius: 10.,
        chart_origin: 0.,
        normal: NormalSense::Outward,
    }
}
pub(super) fn rectangle() -> CylinderRectangle {
    CylinderRectangle {
        support: support(),
        region: SurfaceRegionId(1),
        s: [0., 7.],
        z: [0., 4.],
        corners: [1, 2, 3, 4].map(SurfaceVertexId),
        boundaries: [1, 2, 3, 4].map(SurfaceCurveId),
    }
}
pub(super) fn options() -> NormalExtrusionOptions {
    NormalExtrusionOptions {
        resolution: Resolution::new(
            1e-6,
            1e-7,
            1e-9,
            DisplayTolerance {
                chordal: 1e-3,
                angular: 0.1,
            },
            ToleranceCap::Refuse(1e-5),
        ),
        volume_relative_error: 1e-9,
    }
}
const INVALID_DOMAINS: [fn(&mut CylinderRectangle); 17] = [
    |r: &mut CylinderRectangle| r.support.radius = 0.,
    |r: &mut CylinderRectangle| r.support.radius = -1.,
    |r: &mut CylinderRectangle| r.support.radius = f64::INFINITY,
    |r: &mut CylinderRectangle| r.support.origin[0] = f64::NAN,
    |r: &mut CylinderRectangle| r.support.origin[0] = 1e20,
    |r: &mut CylinderRectangle| r.support.axis = [0., 0., 2.],
    |r: &mut CylinderRectangle| r.support.radial = [0., 0., 1.],
    |r: &mut CylinderRectangle| r.s = [0., 0.],
    |r: &mut CylinderRectangle| r.z = [4., 0.],
    |r: &mut CylinderRectangle| r.z = [0., 1e-9],
    |r: &mut CylinderRectangle| r.s[1] = 10. * std::f64::consts::TAU,
    |r: &mut CylinderRectangle| r.s[1] = 20. * std::f64::consts::TAU,
    |r: &mut CylinderRectangle| r.s[1] = 10. * (std::f64::consts::TAU - 1e-10),
    |r: &mut CylinderRectangle| r.support.chart_origin = 1e20,
    |r: &mut CylinderRectangle| r.corners[1] = r.corners[0],
    |r: &mut CylinderRectangle| r.boundaries[1] = r.boundaries[0],
    |r: &mut CylinderRectangle| r.z = [-f64::MAX, f64::MAX],
];

#[test]
fn invalid_domains_cannot_publish_a_partial_body() {
    for edit in INVALID_DOMAINS {
        let mut request = rectangle();
        edit(&mut request);
        let error = normal_extrude(&request, [-1., 2.], options()).unwrap_err();
        assert_eq!(error.stage, SurfaceStage::Admission, "{error}");
        assert_eq!(error.region, request.region);
    }
}
#[test]
fn depths_and_volume_convergence_are_explicit_refusals() {
    for depths in [
        [0., 0.],
        [2., -1.],
        [-10., 1.],
        [-11., -10.],
        [0., f64::NAN],
    ] {
        assert_eq!(
            normal_extrude(&rectangle(), depths, options())
                .unwrap_err()
                .stage,
            SurfaceStage::Admission
        );
    }
}
#[test]
fn invalid_volume_convergence_cannot_publish_a_result() {
    for relative in [0., -1., f64::NAN, f64::INFINITY, f64::EPSILON, 0.1] {
        let mut options = options();
        options.volume_relative_error = relative;
        assert_eq!(
            normal_extrude(&rectangle(), [-1., 2.], options)
                .unwrap_err()
                .stage,
            SurfaceStage::Admission
        );
    }
}
#[test]
fn report_cap_records_a_finding_while_refuse_cap_prevents_publication() {
    let mut settings = options();
    settings.resolution.cap = ToleranceCap::Report(1e-12);
    let result = normal_extrude(&rectangle(), [-1., 2.], settings).unwrap();
    assert!(result.validation().is_valid());
    assert_eq!(result.options(), settings);
    assert_eq!(result.effective_limits().linear_separation, 2e-6);
    assert_eq!(result.effective_limits().angular_clearance, 1e-9);
    assert_eq!(result.diagnostics().len(), 1);
    let errors = result.volume_error_estimates();
    assert_eq!(
        result.volume().error_bound,
        errors.integration + errors.construction
    );
    assert!(result.achieved_tolerance().max() > 1e-12);
    settings.resolution.cap = ToleranceCap::Refuse(1e-12);
    assert!(normal_extrude(&rectangle(), [-1., 2.], settings).is_err());
}
#[test]
fn physical_end_face_clearance_is_checked_separately_from_angle() {
    let mut request = rectangle();
    request.s[1] = 10. * (std::f64::consts::TAU - 1e-7);
    let error = normal_extrude(&request, [-9.99, -9.98], options()).unwrap_err();
    assert!(matches!(
        error.cause,
        SurfaceFailureCause::InvalidInput {
            field: "end-face separation",
            ..
        }
    ));
}
