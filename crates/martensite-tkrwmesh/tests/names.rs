//! The node names of each format.
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfData;
use martensite_tkrwmesh::rw_mesh::{RwMesh, RwMeshNameFormat as Format};

#[test]
fn every_format_composes_the_labels_names() {
    let data = TdfData::new();
    let root = data.root();
    let instance = root.find_child(1, true).unwrap();
    let product = root.find_child(2, true).unwrap();
    let unnamed = root.find_child(3, true).unwrap();
    TDataStdName::set(&instance, &ExtendedString::new("inst")).unwrap();
    TDataStdName::set(&product, &ExtendedString::new("prod")).unwrap();
    let text = |format: Format, label, reference| {
        String::from_utf8(
            RwMesh::format_name(format, label, reference)
                .unwrap()
                .as_bytes()
                .to_vec(),
        )
        .unwrap()
    };
    assert_eq!(text(Format::EMPTY, &instance, &product), "");
    assert_eq!(text(Format::from_raw(42), &instance, &product), "");
    assert_eq!(text(Format::PRODUCT, &instance, &product), "prod");
    assert_eq!(text(Format::INSTANCE, &instance, &product), "inst");
    assert_eq!(
        text(Format::INSTANCE_OR_PRODUCT, &instance, &product),
        "inst"
    );
    assert_eq!(
        text(Format::INSTANCE_OR_PRODUCT, &unnamed, &product),
        "prod"
    );
    assert_eq!(
        text(Format::PRODUCT_OR_INSTANCE, &instance, &product),
        "prod"
    );
    assert_eq!(
        text(Format::PRODUCT_OR_INSTANCE, &instance, &unnamed),
        "inst"
    );
    assert_eq!(
        text(Format::PRODUCT_AND_INSTANCE, &instance, &product),
        "prod [inst]"
    );
    assert_eq!(
        text(Format::PRODUCT_AND_INSTANCE, &product, &product),
        "prod"
    );
    assert_eq!(
        text(Format::PRODUCT_AND_INSTANCE_AND_OCAF, &instance, &product),
        "prod [inst] [0:1]"
    );
    assert_eq!(
        text(Format::PRODUCT_AND_INSTANCE_AND_OCAF, &product, &product),
        "prod [0:2]"
    );
    assert_eq!(
        RwMesh::read_name_attribute(&unnamed).unwrap(),
        AsciiString::default()
    );
}
