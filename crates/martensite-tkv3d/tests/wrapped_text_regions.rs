//! Owned source outlines exercise material selection without runtime font discovery.
use martensite_tkmath::gp::Xy;
use martensite_tkservice::font::{
    FontGlyphOutline, FontGlyphOutlineContour, FontGlyphOutlineFillRule,
    FontGlyphOutlineSegment as Segment,
};
use martensite_tkv3d::brep_font::{BRepFontRegularizer, BRepFontRegularizerStatus};
use std::collections::BTreeMap;

const GLYPHS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/features/surface-normal-sketches/references/p5-owned-glyphs.tsv"
));

type Contours = BTreeMap<usize, Vec<Segment>>;
fn owned_instances() -> BTreeMap<usize, Contours> {
    let mut instances = BTreeMap::<usize, Contours>::new();
    for row in GLYPHS.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 16);
        let instance = fields[0].parse().unwrap();
        let contour = fields[5].parse().unwrap();
        let curves = instances
            .entry(instance)
            .or_default()
            .entry(contour)
            .or_default();
        assert_eq!(fields[6].parse::<usize>().unwrap(), curves.len());
        curves.push(segment(&fields));
    }
    instances
}
fn point(fields: &[&str], first: usize) -> Xy {
    let bits = |index| u64::from_str_radix(fields[index], 16).unwrap();
    Xy::new(f64::from_bits(bits(first)), f64::from_bits(bits(first + 1)))
}
fn segment(fields: &[&str]) -> Segment {
    let (a, c, d, b) = (
        point(fields, 8),
        point(fields, 10),
        point(fields, 12),
        point(fields, 14),
    );
    match fields[7] {
        "Line" => Segment::line(a, b),
        "QuadraticBezier" => Segment::quadratic(a, c, b),
        "CubicBezier" => Segment::cubic(a, c, d, b),
        other => panic!("curve kind={other}; expected a polynomial segment"),
    }
}
fn outline(contours: Contours) -> FontGlyphOutline {
    let (mut segments, mut ranges) = (Vec::new(), Vec::new());
    for (index, curves) in contours {
        assert_eq!(index, ranges.len());
        check_closed_contour(&curves);
        ranges.push(FontGlyphOutlineContour::new(
            segments.len() as u32,
            curves.len() as u32,
        ));
        segments.extend(curves);
    }
    FontGlyphOutline::create(
        2048.,
        FontGlyphOutlineFillRule::NonZero,
        false,
        false,
        segments,
        ranges,
    )
}
fn check_closed_contour(curves: &[Segment]) {
    assert!(
        curves
            .iter()
            .zip(curves.iter().cycle().skip(1))
            .all(|(a, b)| a.end() == b.start())
    );
}

#[test]
fn pinned_text_preserves_counters_and_disconnected_material() {
    let instances = owned_instances();
    assert_eq!(instances.len(), 15);
    let mut counts = [0; 4];
    for (instance, contours) in instances {
        accumulate_regions(instance, contours, &mut counts);
    }
    assert_eq!(counts, [239, 20, 18, 2]);
    assert!(
        counts[2] > 15,
        "dots/punctuation must remain disconnected material"
    );
    assert!(counts[3] > 0, "counters must remain holes");
    eprintln!("owned glyph counts curves/loops/material/holes={counts:?}");
}

fn accumulate_regions(instance: usize, contours: Contours, counts: &mut [usize; 4]) {
    let source = outline(contours);
    let result = BRepFontRegularizer.build(&source);
    assert_eq!(
        result.status_value(),
        BRepFontRegularizerStatus::Success,
        "instance={instance}"
    );
    let region = result.planar_region().as_ref().unwrap();
    check_source_curves(&source, region);
    counts[0] += region.curves().len();
    counts[1] += region.loops().len();
    counts[2] += region.regions().len();
    counts[3] += region.hole_loops().len();
    check_whole_ranges(region);
}
fn check_whole_ranges(region: &martensite_tkv3d::brep_font::BRepFontPlanarRegion) {
    for curve_use in region.uses().iter() {
        assert_eq!(
            (curve_use.first_parameter(), curve_use.last_parameter()),
            (0., 1.)
        );
    }
}

#[path = "wrapped_text_regions/wrapped.rs"]
mod wrapped;

fn check_source_curves(
    source: &FontGlyphOutline,
    region: &martensite_tkv3d::brep_font::BRepFontPlanarRegion,
) {
    use martensite_tkv3d::brep_font::BRepFontPlanarRegionCurve;
    assert_eq!(source.segments().len(), region.curves().len());
    for (authored, retained) in source.segments().iter().zip(region.curves().iter()) {
        assert_eq!(BRepFontPlanarRegionCurve::new(authored), *retained);
    }
}
