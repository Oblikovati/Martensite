//! Curved open patches distinguish a transported field from the world-origin field (#528).
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalFlux, CylindricalFluxQuery, CylindricalFluxSpan, MetricBezier,
    certify_cylindrical_boundary_flux,
};
const LOCAL: [[[f64; 3]; 2]; 2] = [[[0., 0., 1.], [1., 0., 1.]], [[1., 0., 1.5], [2., 1., 1.5]]];
const OBLIQUE: [[[f64; 3]; 2]; 2] = [
    [[0., 0.5, 1.], [1., 0.5, 1.5]],
    [[1., 0.75, 2.], [2.5, 1.75, 2.5]],
];
const ORIGIN: [f64; 3] = [1048576., -2097152., 524288.];

#[test]
fn centered_flux_matches_independent_translated_curved_patch_integration() {
    let local = integrate(&LOCAL);
    let world_rows = translate(&OBLIQUE, ORIGIN);
    let centered_rows = translate(&world_rows, ORIGIN.map(|value| -value));
    assert_eq!(centered_rows, OBLIQUE);
    let centered = integrate(&centered_rows);
    let transported = product(reports::range(local.enclosure()), [1.125; 2]).unwrap();
    assert!(transported.lower <= centered.enclosure()[0]);
    assert!(transported.upper >= centered.enclosure()[1]);
    assert_eq!(centered.enclosure(), [3. / 64.; 2]);
    assert_eq!(centered.integration_subintervals(), 0);
    assert_world_origin_is_a_different_quantity(&world_rows, transported);
}

fn assert_world_origin_is_a_different_quantity(
    rows: &[[[f64; 3]; 2]; 2],
    centered: WrappedCertifiedRange,
) {
    // S=(u+v,uv,1+v/2) has vector area (1/4,-1/2,0) and flux 1/24.
    // det(A)=9/8 and (t/3).cofactor(A).N=1605632/3 for the dyadic oblique map.
    let world = integrate(rows);
    let expected = 102760457. / 192.;
    assert!(world.enclosure()[0] <= expected && world.enclosure()[1] >= expected);
    assert!(world.enclosure()[0] > centered.upper);
    assert_eq!(world.integration_subintervals(), 0);
}

fn integrate(rows: &[[[f64; 3]; 2]; 2]) -> CertifiedCylindricalFlux {
    let uv = [[0., 0.]; 2];
    let spans = [CylindricalFluxSpan {
        boundaries: rows.each_ref().map(|poles| MetricBezier {
            poles,
            weights: None,
        }),
        cap_pcurves: [&uv; 2],
        side_reversed: false,
    }];
    let query = CylindricalFluxQuery {
        spans: &spans,
        radii: [1., 2.],
        options: Default::default(),
    };
    certify_cylindrical_boundary_flux(query, || false).unwrap()
}

fn translate(rows: &[[[f64; 3]; 2]; 2], origin: [f64; 3]) -> [[[f64; 3]; 2]; 2] {
    rows.map(|row| row.map(|point| std::array::from_fn(|i| point[i] + origin[i])))
}
