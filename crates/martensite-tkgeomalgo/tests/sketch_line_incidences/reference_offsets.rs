// SPDX-License-Identifier: GPL-2.0-only
//! Offset and periodic-spline certified contacts against frozen reference contacts and the
//! mapped Rust intersector. Reference points are rounded solver output;
//! counts must agree and each reference point must lie within the certified error
//! plus 1e-8 of a distinct contact.
use super::corpus::{Curve, mapped_target, parse_target, values};
use super::mapped_offsets::mapped_extension;
use super::reference_curves::{Location, assert_covers_contacts};
use martensite_tkgeomalgo::geom2d_api::InterCurveCurve;
use martensite_tkmath::predicates::roots::sketch_incidence::*;

const ERROR: f64 = 1e-12;

struct OffsetCase {
    id: usize,
    kind: String,
    source: SketchIncidenceCurve,
    target: SketchIncidenceCurve,
}
fn corpus() -> Vec<OffsetCase> {
    include_str!("offset-cases.csv")
        .lines()
        .map(|row| {
            let fields: Vec<&str> = row.split(';').collect();
            OffsetCase {
                id: fields[0].parse().unwrap(),
                kind: fields[1].to_owned(),
                source: parse_target(fields[2]),
                target: parse_target(fields[3]),
            }
        })
        .collect()
}
fn reference_points() -> Vec<Vec<Location>> {
    include_str!("offset-contacts.csv")
        .lines()
        .map(|row| {
            let fields: Vec<&str> = row.split(';').collect();
            assert_eq!(fields[3], "0", "no reference coincidence zones: {row}");
            fields[2]
                .split('|')
                .filter(|text| !text.is_empty())
                .map(|text| {
                    let v = values(text);
                    Location {
                        point: [v[0], v[1]],
                        radius: 0.,
                    }
                })
                .collect()
        })
        .collect()
}
fn certify_case(case: &OffsetCase) -> SketchLineIncidenceReport {
    let source = SketchIncidenceSource {
        id: 0,
        curve: case.source.clone(),
        domain: SketchLineDomain::Segment,
    };
    let target = SketchIncidenceTarget {
        id: case.id as u64,
        curve: case.target.clone(),
    };
    certify_sketch_line_incidences(source, &[target], SketchIncidenceOptions::v1(ERROR), || {
        false
    })
    .unwrap_or_else(|failure| panic!("case {} ({}): {failure}", case.id, case.kind))
}
fn label(case: &OffsetCase) -> String {
    format!("case {} ({})", case.id, case.kind)
}
fn mapped(curve: &SketchIncidenceCurve) -> Curve {
    match curve {
        SketchIncidenceCurve::Offset(_) | SketchIncidenceCurve::PeriodicSpline(_) => {
            mapped_extension(curve)
        }
        other => mapped_target(other),
    }
}
#[test]
fn certified_offset_contacts_match_every_reference_point() {
    let cases = corpus();
    let reference = reference_points();
    assert_eq!((cases.len(), reference.len()), (108, 108));
    for (case, found) in cases.iter().zip(&reference) {
        assert_covers_contacts(&label(case), &certify_case(case), found);
    }
}
#[test]
fn certified_offset_contacts_match_the_mapped_rust_intersector() {
    for case in corpus() {
        let mapped =
            InterCurveCurve::between(&mapped(&case.source), &mapped(&case.target), 1e-9).unwrap();
        assert_eq!(mapped.nb_segments(), 0, "{}", label(&case));
        let found: Vec<Location> = (1..=mapped.nb_points())
            .map(|index| {
                let point = mapped.point(index).unwrap();
                Location {
                    point: [point.x(), point.y()],
                    radius: 0.,
                }
            })
            .collect();
        assert_covers_contacts(&label(&case), &certify_case(&case), &found);
    }
}
