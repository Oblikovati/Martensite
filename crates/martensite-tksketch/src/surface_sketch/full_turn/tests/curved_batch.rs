// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileCurveId, ProfileLoop, ProfileVertex, ProfileVertexId,
    SketchProfile,
};

#[test]
fn mixed_cylinder_fitted_apertures_keep_one_clear_wire_and_two_crossings() {
    let (band, mut request) = curved_aperture::fixture();
    let plain = add_rectangle(
        &mut request.profile,
        [[2., 4.], [3., 4.], [3., 5.], [2., 5.]],
    );
    request.selections.push(plain.clone());
    let result =
        normal_extrude_cylinder_band_with_curved_holes(&band, &request, options()).unwrap();
    check_body(
        result.publication(),
        198. * PI - 10.0375,
        cylinder_discrepancy(&result),
        3,
    );
    assert_eq!(result.tool().len(), 2);
    assert_eq!(result.publication().crossings().len(), 2);
    assert_clear(result.publication(), &plain);
    count_cap_wires(result.publication(), band.region, 2);
    request.selections.reverse();
    let repeat =
        normal_extrude_cylinder_band_with_curved_holes(&band, &request, options()).unwrap();
    assert_same(result.publication(), repeat.publication());
}

#[test]
fn two_crossing_cylinder_quadratics_bind_the_middle_stock_seam() {
    let (band, mut request) = curved_aperture::fixture();
    request
        .selections
        .push(duplicate_curved_contour(&mut request.profile, [0., 2.5]));
    let result =
        normal_extrude_cylinder_band_with_curved_holes(&band, &request, options()).unwrap();
    let publication = result.publication();
    check_body(
        publication,
        198. * PI - 13.475,
        cylinder_discrepancy(&result),
        3,
    );
    assert_eq!(publication.crossings().len(), 4);
    assert_middle(
        publication,
        band.region,
        [
            (3, SheetTransition::Decreasing),
            (11, SheetTransition::Increasing),
        ],
    );
}

#[test]
fn mixed_cone_fitted_apertures_preserve_the_clear_rectangle() {
    let (band, mut request) = narrow_cone();
    let plain = add_rectangle(
        &mut request.profile,
        [
            [6.75, 0.015625],
            [7.25, 0.015625],
            [7.25, 0.03125],
            [6.75, 0.03125],
        ],
    );
    request.selections.push(plain.clone());
    let result = normal_extrude_cone_band_with_curved_holes(&band, &request, options()).unwrap();
    let fit = result
        .tool()
        .values()
        .map(|report| report.volume.discrepancy.upper)
        .sum();
    check_body(
        result.publication(),
        55. * PI / 4. - 175. / 12_288. - 1. / 256.,
        fit,
        3,
    );
    assert_eq!(result.publication().crossings().len(), 2);
    assert_clear(result.publication(), &plain);
}

#[test]
fn two_crossed_cone_quadratics_publish_a_named_middle_seam() {
    let (band, mut request) = narrow_cone();
    request
        .selections
        .push(duplicate_curved_contour(&mut request.profile, [2.75, 0.]));
    let result = normal_extrude_cone_band_with_curved_holes(&band, &request, options()).unwrap();
    let fit = result
        .tool()
        .values()
        .map(|report| report.volume.discrepancy.upper)
        .sum();
    let publication = result.publication();
    check_body(publication, 55. * PI / 4. - 175. / 6_144., fit, 3);
    assert_eq!(publication.crossings().len(), 4);
    assert_middle(
        publication,
        band.region,
        [
            (3, SheetTransition::Increasing),
            (11, SheetTransition::Decreasing),
        ],
    );
}

fn narrow_cone() -> (FullTurnBand, WrappedConeNormalRequest) {
    let (band, mut request) = curved_cone_aperture::fixture();
    for vertex in &mut request.profile.vertices {
        vertex.point[0] = 4. + (vertex.point[0] - 4.) / 4.;
        vertex.point[1] /= 8.;
    }
    if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
        for pole in poles {
            pole[0] = 4. + (pole[0] - 4.) / 4.;
            pole[1] /= 8.;
        }
    }
    (band, request)
}

#[test]
fn all_clear_fitted_apertures_keep_the_unsplit_stock_cut() {
    let (band, mut request) = curved_aperture::fixture();
    for vertex in &mut request.profile.vertices {
        vertex.point[0] += 2.;
    }
    if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
        for pole in poles {
            pole[0] += 2.;
        }
    }
    let plain = add_rectangle(
        &mut request.profile,
        [[2., 4.], [3., 4.], [3., 5.], [2., 5.]],
    );
    request.selections.push(plain);
    let result =
        normal_extrude_cylinder_band_with_curved_holes(&band, &request, options()).unwrap();
    let publication = result.publication();
    check_body(
        publication,
        198. * PI - 10.0375,
        cylinder_discrepancy(&result),
        3,
    );
    assert!(publication.crossings().is_empty());
    count_cap_wires(publication, band.region, 3);
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        let rail = PeriodicImageKey::SeamSegment {
            region: band.region,
            depth,
            ends: band.rims.map(PeriodicSeamEnd::Rim),
        };
        assert!(
            publication
                .geometry()
                .history()
                .binary_search(&rail)
                .is_ok()
        );
    }
}

#[test]
fn fitted_apertures_without_meridian_separation_refuse_before_boolean() {
    let (band, mut request) = curved_aperture::fixture();
    let plain = add_rectangle(
        &mut request.profile,
        [[0., 2.5], [1., 2.5], [1., 3.5], [0., 3.5]],
    );
    // Give the second contour angular clearance from the cut while its physical
    // region still overlaps the positive side of the first contour.
    for vertex in &mut request.profile.vertices[4..] {
        vertex.point[0] += 0.25;
    }
    request.selections.push(plain);
    let failure =
        normal_extrude_cylinder_band_with_curved_holes(&band, &request, options()).unwrap_err();
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::InvalidInput {
            field: "curved aperture meridian separation",
            ..
        }
    ));
}

fn add_rectangle(profile: &mut SketchProfile, points: [[f64; 2]; 4]) -> WrappedRegionSelection {
    profile.vertices.extend(
        points
            .into_iter()
            .enumerate()
            .map(|(index, point)| ProfileVertex {
                id: ProfileVertexId(index as u64 + 11),
                point,
            }),
    );
    profile
        .curves
        .extend((0..4).map(|index| ProfileCurve::Line {
            id: ProfileCurveId(index + 11),
            start: ProfileVertexId(index + 11),
            end: ProfileVertexId((index + 1) % 4 + 11),
        }));
    selection()
}

fn selection() -> WrappedRegionSelection {
    WrappedRegionSelection {
        region: SurfaceRegionId(9),
        outer: ProfileLoop {
            uses: (11..=14)
                .map(|id| LoopUse {
                    curve: ProfileCurveId(id),
                    reversed: false,
                })
                .collect(),
        },
        holes: vec![],
    }
}

fn duplicate_curved_contour(
    profile: &mut SketchProfile,
    shift: [f64; 2],
) -> WrappedRegionSelection {
    let vertices = profile.vertices.clone();
    let curves = profile.curves.clone();
    profile
        .vertices
        .extend(vertices.into_iter().map(|mut corner| {
            corner.id.0 += 10;
            translate(&mut corner.point, shift);
            corner
        }));
    profile.curves.extend(curves.into_iter().map(|mut curve| {
        match &mut curve {
            ProfileCurve::Spline {
                id,
                start,
                end,
                poles,
                ..
            } => {
                id.0 += 10;
                start.0 += 10;
                end.0 += 10;
                for pole in poles {
                    translate(pole, shift);
                }
            }
            ProfileCurve::Line { id, start, end } => {
                id.0 += 10;
                start.0 += 10;
                end.0 += 10;
            }
            _ => panic!("known reference fixture"),
        }
        curve
    }));
    selection()
}

fn translate(point: &mut [f64; 2], shift: [f64; 2]) {
    for (coordinate, delta) in point.iter_mut().zip(shift) {
        *coordinate += delta;
    }
}

fn assert_middle(
    publication: &CrossedBandResult,
    region: SurfaceRegionId,
    crossings: [(u64, SheetTransition); 2],
) {
    let ends = crossings.map(|(curve, transition)| {
        PeriodicSeamEnd::Crossing(PeriodicCrossingId {
            curve: SurfaceCurveId(curve),
            turn: 0,
            ordinal: 0,
            transition,
        })
    });
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        let key = PeriodicImageKey::SeamSegment {
            region,
            depth,
            ends,
        };
        assert!(
            publication.geometry().history().binary_search(&key).is_ok(),
            "{key:?}"
        );
    }
}

fn cylinder_discrepancy(
    result: &CurvedBandResult<std::collections::BTreeMap<SurfaceRegionId, WrappedMemberReport>>,
) -> f64 {
    result
        .tool()
        .values()
        .map(|report| report.measures.retained_ideal_discrepancy.upper)
        .sum()
}

fn check_body(result: &CrossedBandResult, expected: f64, fit: f64, genus: i64) {
    let volume = result.geometry().volume();
    assert!(
        (volume.value - expected).abs() <= volume.error_bound + fit,
        "{volume:?}"
    );
    let [census] = result.geometry().validation().census.as_slice() else {
        panic!("one complete stock shell")
    };
    assert_eq!(census.genus, Some(genus));
    assert_eq!(census.shells, 1);
}

fn assert_same(a: &CrossedBandResult, b: &CrossedBandResult) {
    assert_eq!(a.geometry().bytes, b.geometry().bytes);
    assert_eq!(a.geometry().history(), b.geometry().history());
    assert_eq!(a.crossings(), b.crossings());
}

fn assert_clear(result: &CrossedBandResult, contour: &WrappedRegionSelection) {
    for source in &contour.outer.uses {
        let curve = SurfaceCurveId(source.curve.0);
        let keys = result.geometry().history();
        assert!(keys.iter().any(|key| matches!(key, PeriodicImageKey::WrappedRetained(
            WrappedHistoryKey { source: WrappedEntitySource::Curve { curve: id, .. }, role: HistoryRole::FirstDepthEdge }
        ) if *id == curve)));
        assert!(!keys.iter().any(
            |key| matches!(key, PeriodicImageKey::AperturePiece { curve: id, .. } if *id == curve)
        ));
    }
}

fn count_cap_wires(result: &CrossedBandResult, owner: SurfaceRegionId, expected: usize) {
    use martensite_tkg3d::top_abs::ShapeEnum;
    let detached = result.geometry().export_brep().unwrap();
    for role in [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace] {
        let name = PeriodicImageKey::Retained(HistoryKey {
            source: HistorySource::Region(owner),
            role,
        });
        let (_, face) = detached
            .entities
            .iter()
            .find(|(key, _)| *key == name)
            .unwrap();
        let mut members = martensite_tkbrep::toptools::IndexedMapOfShape::default();
        martensite_tkbrep::topexp::map_shapes_of_type(face, ShapeEnum::Wire, &mut members).unwrap();
        assert_eq!(members.extent(), expected);
    }
}
