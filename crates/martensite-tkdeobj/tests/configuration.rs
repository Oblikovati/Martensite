//! Independent checks of OBJ configuration state and resources.
use martensite_tkde::de::{DeConfigurationContext, DeSectionGlobal};
use martensite_tkdeobj::de_obj::{DeObjConfigurationNode, RwObjInternalSection};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystem;

fn context(text: &str) -> DeConfigurationContext {
    let resource = DeConfigurationContext::new();
    assert!(resource.load_str(&text.into()).unwrap());
    resource
}
#[test]
fn defaults_and_metadata_are_exact() {
    let node = DeObjConfigurationNode::default();
    let p = node.internal_parameters.read();
    assert_eq!(p.file_length_unit, 1.0);
    assert_eq!(
        (p.system_cs, p.file_cs),
        (RwMeshCoordinateSystem::Z_UP, RwMeshCoordinateSystem::Y_UP)
    );
    assert!(!p.read_single_precision && !p.read_create_shapes);
    assert!(p.read_fill_doc && p.read_fill_incomplete);
    assert_eq!(p.read_memory_limit_mib, -1);
    assert!(
        p.read_root_prefix.is_empty() && p.write_author.is_empty() && p.write_comment.is_empty()
    );
}
#[test]
fn metadata_matches_obj_vendor_and_extension() {
    let node = DeObjConfigurationNode::default();
    assert!(node.is_import_supported() && node.is_export_supported());
    assert_eq!(node.get_format(), AsciiString::new("OBJ"));
    assert_eq!(node.get_vendor(), AsciiString::new("Martensite"));
    assert_eq!(
        node.get_extensions().iter().collect::<Vec<_>>(),
        vec![&AsciiString::new("obj")]
    );
}
#[test]
fn load_preserves_cpp_remainder_and_typed_coercions() {
    let node = DeObjConfigurationNode::default();
    let resource = context(
        "provider.OBJ.Martensite.file.length.unit : 0.0254\nprovider.OBJ.Martensite.system.cs : -3\nprovider.OBJ.Martensite.file.cs : 4\nprovider.OBJ.Martensite.read.single.precision : -2\nprovider.OBJ.Martensite.read.create.shapes : 7\nprovider.OBJ.Martensite.read.root.prefix : root\nprovider.OBJ.Martensite.read.fill.doc : 0\nprovider.OBJ.Martensite.read.fill.incomplete : 0\nprovider.OBJ.Martensite.read.memory.limit.mib : 512\nprovider.OBJ.Martensite.write.comment : note\nprovider.OBJ.Martensite.write.author : author\n",
    );
    assert!(node.load(&resource));
    let p = node.internal_parameters.read();
    assert_eq!(p.file_length_unit, 0.0254);
    assert_eq!((p.system_cs.as_raw(), p.file_cs.as_raw()), (-1, 0));
    assert!(p.read_single_precision && p.read_create_shapes);
    assert!(!p.read_fill_doc && !p.read_fill_incomplete);
    assert_eq!(p.read_memory_limit_mib, 512);
    assert_eq!(p.read_root_prefix, AsciiString::new("root"));
    assert_eq!(p.write_comment, AsciiString::new("note"));
    assert_eq!(p.write_author, AsciiString::new("author"));
}
#[test]
fn missing_invalid_and_wrong_scope_values_preserve_state() {
    let node = DeObjConfigurationNode::default();
    node.internal_parameters.write().file_length_unit = 25.4;
    node.internal_parameters.write().read_memory_limit_mib = 64;
    let before = node.internal_parameters.snapshot();
    assert!(node.load(&context("provider.OBJ.Martensite.file.length.unit : invalid\nprovider.OBJ.Martensite.read.memory.limit.mib : 2.5\nprovider.PLY.Martensite.read.fill.doc : 0\n")));
    assert_eq!(node.internal_parameters.snapshot(), before);
    assert!(node.load(&DeConfigurationContext::new()));
    assert_eq!(node.internal_parameters.snapshot(), before);
}
#[test]
fn copy_keeps_independent_base_and_parameter_state() {
    let node = DeObjConfigurationNode::default();
    node.configuration_node_base().set_enabled(false);
    node.configuration_node_base()
        .set_global_parameters(DeSectionGlobal {
            length_unit: 25.4,
            system_unit: 0.001,
        });
    node.internal_parameters.write().write_author = "before".into();
    let copied = DeObjConfigurationNode::copied_from(&node);
    node.configuration_node_base().set_enabled(true);
    node.configuration_node_base()
        .set_global_parameters(DeSectionGlobal::default());
    node.internal_parameters.write().write_author = "after".into();
    assert!(!copied.configuration_node_base().is_enabled());
    assert_eq!(
        copied
            .configuration_node_base()
            .global_parameters()
            .length_unit,
        25.4
    );
    assert_eq!(
        copied
            .configuration_node_base()
            .global_parameters()
            .system_unit,
        0.001
    );
    assert_eq!(
        copied.internal_parameters.read().write_author,
        AsciiString::new("before")
    );
}
#[test]
fn save_has_all_eleven_keys_and_roundtrips_changed_parameters() {
    let node = DeObjConfigurationNode::default();
    node.internal_parameters.write().write_comment = "{{FileCS}} literal".into();
    node.internal_parameters.write().file_length_unit = 0.001;
    let saved = node.save();
    let resource = context(std::str::from_utf8(saved.as_bytes()).unwrap());
    assert_eq!(resource.get_internal_map().extent(), 11);
    assert_eq!(
        resource.string_val(
            "write.comment",
            &AsciiString::default(),
            "provider.OBJ.Martensite"
        ),
        AsciiString::new("{{FileCS}} literal")
    );
    let loaded = DeObjConfigurationNode::default();
    assert!(loaded.load(&resource));
    assert_eq!(
        loaded.internal_parameters.snapshot(),
        node.internal_parameters.snapshot()
    );
    assert!(saved.as_bytes().starts_with(b"!*****************************************************************************\n!Configuration Node  Vendor: Martensite Format: OBJ\n"));
}
#[test]
fn public_configuration_values_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DeObjConfigurationNode>();
    assert_send_sync::<RwObjInternalSection>();
}
#[test]
fn default_resource_preserves_the_complete_reviewed_text() {
    let saved = DeObjConfigurationNode::default().save();
    assert_eq!(saved.as_bytes(), include_bytes!("fixtures/default.conf"));
}
