//! UpdateDrawStats formulas of for every primitive type, text and residency state.
use super::*;
use crate::element_upload::{ElementUpload, PrimitiveUpload, TextUpload};

fn built(vertices: usize, indices: Option<usize>) -> ElementUpload {
    ElementUpload::Primitive(PrimitiveUpload {
        vertex_count: vertices,
        vertex_bytes: 12 * vertices,
        index_count: indices,
        index_bytes: 4 * indices.unwrap_or(0),
    })
}

fn tally(kind: Kind, bounds: Option<usize>, upload: ElementUpload) -> Vec<(Counter, usize)> {
    GroupElementStats::Primitive {
        kind,
        bounds,
        upload,
    }
    .draw_stats(true)
    .0
}

#[test]
fn strips_subtract_two_vertices_per_bound() {
    let counts = tally(Kind::Trianglestrips, Some(2), built(10, None));
    assert_eq!(
        counts,
        [
            (Counter::NbElemsNotCulled, 1),
            (Counter::NbElemsFillNotCulled, 1),
            (Counter::NbTrianglesNotCulled, 6),
        ]
    );
}

#[test]
fn indices_take_precedence_over_vertices() {
    let counts = tally(Kind::Segments, None, built(3, Some(8)));
    assert_eq!(counts[2], (Counter::NbLinesNotCulled, 4));
    assert_eq!(counts[1], (Counter::NbElemsLineNotCulled, 1));
}

#[test]
fn every_native_draw_mode_has_its_count() {
    let cases = [
        (Kind::Points, Counter::NbPointsNotCulled, 12),
        (Kind::Polylines, Counter::NbLinesNotCulled, 11),
        (Kind::LinesAdjacency, Counter::NbLinesNotCulled, 3),
        (Kind::LineStripAdjacency, Counter::NbLinesNotCulled, 8),
        (Kind::Triangles, Counter::NbTrianglesNotCulled, 4),
        (Kind::Trianglefans, Counter::NbTrianglesNotCulled, 10),
        (Kind::TrianglesAdjacency, Counter::NbTrianglesNotCulled, 2),
        (
            Kind::TriangleStripAdjacency,
            Counter::NbTrianglesNotCulled,
            8,
        ),
        (Kind::Quadrangles, Counter::NbTrianglesNotCulled, 6),
        (Kind::Quadranglestrips, Counter::NbTrianglesNotCulled, 10),
    ];
    for (kind, counter, expected) in cases {
        let counts = tally(kind, None, built(12, None));
        assert_eq!(counts[2], (counter, expected), "kind {kind:?}");
    }
}

#[test]
fn polygons_and_undefined_arrays_count_only_their_element() {
    assert_eq!(tally(Kind::Polygons, None, built(5, None)).len(), 2);
    let undefined = tally(Kind::Undefined, None, built(5, None));
    assert_eq!(undefined[1], (Counter::NbElemsLineNotCulled, 1));
    assert_eq!(undefined.len(), 2);
}

#[test]
fn unbuilt_elements_have_no_detailed_counts() {
    assert_eq!(
        tally(Kind::Triangles, None, ElementUpload::Pending).len(),
        2
    );
    assert_eq!(tally(Kind::Triangles, None, ElementUpload::Failed).len(), 2);
}

#[test]
fn negative_strip_counts_wrap_like_native_size_t() {
    let counts = tally(Kind::Trianglestrips, Some(3), built(4, None));
    assert_eq!(
        counts[2],
        (Counter::NbTrianglesNotCulled, (-2_i64) as usize)
    );
}

#[test]
fn text_counts_two_triangles_per_glyph_when_detailed() {
    let upload = ElementUpload::Text(TextUpload {
        glyph_vertices: 18,
        bounds: Default::default(),
        background_vertices: 0,
    });
    let detailed = GroupElementStats::Text(upload).draw_stats(true).0;
    assert_eq!(detailed[2], (Counter::NbTrianglesNotCulled, 6));
    assert_eq!(GroupElementStats::Text(upload).draw_stats(false).0.len(), 2);
}
