use super::*;
use crate::brep_font::BRepFontPlanarRegion;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Xy;

pub(super) fn snapshot(capture: &OwnedFontTextCapture) -> Vec<u64> {
    let mut bits = vec![capture.recipe_version().into(), capture.asset().id().0];
    option_bits(&mut bits, capture.request().options);
    append_point(&mut bits, capture.plan().lower_left());
    append_point(&mut bits, capture.plan().upper_right());
    for occurrence in capture.occurrences() {
        occurrence_bits(&mut bits, occurrence);
    }
    work_bits(&mut bits, capture.work());
    bits
}
fn option_bits(bits: &mut Vec<u64>, options: FontTextCaptureOptions) {
    bits.extend([
        options.size.to_bits(),
        options.tolerance.to_bits(),
        options.outline_tolerance.to_bits(),
        u64::from(options.width_scaling.to_bits()),
        options.horizontal as u64,
        options.vertical as u64,
    ]);
}
fn work_bits(bits: &mut Vec<u64>, work: FontTextCaptureWork) {
    bits.extend(
        [
            work.checkpoints,
            work.glyphs,
            work.segments,
            work.contours,
            work.points,
            work.processed_entities,
        ]
        .map(|count| count as u64),
    );
}

fn occurrence_bits(bits: &mut Vec<u64>, occurrence: &OwnedFontTextOccurrence) {
    let source = occurrence.source();
    bits.extend([
        source.id.0,
        source.utf8_span.start as u64,
        source.utf8_span.end as u64,
        u64::from(u32::from(occurrence.scalar())),
        occurrence.kind() as u64,
    ]);
    append_point(bits, occurrence.position());
    if let Some(glyph) = occurrence.glyph() {
        bits.push(u64::from(glyph.index()));
        outline_bits(bits, glyph.outline());
        region_bits(bits, glyph.region());
    }
}
fn outline_bits(bits: &mut Vec<u64>, outline: &martensite_tkservice::font::FontGlyphOutline) {
    bits.extend([
        outline.units_per_em().to_bits(),
        outline.rule() as u64,
        outline.has_possible_overlaps().into(),
        outline.is_single_stroke().into(),
        outline.contours().len() as u64,
        outline.segments().len() as u64,
    ]);
    for contour in outline.contours() {
        bits.extend([
            u64::from(contour.first_segment()),
            u64::from(contour.nb_segments()),
        ]);
    }
    for segment in outline.segments() {
        segment_bits(bits, segment);
    }
}
fn segment_bits(
    bits: &mut Vec<u64>,
    segment: &martensite_tkservice::font::FontGlyphOutlineSegment,
) {
    bits.push(segment.kind() as u64);
    for point in [
        segment.start(),
        segment.control1(),
        segment.control2(),
        segment.end(),
    ] {
        append_point(bits, &point);
    }
}

fn region_bits(bits: &mut Vec<u64>, region: &BRepFontPlanarRegion) {
    bits.extend([
        region.units_per_em().to_bits(),
        region.tolerance().to_bits(),
        region.is_single_stroke().into(),
    ]);
    region_cardinalities(bits, region);
    for point in region.vertices().iter() {
        append_point(bits, point);
    }
    for curve in region.curves().iter() {
        curve_bits(bits, curve);
    }
    region_topology_bits(bits, region);
}
fn region_cardinalities(bits: &mut Vec<u64>, region: &BRepFontPlanarRegion) {
    bits.extend(
        [
            region.vertices().len(),
            region.curves().len(),
            region.uses().len(),
            region.loops().len(),
            region.hole_loops().len(),
            region.regions().len(),
        ]
        .map(|count| count as u64),
    );
}

fn curve_bits(bits: &mut Vec<u64>, curve: &crate::brep_font::BRepFontPlanarRegionCurve) {
    bits.push(curve.kind() as u64);
    for point in [
        curve.start(),
        curve.control1(),
        curve.control2(),
        curve.end(),
    ] {
        append_point(bits, point);
    }
}

fn region_topology_bits(bits: &mut Vec<u64>, region: &BRepFontPlanarRegion) {
    for incidence in region.uses().iter() {
        incidence_bits(bits, incidence);
    }
    for boundary in region.loops().iter() {
        bits.extend([
            u64::from(boundary.first_use()),
            u64::from(boundary.nb_uses()),
            boundary.signed_area().to_bits(),
        ]);
    }
    for hole in region.hole_loops().iter() {
        bits.push(u64::from(hole.value()));
    }
    material_bits(bits, region);
}
fn material_bits(bits: &mut Vec<u64>, region: &BRepFontPlanarRegion) {
    for material in region.regions().iter() {
        bits.extend([
            u64::from(material.outer_loop().value()),
            u64::from(material.first_hole()),
            u64::from(material.nb_holes()),
        ]);
    }
}

fn incidence_bits(bits: &mut Vec<u64>, incidence: &crate::brep_font::BRepFontPlanarRegionUse) {
    bits.extend([
        u64::from(incidence.curve_index().value()),
        u64::from(incidence.first_vertex().value()),
        u64::from(incidence.last_vertex().value()),
        incidence.first_parameter().to_bits(),
        incidence.last_parameter().to_bits(),
        incidence.is_reversed().into(),
    ]);
}

fn append_point(bits: &mut Vec<u64>, point: &Xy) {
    bits.extend([point.x().to_bits(), point.y().to_bits()]);
}
pub(super) fn built_geometry(capture: &OwnedFontTextCapture) -> Vec<Vec<u64>> {
    let shape = crate::brep_font::BRepFontBuilder::build_text(capture.plan());
    assert!(!shape.is_null());
    let mut values = Vec::new();
    for kind in [
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        ShapeEnum::Wire,
        ShapeEnum::Face,
    ] {
        shape_kind_values(&mut values, &shape, kind);
    }
    // Canonicalize a comparison multiset only; these coordinates never become entity IDs.
    values.sort();
    values
}
fn shape_kind_values(values: &mut Vec<Vec<u64>>, shape: &topods::Shape, kind: ShapeEnum) {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    let mut count = 0;
    while explorer.more() {
        append_shape_value(values, explorer.current(), kind);
        count += 1;
        explorer.next().unwrap();
    }
    values.push(vec![kind as u64, count]);
}

fn append_shape_value(values: &mut Vec<Vec<u64>>, shape: &topods::Shape, kind: ShapeEnum) {
    if kind == ShapeEnum::Vertex {
        let point = BRepTool::pnt(&topods::vertex(shape).unwrap()).unwrap();
        values.push(vec![
            kind as u64,
            point.x().to_bits(),
            point.y().to_bits(),
            point.z().to_bits(),
        ]);
    }
    if kind == ShapeEnum::Edge {
        values.push(edge_values(shape));
    }
}
fn edge_values(shape: &topods::Shape) -> Vec<u64> {
    let (curve, first, last) = BRepTool::curve(&topods::edge(shape).unwrap()).unwrap();
    let curve = curve.get().expect("built analytic edge curve");
    let mut bits = vec![ShapeEnum::Edge as u64, first.to_bits(), last.to_bits()];
    for parameter in [first, first + (last - first) * 0.5, last] {
        let point = curve.value(parameter).unwrap();
        bits.extend([
            point.x().to_bits(),
            point.y().to_bits(),
            point.z().to_bits(),
        ]);
    }
    bits
}
