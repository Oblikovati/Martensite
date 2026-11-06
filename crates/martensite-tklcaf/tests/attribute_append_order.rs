//! A label keeps its attributes in the order they were created: every constructor of a
//! standard attribute adds the new attribute after the ones already there.
use martensite_tkernel::standard::{Failure, Guid, Handle};
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdata_std::{
    TDataStdBooleanArray, TDataStdExpression, TDataStdIntPackedMap, TDataStdInteger,
    TDataStdIntegerArray, TDataStdIntegerList, TDataStdNamedData, TDataStdReferenceArray,
    TDataStdRelation, TDataStdTreeNode, TDataStdVariable,
};
use martensite_tklcaf::tdf::{TdfAttribute, TdfAttributeIterator, TdfData, TdfLabel};
use martensite_tklcaf::tdoc_std::{
    TDocStdApplication, TDocStdDocument, TDocStdStandardApplication,
};
use martensite_tklcaf::tfunction::{
    TFunctionFunction, TFunctionGraphNode, TFunctionLogbook, TFunctionScope,
};

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

type LabelConstructor = fn(&TdfLabel) -> Result<Guid, Failure>;

fn label_constructors() -> [(&'static str, LabelConstructor); 11] {
    [
        ("expression", |l| Ok(TDataStdExpression::set(l)?.id())),
        ("relation", |l| Ok(TDataStdRelation::set(l)?.id())),
        ("variable", |l| Ok(TDataStdVariable::set(l)?.id())),
        ("named data", |l| Ok(TDataStdNamedData::set(l)?.id())),
        ("tree node", |l| Ok(TDataStdTreeNode::set(l)?.id())),
        ("packed map", |l| Ok(TDataStdIntPackedMap::set(l, false)?.id())),
        ("list", |l| Ok(TDataStdIntegerList::set(l)?.id())),
        ("array", |l| Ok(TDataStdIntegerArray::set(l, 1, 2, false)?.id())),
        ("boolean array", |l| Ok(TDataStdBooleanArray::set(l, 1, 2)?.id())),
        ("reference array", |l| Ok(TDataStdReferenceArray::set(l, 1, 2)?.id())),
        ("function", |l| Ok(TFunctionFunction::set(l)?.id())),
    ]
}

#[test]
fn a_new_attribute_follows_the_ones_already_on_its_label() {
    let data = TdfData::new();
    let root = data.root();
    let mut misplaced = Vec::new();
    for (tag, (name, construct)) in (1..).zip(label_constructors()) {
        let label = root.find_child(tag, true).unwrap();
        let first = TDataStdInteger::set(&label, 7).unwrap().id();
        let added = construct(&label).unwrap();
        if attribute_ids(&label) != [first, added] {
            misplaced.push(name);
        }
    }
    assert_eq!(misplaced, Vec::<&str>::new());
}

#[test]
fn a_function_graph_node_follows_the_ones_already_on_its_label() {
    let data = TdfData::new();
    let label = data.root().find_child(1, true).unwrap();
    let first = TDataStdInteger::set(&label, 7).unwrap().id();
    let added = TFunctionGraphNode::set(&label).unwrap().id();
    assert_eq!(attribute_ids(&label), vec![first, added]);
}

/// The scope and the logbook live on the root of the framework of a document.
#[test]
fn the_scope_and_the_logbook_follow_the_attributes_of_the_root() {
    let application = TDocStdStandardApplication::new();
    let mut document: Handle<TDocStdDocument> = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::new("BinOcaf"), &mut document)
        .unwrap();
    let root = document.main().unwrap().root().unwrap();
    let scope = TFunctionScope::set(&root).unwrap().id();
    assert_eq!(attribute_ids(&root).last(), Some(&scope));
    let logbook = TFunctionLogbook::set(&root).unwrap().id();
    assert_eq!(attribute_ids(&root).last(), Some(&logbook));
}
