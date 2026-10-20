//! Private core observations avoid exposing a production lifetime-inspection API.
use super::TdfData;
use crate::{tdata_std::TDataStdName, tdoc_std::TDocStdDocument};
use martensite_tkernel::tcollection::ExtendedString;
use std::sync::Arc;

#[test]
fn document_release_expires_framework_core_and_attribute_label_nodes() {
    let document = TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).unwrap();
    let framework = document.get_data();
    let root = framework.root();
    let main = document.main().unwrap();
    let attribute = TDataStdName::set(&main, &ExtendedString::from_utf8("box")).unwrap();
    let weak_core = Arc::downgrade(&framework.get().unwrap().core);
    let weak_root = Arc::downgrade(root.node().unwrap());
    let weak_main = Arc::downgrade(main.node().unwrap());
    let weak_attribute = attribute.downgrade();
    drop((attribute, main, root, framework, document));
    assert!(weak_core.upgrade().is_none());
    assert!(weak_root.upgrade().is_none());
    assert!(weak_main.upgrade().is_none());
    assert!(weak_attribute.upgrade().is_null());
}

#[test]
fn framework_core_has_one_owner_and_weak_label_links() {
    let framework = TdfData::new();
    let core = &framework.get().unwrap().core;
    assert_eq!(Arc::strong_count(core), 1);
    let root = framework.root();
    let child = root.find_child(1, true).unwrap();
    assert_eq!(Arc::strong_count(core), 1);
    assert!(Arc::ptr_eq(&root.node().unwrap().core().unwrap(), core));
    assert!(Arc::ptr_eq(&child.node().unwrap().core().unwrap(), core));
}
