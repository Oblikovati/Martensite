//! Weak observations do not keep the document graph alive while it is released.
use martensite_tkbrep::{
    brep::{BRepTEdge, BRepTVertex, BRepTool},
    topexp,
    topods::{Edge, Face, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkcaf::tnaming::{TNamingNode, TNamingRefShape, TNamingUsedShapes};
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient, WeakHandle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::{TdfAttributeIterator, TdfChildIterator, TdfLabel, TdfLabelNode};
use std::sync::{Arc, Weak};

const RELEASE_PROBE_COUNTS: [(&str, usize); 5] = [
    ("topology", 34),
    ("surface", 6),
    ("3d curve", 12),
    ("pcurve", 24),
    ("curve representation", 36),
];

#[derive(Default)]
pub(crate) struct ReleasedDocumentGraph {
    transients: Vec<(&'static str, WeakHandle<StandardTransient>)>,
    labels: Vec<Weak<TdfLabelNode>>,
    references: Vec<Weak<TNamingRefShape>>,
    evolutions: Vec<Weak<TNamingNode>>,
}

impl ReleasedDocumentGraph {
    pub(crate) fn observe<T: ?Sized + RuntimeTyped>(
        &mut self,
        kind: &'static str,
        handle: &Handle<T>,
    ) {
        if handle.is_null() {
            return;
        }
        let erased = handle.cast::<StandardTransient>();
        assert!(
            erased.get().is_some(),
            "{kind} must project to its transient base"
        );
        self.transients.push((kind, erased.downgrade()));
    }

    pub(crate) fn observe_topology(&mut self, shape: &Shape) {
        let mut unique = IndexedMapOfShape::default();
        topexp::map_shapes(shape, &mut unique, true, true);
        assert_eq!(
            unique.extent(),
            34,
            "10-cube must have 34 unique topology owners"
        );
        for subshape in unique.iter() {
            self.observe("topology", subshape.tshape());
            self.observe_geometry(subshape);
        }
    }

    fn observe_geometry(&mut self, shape: &Shape) {
        match shape.shape_type().unwrap() {
            ShapeEnum::Face => {
                let stored =
                    BRepTool::surface_with_location(&Face::unchecked(shape.clone())).unwrap();
                self.observe("surface", &stored.0);
            }
            ShapeEnum::Edge => self.observe_edge_geometry(&Edge::unchecked(shape.clone())),
            ShapeEnum::Vertex => self.observe_vertex_representations(shape),
            _ => {}
        }
    }

    fn observe_vertex_representations(&mut self, vertex: &Shape) {
        for representation in vertex
            .tshape()
            .cast::<BRepTVertex>()
            .get()
            .unwrap()
            .points()
        {
            self.observe("point representation", &representation);
        }
    }

    fn observe_edge_geometry(&mut self, edge: &Edge) {
        self.observe(
            "3d curve",
            &BRepTool::curve_with_location(edge).unwrap().curve,
        );
        for representation in edge.tshape().cast::<BRepTEdge>().get().unwrap().curves() {
            self.observe("curve representation", &representation);
        }
        for rank in 1..=2 {
            let stored = BRepTool::curve_on_surface_by_index(edge, rank).unwrap();
            self.observe("pcurve", &stored.pcurve);
            self.observe("pcurve surface", &stored.surface);
        }
    }

    pub(crate) fn observe_framework(&mut self, root: &TdfLabel) {
        self.observe_label(root);
        let mut children = TdfChildIterator::new(root, true).unwrap();
        while children.more() {
            self.observe_label(&children.value());
            children.next();
        }
        let mut used = Handle::<TNamingUsedShapes>::default();
        assert!(
            root.find_attribute_as(&TNamingUsedShapes::get_id(), &mut used)
                .unwrap()
        );
        self.observe_naming(&used);
    }

    fn observe_label(&mut self, label: &TdfLabel) {
        self.labels.push(Arc::downgrade(label.node().unwrap()));
        let mut attributes = TdfAttributeIterator::new(label, false).unwrap();
        while attributes.more() {
            self.observe("attribute", &attributes.value());
            attributes.next();
        }
    }

    fn observe_naming(&mut self, used: &Handle<TNamingUsedShapes>) {
        self.observe("naming map attribute", used);
        let map = used.get().unwrap().map();
        assert!(!map.is_empty());
        for (_, reference) in map.items() {
            self.references.push(Arc::downgrade(reference));
            let mut current = reference.first_use();
            while let Some(node) = current {
                self.evolutions.push(Arc::downgrade(&node));
                current = node.next_same_shape(&Some(reference.clone()));
            }
        }
    }

    pub(crate) fn assert_expired(&self) {
        for (kind, owner) in &self.transients {
            assert!(
                owner.upgrade().is_null(),
                "released document retains {kind}"
            );
        }
        assert_arc_expiry("label nodes", &self.labels);
        assert_arc_expiry("naming references", &self.references);
        assert_arc_expiry("naming evolutions", &self.evolutions);
    }
}

impl ReleasedDocumentGraph {
    pub(crate) fn assert_observed_scope(&self) {
        for (kind, expected) in RELEASE_PROBE_COUNTS {
            let observed = self
                .transients
                .iter()
                .filter(|(actual, _)| *actual == kind)
                .count();
            assert_eq!(observed, expected, "missing {kind} observations");
        }
        assert!(!self.labels.is_empty());
        assert!(!self.references.is_empty());
        assert!(!self.evolutions.is_empty());
    }
}

fn assert_arc_expiry<T>(kind: &str, owners: &[Weak<T>]) {
    assert!(
        owners.iter().all(|owner| owner.upgrade().is_none()),
        "released document retains {kind}"
    );
}
