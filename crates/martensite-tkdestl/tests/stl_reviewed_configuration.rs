//! Reviewed cached-serialization baseline, separate from authored invariants.
use martensite_tkde::de::{DeConfigurationContext, DeSectionGlobal};
use martensite_tkdestl::de_stl::DeStlConfigurationNode;
use martensite_tkernel::tcollection::AsciiString;

type ConfigurationCase = (&'static str, &'static [&'static str], &'static [u8]);
const CASES: &[ConfigurationCase] = &[
    (
        "default",
        &[],
        include_bytes!("data/configuration/stl_default.conf"),
    ),
    (
        "all_0",
        &[
            "provider.STL.Martensite.read.merge.angle : 0\nprovider.STL.Martensite.read.brep : 0\nprovider.STL.Martensite.write.ascii : 0\n",
        ],
        include_bytes!("data/configuration/stl_all_0.conf"),
    ),
    (
        "all_1",
        &[
            "provider.STL.Martensite.read.merge.angle : 1\nprovider.STL.Martensite.read.brep : 1\nprovider.STL.Martensite.write.ascii : 1\n",
        ],
        include_bytes!("data/configuration/stl_all_1.conf"),
    ),
    (
        "malformed_after_current",
        &[
            "provider.STL.Martensite.read.merge.angle : 17.25\nprovider.STL.Martensite.read.brep : 1\nprovider.STL.Martensite.write.ascii : 0\n",
            "provider.STL.Martensite.read.merge.angle : bad\nprovider.STL.Martensite.read.brep : bad\nprovider.STL.Martensite.write.ascii : bad\n",
        ],
        include_bytes!("data/configuration/stl_malformed_after_current.conf"),
    ),
    (
        "missing_after_current",
        &[
            "provider.STL.Martensite.read.merge.angle : 17.25\nprovider.STL.Martensite.read.brep : 1\nprovider.STL.Martensite.write.ascii : 0\n",
            "",
        ],
        include_bytes!("data/configuration/stl_missing_after_current.conf"),
    ),
    (
        "outside_documented_range",
        &[
            "provider.STL.Martensite.read.merge.angle : -5\nprovider.STL.Martensite.read.brep : -1\nprovider.STL.Martensite.write.ascii : 0\n",
        ],
        include_bytes!("data/configuration/stl_outside_documented_range.conf"),
    ),
];

#[test]
fn configuration_saves_match_reviewed_resource_bytes() {
    for (name, loads, expected) in CASES {
        let node = DeStlConfigurationNode::new();
        for input in *loads {
            let context = DeConfigurationContext::new();
            assert!(
                context.load_str(&AsciiString::new(input)).unwrap(),
                "{name}"
            );
            assert!(node.load(&context), "{name}");
        }
        assert_eq!(node.save().as_bytes(), *expected, "case {name}");
    }
}

#[test]
fn copy_values_survive_later_original_mutation() {
    let original = DeStlConfigurationNode::new();
    set_base_for_copy(&original, false, 25.4);
    let direct = DeStlConfigurationNode::copied_from(&original);
    let copied = original.copy();
    set_base_for_copy(&original, true, 2.0);
    mutate_internal(&original);
    assert_eq!(
        original.save().as_bytes(),
        include_bytes!("data/configuration/stl_source_after_copy_mutation.conf")
    );
    for copy in [direct, copied] {
        assert_copy_values(&copy);
    }
}

fn set_base_for_copy(node: &DeStlConfigurationNode, enabled: bool, length_unit: f64) {
    node.configuration_node_base().set_enabled(enabled);
    node.configuration_node_base()
        .set_global_parameters(DeSectionGlobal {
            length_unit,
            system_unit: 0.001,
        });
}

fn mutate_internal(original: &DeStlConfigurationNode) {
    original.internal_parameters.write().read_merge_angle = 17.25;
}

fn assert_copy_values(copy: &DeStlConfigurationNode) {
    assert!(!copy.configuration_node_base().is_enabled());
    assert_eq!(
        copy.configuration_node_base().global_parameters(),
        DeSectionGlobal {
            length_unit: 25.4,
            system_unit: 0.001
        }
    );
    assert_eq!(
        copy.save().as_bytes(),
        include_bytes!("data/configuration/stl_default.conf")
    );
}

const CONTENT_CASES: &[(&[u8], bool)] = &[
    (b"DBRep_DrawableShape", false),
    (b"DBRep_DrawableShape\0", false),
    (b"CASCADE Topology V1\n\0", false),
    (b"CASCADE Topology V2\n\0", false),
    (b"CASCADE Topology V3\n\0", false),
    (b"\0CASCADE Topology V3\0", false),
    (b"solid name", false),
    (b"SOLID name", false),
    (b"solid\t\0", false),
    (b"BINFILE", false),
    (b"BINFILE\0", false),
    (b"#VRML V2.0 utf8\n", false),
];

#[test]
fn content_recognition_matches_reviewed_request_truth_table() {
    let node = DeStlConfigurationNode::new();
    assert!(!node.check_content(None));
    for (bytes, expected) in CONTENT_CASES {
        assert_eq!(node.check_content(Some(bytes)), *expected, "{bytes:?}");
    }
}
