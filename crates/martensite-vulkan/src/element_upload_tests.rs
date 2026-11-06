//! Residency follows native buildVBO counts: attribute stride bytes, 16/32-bit indices,
//! permanent failure for empty attributes and the mutable-buffer update rule.
use super::*;
use martensite_tkservice::graphic3d::{Graphic3dArrayOfPrimitives, Graphic3dTypeOfPrimitiveArray};

fn submission(array: &Graphic3dArrayOfPrimitives) -> PrimitiveSubmission {
    PrimitiveSubmission {
        kind: array.primitive_type(),
        indices: array.indices(),
        attributes: array.attributes(),
        bounds: array.bounds(),
        upload: ElementUploadSlot::default(),
        uid: 0,
    }
}

fn indexed_quad() -> Graphic3dArrayOfPrimitives {
    let array =
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 4, 0, 6, 0)
            .unwrap();
    for [x, y] in [[0., 0.], [1., 0.], [1., 1.], [0., 1.]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    array.add_edges3(1, 2, 3).unwrap();
    array.add_edges3(1, 3, 4).unwrap();
    array
}

#[test]
fn indexed_triangles_record_vertex_and_index_bytes() {
    let primitive = submission(&indexed_quad());
    primitive.upload.record_primitive(&primitive);
    let expected = PrimitiveUpload {
        vertex_count: 4,
        vertex_bytes: 4 * 12,
        index_count: Some(6),
        index_bytes: 6 * 2,
    };
    assert_eq!(
        primitive.upload.current(),
        ElementUpload::Primitive(expected)
    );
    assert_eq!(primitive.upload.current().estimated_bytes(), 60);
}

#[test]
fn empty_attributes_fail_permanently() {
    let array =
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Points, 2, 0, 0, 0).unwrap();
    let primitive = submission(&array);
    primitive.upload.record_primitive(&primitive);
    array.add_vertex(0., 0., 0.).unwrap();
    primitive.upload.record_primitive(&primitive);
    assert_eq!(primitive.upload.current(), ElementUpload::Failed);
    assert_eq!(primitive.upload.current().estimated_bytes(), 0);
}

#[test]
fn unsubmitted_elements_stay_pending() {
    let primitive = submission(&indexed_quad());
    assert_eq!(primitive.upload.current(), ElementUpload::Pending);
}

#[test]
fn text_keeps_its_subtitle_rectangle_after_style_changes() {
    let slot = ElementUploadSlot::default();
    slot.record_text(12, true, Default::default());
    slot.record_text(18, false, Default::default());
    let expected = TextUpload {
        glyph_vertices: 18,
        bounds: Default::default(),
        background_vertices: 4,
    };
    assert_eq!(slot.current(), ElementUpload::Text(expected));
    assert_eq!(slot.current().estimated_bytes(), 8 * (2 * 18 + 4));
}
