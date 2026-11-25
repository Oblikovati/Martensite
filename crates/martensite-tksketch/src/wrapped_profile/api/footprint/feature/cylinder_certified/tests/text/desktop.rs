use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    MetricBezier, MetricCurve, OrientedMetricCurve, certify_polynomial_boundary_integral,
};
use std::collections::BTreeMap;

const BREP: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/wrapped_text/desktop-text-cylinder.brep"
));
const MATERIAL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/wrapped_text/desktop-text-material.tsv"
));

pub(super) fn stock() -> CylinderFaceAttachment {
    let resolution = Resolution::new(
        2.8284271256876392e-9,
        0.0002828427125687639,
        1e-9,
        DisplayTolerance {
            chordal: 0.05,
            angular: 0.5,
        },
        ToleranceCap::Refuse(0.0002828427125687639),
    );
    CylinderFaceAttachment::restore(CylinderAttachmentRecord {
        id: FaceAttachmentId(2),
        revision: REVISION,
        resolution,
        brep: BREP.to_vec(),
    })
    .unwrap()
}

pub(super) fn request(stock: &CylinderFaceAttachment) -> WrappedNormalRequest {
    let mut sketch = super::request(stock, [0., 1.], [0., 1.]);
    sketch.depths = [0., 0.25 * sketch.support.radius];
    sketch.options.planar_work = PlanarProofWorkLimit::PairBisections(8192);
    sketch.profile.curves.clear();
    sketch.profile.vertices.clear();
    sketch.selections.clear();
    for line in MATERIAL.lines().filter(|line| !line.starts_with('#')) {
        row(&mut sketch, line);
    }
    assert_eq!(sketch.profile.curves.len(), 265);
    assert_eq!(sketch.selections.len(), 18);
    sketch
}

fn row(sketch: &mut WrappedNormalRequest, line: &str) {
    let words: Vec<_> = line.split('\t').collect();
    let id = words[1].parse::<u64>().unwrap();
    match words[0] {
        "v" => sketch.profile.vertices.push(ProfileVertex {
            id: ProfileVertexId(id),
            point: point(&words[2..]),
        }),
        "c" => sketch.profile.curves.push(curve(id, &words[2..])),
        "r" => sketch.selections.push(WrappedRegionSelection {
            region: SurfaceRegionId(id),
            outer: ring(words[2]),
            holes: words[3..].iter().map(|value| ring(value)).collect(),
        }),
        _ => panic!("unknown desktop fixture statement {}", words[0]),
    }
}

fn point(words: &[&str]) -> [f64; 2] {
    [0, 1].map(|i| f64::from_bits(u64::from_str_radix(words[i], 16).unwrap()))
}

fn curve(id: u64, words: &[&str]) -> ProfileCurve {
    let id = ProfileCurveId(id);
    let start = ProfileVertexId(words[0].parse().unwrap());
    let end = ProfileVertexId(words[1].parse().unwrap());
    if words.len() == 2 {
        return ProfileCurve::Line { id, start, end };
    }
    let (pairs, remainder) = words[2..].as_chunks::<2>();
    assert!(remainder.is_empty());
    let poles: Vec<_> = pairs.iter().map(|pair| point(pair)).collect();
    let count = poles.len();
    let knots = std::iter::repeat_n(0., count)
        .chain(std::iter::repeat_n(1., count))
        .collect();
    ProfileCurve::Spline {
        id,
        start,
        end,
        degree: (count - 1) as u32,
        poles,
        weights: None,
        knots,
    }
}

fn ring(uses: &str) -> ProfileLoop {
    ProfileLoop {
        uses: uses
            .split(',')
            .map(|use_| {
                let signed: i64 = use_.parse().unwrap();
                LoopUse {
                    curve: ProfileCurveId(signed.unsigned_abs()),
                    reversed: signed < 0,
                }
            })
            .collect(),
    }
}

pub(super) fn developed_area(sketch: &WrappedNormalRequest) -> [f64; 2] {
    let controls = authored_controls(&sketch.profile);
    let sum = sketch.selections.iter().fold([0., 0.], |sum, region| {
        let outer = enclosed_area(&region.outer, &controls);
        let material = region.holes.iter().fold(outer, |area, hole| {
            let [lower, upper] = enclosed_area(hole, &controls);
            [(area[0] - upper).next_down(), (area[1] - lower).next_up()]
        });
        [
            (sum[0] + material[0]).next_down(),
            (sum[1] + material[1]).next_up(),
        ]
    });
    assert!(sum[0] > 0. && sum[1].is_finite());
    sum
}

fn authored_controls(profile: &SketchProfile) -> BTreeMap<ProfileCurveId, Vec<[f64; 2]>> {
    let vertices: BTreeMap<_, _> = profile.vertices.iter().map(|v| (v.id, v.point)).collect();
    profile
        .curves
        .iter()
        .map(|curve| match curve {
            ProfileCurve::Line { id, start, end } => (*id, vec![vertices[start], vertices[end]]),
            ProfileCurve::Spline {
                id,
                poles,
                weights: None,
                ..
            } => (*id, poles.clone()),
            unexpected => panic!("unexpected desktop fixture curve {unexpected:?}"),
        })
        .collect()
}

fn enclosed_area(
    ring: &ProfileLoop,
    controls: &BTreeMap<ProfileCurveId, Vec<[f64; 2]>>,
) -> [f64; 2] {
    let curves: Vec<_> = ring
        .uses
        .iter()
        .map(|edge| OrientedMetricCurve {
            curve: MetricCurve::Bezier(MetricBezier {
                poles: &controls[&edge.curve],
                weights: None,
            }),
            reversed: edge.reversed,
        })
        .collect();
    let [low, high] = certify_polynomial_boundary_integral(&curves, Default::default(), || false)
        .unwrap()
        .enclosure();
    assert!(low > 0. || high < 0.);
    if low > 0. { [low, high] } else { [-high, -low] }
}
