//! The ordinary default provider re-creates stored references through public document APIs.
#[path = "support/mod.rs"]
mod support;
use martensite_tkcdf::{
    cdm::{CdmDocumentReferences, CdmMetaData, CdmReference},
    pcdm::PcdmReferenceIterator,
};
use martensite_tkernel::{
    standard::{Handle, dump::DumpStream},
    tcollection::ExtendedString,
};

fn file(kind: &str, scenario: usize) -> ExtendedString {
    let path = format!(
        "{}/tests/header_references/fixtures/{kind}-{scenario}.fsd",
        env!("CARGO_MANIFEST_DIR")
    );
    ExtendedString::from_c_string(path.as_bytes(), true)
}

fn assert_storage_configuration(reference: &Handle<CdmReference>, enabled: bool) {
    let mut dump = DumpStream::default();
    reference.dump_json(&mut dump, 0).unwrap();
    let fragment = format!("\"UseStorageConfiguration\": {}", i32::from(enabled));
    let rendered = String::from_utf8_lossy(dump.bytes());
    assert!(
        rendered.contains(&fragment),
        "expected {fragment}; dump was {rendered}"
    );
}

#[test]
fn default_provider_loads_original_identifiers_versions_and_names_from_each_container() {
    for kind in ["cmp", "text", "binary"] {
        let (application, _) = support::new_application(42);
        let (document, _) = support::new_document();
        let path = file(kind, 0);
        let meta = CdmMetaData::look_up(
            &mut application.meta_data_look_up_table(),
            &support::text("folder"),
            &support::text("header"),
            &path,
            &path,
            false,
        );
        PcdmReferenceIterator::with_optional_message_driver(None)
            .load_references(&document, &meta, &application, false)
            .unwrap();
        assert_eq!(document.to_references_number(), 2);
        assert_eq!(document.reference_counter(), 2);
        assert_eq!(document.reference(1).document_version(), 5);
        assert_eq!(document.reference(2).document_version(), 7);
        assert_eq!(document.name(1).unwrap(), support::text("child.std"));
        assert_eq!(document.name(2).unwrap(), support::text("sibling.std"));
        assert!(document.is_stored_reference(1).unwrap());
        assert_storage_configuration(&document.reference(1), false);
    }
}

#[test]
fn repeated_delimiters_load_only_the_last_selected_group_with_its_original_identifier() {
    let (application, _) = support::new_application(42);
    let (document, _) = support::new_document();
    let path = file("binary", 2);
    let meta = CdmMetaData::look_up(
        &mut application.meta_data_look_up_table(),
        &support::text("folder"),
        &support::text("header"),
        &path,
        &path,
        false,
    );
    PcdmReferenceIterator::with_optional_message_driver(None)
        .load_references(&document, &meta, &application, true)
        .unwrap();
    assert_eq!(document.to_references_number(), 1);
    assert_eq!(document.reference_counter(), 7);
    assert_eq!(document.reference(7).document_version(), 21);
    assert_eq!(document.name(7).unwrap(), support::text("selected.std"));
    assert_storage_configuration(&document.reference(7), true);
}
