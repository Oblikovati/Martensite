//! Graphic3d primitive arrays and buffers; the byte layout is compared with the native
//! classes by the sv_array probe.
use martensite_tkernel::{ncollection::Vec3, standard::Handle};
use martensite_tkservice::graphic3d::*;

#[test]
fn triangles_store_interleaved_vertices() {
    let array = Graphic3dArrayOfTriangles::with_options(3, 0, true, true, false).unwrap();
    assert_eq!(
        array
            .add_vertex_normal_f64([1.0, 2.0, 3.0], [0.0, 0.0, 1.0])
            .unwrap(),
        1
    );
    array.set_vertex_color_rgb(1, 1.0, 0.5, 0.0).unwrap();
    let buffer = array.attributes();
    assert_eq!(buffer.stride(), 12 + 12 + 4);
    assert_eq!(buffer.read_at::<f32>(4), Some(2.0));
    assert_eq!(array.vertex_color4(1).unwrap().y, 127);
    assert_eq!(array.vertice_xyz(1).unwrap(), [1.0, 2.0, 3.0]);
    assert_eq!(
        array
            .set_vertice(4, 0.0, 0.0, 0.0)
            .unwrap_err()
            .exception_type(),
        "Standard_OutOfRange"
    );
}

#[test]
fn large_arrays_use_32_bit_indices() {
    let small = Graphic3dArrayOfTriangles::new(65534, 3, 0).unwrap();
    assert_eq!(small.indices().stride(), 2);
    let large = Graphic3dArrayOfTriangles::new(65535, 3, 0).unwrap();
    assert_eq!(large.indices().stride(), 4);
    large.add_vertex(0.0, 0.0, 0.0).unwrap();
    assert_eq!(large.add_edge(1).unwrap(), 1);
    assert_eq!(large.edge(1).unwrap(), 1);
}

#[test]
fn create_array_returns_the_class_of_the_type() {
    let array =
        Graphic3dArrayOfPrimitives::create_array(Graphic3dTypeOfPrimitiveArray::Segments, 4, 4, 0)
            .unwrap();
    assert_eq!(
        array
            .dynamic_type()
            .map(|kind| kind.name().to_owned())
            .as_deref(),
        Some("Graphic3d_ArrayOfSegments")
    );
    assert_eq!(array.string_type(), "ArrayOfSegments");
    assert!(
        Graphic3dArrayOfPrimitives::create_array(Graphic3dTypeOfPrimitiveArray::Undefined, 4, 0, 0)
            .unwrap()
            .is_null()
    );
    let error = array.add_triangle_edges(1, 2, 3).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_TypeMismatch");
}

#[test]
fn is_valid_truncates_partial_triangles() {
    let array = Graphic3dArrayOfTriangles::new(5, 0, 0).unwrap();
    (0..5).for_each(|i| {
        array.add_vertex(i as f32, 0.0, 0.0).unwrap();
    });
    assert!(array.is_valid());
    assert_eq!(array.vertex_number(), 3);
    assert_eq!(array.item_number(), 1);
}

fn indexed_triangle(vertices: i32, edge_capacity: i32) -> Graphic3dArrayOfTriangles {
    let array = Graphic3dArrayOfTriangles::new(vertices, edge_capacity, 0).unwrap();
    for vertex in 0..vertices {
        array.add_vertex(vertex as f32, 0.0, 0.0).unwrap();
    }
    for rank in [1, 2, vertices] {
        array.add_edge(rank).unwrap();
    }
    array
}

#[test]
fn is_valid_rechecks_live_vertex_count_and_direct_index_edits() {
    let array = indexed_triangle(4, 3);
    let indices = array.indices();
    let vertices = array.attributes();
    assert!(array.is_valid());
    vertices.set_nb_elements(3);
    assert!(!array.is_valid());
    vertices.set_nb_elements(4);
    assert!(array.is_valid());
    let mut index_storage = indices.buffer_base().state.write();
    index_storage.data.as_mut_slice()[..2].copy_from_slice(&4_u16.to_le_bytes());
    drop(index_storage);
    assert!(!array.is_valid());
    assert!(indices.set_index(0, 0));
    assert!(array.is_valid());
}

#[test]
fn is_valid_checks_the_whole_pre_truncation_prefix() {
    let array = indexed_triangle(3, 4);
    let indices = array.indices();
    assert!(array.is_valid());
    assert!(indices.set_index(3, 3));
    indices.set_nb_elements(4);
    assert!(!array.is_valid());
    assert_eq!(array.edge_number(), 3);
    assert!(array.is_valid());
}

#[test]
fn is_valid_rejects_an_index_prefix_beyond_storage() {
    let array = indexed_triangle(3, 3);
    let indices = array.indices();
    assert!(array.is_valid());
    indices.set_nb_elements(6);
    assert!(!array.is_valid());
    assert!(!array.is_valid());
    indices.set_nb_elements(3);
    assert!(array.is_valid());
}

#[test]
fn is_valid_preserves_signed_words_and_uses_actual_stride() {
    let array = indexed_triangle(3, 3);
    let indices = array.indices();
    assert!(indices.init_int32(3).unwrap());
    let mut state = indices.buffer_base().state.write();
    state.data.as_mut_slice().fill(0xff);
    drop(state);
    assert!(array.is_valid());
    indices.buffer_base().state.write().stride = 3;
    assert!(array.is_valid());
    indices.buffer_base().state.write().stride = 8;
    assert!(!array.is_valid());
    indices.buffer_base().state.write().stride = 0;
    assert!(array.is_valid());
}

#[test]
fn bounds_track_counts_and_colors() {
    let array = Graphic3dArrayOfPolylines::with_options(4, 2, 0, false, true).unwrap();
    assert_eq!(array.add_bound_rgb(2, 1.0, 0.0, 0.0).unwrap(), 1);
    assert_eq!(array.bound_color_rgb(1).unwrap(), [1.0, 0.0, 0.0]);
    assert_eq!(array.add_bound(2).unwrap(), 2);
    assert_eq!(
        array.add_bound(1).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
    assert_eq!(array.item_number(), 2);
}

#[test]
fn mutable_buffers_track_invalidated_ranges() {
    let array = Graphic3dArrayOfPoints::new(4, GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE).unwrap();
    let buffer = array.attributes();
    assert!(buffer.is_mutable());
    assert_eq!(array.vertex_number(), 4);
    buffer.invalidate().unwrap();
    assert_eq!(buffer.invalidated_range(), Graphic3dBufferRange::new(0, 48));
    buffer.validate();
    assert!(buffer.invalidated_range().is_empty());
    let indices =
        Graphic3dMutableIndexBuffer::new(Graphic3dBufferBase::default_allocator()).unwrap();
    indices.init_indices::<u16>(4).unwrap();
    assert!(indices.invalidate_indices(2, 1).is_err());
    indices.invalidate_indices(1, 2).unwrap();
    assert_eq!(indices.invalidated_range(), Graphic3dBufferRange::new(2, 4));
}

#[test]
fn deinterleaved_buffers_pack_each_attribute() {
    let array = Graphic3dArrayOfTriangles::new(
        3,
        0,
        GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED | GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL,
    )
    .unwrap();
    let buffer = array.attributes();
    assert!(!buffer.is_interleaved());
    assert_eq!(
        buffer.attribute_data(Graphic3dTypeOfAttribute::Norm),
        Some((1, 36, 12))
    );
    let packed =
        Handle::new(Graphic3dAttribBuffer::new(Graphic3dBufferBase::default_allocator()).unwrap());
    let attributes = [Graphic3dAttribute {
        id: Graphic3dTypeOfAttribute::Pos,
        data_type: Graphic3dTypeOfData::Vec3,
    }];
    assert!(packed.init_checked(2, &attributes).unwrap());
    assert!(packed.set_interleaved(false).is_err());
}

#[test]
fn bulk_read_keeps_packed_offsets_at_allocated_capacity() {
    let flags = GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED | GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL;
    let array = Graphic3dArrayOfTriangles::new(3, 0, flags).unwrap();
    let buffer = array.attributes();
    assert!(buffer.write_at(36, Vec3::new(0f32, 1., 0.)));
    buffer.set_nb_elements(1);
    let read = buffer.read_buffer();
    assert_eq!(read.element_count(), 1);
    assert_eq!(read.element_stride(), 24);
    assert_eq!(read.bytes().len(), 72);
    let normals = read.attribute(Graphic3dTypeOfAttribute::Norm).unwrap();
    assert_eq!(normals.kind, Graphic3dTypeOfData::Vec3);
    assert_eq!(normals.byte_offset(0), Some(36));
    assert_eq!(normals.value::<Vec3<f32>>(0).unwrap().data(), [0., 1., 0.]);
    assert_eq!(normals.value::<f32>(3), None);
    assert!(buffer.buffer_base().state.try_write().is_none());
}
