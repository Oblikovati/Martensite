//! DEFINE_STANDARD_RTTI ancestry of the Graphic3d and Image classes that are both concrete and
//! a base: one descriptor per class, its native parent, and catch-by-base from every subclass.
//! A same-name interface descriptor would invent an extra ancestor and split `is_kind`.
use super::*;
use crate::image::{ImageTexture, ImageTextureSource};
use martensite_tkernel::standard::{Handle, RuntimeType, RuntimeTyped, Transient};
use martensite_tkernel::tcollection::AsciiString;
use std::sync::{Arc, Mutex};

/// `name parent` of the dynamic descriptor of `object`.
fn descriptor_row(object: &dyn Transient) -> String {
    let descriptor = object.dynamic_type();
    let parent = descriptor.parent().map_or("null", RuntimeType::name);
    format!("{} {parent}", descriptor.name())
}

fn buffer_allocator() -> martensite_tkernel::ncollection::BaseAllocator {
    Graphic3dBufferBase::default_allocator()
}

#[test]
fn concrete_base_classes_have_one_descriptor_with_the_native_parent() {
    assert!(std::ptr::eq(
        <dyn ImageTextureSource>::static_type(),
        ImageTexture::static_type()
    ));
    assert!(std::ptr::eq(
        <dyn Graphic3dAspectsSource>::static_type(),
        Graphic3dAspects::static_type()
    ));
    assert!(std::ptr::eq(
        <dyn Graphic3dBufferSource>::static_type(),
        Graphic3dBuffer::static_type()
    ));
    assert!(std::ptr::eq(
        <dyn Graphic3dIndexBufferSource>::static_type(),
        Graphic3dIndexBuffer::static_type()
    ));
    let texture = ImageTexture::new(&AsciiString::from("wood.png"));
    let buffer = Graphic3dBuffer::new(buffer_allocator()).unwrap();
    let indices = Graphic3dIndexBuffer::new(buffer_allocator()).unwrap();
    let rows = [
        descriptor_row(&texture),
        descriptor_row(&Graphic3dAspects::default()),
        descriptor_row(&buffer),
        descriptor_row(&indices),
    ];
    assert_eq!(
        rows,
        [
            "Image_Texture Standard_Transient",
            "Graphic3d_Aspects Standard_Transient",
            "Graphic3d_Buffer NCollection_Buffer",
            "Graphic3d_IndexBuffer Graphic3d_Buffer",
        ]
    );
}

#[test]
fn subclasses_are_caught_by_their_concrete_base_descriptor() {
    let mutable = Graphic3dMutableIndexBuffer::new(buffer_allocator()).unwrap();
    let attrib = Graphic3dAttribBuffer::new(buffer_allocator()).unwrap();
    let line = Graphic3dAspectLine3d::default();
    let lock: Graphic3dMediaFrameLock = Arc::new(Mutex::new(Graphic3dMediaFrameSwap::default()));
    let media = Graphic3dMediaTexture::new(&lock, 0);
    let rows = [
        descriptor_row(&mutable),
        descriptor_row(&attrib),
        descriptor_row(&line),
        descriptor_row(&media),
    ];
    assert_eq!(
        rows,
        [
            "Graphic3d_MutableIndexBuffer Graphic3d_IndexBuffer",
            "Graphic3d_AttribBuffer Graphic3d_Buffer",
            "Graphic3d_AspectLine3d Graphic3d_Aspects",
            "Graphic3d_MediaTexture Graphic3d_Texture2D",
        ]
    );
    assert!(mutable.is_kind(Graphic3dIndexBuffer::static_type()));
    assert!(mutable.is_kind(Graphic3dBuffer::static_type()));
    assert!(attrib.is_kind(Graphic3dBuffer::static_type()));
    assert!(line.is_kind(Graphic3dAspects::static_type()));
    assert!(media.is_kind(Graphic3dTexture2D::static_type()));
    let handle: Handle<Graphic3dIndexBuffer> =
        Handle::new(Graphic3dIndexBuffer::new(buffer_allocator()).unwrap());
    let as_source = handle.clone().cast::<dyn Graphic3dBufferSource>();
    assert_eq!(as_source.object_address(), handle.object_address());
}
