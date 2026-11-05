//! The private readers behind the STL entry points: one mesh, or one mesh per domain.
use super::{RwStlReader, RwStlReaderBase};
use martensite_tkernel::{
    impl_transient,
    ncollection::Sequence,
    standard::{Failure, Handle, SharedState, StandardTransient},
};
use martensite_tkmath::{
    gp::{Point3, Xyz},
    poly::{Triangle, Triangulation},
};

/// Collects nodes and triangles; `keeps_domains` selects `MultiDomainReader`.
#[derive(Default)]
pub(super) struct MeshCollector {
    transient: StandardTransient,
    base: RwStlReaderBase,
    nodes: SharedState<Vec<Xyz>>,
    triangles: SharedState<Vec<Triangle>>,
    keeps_domains: bool,
    domains: SharedState<Vec<Handle<Triangulation>>>,
}
impl_transient!(MeshCollector, "MeshCollector", dyn RwStlReader, transient, interfaces: [dyn RwStlReader]);

impl MeshCollector {
    /// A reader for one mesh; `MeshCollector::with_merge_angle(FRAC_PI_2)`.
    pub(super) fn with_merge_angle(angle: f64) -> Self {
        let reader = Self::default();
        reader.set_merge_angle(angle);
        reader
    }
    /// A reader making one triangulation per domain.
    pub(super) fn multi_domain(angle: f64) -> Self {
        let mut reader = Self::with_merge_angle(angle);
        reader.keeps_domains = true;
        reader
    }
    /// `get_triangulation`: a null handle when no triangle was collected.
    pub(super) fn triangulation(&self) -> Result<Handle<Triangulation>, Failure> {
        let nodes = self.nodes.read();
        let triangles = self.triangles.read();
        if triangles.is_empty() {
            return Ok(Handle::default());
        }
        let mesh =
            Triangulation::with_sizes(nodes.len() as i32, triangles.len() as i32, false, false)?;
        for (index, node) in nodes.iter().enumerate() {
            mesh.set_node(index as i32 + 1, Point3::from(*node))?;
        }
        for (index, triangle) in triangles.iter().enumerate() {
            mesh.set_triangle(index as i32 + 1, *triangle)?;
        }
        Ok(Handle::new(mesh))
    }
    /// `ChangeTriangulationList`, into the caller's sequence.
    pub(super) fn move_domains_to(&self, list: &mut Sequence<Handle<Triangulation>>) {
        for domain in self.domains.write().drain(..) {
            list.append(domain);
        }
    }
}
impl RwStlReader for MeshCollector {
    fn reader_base(&self) -> &RwStlReaderBase {
        &self.base
    }
    fn add_node(&self, point: &Xyz) -> i32 {
        let mut nodes = self.nodes.write();
        nodes.push(*point);
        nodes.len() as i32
    }
    fn add_triangle(&self, node1: i32, node2: i32, node3: i32) {
        self.triangles
            .write()
            .push(Triangle::new(node1, node2, node3));
    }
    fn add_solid(&self) -> Result<(), Failure> {
        if !self.keeps_domains {
            return Ok(());
        }
        let domain = self.triangulation()?;
        if !domain.is_null() {
            self.domains.write().push(domain);
        }
        self.nodes.write().clear();
        self.triangles.write().clear();
        Ok(())
    }
}
