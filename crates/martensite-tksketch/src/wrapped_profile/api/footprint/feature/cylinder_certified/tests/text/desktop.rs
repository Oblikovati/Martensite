use super::*;
use std::collections::BTreeMap;

const BREP: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/features/surface-normal-sketches/references/desktop-text-cylinder.brep"
));
const MATERIAL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/features/surface-normal-sketches/references/desktop-text-material.tsv"
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
    let poles: Vec<_> = words[2..].chunks_exact(2).map(point).collect();
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

pub(super) fn developed_area(sketch: &WrappedNormalRequest) -> f64 {
    let vertices: BTreeMap<_, _> = sketch
        .profile
        .vertices
        .iter()
        .map(|vertex| (vertex.id, vertex.point))
        .collect();
    let integrals: BTreeMap<_, _> = sketch
        .profile
        .curves
        .iter()
        .map(|curve| match curve {
            ProfileCurve::Line { id, start, end } => (*id, cross(vertices[start], vertices[end])),
            ProfileCurve::Spline { id, poles, .. } => (*id, spline_integral(poles)),
            unexpected => panic!("unexpected desktop fixture curve {unexpected:?}"),
        })
        .collect();
    let area = |ring: &ProfileLoop| {
        0.5 * ring
            .uses
            .iter()
            .map(|edge| {
                if edge.reversed {
                    -integrals[&edge.curve]
                } else {
                    integrals[&edge.curve]
                }
            })
            .sum::<f64>()
            .abs()
    };
    sketch
        .selections
        .iter()
        .map(|face| area(&face.outer) - face.holes.iter().map(area).sum::<f64>())
        .sum()
}

fn cross([x, y]: [f64; 2], [u, v]: [f64; 2]) -> f64 {
    x * v - y * u
}

fn spline_integral(poles: &[[f64; 2]]) -> f64 {
    // Three-point Gauss-Legendre integrates x dy − y dx for each polynomial
    // of degree at most three in this frozen source fixture.
    assert!((2..=4).contains(&poles.len()));
    let derivative: Vec<_> = poles
        .windows(2)
        .map(|pair| std::array::from_fn(|i| (poles.len() - 1) as f64 * (pair[1][i] - pair[0][i])))
        .collect();
    let r = (3_f64 / 5.).sqrt() / 2.;
    [(0.5 - r, 5. / 18.), (0.5, 4. / 9.), (0.5 + r, 5. / 18.)]
        .iter()
        .map(|&(t, w)| w * cross(evaluate(poles, t), evaluate(&derivative, t)))
        .sum()
}

fn evaluate(controls: &[[f64; 2]], t: f64) -> [f64; 2] {
    let mut points = controls.to_vec();
    for len in (1..points.len()).rev() {
        for i in 0..len {
            points[i] = std::array::from_fn(|j| (1. - t) * points[i][j] + t * points[i + 1][j]);
        }
    }
    points[0]
}
