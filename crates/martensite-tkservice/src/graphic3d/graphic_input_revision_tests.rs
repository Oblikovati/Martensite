//! Each buffer write advances the revision of the result that observes it (#594).
use super::super::*;

fn allocator() -> martensite_tkernel::ncollection::BaseAllocator {
    Graphic3dBufferBase::default_allocator()
}

#[test]
fn an_attribute_buffer_layout_or_mutability_write_advances_its_revisions() {
    let buffer = Graphic3dAttribBuffer::new(allocator()).unwrap();
    let state = buffer.buffer_base().state.revision();
    let attributes = [Graphic3dAttribute {
        id: Graphic3dTypeOfAttribute::Pos,
        data_type: Graphic3dTypeOfData::Vec3,
    }];
    assert!(buffer.init(3, &attributes).unwrap());
    assert!(buffer.buffer_base().state.revision() > state, "init");
    let scope = GraphicInputRevision::default();
    assert!(buffer.observe_buffer_input(&scope));
    buffer.set_mutable(true).unwrap();
    assert_eq!(scope.revision(), 1, "the mutable flag");
}

#[test]
fn an_index_buffer_write_advances_its_state_revision() {
    let indices = Graphic3dIndexBuffer::new(allocator()).unwrap();
    let state = indices.buffer_base().state.revision();
    let scope = GraphicInputRevision::default();
    assert!(indices.observe_buffer_input(&scope));
    assert!(indices.init_int32(6).unwrap());
    assert!(scope.revision() > 0);
    assert!(indices.buffer_base().state.revision() > state);
}
