//! The owned native filename prefix survives protected field edits and object retirement.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{standard::Handle, tcollection::AsciiString};
use martensite_tkv3d::ais::{AisTexturedShape, AisTexturedShapeMembers, AisTexturedShapeSource};

#[test]
fn current_filename_preserves_native_c_string_bytes() {
    let object = AisTexturedShape::new(&Shape::default());
    assert_eq!(object.texture_file().as_bytes(), b"");
    for text in ["abcdefghi.png", "good.dat", "a-longer-filename.png"] {
        object.set_texture_file_name(&text.into()).unwrap();
        assert_eq!(object.texture_file().as_bytes(), text.as_bytes());
    }
    let embedded = AsciiString::from_raw(b"ABC\0tail");
    object.set_texture_file_name(&embedded).unwrap();
    assert_eq!(object.texture_file().as_bytes_with_nul(), b"ABC\0");
    let base = object.textured_shape_base();
    assert_eq!(base.fields.read().texture_file.as_bytes(), b"ABC\0tail");
    base.fields.write().texture_file.set_value_char(1, b'Z').unwrap();
    assert_eq!(object.texture_file().as_bytes(), b"ZBC");
    base.fields.write().texture_file.truncate(0).unwrap();
    assert_eq!(object.texture_file().as_bytes(), b"");
}

#[test]
fn filename_copy_survives_replacement_and_last_object_owner() {
    let object = AisTexturedShape::new(&Shape::default());
    object.set_texture_file_name(&"prior.png".into()).unwrap();
    let captured = object.texture_file();
    object.set_texture_file_name(&"0".into()).unwrap();
    assert_eq!(object.texture_file().as_bytes(), b"");
    object.set_texture_file_name(&"next.png".into()).unwrap();
    object.set_texture_pix_map(&Handle::default());
    assert_eq!(object.texture_file().as_bytes(), b"");
    assert_eq!(object.ref_count(), 1);
    drop(object);
    assert_eq!(captured.as_bytes(), b"prior.png");
}
