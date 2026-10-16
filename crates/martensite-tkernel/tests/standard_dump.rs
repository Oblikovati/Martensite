//! Dump legacy assertions on the JSON stream itself.
//!
//! The cases that dump points, axes and boxes need the math toolkit and stay
//! with it until it migrates.
#![allow(non_snake_case)]
use martensite_tkernel::standard::dump::{DumpStream, add_values_separator};
#[test]
fn Standard_DumpTest__AddValuesSeparator_EmptyStream() {
    let mut stream = DumpStream::default();
    add_values_separator(&mut stream).unwrap();
    assert!(stream.text().is_empty());
}
#[test]
fn Standard_DumpTest__AddValuesSeparator_AfterContent() {
    let mut stream = DumpStream::default();
    stream.append("\"Field1\": 42");
    add_values_separator(&mut stream).unwrap();
    assert_eq!(stream.text(), "\"Field1\": 42, ");
}
#[test]
fn Standard_DumpTest__AddValuesSeparator_AfterOpeningBrace() {
    let mut stream = DumpStream::default();
    stream.append("{");
    add_values_separator(&mut stream).unwrap();
    assert_eq!(stream.text(), "{");
}
#[test]
fn Standard_DumpTest__AddValuesSeparator_AfterExistingSeparator() {
    let mut stream = DumpStream::default();
    stream.append("\"Field1\": 42, ");
    add_values_separator(&mut stream).unwrap();
    assert_eq!(stream.text(), "\"Field1\": 42, ");
}
#[test]
fn Standard_DumpTest__AddValuesSeparator_MultipleFields() {
    let mut stream = DumpStream::default();
    stream.append("\"Field1\": 42");
    add_values_separator(&mut stream).unwrap();
    stream.append("\"Field2\": \"value\"");
    add_values_separator(&mut stream).unwrap();
    stream.append("\"Field3\": [1, 2, 3]");
    assert_eq!(
        stream.text(),
        "\"Field1\": 42, \"Field2\": \"value\", \"Field3\": [1, 2, 3]"
    );
}
#[test]
fn Standard_DumpTest__TellP_BehaviorValidation() {
    let mut stream = DumpStream::default();
    let first = stream.position();
    assert!(first == 0 || first == -1);
    stream.append("test");
    assert!(stream.position() > 0);
    let mut empty = DumpStream::default();
    add_values_separator(&mut empty).unwrap();
    assert!(empty.text().is_empty());
    let mut content = DumpStream::default();
    content.append("content");
    let before = content.text().to_owned();
    add_values_separator(&mut content).unwrap();
    assert_ne!(before, content.text());
}
#[test]
fn Standard_DumpTest__StreamWithOnlyOpenBrace() {
    let mut stream = DumpStream::default();
    stream.append("{");
    let before = stream.text().to_owned();
    add_values_separator(&mut stream).unwrap();
    assert_eq!(before, stream.text());
}
