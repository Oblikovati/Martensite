//! Independently authored settings checks.
use martensite_tkde::de::{DeConfigurationContext, DeSectionGlobal};
use martensite_tkdestl::de_stl::DeStlConfigurationNode;
use martensite_tkernel::tcollection::AsciiString;

fn context(text: &str) -> DeConfigurationContext {
    let context = DeConfigurationContext::new();
    assert!(context.load_str(&AsciiString::new(text)).unwrap());
    context
}

#[test]
fn stl_load_save_defaults_and_missing_values_are_explicit() {
    let node = DeStlConfigurationNode::new();
    assert_eq!(node.internal_parameters.read().read_merge_angle, 90.0);
    assert!(!node.internal_parameters.read().read_brep);
    assert!(node.internal_parameters.read().write_ascii);
    assert!(node.load(&context("provider.STL.Martensite.read.merge.angle:17.25\nprovider.STL.Martensite.read.brep:1\nprovider.STL.Martensite.write.ascii:0\n")));
    let saved = context(std::str::from_utf8(node.save().as_bytes()).unwrap());
    assert_eq!(
        saved.real_val("read.merge.angle", -1.0, "provider.STL.Martensite"),
        17.25
    );
    assert_eq!(
        saved.integer_val("read.brep", -1, "provider.STL.Martensite"),
        1
    );
    assert_eq!(
        saved.integer_val("write.ascii", -1, "provider.STL.Martensite"),
        0
    );
    assert!(node.load(&context("provider.STL.Martensite.read.merge.angle:bad\n")));
    assert_eq!(node.internal_parameters.read().read_merge_angle, 17.25);
}

#[test]
fn stl_copy_keeps_enabled_and_units_without_aliasing() {
    let node = DeStlConfigurationNode::default();
    node.configuration_node_base().set_enabled(false);
    node.configuration_node_base()
        .set_global_parameters(DeSectionGlobal {
            length_unit: 2.0,
            system_unit: 3.0,
        });
    let copy = node.copy();
    copy.internal_parameters.write().read_merge_angle = 1.0;
    copy.configuration_node_base().set_enabled(true);
    assert_eq!(node.internal_parameters.read().read_merge_angle, 90.0);
    assert!(!node.configuration_node_base().is_enabled());
    assert_eq!(
        copy.configuration_node_base()
            .global_parameters()
            .length_unit,
        2.0
    );
    assert_eq!(
        copy.configuration_node_base()
            .global_parameters()
            .system_unit,
        3.0
    );
    assert_eq!(
        DeStlConfigurationNode::copied_from(&copy).save(),
        copy.save()
    );
}

#[test]
fn stl_metadata_and_content_preserve_contradictory_case_comparisons() {
    let node = DeStlConfigurationNode::new();
    assert_eq!(node.get_format().as_bytes(), b"STL");
    assert_eq!(node.get_vendor().as_bytes(), b"Martensite");
    assert_eq!(
        node.get_extensions()
            .iter()
            .map(|s| s.as_bytes())
            .collect::<Vec<_>>(),
        [b"stl"]
    );
    assert!(node.is_import_supported() && node.is_export_supported() && node.is_stream_supported());
    assert!(!node.check_content(None));
    for bytes in [
        b"solid".as_slice(),
        b"solid name\0",
        b"SOLID name\0",
        b"solid\t\0",
        b"binarySTL\0",
    ] {
        assert!(!node.check_content(Some(bytes)));
    }
}

#[test]
fn stl_source_loader_accepts_outside_documented_angle_range() {
    let node = DeStlConfigurationNode::new();
    assert!(node.load(&context(
        "provider.STL.Martensite.read.merge.angle:-5\nprovider.STL.Martensite.read.brep:-1\n"
    )));
    assert_eq!(node.internal_parameters.read().read_merge_angle, -5.0);
    assert!(node.internal_parameters.read().read_brep);
    assert!(node.internal_parameters.read().write_ascii);
}

#[test]
fn mutable_configuration_types_are_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<martensite_tkdestl::de_stl::DeStlConfigurationNode>();
    require_send_sync::<martensite_tkdestl::de_stl::DeStlInternalSection>();
}
