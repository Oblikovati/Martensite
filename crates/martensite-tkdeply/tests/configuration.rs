//! Independent PLY configuration and content recognition contracts.
use martensite_tkde::de::{DeConfigurationContext, DeSectionGlobal};
use martensite_tkdeply::de_ply::{DePlyConfigurationNode, RwPlyInternalSection};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystem;
fn context(text: &str) -> DeConfigurationContext {
    let result = DeConfigurationContext::new();
    assert!(result.load_str(&text.into()).unwrap());
    result
}
#[test]
fn defaults_metadata_and_direction_are_exact() {
    let node = DePlyConfigurationNode::default();
    let p = node.internal_parameters.snapshot();
    assert_eq!(p.file_length_unit, 1.0);
    assert_eq!(
        (p.system_cs, p.file_cs),
        (RwMeshCoordinateSystem::Z_UP, RwMeshCoordinateSystem::Y_UP)
    );
    assert!(p.write_normals && p.write_colors && p.write_part_id);
    assert!(!p.write_tex_coords && !p.write_face_id);
    assert!(p.write_comment.is_empty() && p.write_author.is_empty());
    assert!(!node.is_import_supported() && node.is_export_supported());
    assert_eq!(node.get_format(), AsciiString::new("PLY"));
    assert_eq!(node.get_vendor(), AsciiString::new("Martensite"));
    assert_eq!(
        node.get_extensions().iter().collect::<Vec<_>>(),
        vec![&AsciiString::new("ply")]
    );
}
#[test]
fn load_keeps_independent_part_and_face_flags_and_signed_remainder() {
    let node = DePlyConfigurationNode::default();
    assert!(node.load(&context("provider.PLY.Martensite.file.length.unit : 25.4\nprovider.PLY.Martensite.system.cs : -3\nprovider.PLY.Martensite.file.cs : 6\nprovider.PLY.Martensite.write.normals : 0\nprovider.PLY.Martensite.write.colors : 0\nprovider.PLY.Martensite.write.tex.coords : -2\nprovider.PLY.Martensite.write.part.id : 3\nprovider.PLY.Martensite.write.face.id : 4\nprovider.PLY.Martensite.write.comment : comment\nprovider.PLY.Martensite.write.author : author\n")));
    let p = node.internal_parameters.snapshot();
    assert_eq!(p.file_length_unit, 25.4);
    assert_eq!((p.system_cs.as_raw(), p.file_cs.as_raw()), (-1, 0));
    assert!(!p.write_normals && !p.write_colors);
    assert!(p.write_tex_coords && p.write_part_id && p.write_face_id);
    assert_eq!(p.write_comment, AsciiString::new("comment"));
    assert_eq!(p.write_author, AsciiString::new("author"));
}
#[test]
fn invalid_missing_and_other_scope_values_leave_fields_unchanged() {
    let node = DePlyConfigurationNode::default();
    node.internal_parameters.write().file_length_unit = 12.5;
    let before = node.internal_parameters.snapshot();
    assert!(node.load(&context("provider.PLY.Martensite.file.length.unit : x\nprovider.PLY.Martensite.write.colors : no\nprovider.OBJ.Martensite.write.author : other\n")));
    assert_eq!(node.internal_parameters.snapshot(), before);
    assert!(node.load(&DeConfigurationContext::new()));
    assert_eq!(node.internal_parameters.snapshot(), before);
}
#[test]
fn copy_does_not_share_base_or_parameter_values() {
    let node = DePlyConfigurationNode::default();
    let base = node.configuration_node_base();
    base.set_enabled(false);
    base.set_global_parameters(DeSectionGlobal {
        length_unit: 2.0,
        system_unit: 3.0,
    });
    node.internal_parameters.write().write_author = "before".into();
    let copied = DePlyConfigurationNode::copied_from(&node);
    base.set_enabled(true);
    base.set_global_parameters(DeSectionGlobal::default());
    node.internal_parameters.write().write_author = "after".into();
    let copied_base = copied.configuration_node_base();
    assert!(!copied_base.is_enabled());
    assert_eq!(copied_base.global_parameters().length_unit, 2.0);
    assert_eq!(copied_base.global_parameters().system_unit, 3.0);
    assert_eq!(
        copied.internal_parameters.snapshot().write_author,
        AsciiString::new("before")
    );
}
#[test]
fn content_requires_lowercase_magic_and_c_locale_whitespace() {
    let node = DePlyConfigurationNode::default();
    assert!(!node.check_content(None));
    for length in 0..4 {
        assert!(!node.check_content(Some(&b"ply\n"[..length])));
    }
    for byte in 0_u8..=127 {
        let expected = matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c);
        assert_eq!(
            node.check_content(Some(&[b'p', b'l', b'y', byte])),
            expected
        );
    }
    assert!(!node.check_content(Some(b"PLY\n")));
    assert!(!node.check_content(Some(b" ply\n")));
    assert!(node.check_content(Some(b"ply\nanything")));
}
#[test]
fn serialization_has_ten_keys_and_preserves_literal_comment() {
    let node = DePlyConfigurationNode::default();
    node.internal_parameters.write().write_comment = "{{WriteColors}}".into();
    node.internal_parameters.write().write_face_id = true;
    let saved = node.save();
    let resource = context(std::str::from_utf8(saved.as_bytes()).unwrap());
    assert_eq!(resource.get_internal_map().extent(), 10);
    let loaded = DePlyConfigurationNode::default();
    assert!(loaded.load(&resource));
    assert_eq!(
        loaded.internal_parameters.snapshot(),
        node.internal_parameters.snapshot()
    );
}
#[test]
fn default_resource_matches_complete_reviewed_baseline() {
    assert_eq!(
        DePlyConfigurationNode::default().save().as_bytes(),
        include_bytes!("fixtures/default.conf")
    );
}
#[test]
fn configuration_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DePlyConfigurationNode>();
    assert_send_sync::<RwPlyInternalSection>();
}
