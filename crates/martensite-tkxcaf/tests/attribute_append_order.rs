//! A label keeps its attributes in the order they were created: the attributes of a document
//! are added after the ones already on their label, and a metadata copy keeps that order.
mod support;

use martensite_tkernel::standard::{Failure, Guid, Handle};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::osd::{File, OpenMode, Protection};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdata_std::{TDataStdInteger, TDataStdReal};
use martensite_tklcaf::tdf::{TdfAttribute, TdfAttributeIterator, TdfLabel};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocAssemblyItemId, XCAFDocAssemblyItemRef, XCAFDocCloneFlags, XCAFDocDocumentTool,
    XCAFDocEditor, XCAFDocGraphNode, XCAFDocNoteBalloon, XCAFDocNoteBinData, XCAFDocNoteComment,
    XCAFDocVolume,
};

fn text(value: &str) -> ExtendedString {
    ExtendedString::from_utf8(value)
}

fn document() -> Handle<TDocStdDocument> {
    let (_application, document) = support::new_document("BinXCAF");
    document
}

/// The attribute IDs of `label`, in iteration order.
fn attribute_ids(label: &TdfLabel) -> Vec<Guid> {
    let mut ids = Vec::new();
    let mut iterator = TdfAttributeIterator::new(label, true).unwrap();
    while iterator.more() {
        ids.push(iterator.value().get().unwrap().id());
        iterator.next();
    }
    ids
}

fn bin_data_from_memory(label: &TdfLabel) -> Result<Guid, Failure> {
    let bytes = Handle::new(HArray1::<u8>::with_bounds(1, 2).unwrap());
    let note = XCAFDocNoteBinData::set(
        label,
        &text("user"),
        &text("stamp"),
        &text("title"),
        &AsciiString::from("text/plain"),
        &bytes,
    )?;
    Ok(note.id())
}

fn bin_data_from_file(label: &TdfLabel) -> Result<Guid, Failure> {
    let mut file = File::new(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
    file.open(OpenMode::ReadOnly, &Protection::new())?;
    let note = XCAFDocNoteBinData::set_from_file(
        label,
        &text("user"),
        &text("stamp"),
        &text("title"),
        &AsciiString::from("text/plain"),
        &mut file,
    )?;
    Ok(note.id())
}

type LabelConstructor = fn(&TdfLabel) -> Result<Guid, Failure>;

fn label_constructors() -> [(&'static str, LabelConstructor); 7] {
    [
        ("measure", |l| Ok(XCAFDocVolume::set(l, 2.0)?.id())),
        ("graph node", |l| Ok(XCAFDocGraphNode::set(l)?.id())),
        ("comment", |l| {
            Ok(XCAFDocNoteComment::set(l, &text("user"), &text("stamp"), &text("words"))?.id())
        }),
        ("balloon", |l| {
            Ok(XCAFDocNoteBalloon::set(l, &text("user"), &text("stamp"), &text("words"))?.id())
        }),
        ("binary data", bin_data_from_memory),
        ("binary file", bin_data_from_file),
        ("item reference", |l| {
            let item = XCAFDocAssemblyItemId::from_string(&AsciiString::from("0:1:1:1"));
            Ok(XCAFDocAssemblyItemRef::set(l, &item)?.id())
        }),
    ]
}

#[test]
fn a_new_document_attribute_follows_the_ones_already_on_its_label() {
    let document = document();
    let main = document.main().unwrap();
    let mut misplaced = Vec::new();
    for (tag, (name, construct)) in (1..).zip(label_constructors()) {
        let label = main.find_child(tag, true).unwrap();
        let first = TDataStdInteger::set(&label, 7).unwrap().id();
        let added = construct(&label).unwrap();
        if attribute_ids(&label) != [first, added] {
            misplaced.push(name);
        }
    }
    assert_eq!(misplaced, Vec::<&str>::new());
}

#[test]
fn the_document_tool_follows_the_attributes_of_its_label() {
    let document = document();
    let label = document.main().unwrap().find_child(1, true).unwrap();
    let first = TDataStdInteger::set(&label, 7).unwrap().id();
    XCAFDocDocumentTool::set(&label, false).unwrap();
    assert_eq!(attribute_ids(&label)[..2], [first, XCAFDocDocumentTool::get_id()]);
}

#[test]
fn a_metadata_copy_adds_attributes_after_the_ones_on_the_destination() {
    let document = document();
    let main = document.main().unwrap();
    let source = main.find_child(1, true).unwrap();
    let integer = TDataStdInteger::set(&source, 7).unwrap().id();
    let real = TDataStdReal::set(&source, 2.5).unwrap().id();
    let destination = main.find_child(2, true).unwrap();
    let volume = XCAFDocVolume::set(&destination, 2.0).unwrap().id();
    let flags = XCAFDocCloneFlags {
        color: false,
        layer: false,
        material: false,
        vis_material: false,
        attributes: true,
    };
    XCAFDocEditor::clone_meta_data(&source, &destination, None, flags).unwrap();
    assert_eq!(attribute_ids(&destination), [volume, integer, real]);
}
