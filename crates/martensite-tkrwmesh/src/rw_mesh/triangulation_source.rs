//! `RwMeshTriangulationSource`: a triangulation whose data can be loaded later.
use super::RwMeshTriangulationReader;
use martensite_tkernel::impl_transient;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::osd::FileSystemHandle;
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkmath::poly::{Triangulation, TriangulationHooks};
use std::sync::Arc;

/// What a source keeps beyond its triangulation.
struct SourceFields {
    reader: Option<Arc<dyn RwMeshTriangulationReader>>,
    edges: Array1<'static, i32>,
    nb_def_nodes: i32,
    nb_def_triangles: i32,
    degenerated_tri_nb: i32,
}

struct SourceShared {
    fields: SharedState<SourceFields>,
}

/// The virtual members a source overrides on its triangulation.
struct SourceHooks {
    shared: Arc<SourceShared>,
}

impl TriangulationHooks for SourceHooks {
    fn has_geometry(&self, base: &Triangulation) -> Option<bool> {
        let has_elements = base.nb_triangles() > 0 || !self.shared.fields.read().edges.is_empty();
        Some(base.nb_nodes() > 0 && has_elements)
    }
    fn nb_deferred_nodes(&self) -> i32 {
        self.shared.fields.read().nb_def_nodes
    }
    fn nb_deferred_triangles(&self) -> i32 {
        self.shared.fields.read().nb_def_triangles
    }
    fn create_new_entity(&self) -> Handle<Triangulation> {
        Handle::new(Triangulation::new())
    }
    fn load_deferred_data_with_file_system(
        &self,
        file_system: Option<&FileSystemHandle>,
        destination: &Triangulation,
    ) -> bool {
        let mut fields = self.shared.fields.write();
        fields.degenerated_tri_nb = 0;
        let reader = fields.reader.clone();
        drop(fields);
        let Some(reader) = reader else {
            return false;
        };
        let source = RwMeshTriangulationSource::from_shared(self.shared.clone());
        reader.load(&source, destination, file_system)
    }
}

/// A triangulation that a reader can fill later, and may hold edges.
///
/// ```
/// use martensite_tkrwmesh::rw_mesh::RwMeshTriangulationSource;
/// let source = RwMeshTriangulationSource::new();
/// source.set_nb_deferred_nodes(3);
/// assert_eq!(source.triangulation().get().unwrap().nb_deferred_nodes(), 3);
/// ```
#[derive(Clone)]
pub struct RwMeshTriangulationSource {
    transient: StandardTransient,
    shared: Arc<SourceShared>,
    triangulation: Handle<Triangulation>,
}

impl_transient!(
    RwMeshTriangulationSource,
    "RwMeshTriangulationSource",
    Triangulation,
    transient
);

impl Default for RwMeshTriangulationSource {
    fn default() -> Self {
        Self::new()
    }
}

impl RwMeshTriangulationSource {
    /// An empty source; `RwMeshTriangulationSource::new()`.
    pub fn new() -> Self {
        let shared = Arc::new(SourceShared {
            fields: SharedState::new(SourceFields {
                reader: None,
                edges: Array1::default(),
                nb_def_nodes: 0,
                nb_def_triangles: 0,
                degenerated_tri_nb: 0,
            }),
        });
        Self::from_shared(shared)
    }
    fn from_shared(shared: Arc<SourceShared>) -> Self {
        let hooks = SourceHooks {
            shared: shared.clone(),
        };
        Self {
            transient: StandardTransient::default(),
            shared,
            triangulation: Handle::new(Triangulation::with_hooks(Box::new(hooks))),
        }
    }
    /// The triangulation that holds the loaded data; `source.triangulation()`.
    pub fn triangulation(&self) -> &Handle<Triangulation> {
        &self.triangulation
    }
    /// The reader of the deferred data; `source.reader()`.
    pub fn reader(&self) -> Option<Arc<dyn RwMeshTriangulationReader>> {
        self.shared.fields.read().reader.clone()
    }
    /// Sets the reader of the deferred data; `source.set_reader(Some(reader))`.
    pub fn set_reader(&self, reader: Option<Arc<dyn RwMeshTriangulationReader>>) {
        self.shared.fields.write().reader = reader;
    }
    /// The degenerated triangles counted during a read; `source.degenerated_tri_nb()`.
    pub fn degenerated_tri_nb(&self) -> i32 {
        self.shared.fields.read().degenerated_tri_nb
    }
    /// Sets the degenerated triangle count; `source.set_degenerated_tri_nb(2)`.
    pub fn set_degenerated_tri_nb(&self, count: i32) {
        self.shared.fields.write().degenerated_tri_nb = count;
    }
    /// True with nodes and either triangles or edges; `source.has_geometry()`.
    pub fn has_geometry(&self) -> bool {
        self.triangulation
            .get()
            .is_some_and(Triangulation::has_geometry)
    }
    /// The number of edges; `source.nb_edges()`.
    pub fn nb_edges(&self) -> i32 {
        self.shared.fields.read().edges.len() as i32
    }
    /// An edge's node index, `index` within `1..=nb_edges`; `source.edge(1)?`.
    pub fn edge(&self, index: i32) -> Result<i32, Failure> {
        let fields = self.shared.fields.read();
        let value = fields
            .edges
            .value(index)
            .map_err(|e| e.into_failure("RwMeshTriangulationSource::edge"))?;
        Ok(*value)
    }
    /// Sets an edge's node index; `source.set_edge(1, 4)?`.
    pub fn set_edge(&self, index: i32, edge: i32) -> Result<(), Failure> {
        let mut fields = self.shared.fields.write();
        fields
            .edges
            .set_value(index, edge)
            .map_err(|e| e.into_failure("RwMeshTriangulationSource::set_edge"))
    }
    /// The estimated node count of the deferred data; `source.nb_deferred_nodes()`.
    pub fn nb_deferred_nodes(&self) -> i32 {
        self.shared.fields.read().nb_def_nodes
    }
    /// Sets the estimated node count; `source.set_nb_deferred_nodes(8)`.
    pub fn set_nb_deferred_nodes(&self, count: i32) {
        self.shared.fields.write().nb_def_nodes = count;
    }
    /// The estimated triangle count of the deferred data; `source.nb_deferred_triangles()`.
    pub fn nb_deferred_triangles(&self) -> i32 {
        self.shared.fields.read().nb_def_triangles
    }
    /// Sets the estimated triangle count; `source.set_nb_deferred_triangles(12)`.
    pub fn set_nb_deferred_triangles(&self, count: i32) {
        self.shared.fields.write().nb_def_triangles = count;
    }
    /// A copy of the edge array; `source.internal_edges()`.
    pub fn internal_edges(&self) -> Array1<'static, i32> {
        self.shared.fields.read().edges.clone()
    }
    /// Resizes the edge array, from index 1; `source.resize_edges(4, true)?`.
    pub fn resize_edges(&self, nb_edges: i32, to_copy_old: bool) -> Result<(), Failure> {
        let mut fields = self.shared.fields.write();
        fields
            .edges
            .resize_bounds(1, nb_edges, to_copy_old)
            .map_err(|e| e.into_failure("RwMeshTriangulationSource::resize_edges"))
    }
}
