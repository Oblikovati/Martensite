//! Root naming detaches before node teardown; its retained-owner outcome is explicit.
mod support;
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::{
    TNamingBuilder, TNamingNamedShape, TNamingNode, TNamingRefShape, TNamingUsedShapes,
};
use martensite_tkernel::{
    standard::{Handle, WeakHandle},
    tcollection::ExtendedString,
};
use martensite_tklcaf::{
    tdf::{TdfAttributeIterator, TdfLabel, TdfLabelNode},
    tdoc_std::TDocStdDocument,
};
use std::sync::{Arc, Weak};

struct RootNamingOwners {
    document: WeakHandle<TDocStdDocument>,
    named: WeakHandle<TNamingNamedShape>,
    used: WeakHandle<TNamingUsedShapes>,
    root: Weak<TdfLabelNode>,
    reference: Weak<TNamingRefShape>,
    nodes: Vec<(bool, Weak<TNamingNode>)>,
}

fn generate_label_shape(label: &TdfLabel, shape: &Shape) {
    TNamingBuilder::new(label)
        .unwrap()
        .generated(shape)
        .unwrap();
}

fn seed_root_naming(
    document: &Handle<TDocStdDocument>,
    shape: &Shape,
    named_first: bool,
    child_also: bool,
) {
    let root = document.get_data().root();
    if named_first {
        root.add_attribute(&Handle::new(TNamingNamedShape::new()).cast(), true)
            .unwrap();
    }
    generate_label_shape(&root, shape);
    if child_also {
        generate_label_shape(&document.main().unwrap(), shape);
    }
}

fn root_attribute_order(document: &Handle<TDocStdDocument>) -> Vec<String> {
    let mut attributes = TdfAttributeIterator::new(&document.get_data().root(), false).unwrap();
    let mut names = Vec::new();
    while attributes.more() {
        let attribute = attributes.value();
        names.push(attribute.dynamic_type().unwrap().name().to_string());
        attributes.next();
    }
    names
}

fn assert_root_attribute_order(document: &Handle<TDocStdDocument>, named_first: bool) {
    let mut expected = ["TDocStd_Owner", "TNamingUsedShapes", "TNamingNamedShape"];
    if named_first {
        expected.swap(1, 2);
    }
    assert_eq!(root_attribute_order(document), expected);
}

fn find_root_named_shape(document: &Handle<TDocStdDocument>) -> Handle<TNamingNamedShape> {
    let mut named = Handle::<TNamingNamedShape>::default();
    assert!(
        document
            .get_data()
            .root()
            .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
            .unwrap()
    );
    named
}

fn find_root_used_shapes(document: &Handle<TDocStdDocument>) -> Handle<TNamingUsedShapes> {
    let mut used = Handle::<TNamingUsedShapes>::default();
    assert!(
        document
            .get_data()
            .root()
            .find_attribute_as(&TNamingUsedShapes::get_id(), &mut used)
            .unwrap()
    );
    used
}

fn observe_evolution_nodes(reference: &Arc<TNamingRefShape>) -> Vec<(bool, Weak<TNamingNode>)> {
    let mut owners = Vec::new();
    let mut current = reference.first_use();
    while let Some(node) = current {
        owners.push((node.label().unwrap().is_root(), Arc::downgrade(&node)));
        current = node.next_same_shape(&Some(reference.clone()));
    }
    owners
}

impl RootNamingOwners {
    fn observe(document: &Handle<TDocStdDocument>, shape: &Shape) -> Self {
        let named = find_root_named_shape(document);
        let used = find_root_used_shapes(document);
        let map = used.get().unwrap().map();
        let reference = map.seek(shape).unwrap();
        Self {
            document: document.downgrade(),
            named: named.downgrade(),
            used: used.downgrade(),
            root: Arc::downgrade(document.get_data().root().node().unwrap()),
            reference: Arc::downgrade(reference),
            nodes: observe_evolution_nodes(reference),
        }
    }

    fn assert_source_drop_outcome(&self, child_also: bool) {
        assert!(self.document.upgrade().is_null());
        assert!(self.named.upgrade().is_null());
        assert!(self.used.upgrade().is_null());
        assert!(self.root.upgrade().is_none());
        assert!(self.reference.upgrade().is_some());
        assert_eq!(self.nodes.len(), 1 + usize::from(child_also));
        for (is_root, node) in &self.nodes {
            assert_eq!(
                node.upgrade().is_some(),
                *is_root,
                "only the root evolution remains"
            );
        }
    }
}

fn observe_root_named_shape_drop(named_first: bool, child_also: bool) {
    let document = TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).unwrap();
    let shape = support::make_box(10., 10., 10.);
    let weak_shape = shape.tshape().downgrade();
    seed_root_naming(&document, &shape, named_first, child_also);
    assert_root_attribute_order(&document, named_first);
    let owners = RootNamingOwners::observe(&document, &shape);
    drop(document);
    owners.assert_source_drop_outcome(child_also);
    assert_eq!(shape.tshape().ref_count(), 2);
    drop(shape);
    assert!(!weak_shape.upgrade().is_null());
}

#[test]
fn root_naming_drop_retains_one_shape_owner_in_both_attribute_orders() {
    for named_first in [false, true] {
        for child_also in [false, true] {
            observe_root_named_shape_drop(named_first, child_also);
        }
    }
}
