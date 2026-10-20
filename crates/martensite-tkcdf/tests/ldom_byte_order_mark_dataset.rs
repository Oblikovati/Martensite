//! The light XML parser reads a file that starts with a UTF-8 byte order mark.
//!
//! The test passes when the parse summary contains `Document parsed` and one of its
//! white-space separated words is `UTF-8` (the summary prints `BOM is UTF-8`). The input is
//! the unchanged public dataset member `others/bug31340.xml`, read from the
//! `external/martensite-test-data` submodule or `$MARTENSITE_TEST_DATA_DIR`.
use martensite_tkcdf::ldom::{LdomBomType, LdomElement, LdomParser};
use std::path::PathBuf;

/// The dataset member and its pinned byte count.
const BOM_XML: &str = "others/bug31340.xml";
const BOM_XML_BYTES: u64 = 203;

/// A member of the `dataset-7.9.0` tree; fails when the input is missing.
fn dataset_file(name: &str) -> PathBuf {
    let directory = std::env::var_os("MARTENSITE_TEST_DATA_DIR").map_or_else(
        || PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../external/martensite-test-data"),
        PathBuf::from,
    );
    let path = directory.join("dataset-7.9.0").join(name);
    let length = std::fs::metadata(&path)
        .map(|meta| meta.len())
        .unwrap_or_else(|error| {
            panic!(
                "{}: {error}; run `git submodule update --init external/martensite-test-data` \
             (or set MARTENSITE_TEST_DATA_DIR)",
                path.display()
            )
        });
    assert_eq!(
        length,
        BOM_XML_BYTES,
        "{}: changed dataset member",
        path.display()
    );
    path
}

/// The name of a byte order mark as the parse summary prints it.
fn bom_name(bom: LdomBomType) -> &'static str {
    match bom {
        LdomBomType::BomUndefined => "",
        LdomBomType::BomUtf8 => "UTF-8",
        LdomBomType::BomUtf16be => "UTF-16 (BE)",
        LdomBomType::BomUtf16le => "UTF-16 (LE)",
        LdomBomType::BomUtf32be => "UTF-32 (BE)",
        LdomBomType::BomUtf32le => "UTF-32 (LE)",
        LdomBomType::BomUtf7 => "UTF-7",
        LdomBomType::BomUtf1 => "UTF-1",
        LdomBomType::BomUtfebcdic => "UTF-EBCDIC",
        LdomBomType::BomScsu => "SCSU",
        LdomBomType::BomBocu1 => "BOCU-1",
        LdomBomType::BomGb18030 => "GB-18030",
    }
}

/// The attribute lines the parse summary prints for `element`.
fn attribute_lines(element: &LdomElement, out: &mut String) {
    let attributes = element.get_attributes_list();
    for index in 0..attributes.get_length() {
        let item = attributes.item(index);
        let name = String::from_utf8_lossy(item.get_node_name().get_string()).into_owned();
        let value = String::from_utf8_lossy(item.get_node_value().get_string()).into_owned();
        out.push_str(&format!("       AttributeName = {name}\n"));
        out.push_str(&format!("       AttributeValue = {value}\n"));
    }
}

/// The whole parse summary of a file.
fn parse_summary(file: &[u8]) -> String {
    let mut parser = LdomParser::new();
    if parser.parse_file(file) {
        return "Document not parsed\n".to_owned();
    }
    let mut out = "Document parsed\n".to_owned();
    let root = parser.get_document().get_document_element();
    if root.is_null() {
        out.push_str("Root element is null\n");
        return out;
    }
    let root_name = String::from_utf8_lossy(root.get_tag_name().get_string()).into_owned();
    out.push_str(&format!("   RootName = {root_name}\n"));
    attribute_lines(&root, &mut out);
    let mut element = LdomElement::from_node(root.get_first_child());
    while !element.is_null() {
        let name = String::from_utf8_lossy(element.get_tag_name().get_string()).into_owned();
        out.push_str(&format!("   ElementName = {name}\n"));
        attribute_lines(&element, &mut out);
        element = LdomElement::from_node(element.get_next_sibling());
    }
    let bom = parser.get_bom();
    if bom != LdomBomType::BomUndefined {
        out.push_str(&format!("BOM is {}\n", bom_name(bom)));
    }
    out
}

#[test]
fn utf8_byte_order_mark_is_reported() {
    let path = dataset_file(BOM_XML);
    let list = parse_summary(path.to_string_lossy().as_bytes());
    assert!(
        list.contains("Document parsed"),
        "Error : document not parsed: {list}"
    );
    assert!(
        list.split_whitespace().any(|word| word == "UTF-8"),
        "Error : BOM was not found in {list}"
    );
}
