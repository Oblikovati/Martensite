use super::*;
use martensite_tkmath::gp::{Axis2, Point3, StandardDirection3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use std::collections::BTreeMap;

const GLYPHS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/features/surface-normal-sketches/references/p5-owned-glyphs.tsv"
));
const AREAS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/features/surface-normal-sketches/references/p5-owned-glyph-areas.tsv"
));
pub(super) fn text_support() -> CylinderFaceAttachment {
    let axes = Axis2::with_standard_frame(
        Point3::new(0., 0., -4.),
        StandardDirection3::Z,
        StandardDirection3::X,
    );
    let mut cylinder = BRepPrimAPIMakeCylinder::with_axes(axes, 10., 8.);
    super::super::super::super::super::tests::freeze(cylinder.shape().unwrap())
}

pub(super) fn text_request(attachment: &CylinderFaceAttachment) -> WrappedNormalRequest {
    let mut request = request(attachment, [1., 2.], [1., 2.]);
    request.support.normal = NormalSense::Outward;
    request.options.planar_work = PlanarProofWorkLimit::PairBisections(8192);
    request.profile = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: Vec::new(),
        curves: Vec::new(),
    };
    request.selections = Vec::new();
    append_owned_glyph_material(&mut request);
    assert_eq!(request.profile.curves.len(), 239);
    assert_eq!(request.selections.len(), 18);
    request
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Contour {
    instance: u32,
    owner: u32,
}

fn number(value: &str) -> u32 {
    value.parse().unwrap()
}

fn authored(contour: Contour, segment: u32) -> u64 {
    u64::from(contour.instance + 1) * 1000 + u64::from(contour.owner * 100 + segment + 1)
}

fn append_owned_glyph_material(request: &mut WrappedNormalRequest) {
    let mut contours: BTreeMap<Contour, Vec<Vec<&str>>> = BTreeMap::new();
    for source in GLYPHS.lines().filter(|row| !row.starts_with('#')) {
        let row: Vec<_> = source.split('\t').collect();
        let contour = Contour {
            instance: number(row[0]),
            owner: number(row[5]),
        };
        contours.entry(contour).or_default().push(row);
    }
    let area_signs = contour_area_signs();
    let mut holes = BTreeMap::new();
    for (contour, rows) in &contours {
        let wire = append_owned_contour(*contour, rows, &mut request.profile);
        if area_signs[contour] < 0 {
            let selection = WrappedRegionSelection {
                region: SurfaceRegionId(authored(*contour, 0)),
                outer: wire,
                holes: Vec::new(),
            };
            request.selections.push(selection);
        } else {
            assert!(holes.insert(contour.instance, wire).is_none());
        }
    }
    for (instance, wire) in holes {
        let selection = request
            .selections
            .iter_mut()
            .find(|selection| selection.region.0 / 1000 == u64::from(instance + 1))
            .unwrap();
        selection.holes.push(wire);
    }
}

fn contour_area_signs() -> BTreeMap<Contour, i64> {
    AREAS
        .lines()
        .skip(2)
        .map(|row| {
            let row: Vec<_> = row.split('\t').collect();
            let key = Contour {
                instance: number(row[0]),
                owner: number(row[1]),
            };
            (key, row[2].parse::<i64>().unwrap().signum())
        })
        .collect()
}

fn append_owned_contour(
    contour: Contour,
    rows: &[Vec<&str>],
    profile: &mut SketchProfile,
) -> ProfileLoop {
    let mut uses = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(number(row[6]) as usize, index);
        let id = authored(contour, index as u32);
        let next = authored(contour, ((index + 1) % rows.len()) as u32);
        let start = ProfileVertexId(id);
        let end = ProfileVertexId(next);
        profile.vertices.push(ProfileVertex {
            id: start,
            point: positioned(row, 8),
        });
        assert_eq!(
            positioned(row, 14),
            positioned(&rows[(index + 1) % rows.len()], 8)
        );
        let id = ProfileCurveId(id);
        profile.curves.push(prototype(row, id, start, end));
        uses.push(LoopUse {
            curve: id,
            reversed: true,
        });
    }
    uses.reverse();
    ProfileLoop { uses }
}

fn positioned(row: &[&str], start: usize) -> [f64; 2] {
    let coordinate = |index: usize| f64::from_bits(u64::from_str_radix(row[index], 16).unwrap());
    [
        coordinate(start) / 1024. + 2. * f64::from(number(row[2])) + 0.5,
        coordinate(start + 1) / 1024. - 3. * f64::from(number(row[1])) + 4.,
    ]
}

fn prototype(
    row: &[&str],
    id: ProfileCurveId,
    start: ProfileVertexId,
    end: ProfileVertexId,
) -> ProfileCurve {
    if row[7] == "Line" {
        return ProfileCurve::Line { id, start, end };
    }
    let mut poles = vec![positioned(row, 8), positioned(row, 10)];
    match row[7] {
        "QuadraticBezier" => {}
        "CubicBezier" => poles.push(positioned(row, 12)),
        kind => panic!("unexpected pinned outline segment {kind}"),
    }
    poles.push(positioned(row, 14));
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
