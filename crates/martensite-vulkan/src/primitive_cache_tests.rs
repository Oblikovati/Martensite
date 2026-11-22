use super::{RetainedPrimitives, SourceAddress};
use crate::primitive::PrimitiveSubmission;
use martensite_tkernel::{ncollection::Vec3, standard::Handle};
use martensite_tkservice::graphic3d::{
    Graphic3dAttribBuffer, Graphic3dAttribute, Graphic3dBoundBuffer, Graphic3dBuffer,
    Graphic3dBufferBase, Graphic3dBufferMembers, Graphic3dBufferSource, Graphic3dIndexBuffer,
    Graphic3dIndexBufferMembers, Graphic3dMutableIndexBuffer, Graphic3dTypeOfAttribute,
    Graphic3dTypeOfData, Graphic3dTypeOfPrimitiveArray,
};
use std::sync::Arc;

fn source(store: &RetainedPrimitives) -> (PrimitiveSubmission, Handle<Graphic3dBuffer>) {
    let allocator = Graphic3dBufferBase::default_allocator();
    let attributes = Handle::new(Graphic3dBuffer::new(allocator).unwrap());
    let layout = [
        Graphic3dAttribute {
            id: Graphic3dTypeOfAttribute::Pos,
            data_type: Graphic3dTypeOfData::Vec3,
        },
        Graphic3dAttribute {
            id: Graphic3dTypeOfAttribute::Norm,
            data_type: Graphic3dTypeOfData::Vec3,
        },
    ];
    assert!(attributes.init(3, &layout).unwrap());
    assert!(attributes.write_at(24, Vec3::new(1.0_f32, 0.0, 0.0)));
    assert!(attributes.write_at(48, Vec3::new(0.0_f32, 1.0, 0.0)));
    let mut primitive = PrimitiveSubmission {
        kind: Graphic3dTypeOfPrimitiveArray::Triangles,
        attributes: attributes.cast(),
        indices: Handle::default(),
        bounds: Handle::default(),
        upload: Default::default(),
        uid: 1,
        retained: None,
    };
    primitive.retained = store.entry(&primitive);
    (primitive, attributes)
}

fn draw(primitive: &PrimitiveSubmission, color: [f32; 4]) -> crate::PrimitiveDraw {
    primitive
        .draws(color, crate::identity_clip(), None, None)
        .unwrap()
        .remove(0)
}

#[test]
fn new_presentations_share_geometry_and_keep_live_transform_and_color() {
    let store = RetainedPrimitives::default();
    let (original, _) = source(&store);
    let first = draw(&original, [1.; 4]);
    let mut second = original.clone();
    second.uid = 2;
    let slot = Arc::downgrade(original.retained.as_ref().unwrap());
    second.retained = None;
    drop(original);
    second.retained = store.entry(&second);
    assert!(Arc::ptr_eq(
        &slot.upgrade().unwrap(),
        second.retained.as_ref().unwrap()
    ));
    let mut matrix = crate::identity_clip();
    matrix[12] = 0.25;
    let moved = second.draws([1.; 4], matrix, None, None).unwrap().remove(0);
    assert_eq!(moved.clip_transform, matrix);
    assert_eq!(first.vertices.identity(), moved.vertices.identity());
    assert_eq!(first.indices.identity(), moved.indices.identity());
    let recolored = draw(&second, [0.5, 0.0, 0.0, 1.0]);
    assert_eq!(recolored.vertices[0].color, [0.5, 0.0, 0.0, 1.0]);
    assert_ne!(first.vertices.identity(), recolored.vertices.identity());
    matrix[0] = f32::NAN;
    assert!(second.draws([1.; 4], matrix, None, None).is_err());
}

#[test]
fn direct_byte_and_layout_writes_are_visible_without_explicit_invalidation() {
    let (array, attributes) = source(&RetainedPrimitives::default());
    let before = draw(&array, [1.; 4]);
    let original_normals = array.shared_normals().unwrap().unwrap();
    assert!(attributes.write_at(12, Vec3::new(0.0_f32, 0.0, 1.0)));
    let mut bytes = attributes.buffer_base().state.write();
    bytes.data.as_mut_slice()[..4].copy_from_slice(&1.25_f32.to_le_bytes());
    drop(bytes);
    let edited = draw(&array, [1.; 4]);
    assert_eq!(edited.vertices[0].position[0], 1.25);
    assert_eq!(before.vertices[0].position[0], 0.);
    let changed_normals = array.shared_normals().unwrap().unwrap();
    assert_eq!(changed_normals[0], [0.0, 0.0, 1.0]);
    assert_ne!(changed_normals.identity(), original_normals.identity());
    let mut layout = attributes.buffer_base().state.write();
    layout.stride = 12;
    drop(layout);
    let relocated = draw(&array, [1.; 4]);
    assert_eq!(relocated.vertices[1].position, [0., 0., 1.]);
    assert_ne!(edited.vertices.identity(), relocated.vertices.identity());
}

#[test]
fn index_and_bound_changes_select_new_streams() {
    let store = RetainedPrimitives::default();
    let (mut array, _) = source(&store);
    let allocator = Graphic3dBufferBase::default_allocator();
    let indices = Handle::new(Graphic3dIndexBuffer::new(allocator.clone()).unwrap());
    assert!(indices.init_indices::<u16>(3).unwrap());
    for i in 0..3 {
        assert!(indices.set_index(i, i));
    }
    let bounds = Handle::new(Graphic3dBoundBuffer::new(allocator).unwrap());
    assert!(bounds.init(1, false).unwrap());
    assert!(bounds.set_bound(0, 3));
    array.indices = indices.cast();
    array.bounds = bounds.clone();
    array.retained = store.entry(&array);
    let first = draw(&array, [1.; 4]);
    assert!(indices.set_index(0, 2));
    let indexed = draw(&array, [1.; 4]);
    assert_eq!(indexed.indices(), [2, 1, 2]);
    assert_eq!(first.indices(), [0, 1, 2]);
    bounds.set_nb_bounds(0);
    assert!(
        array
            .draws([1.; 4], crate::identity_clip(), None, None)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn raw_address_exposure_uses_live_reads_and_dead_sources_leave_the_store() {
    let store = RetainedPrimitives::default();
    let (array, attributes) = source(&store);
    let first = draw(&array, [1.; 4]);
    let key = SourceAddress::of(&array);
    assert!(
        !attributes
            .buffer_base()
            .state
            .read()
            .data
            .data_ptr()
            .is_null()
    );
    let fallback = draw(&array, [1.; 4]);
    assert_ne!(first.vertices.identity(), fallback.vertices.identity());
    assert_eq!(first, fallback);
    drop(attributes);
    drop(array);
    let _next = source(&store);
    assert!(!store.0.read().entries.contains_key(&key));
}

fn mutable_source(
    store: &RetainedPrimitives,
) -> (PrimitiveSubmission, Handle<Graphic3dAttribBuffer>) {
    let allocator = Graphic3dBufferBase::default_allocator();
    let attributes = Handle::new(Graphic3dAttribBuffer::new(allocator).unwrap());
    attributes.set_mutable(true).unwrap();
    let layout = [Graphic3dAttribute {
        id: Graphic3dTypeOfAttribute::Pos,
        data_type: Graphic3dTypeOfData::Vec3,
    }];
    assert!(attributes.init(3, &layout).unwrap());
    assert!(attributes.write_at(12, Vec3::new(1.0_f32, 0.0, 0.0)));
    assert!(attributes.write_at(24, Vec3::new(0.0_f32, 1.0, 0.0)));
    let mut primitive = PrimitiveSubmission {
        kind: Graphic3dTypeOfPrimitiveArray::Triangles,
        attributes: attributes.cast(),
        indices: Handle::default(),
        bounds: Handle::default(),
        upload: Default::default(),
        uid: 1,
        retained: None,
    };
    primitive.retained = store.entry(&primitive);
    (primitive, attributes)
}

#[test]
fn mutable_source_edit_preserves_indices_and_covers_signed_zero() {
    let (array, buffer) = mutable_source(&RetainedPrimitives::default());
    let before = draw(&array, [1.; 4]);
    assert!(buffer.write_at(0, -0.0_f32));
    // A caller can write directly without explicitly invalidating the native range.
    let after = draw(&array, [1.; 4]);
    let edit = after.vertex_revision().unwrap();
    assert_eq!(
        edit.previous_identity(),
        before.vertices.identity().unwrap()
    );
    assert_eq!(edit.dirty_range(), 0..1);
    assert_eq!(after.indices.identity(), before.indices.identity());
    assert_eq!(
        after.vertices[0].position[0].to_bits(),
        (-0.0_f32).to_bits()
    );
    assert!(after.vertices.identity() != before.vertices.identity());
}

#[test]
fn geometry_replacement_and_count_change_invalidate_the_partial_upload_hint() {
    let (array, buffer) = mutable_source(&RetainedPrimitives::default());
    let original = draw(&array, [1.; 4]);
    assert!(buffer.write_at(12, 2.0_f32));
    let mut edited = draw(&array, [1.; 4]);
    assert!(edited.vertex_revision().is_some());
    edited.vertices.make_mut()[1].position[0] = 3.;
    assert!(edited.vertex_revision().is_none());
    assert_eq!(original.vertices[1].position[0], 1.);
    buffer.set_nb_elements(0);
    let empty = draw(&array, [1.; 4]);
    assert!(empty.vertex_revision().is_none());
    assert!(empty.vertices.is_empty());
}

#[test]
fn index_edits_reuse_vertices_and_record_only_the_changed_words() {
    let store = RetainedPrimitives::default();
    let (mut array, _) = source(&store);
    let allocator = Graphic3dBufferBase::default_allocator();
    let indices = Handle::new(Graphic3dMutableIndexBuffer::new(allocator).unwrap());
    assert!(indices.init_indices::<u16>(3).unwrap());
    for (offset, index) in [0, 1, 2].into_iter().enumerate() {
        assert!(indices.set_index(offset as i32, index));
    }
    array.indices = indices.cast();
    array.retained = store.entry(&array);
    let before = draw(&array, [1.; 4]);
    assert!(indices.set_index(1, 2));
    let mut after = draw(&array, [1.; 4]);
    let edit = after.index_revision().unwrap();
    assert_eq!(edit.previous_identity(), before.indices.identity().unwrap());
    assert_eq!(edit.dirty_range(), 1..2);
    assert_eq!(after.vertices.identity(), before.vertices.identity());
    assert!(after.vertex_revision().is_none());
    after.indices.make_mut()[1] = 1;
    assert!(after.index_revision().is_none());
}

#[test]
fn combined_edits_require_both_current_stream_witnesses() {
    let store = RetainedPrimitives::default();
    let (mut array, attributes) = mutable_source(&store);
    let allocator = Graphic3dBufferBase::default_allocator();
    let indices = Handle::new(Graphic3dMutableIndexBuffer::new(allocator).unwrap());
    assert!(indices.init_indices::<u16>(3).unwrap());
    for (offset, index) in [0, 1, 2].into_iter().enumerate() {
        assert!(indices.set_index(offset as i32, index));
    }
    array.indices = indices.cast();
    array.retained = store.entry(&array);
    let before = draw(&array, [1.; 4]);
    assert!(attributes.write_at(12, 2.0_f32));
    assert!(indices.set_index(2, 1));
    let mut after = draw(&array, [1.; 4]);
    let (vertices, indices) = after.combined_revision().unwrap();
    assert_eq!(
        vertices.previous_identity(),
        before.vertices.identity().unwrap()
    );
    assert_eq!(
        indices.previous_identity(),
        before.indices.identity().unwrap()
    );
    assert_eq!(
        (vertices.dirty_range(), indices.dirty_range()),
        (1..2, 2..3)
    );
    after.indices.make_mut()[2] = 2;
    assert!(after.combined_revision().is_none());
    assert!(after.vertex_revision().is_none());
}

#[test]
fn only_large_retained_lines_without_vertex_colors_or_mutability_get_uniform_witnesses() {
    use martensite_tkservice::graphic3d::{
        GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE, GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR,
    };
    let store = RetainedPrimitives::default();
    for (count, flags, expected) in [
        (2, 0, None),
        (4096, 0, Some([1.; 4])),
        (4096, GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE, None),
        (4096, GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR, None),
    ] {
        let source = line_array(&store, count, flags);
        for _ in 0..2 {
            let line = crate::LineStyle::new(1., u16::MAX, 1).unwrap();
            let draws = source
                .draws([1.; 4], crate::identity_clip(), Some(line), None)
                .unwrap();
            assert_eq!(
                draws[0].uniform_line_color(),
                expected,
                "count={count}, flags={flags}"
            );
        }
    }
}

fn line_array(store: &RetainedPrimitives, count: i32, flags: i32) -> PrimitiveSubmission {
    use martensite_tkservice::graphic3d::Graphic3dArrayOfPrimitives;
    let kind = Graphic3dTypeOfPrimitiveArray::Segments;
    let array = Graphic3dArrayOfPrimitives::new(kind, count, 0, 0, flags).unwrap();
    for index in 1..=count {
        array
            .set_vertice(index, (index % 2) as f32, 0., 0.)
            .unwrap();
    }
    let mut source = PrimitiveSubmission {
        kind,
        attributes: array.attributes(),
        indices: array.indices(),
        bounds: array.bounds(),
        upload: Default::default(),
        uid: 1,
        retained: None,
    };
    source.retained = store.entry(&source);
    source
}
