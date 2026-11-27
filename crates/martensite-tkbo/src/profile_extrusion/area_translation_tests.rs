//! The public operation certifies represented coordinates, independently of chart offset (#446).
use super::*;
fn issue_resolution() -> Resolution {
    Resolution::new(
        1e-12,
        1e-11,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    )
}
mod controls;
mod fixtures;
#[test]
fn translated_unit_square_keeps_its_certified_area() {
    let result = build_planar_regions(
        &fixtures::rectangle(1e8, [1.; 2], false),
        issue_resolution(),
    )
    .unwrap();
    assert_eq!(result.regions().len(), 1);
    assert!(result.regions()[0].area().encloses(1.));
}
#[test]
fn translated_decimal_square_uses_exact_represented_width() {
    let result = build_planar_regions(
        &fixtures::rectangle(1e8, [0.01; 2], false),
        issue_resolution(),
    )
    .unwrap();
    let represented_area = 450_360_445_921.0 / 4_503_599_627_370_496.0;
    assert_eq!(result.regions().len(), 1);
    assert!(result.regions()[0].area().encloses(represented_area));
}

#[test]
fn translated_circle_certifies_with_analytic_area_across_proof_versions() {
    let versions = [
        PlanarProofVersion::CarrierV2,
        PlanarProofVersion::BernsteinV3,
        PlanarProofVersion::FilteredBernsteinV4,
    ];
    for radius in [0.1, 0.2, 0.3, 0.7] {
        let diagonal = 2.0_f64.sqrt() * 2.0 * radius;
        let resolution = Resolution::new(
            1e-9 * diagonal,
            1e-4 * diagonal,
            1e-9,
            DisplayTolerance {
                chordal: 0.05,
                angular: 0.5,
            },
            ToleranceCap::Refuse(1e-4 * diagonal),
        );
        for origin in [
            [0., 0., 0.],
            [2., 0., 2.],
            [0., 2., 2.],
            [2., 2., 2.],
            [-3.7, 5.3, 1.1],
        ] {
            let profile = SketchProfile {
                plane: ProfilePlane {
                    origin,
                    ..ProfilePlane::world_xy()
                },
                vertices: vec![],
                curves: vec![ProfileCurve::Circle {
                    id: ProfileCurveId(6),
                    center: [0., 0.],
                    radius,
                }],
            };
            let expected = std::f64::consts::PI * radius * radius;
            let default = build_planar_regions(&profile, resolution)
                .unwrap_or_else(|cause| panic!("default r={radius} at {origin:?}: {cause}"));
            assert_eq!(default.regions().len(), 1);
            assert!(default.regions()[0].area().encloses(expected));
            for version in versions {
                let result = build_planar_regions_with_proof_and_cancel(
                    &profile,
                    resolution,
                    PlanarProofWorkLimit::CurrentDefault,
                    version,
                    || false,
                )
                .unwrap_or_else(|cause| panic!("{version:?} r={radius} at {origin:?}: {cause}"));
                assert_eq!(result.regions().len(), 1);
                assert!(result.regions()[0].area().encloses(expected));
            }
        }
    }
}
