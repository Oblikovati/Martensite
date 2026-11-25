// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, pid, point, seed};
use martensite_tksketch3d::{
    ControlSplineProofPolicy, CurveId, EditFailure, EntityId, FitSplineId, FitSplineParameters,
    FitSplinePolicy, MinimumLength, PointRole, SketchLine, SketchPoint, Snapshot,
    import::{ImportCurves, ImportFailure, ImportedSketch, SketchImport},
};
use std::{num::NonZeroU64, thread};

const XYZ: [[f64; 3]; 3] = [[0., 0., 0.], [4., 0., 0.], [4., 3., 0.]];

fn empty() -> Snapshot {
    Snapshot::empty(MinimumLength::new(1e-3).unwrap())
}

fn row(id: u64, xyz: [f64; 3], role: PointRole) -> SketchPoint {
    SketchPoint {
        id: pid(id),
        position: point(xyz),
        role,
    }
}

fn read_rows() -> [SketchPoint; 3] {
    let [a, b, c] = XYZ;
    [
        row(42, a, PointRole::SketchMarker),
        row(4, b, PointRole::CenterMarker),
        row(23, c, PointRole::SketchMarker),
    ]
}

fn plan<'a>(points: &'a [SketchPoint], curves: ImportCurves<'a>, max: usize) -> SketchImport<'a> {
    SketchImport {
        points,
        curves,
        max_new_entities: max,
    }
}

#[test]
fn imports_the_owned_millimetre_rows_and_directed_polyline_in_source_order() {
    let rows = read_rows();
    let source = empty();
    let markers = source
        .import_normalized(plan(&rows, ImportCurves::Points, 3), || false)
        .unwrap();
    let result = source
        .import_normalized(
            plan(&rows, ImportCurves::Polyline(&[lid(76), lid(51)]), 5),
            || false,
        )
        .unwrap();
    let [a, b, c] = rows.map(|row| row.id);
    for imported in [&markers, &result] {
        assert_eq!(imported.rows, [a, b, c]);
        for row in rows {
            assert_eq!(imported.sketch.point(row.id), Some(row));
        }
    }
    assert!(markers.curves.is_empty());
    assert_eq!(
        result.curves,
        [CurveId::Line(lid(76)), CurveId::Line(lid(51))]
    );
    let start = result.sketch.line(lid(76)).unwrap();
    let end = result.sketch.line(lid(51)).unwrap();
    assert_eq!(start.line.endpoints, [a, b]);
    assert_eq!(end.line.endpoints, [b, c]);
    assert_eq!(start.length_estimate + end.length_estimate, 7.);
    assert_eq!(source.owned_ids().count(), 0);
}

#[test]
fn drawing_segments_keep_explicit_order_and_reject_a_retained_non_import_endpoint() {
    let rows = read_rows();
    let source = empty()
        .apply(&[seed(7, [9., 9., 9.], PointRole::SketchMarker)], || false)
        .unwrap();
    let [a, b, c] = rows.map(|row| row.id);
    let mut edges = [
        SketchLine {
            id: lid(8),
            endpoints: [c, b],
        },
        SketchLine {
            id: lid(5),
            endpoints: [a, b],
        },
    ];
    let accepted = source
        .import_normalized(
            plan(&rows, ImportCurves::DrawingSegments(&edges), 5),
            || false,
        )
        .unwrap();
    for edge in edges {
        assert_eq!(accepted.sketch.line(edge.id).unwrap().line, edge);
    }
    assert_eq!(
        accepted.curves,
        [CurveId::Line(lid(8)), CurveId::Line(lid(5))]
    );
    edges[1].endpoints[1] = pid(7);
    assert_eq!(
        source.import_normalized(
            plan(&rows, ImportCurves::DrawingSegments(&edges), 5),
            || false
        ),
        Err(ImportFailure::ForeignEndpoint {
            line: lid(5),
            point: pid(7)
        })
    );
    assert_eq!(
        source.owned_ids().collect::<Vec<_>>(),
        [EntityId::Point(pid(7))]
    );
}

#[test]
fn import_fits_the_four_ordered_rows_with_the_existing_exact_station_checks() {
    let t = [0., 0.25, 0.75, 1.];
    let p = |t: f64| [3. * t, 3. * t * t, t * t * t];
    let rows =
        std::array::from_fn::<_, 4, _>(|i| row([7, 3, 8, 4][i], p(t[i]), PointRole::Defining));
    let id = FitSplineId::new(NonZeroU64::new(12).unwrap());
    let curves = ImportCurves::FitCubic {
        id,
        parameters: FitSplineParameters::new(t).unwrap(),
        policy: FitSplinePolicy {
            max_distortion: 8.,
            max_inverse_gain: 32.,
            max_station_identity_error: 1e-12,
            max_station_miss: MinimumLength::new(1e-10).unwrap(),
            regularity: ControlSplineProofPolicy::new(1e-8, 128).unwrap(),
        },
    };
    let accepted = empty()
        .import_normalized(plan(&rows, curves, 5), || false)
        .unwrap();
    let fit = accepted.sketch.fit_spline(id).unwrap();
    assert_eq!(fit.recipe().stations, rows.map(|row| row.id));
    assert_eq!(fit.stations(), t.map(p).map(point));
    assert_eq!(accepted.curves, [CurveId::FitSpline(id)]);
    let expected = p(0.5);
    let midpoint = fit.value(0.5).unwrap();
    for (&actual, expected) in midpoint.coordinates().iter().zip(expected) {
        assert!((actual - expected).abs() <= 1e-12);
    }
    assert!(matches!(
        empty().import_normalized(plan(&rows[..3], curves, 5), || false),
        Err(ImportFailure::Counts {
            points: 3,
            curves: 1,
            ..
        })
    ));
}

#[test]
fn refusals_and_late_cancellation_do_not_publish_any_rows() {
    let source = empty();
    let mut rows = read_rows();
    let curve_ids = [lid(70), lid(71)];
    let polyline = ImportCurves::Polyline(&curve_ids);
    assert_eq!(
        source.import_normalized(plan(&rows, polyline, 4), || false),
        Err(ImportFailure::Limit {
            requested: 5,
            maximum: 4
        })
    );
    assert!(matches!(
        source.import_normalized(
            plan(&rows, ImportCurves::Polyline(&curve_ids[..1]), 5),
            || false
        ),
        Err(ImportFailure::Counts {
            points: 3,
            curves: 1,
            ..
        })
    ));
    assert_eq!(
        source.import_normalized(plan(&rows, polyline, 5), || true),
        Err(ImportFailure::Cancelled)
    );
    let mut polls = 0;
    let valid = plan(&rows, polyline, 5);
    source
        .import_normalized(valid, || {
            polls += 1;
            false
        })
        .unwrap();
    let mut remaining = polls;
    assert!(matches!(
        source.import_normalized(valid, || {
            remaining -= 1;
            remaining == 0
        }),
        Err(ImportFailure::Edit(EditFailure::Cancelled(_)))
    ));
    rows[2].position = point([0., f64::INFINITY, 0.]);
    assert!(matches!(
        source.import_normalized(plan(&rows, polyline, 5), || false),
        Err(ImportFailure::Edit(EditFailure::NonFinitePoint { point, axis: 1, .. }))
            if point == pid(23)
    ));
    rows = read_rows();
    rows[2].id = rows[0].id;
    assert!(matches!(
        source.import_normalized(plan(&rows, polyline, 5), || false),
        Err(ImportFailure::Edit(EditFailure::DuplicateIdentity { .. }))
    ));
    assert_eq!(source.owned_ids().count(), 0);
}

#[test]
fn one_frozen_input_can_be_imported_concurrently_with_equal_results() {
    fn assert_shareable<T: Send + Sync>() {}
    assert_shareable::<SketchImport<'static>>();
    assert_shareable::<ImportedSketch>();
    let source = empty();
    let rows = read_rows();
    let lines = [lid(76), lid(51)];
    let input = plan(&rows, ImportCurves::Polyline(&lines), 5);
    let expected = source.import_normalized(input, || false).unwrap();
    thread::scope(|scope| {
        let runs: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| source.import_normalized(input, || false).unwrap()))
            .collect();
        for run in runs {
            assert_eq!(run.join().unwrap(), expected);
        }
    });
}
