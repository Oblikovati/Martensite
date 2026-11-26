// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::gp::{Axis3, Direction2, Direction3, Point2};

const IDENTITY: SectionPlacement = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];

fn xy(origin: Point3, placement: SectionPlacement) -> SectionChartSource {
    let frame = Axis3::new(origin, Direction3::Z, Direction3::X).unwrap();
    SectionChartSource {
        plane: Plane3::from_position(frame),
        placement,
    }
}

fn rectangle() -> [SectionBorder; 4] {
    let points = [
        Point3::new(-3., -2., 5.),
        Point3::new(3., -2., 5.),
        Point3::new(3., 2., 5.),
        Point3::new(-3., 2., 5.),
    ];
    let directions = [
        Direction3::X,
        Direction3::Y,
        Direction3::new(-1., 0., 0.).unwrap(),
        Direction3::new(0., -1., 0.).unwrap(),
    ];
    std::array::from_fn(|i| {
        let j = (i + 1) % 4;
        let range = [0., points[i].distance(&points[j])];
        let [dx, dy, _] = *directions[i].coordinates();
        SectionBorder {
            space: SectionLineSource {
                line: Line3::new(points[i], directions[i]),
                placement: IDENTITY,
                range,
            },
            vertices: [points[i], points[j]].map(|point| SectionVertexSource {
                point,
                placement: IDENTITY,
            }),
            reversed: false,
            pcurve: Some(SectionUvLine {
                line: Line2::new(
                    Point2::new(points[i].x(), points[i].y()),
                    Direction2::new(dx, dy).unwrap(),
                ),
                range,
            }),
        }
    })
}

fn two_thirds() -> [SectionChartSource; 2] {
    let source = xy(Point3::new(0., 0., 5.), IDENTITY);
    let work = xy(
        Point3::default(),
        [[-1., 0., 3., 0.], [3., 0., 1., 0.], [0., 1., 0., 0.]],
    );
    [source, work]
}

fn rounded_native() -> (SectionLineSource, [SectionVertexSource; 2]) {
    let placement = [[-2. / 3., 0., 1., 0.], [2., 0., 0., 0.], [0., 1., 0., 5.]];
    let line = SectionLineSource {
        line: Line3::new(Point3::default(), Direction3::X),
        placement,
        range: [-1., 1.],
    };
    let vertices = [-1., 1.].map(|parameter| SectionVertexSource {
        point: Point3::new(parameter, 0., 0.),
        placement,
    });
    (line, vertices)
}

#[test]
fn exact_oblique_trim_encloses_non_binary_native_boundary_parameters() {
    let trim = certify_planar_section_trim(two_thirds(), false, &rectangle(), || false).unwrap();
    let PlanarSectionTrim::Segment(witness) = trim else {
        panic!("positive oblique trim");
    };
    let (line, vertices) = rounded_native();
    let evidence = witness
        .certify_native(line, vertices, [None; 2], 1e-15, || false)
        .unwrap();
    assert_eq!(evidence.endpoints, [[2. / 3., -2., 5.], [-2. / 3., 2., 5.]]);
    assert_eq!(evidence.native_parameters, [-1., 1.]);
    assert_eq!(evidence.vertex_error_bound, 0.);
    assert!(evidence.native_error_bound > 0. && evidence.native_error_bound <= 1e-15);
    assert_eq!(evidence.position_error_bound, evidence.native_error_bound);
    let thirds = exact::Scalar::from_finite(11.).divide(&exact::Scalar::from_finite(3.));
    for (end, anchor) in evidence.anchors.into_iter().enumerate() {
        let PlanarSectionAnchor::Edge {
            index,
            parameter: [lower, upper],
        } = anchor
        else {
            panic!("interior edge root");
        };
        assert_eq!(index, 2 * end);
        assert_eq!(lower.next_up(), upper);
        assert!(
            exact::Scalar::from_finite(lower) < thirds
                && thirds < exact::Scalar::from_finite(upper)
        );
    }
    let tight = witness
        .certify_native(line, vertices, [None; 2], 0., || false)
        .unwrap_err();
    assert!(
        matches!(tight, PlanarSectionError::Position { upper, limit } if upper > 0. && limit == 0.)
    );
}

#[test]
fn source_pcurve_and_actual_candidate_own_independent_obligations() {
    let mut border = rectangle();
    border[1].pcurve.as_mut().unwrap().range = [0., 3.];
    let invalid = certify_planar_section_trim(two_thirds(), false, &border, || false).unwrap_err();
    assert_eq!(invalid, PlanarSectionError::InconsistentBorder(1));

    let trim = certify_planar_section_trim(two_thirds(), false, &rectangle(), || false).unwrap();
    let PlanarSectionTrim::Segment(witness) = trim else {
        panic!("positive oblique trim");
    };
    let (line, mut vertices) = rounded_native();
    let wrong_pcurve = SectionUvLine {
        line: Line2::default(),
        range: line.range,
    };
    let mismatch = witness
        .certify_native(line, vertices, [Some(wrong_pcurve), None], 1e-7, || false)
        .unwrap_err();
    assert!(matches!(mismatch, PlanarSectionError::Position { .. }));
    vertices[0].point = Point3::new(0., 0., 0.);
    let mismatch = witness
        .certify_native(line, vertices, [None; 2], 1e-7, || false)
        .unwrap_err();
    assert!(matches!(mismatch, PlanarSectionError::Position { .. }));
}
