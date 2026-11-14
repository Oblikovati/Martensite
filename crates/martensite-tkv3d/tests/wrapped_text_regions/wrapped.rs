//! Fixed dyadic layout isolates geometry qualification from platform font metrics.
use super::*;
use martensite_tkbo::profile_extrusion::*;
use martensite_tksketch::surface_sketch::*;
use martensite_tkv3d::brep_font::{BRepFontLoopId, BRepFontPlanarRegion as Region};

fn source_id(instance: usize, index: u32) -> u64 {
    (instance as u64 + 1) * 1000 + u64::from(index) + 1
}
fn placed(instance: usize, p: Xy) -> [f64; 2] {
    let row = GLYPHS
        .lines()
        .filter(|r| !r.starts_with('#'))
        .find(|r| r.split('\t').next().unwrap().parse::<usize>().unwrap() == instance)
        .unwrap();
    let f: Vec<_> = row.split('\t').collect();
    let line: f64 = f[1].parse().unwrap();
    let column: f64 = f[2].parse().unwrap();
    [p.x() / 1024. + 2. * column, p.y() / 1024. - 3. * line]
}
fn loop_selection(
    instance: usize,
    region: &Region,
    id: BRepFontLoopId,
    outer: bool,
) -> ProfileLoop {
    let contour = &region.loops().as_slice()[id.value() as usize];
    let first = contour.first_use() as usize;
    let uses: Vec<LoopUse> = region.uses().as_slice()[first..first + contour.nb_uses() as usize]
        .iter()
        .map(|u| LoopUse {
            curve: ProfileCurveId(source_id(instance, u.curve_index().value())),
            reversed: u.is_reversed(),
        })
        .collect();
    oriented_profile_loop(uses, (contour.signed_area() > 0.) != outer)
}
fn oriented_profile_loop(mut uses: Vec<LoopUse>, reverse: bool) -> ProfileLoop {
    if reverse {
        uses.reverse();
        for curve_use in &mut uses {
            curve_use.reversed = !curve_use.reversed;
        }
    }
    ProfileLoop { uses }
}
fn append_curves(instance: usize, region: &Region, profile: &mut SketchProfile) {
    for (i, p) in region.vertices().iter().enumerate() {
        profile.vertices.push(ProfileVertex {
            id: ProfileVertexId(source_id(instance, i as u32)),
            point: placed(instance, *p),
        });
    }
    for u in region.uses().iter() {
        assert_eq!((u.first_parameter(), u.last_parameter()), (0., 1.));
        let c = &region.curves().as_slice()[u.curve_index().value() as usize];
        let id = ProfileCurveId(source_id(instance, u.curve_index().value()));
        let start = ProfileVertexId(source_id(instance, u.first_vertex().value()));
        let end = ProfileVertexId(source_id(instance, u.last_vertex().value()));
        profile
            .curves
            .push(profile_curve(instance, c, id, start, end));
    }
}
fn profile_curve(
    instance: usize,
    c: &martensite_tkv3d::brep_font::BRepFontPlanarRegionCurve,
    id: ProfileCurveId,
    start: ProfileVertexId,
    end: ProfileVertexId,
) -> ProfileCurve {
    use martensite_tkservice::font::FontGlyphOutlineSegmentKind as Kind;
    if c.kind() == Kind::Line {
        return ProfileCurve::Line { id, start, end };
    }
    clamped_profile_spline(id, start, end, source_poles(instance, c))
}
fn clamped_profile_spline(
    id: ProfileCurveId,
    start: ProfileVertexId,
    end: ProfileVertexId,
    poles: Vec<[f64; 2]>,
) -> ProfileCurve {
    let count = poles.len();
    ProfileCurve::Spline {
        id,
        start,
        end,
        degree: (count - 1) as u32,
        poles,
        weights: None,
        knots: std::iter::repeat_n(0., count)
            .chain(std::iter::repeat_n(1., count))
            .collect(),
    }
}
fn source_poles(
    instance: usize,
    c: &martensite_tkv3d::brep_font::BRepFontPlanarRegionCurve,
) -> Vec<[f64; 2]> {
    use martensite_tkservice::font::FontGlyphOutlineSegmentKind as Kind;
    let mut poles = vec![
        placed(instance, *c.start()),
        placed(instance, *c.control1()),
    ];
    if c.kind() == Kind::CubicBezier {
        poles.push(placed(instance, *c.control2()));
    }
    poles.push(placed(instance, *c.end()));
    poles
}
fn append_selections(
    instance: usize,
    region: &Region,
    selections: &mut Vec<WrappedRegionSelection>,
) {
    for material in region.regions().iter() {
        let first = material.first_hole() as usize;
        let holes = region.hole_loops().as_slice()[first..first + material.nb_holes() as usize]
            .iter()
            .map(|id| loop_selection(instance, region, *id, false))
            .collect();
        selections.push(WrappedRegionSelection {
            region: SurfaceRegionId(source_id(instance, material.outer_loop().value())),
            outer: loop_selection(instance, region, *material.outer_loop(), true),
            holes,
        });
    }
}
fn solved_text() -> (SketchProfile, Vec<WrappedRegionSelection>) {
    let mut profile = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vec![],
        curves: vec![],
    };
    let mut selections = vec![];
    for (instance, contours) in owned_instances() {
        let built = BRepFontRegularizer.build(&outline(contours));
        let region = built.planar_region().as_ref().unwrap();
        append_curves(instance, region, &mut profile);
        append_selections(instance, region, &mut selections);
    }
    (profile, selections)
}
fn text_request() -> WrappedNormalRequest {
    let (profile, selections) = solved_text();
    WrappedNormalRequest {
        profile,
        selections,
        support: text_support(),
        depths: [0., 0.25],
        options: text_options(),
    }
}
fn text_support() -> CylinderSupport {
    CylinderSupport {
        origin: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        radius: 10.,
        chart_origin: 0.,
        normal: NormalSense::Outward,
    }
}
fn text_options() -> WrappedNormalOptions {
    WrappedNormalOptions {
        spatial_error: 1e-5,
        maximum_fit_level: 8,
        volume: WrappedVolumePolicy::Report,
    }
}
#[test]
fn owned_multiline_text_wraps_all_selected_material() {
    let request = text_request();
    assert_eq!(request.profile.curves.len(), 239);
    assert_eq!(request.selections.len(), 18);
    let result = normal_extrude_profile(&request).unwrap();
    assert_eq!(result.reports().len(), 18);
    for report in result.reports().values() {
        assert!(report.validation.is_valid());
        assert!(report.spatial.cap_realization_error.upper <= request.options.spatial_error);
    }
    assert!(!result.encoded_brep().is_empty());
    check_authored_volume(&result);
}

fn check_authored_volume(result: &WrappedNormalResult) {
    // Independent exact Green integrals of captured controls, times the radial Jacobian.
    check_volume_oracle(result, 226802403. / 134217728.);
}
fn check_volume_oracle(result: &WrappedNormalResult, expected: f64) {
    let (mut lower, mut upper) = (0_f64, 0_f64);
    for report in result.reports().values() {
        lower = (lower + report.measures.authored_volume.lower).next_down();
        upper = (upper + report.measures.authored_volume.upper).next_up();
    }
    assert!(
        lower <= expected && expected <= upper,
        "authored volume range=[{lower},{upper}]; expected exact oracle={expected}"
    );
}

fn glyph_request(instance: u64) -> WrappedNormalRequest {
    let mut request = text_request();
    let prefix = instance + 1;
    request.profile.vertices.retain(|v| v.id.0 / 1000 == prefix);
    request.profile.curves.retain(|c| match c {
        ProfileCurve::Line { id, .. } | ProfileCurve::Spline { id, .. } => id.0 / 1000 == prefix,
        _ => false,
    });
    request.selections.retain(|s| s.region.0 / 1000 == prefix);
    request
}
#[test]
fn first_glyph_wraps_normalized_material_orientation() {
    let result = normal_extrude_profile(&glyph_request(0)).unwrap();
    assert_eq!(result.reports().len(), 1);
}
#[test]
fn counter_glyph_wraps_outer_and_hole_with_opposite_directions() {
    let request = glyph_request(8);
    assert_eq!(request.selections.len(), 1);
    assert_eq!(request.selections[0].holes.len(), 1);
    let result = normal_extrude_profile(&request).unwrap();
    let report = result.reports().values().next().unwrap();
    // Captured g: (3491255/3 - 863181/2) / 1024², times 81/320.
    let expected = 118610109. / 671088640.;
    assert!(report.measures.authored_volume.lower <= expected);
    assert!(expected <= report.measures.authored_volume.upper);
    assert!(report.validation.is_valid());
}

#[test]
fn inward_counter_glyph_preserves_hole_volume_and_input() {
    let mut request = glyph_request(8);
    request.support.normal = NormalSense::Inward;
    let original = request.clone();
    let result = normal_extrude_profile(&request).unwrap();
    let report = result.reports().values().next().unwrap();
    // The same captured g area times the inward radial integral 79/320.
    let expected = 347044393. / 2013265920.;
    assert!(report.measures.authored_volume.lower <= expected);
    assert!(expected <= report.measures.authored_volume.upper);
    assert!(report.validation.is_valid());
    assert!(report.spatial.cap_realization_error.upper <= request.options.spatial_error);
    assert_eq!(request, original);
    assert_eq!(result.reports().len(), 1);
}

#[test]
fn separated_outer_and_counter_glyphs_publish_as_one_family() {
    let mut request = text_request();
    let selected = |id: u64| [1, 9].contains(&(id / 1000));
    request.profile.vertices.retain(|v| selected(v.id.0));
    request.profile.curves.retain(|c| match c {
        ProfileCurve::Line { id, .. } | ProfileCurve::Spline { id, .. } => selected(id.0),
        _ => false,
    });
    request.selections.retain(|s| selected(s.region.0));
    let started = std::time::Instant::now();
    let result = normal_extrude_profile(&request).unwrap();
    eprintln!("two-glyph family elapsed={:?}", started.elapsed());
    assert_eq!(result.reports().len(), 2);
    check_volume_oracle(&result, 196901307. / 671088640.);
}

fn shift_developed_profile(request: &mut WrappedNormalRequest, shift: f64) {
    for vertex in &mut request.profile.vertices {
        vertex.point[0] += shift;
    }
    for curve in &mut request.profile.curves {
        if let ProfileCurve::Spline { poles, .. } = curve {
            for pole in poles {
                pole[0] += shift;
            }
        }
    }
}
#[test]
fn captured_glyph_crosses_cylinder_parameter_seam() {
    let mut request = glyph_request(0);
    shift_developed_profile(&mut request, 62.5);
    let seam = std::f64::consts::TAU * request.support.radius;
    assert!(request.profile.vertices.iter().any(|v| v.point[0] < seam));
    assert!(request.profile.vertices.iter().any(|v| v.point[0] > seam));
    let original = request.clone();
    let result = normal_extrude_profile(&request).unwrap();
    assert!(result.reports().values().all(|r| r.validation.is_valid()));
    check_volume_oracle(&result, 39145599. / 335544320.);
    assert_eq!(request, original);
}

fn resize_developed_profile(request: &mut WrappedNormalRequest, scale: f64) {
    for vertex in &mut request.profile.vertices {
        vertex.point.iter_mut().for_each(|x| *x *= scale);
    }
    for curve in &mut request.profile.curves {
        if let ProfileCurve::Spline { poles, .. } = curve {
            poles.iter_mut().flatten().for_each(|x| *x *= scale);
        }
    }
}
#[test]
fn resized_counter_glyph_keeps_the_requested_spatial_accuracy() {
    let mut request = glyph_request(8);
    resize_developed_profile(&mut request, 2.);
    let original = request.clone();
    let result = normal_extrude_profile(&request).unwrap();
    check_volume_oracle(&result, 118610109. / 167772160.);
    for report in result.reports().values() {
        assert!(report.validation.is_valid());
        assert!(report.spatial.cap_realization_error.upper <= 1e-5);
    }
    assert_eq!(request, original);
}

#[path = "body_controls.rs"]
mod body_controls;

#[test]
fn glyph_request_edits_preserve_prior_publication_and_replay() {
    let original = glyph_request(0);
    let publication = normal_extrude_profile(&original).unwrap();
    let bytes = publication.encoded_brep().to_vec();
    for (changed, expected) in edited_glyph_requests(&original) {
        let revised = normal_extrude_profile(&changed).unwrap();
        check_volume_oracle(&revised, expected);
        assert_ne!(revised.encoded_brep(), bytes);
        assert_eq!(publication.encoded_brep(), bytes);
        assert_eq!(revised.support(), changed.support);
        assert_eq!(revised.depths(), changed.depths);
    }
    let replay = normal_extrude_profile(&original).unwrap();
    assert_eq!(replay.encoded_brep(), bytes);
    assert_eq!(replay.history(), publication.history());
}
fn edited_glyph_requests(original: &WrappedNormalRequest) -> Vec<(WrappedNormalRequest, f64)> {
    let mut moved = original.clone();
    shift_developed_profile(&mut moved, 0.5);
    let mut support = original.clone();
    support.support.radius = 12.;
    let mut depth = original.clone();
    depth.depths[1] = 0.5;
    vec![
        (moved, 39145599. / 335544320.),
        (support, 15626021. / 134217728.),
        (depth, 19814439. / 83886080.),
        (glyph_request(4), 62194689. / 671088640.),
    ]
}
