//! Ownership and state lending across the native service object boundaries (#592).
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindow};
use martensite_tkservice::font::FontSystemFont;
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::*;

#[test]
fn alien_image_interface_preserves_allocation_and_original_pixels() {
    let alien = Handle::new(ImageAlienPixMap::default());
    let image = alien.upcast::<dyn ImagePixMapObject>();
    assert!(image.init_zero(ImageFormat::Gray, 2, 3, 0, 17));
    assert_eq!(alien.data().data.data(), &[17; 6]);
    assert_eq!(image.object_address(), alien.object_address());
    assert!(image.is_kind(ImagePixMap::static_type()));
    assert!(image.is_instance_name("Image_AlienPixMap"));
    assert_eq!(image.downcast::<ImageAlienPixMap>(), alien);
    assert!(image.downcast::<ImagePixMap>().is_null());
    let source = image.cast::<dyn ImagePixMapSource>();
    assert!(std::ptr::eq(
        image.pix_map_base(),
        image_pix_map_original_base(source.get().unwrap()).unwrap()
    ));
}

#[test]
fn image_views_and_weak_identity_do_not_retain_dead_objects() {
    let alien = Handle::new(ImageAlienPixMap::default());
    let weak = alien.downgrade();
    let image = alien.into_upcast::<dyn ImagePixMapObject>();
    let view = image.view::<ImageAlienPixMap>();
    assert_eq!(view.object_address(), image.object_address());
    assert_eq!(weak.ref_count(), 1);
    drop(image);
    assert!(weak.upgrade().is_null());
}

#[test]
fn array_factory_keeps_the_actual_primitive_class_and_storage() {
    let array =
        Graphic3dArrayOfPrimitives::create_array(Graphic3dTypeOfPrimitiveArray::Triangles, 3, 0, 0)
            .unwrap();
    assert!(array.is_instance_name("Graphic3d_ArrayOfTriangles"));
    assert!(array.is_kind(Graphic3dArrayOfPrimitives::static_type()));
    let triangles = array.downcast::<Graphic3dArrayOfTriangles>();
    assert!(!triangles.is_null());
    assert_eq!(array.object_address(), triangles.object_address());
    assert!(std::ptr::eq(array.primitive_array(), &**triangles));
    assert!(array.downcast::<Graphic3dArrayOfPrimitives>().is_null());
}

#[test]
fn media_texture_set_interface_lends_the_original_four_textures() {
    let media = Handle::new(Graphic3dMediaTextureSet::new().unwrap());
    let set = media.upcast::<dyn Graphic3dTextureSetSource>();
    assert_eq!(set.object_address(), media.object_address());
    assert!(set.is_instance_name("Graphic3d_MediaTextureSet"));
    assert!(set.is_kind(Graphic3dTextureSet::static_type()));
    assert_eq!(set.size(), 4);
    assert_eq!(Graphic3dTextureSet::iter(&set).count(), 4);
    set.set_value(0, Handle::default()).unwrap();
    assert!(media.value(0).unwrap().is_null());
    assert!(std::ptr::eq(set.texture_set_base(), &**media));
}

#[test]
fn index_buffer_supertrait_upcast_preserves_dynamic_class_and_identity() {
    let concrete = Handle::new(Graphic3dIndexBuffer::new(BaseAllocator::common()).unwrap());
    let index = concrete.upcast::<dyn Graphic3dIndexBufferSource>();
    let buffer = index.into_upcast::<dyn Graphic3dBufferSource>();
    assert_eq!(buffer.object_address(), concrete.object_address());
    assert!(buffer.is_instance_name("Graphic3d_IndexBuffer"));
    assert!(buffer.is_kind(Graphic3dBuffer::static_type()));
    assert!(std::ptr::eq(buffer.buffer_base(), concrete.buffer_base()));
}

#[test]
fn texture_supertrait_upcasts_preserve_the_original_texture() {
    let image = Handle::new(ImagePixMap::default()).into_upcast();
    let concrete = Handle::new(Graphic3dTexture2D::from_pix_map(image));
    let source = concrete.upcast::<dyn Graphic3dTexture2DSource>();
    let map = source.upcast::<dyn Graphic3dTextureMap>();
    let root = source.into_upcast::<dyn Graphic3dTextureRoot>();
    assert_eq!(root.object_address(), concrete.object_address());
    assert_eq!(map.object_address(), concrete.object_address());
    assert!(root.is_instance_name("Graphic3d_Texture2D"));
    assert!(std::ptr::eq(
        root.texture_root_base(),
        concrete.texture_root_base()
    ));
}

#[test]
fn window_interface_mutation_uses_original_neutral_window_state() {
    let neutral = Handle::new(AspectNeutralWindow::default());
    let window = neutral.upcast::<dyn AspectWindow>();
    neutral.set_size(37, 19);
    assert_eq!(window.size(), (37, 19));
    assert_eq!(window.object_address(), neutral.object_address());
    assert!(window.is_instance_name("Aspect_NeutralWindow"));
    assert_eq!(window.downcast::<AspectNeutralWindow>(), neutral);
}

#[test]
fn font_root_handles_keep_the_native_name_without_embedded_counters() {
    let font = Handle::new(FontSystemFont::new(&AsciiString::new("Owned Family")).unwrap());
    let root = font.upcast::<dyn Transient>();
    assert_eq!(root.object_address(), font.object_address());
    assert!(root.is_instance_name("Font_SystemFont"));
    assert_eq!(
        root.into_downcast::<FontSystemFont>()
            .font_name()
            .as_bytes(),
        b"Owned Family"
    );
    assert_eq!(font.ref_count(), 1);
}

#[test]
fn rendering_statistics_preserve_derived_transform_dispatch() {
    let scaled = Handle::new(
        Graphic3dTransformPersScaledAbove::new(
            100.0,
            &martensite_tkmath::gp::Point3::new(0.0, 0.0, 0.0),
        )
        .unwrap(),
    );
    let mut params = Graphic3dRenderingParams::new().unwrap();
    params.stats.stats_position = scaled.upcast();
    let position = &params.stats.stats_position;
    assert_eq!(position.object_address(), scaled.object_address());
    assert!(position.is_instance_name("Graphic3d_TransformPersScaledAbove"));
    let camera = Graphic3dCamera::default();
    assert!(position.persistent_scale(&camera, 64, 64) >= 100.0);
}
