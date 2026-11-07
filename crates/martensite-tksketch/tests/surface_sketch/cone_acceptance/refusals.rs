//! Invalid domains cannot publish a body or change the caller's request.
use super::sector;
use martensite_tksketch::surface_sketch::*;

type InvalidEdit = (&'static str, fn(&mut ConeSector));
const INVALID: &[InvalidEdit] = &[
    ("original apex", |r| r.slant[0] = 0.),
    ("negative slant", |r| r.slant[0] = -1.),
    ("collapsed slant", |r| r.slant = [8., 8.]),
    ("reversed slant", |r| r.slant = [12., 8.]),
    ("unresolved slant", |r| r.slant = [8., 8. + 1e-10]),
    ("collapsed angle", |r| r.developed_angle = [0., 0.]),
    ("reversed angle", |r| r.developed_angle = [0.42, 0.]),
    ("full turn", |r| {
        r.developed_angle[1] = 0.6 * std::f64::consts::TAU
    }),
    ("multiple turns", |r| {
        r.developed_angle[1] = 1.2 * std::f64::consts::TAU
    }),
    ("angular closure", |r| {
        r.developed_angle[1] = 0.6 * (std::f64::consts::TAU - 1e-10)
    }),
    ("zero semiangle", |r| r.support.semi_angle = 0.),
    ("negative semiangle", |r| r.support.semi_angle = -0.1),
    ("plane limit", |r| {
        r.support.semi_angle = std::f64::consts::FRAC_PI_2
    }),
    ("unresolved axis limit", |r| r.support.semi_angle = 1e-12),
    ("unresolved plane limit", |r| {
        r.support.semi_angle = std::f64::consts::FRAC_PI_2 - 1e-12
    }),
    ("nonfinite semiangle", |r| r.support.semi_angle = f64::NAN),
    ("nonfinite apex", |r| r.support.apex[0] = f64::INFINITY),
    ("coordinate loss", |r| r.support.apex[0] = 1e20),
    ("bad axis", |r| r.support.axis = [0., 0., 2.]),
    ("parallel frame", |r| r.support.radial = [0., 0., 1.]),
    ("chart resolution", |r| r.support.chart_origin = 1e20),
    ("angular conversion", |r| {
        r.developed_angle = [-f64::MAX, f64::MAX]
    }),
    ("carrier overflow", |r| r.slant = [f64::MAX / 2., f64::MAX]),
    ("duplicate corner", |r| r.corners[1] = r.corners[0]),
    ("duplicate boundary", |r| r.boundaries[1] = r.boundaries[0]),
];

#[test]
fn unresolved_or_invalid_cone_domains_are_typed_admission_failures() {
    for (label, edit) in INVALID {
        let mut request = sector();
        edit(&mut request);
        let original = format!("{request:?}");
        let error =
            normal_extrude_cone(&request, [-1., 2.], crate::refusals::options()).unwrap_err();
        assert_eq!(error.stage, SurfaceStage::Admission, "{label}: {error}");
        assert_eq!(error.region, request.region);
        assert_eq!(format!("{request:?}"), original, "{label}");
    }
}

#[test]
fn cone_axis_contact_and_invalid_depths_cannot_publish_success() {
    for (normal, depths) in [
        (NormalSense::Outward, [-6., 2.]),
        (NormalSense::Outward, [-7., 2.]),
        (NormalSense::Inward, [-1., 6.]),
        (NormalSense::Inward, [-1., 7.]),
        (NormalSense::Outward, [0., 0.]),
        (NormalSense::Outward, [2., -1.]),
        (NormalSense::Outward, [0., f64::NAN]),
    ] {
        let mut request = sector();
        request.support.normal = normal;
        let original = request;
        let error = normal_extrude_cone(&request, depths, crate::refusals::options()).unwrap_err();
        assert_eq!(error.stage, SurfaceStage::Admission, "{depths:?}: {error}");
        assert_eq!(request, original);
    }
}

#[test]
fn physical_end_clearance_is_checked_separately_from_angular_clearance() {
    let mut request = sector();
    request.slant = [0.0001, 0.0002];
    request.developed_angle = [0., 0.6 * (std::f64::consts::TAU - 1e-4)];
    let error =
        normal_extrude_cone(&request, [-0.00007, 0.], crate::refusals::options()).unwrap_err();
    assert_eq!(error.stage, SurfaceStage::Admission, "{error}");
}
