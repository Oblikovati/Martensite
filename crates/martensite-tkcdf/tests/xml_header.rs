//! Independent header fixtures: only the document start tag determines its format.
#[path = "xml_header/cases.rs"]
mod cases;
#[path = "xml_header/support.rs"]
mod support;
use martensite_tkcdf::pcdm::PcdmReadWriterBase;
use martensite_tkernel::tcollection::ExtendedString;
use support::HeaderFixture;

#[test]
fn filename_probe_reads_root_and_nested_document_headers_before_the_body() {
    for &(case, xml, expected) in cases::VALID_HEADERS {
        let fixture = HeaderFixture::new(case, Some(xml.as_bytes()));
        assert_eq!(
            PcdmReadWriterBase::file_format(&fixture.path),
            ExtendedString::new(expected),
            "{case}"
        );
    }
}

#[test]
fn filename_probe_widens_utf8_attribute_bytes_without_decoding_them() {
    let fixture = HeaderFixture::new("byte-widening", Some("<document format='Fó'/>".as_bytes()));
    assert_eq!(
        PcdmReadWriterBase::file_format(&fixture.path).as_utf16(),
        &[70, 195, 179]
    );
}

#[test]
fn filename_probe_does_not_invent_a_format_from_other_or_invalid_elements() {
    for &(case, xml) in cases::EMPTY_HEADERS {
        let fixture = HeaderFixture::new(case, Some(xml.as_bytes()));
        assert!(
            PcdmReadWriterBase::file_format(&fixture.path).is_empty(),
            "{case}"
        );
    }
    let missing = HeaderFixture::new("missing", None);
    assert!(PcdmReadWriterBase::file_format(&missing.path).is_empty());
}
