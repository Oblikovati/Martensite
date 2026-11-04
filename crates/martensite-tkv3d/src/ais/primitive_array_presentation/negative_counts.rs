//! Public patch refusal must preserve retained storage for every negative count position.
use super::*;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE, Graphic3dArrayOfTriangles,
};

#[test]
fn negative_patch_counts_fail_before_storage_mutation() {
    for counts in [[-1, 0, 0], [0, 0, 0], [3, -1, 0], [3, 0, -1]] {
        let array =
            Graphic3dArrayOfTriangles::new(3, 0, GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE).unwrap();
        array.attributes().validate();
        let before = array.vertex_number();
        let error = rejected_patch(&array, counts);
        assert_eq!(
            array.vertex_number(),
            before,
            "counts {counts:?} mutated storage"
        );
        assert!(array.attributes().invalidated_range().is_empty());
        assert!(error.message().contains(&format!("{counts:?}")));
        assert!(error.message().contains("expected"));
    }
}

#[test]
fn legacy_raw_adapter_preserves_zero_and_negative_vertex_partial_failure() {
    for vertices in [-1, 0] {
        let array =
            Graphic3dArrayOfTriangles::new(3, 0, GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE).unwrap();
        assert!(
            PrimitiveArrayPresentation::patch_counts_legacy_raw(
                &array,
                Graphic3dTypeOfPrimitiveArray::Triangles,
                [vertices, 0, 0]
            )
            .is_err()
        );
        assert_eq!(array.vertex_number(), vertices);
    }
}

#[test]
fn legacy_raw_adapter_preserves_negative_unallocated_bound_and_edge_counts() {
    for counts in [[3, -1, 0], [3, 0, -1]] {
        let array =
            Graphic3dArrayOfTriangles::new(3, 0, GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE).unwrap();
        assert!(
            PrimitiveArrayPresentation::patch_counts_legacy_raw(
                &array,
                Graphic3dTypeOfPrimitiveArray::Triangles,
                counts
            )
            .unwrap()
        );
        assert_eq!(array.vertex_number(), 3);
    }
}

fn rejected_patch(array: &Graphic3dArrayOfPrimitives, counts: [i32; 3]) -> Failure {
    PrimitiveArrayPresentation::patch_counts(
        array,
        Graphic3dTypeOfPrimitiveArray::Triangles,
        counts,
    )
    .unwrap_err()
}
