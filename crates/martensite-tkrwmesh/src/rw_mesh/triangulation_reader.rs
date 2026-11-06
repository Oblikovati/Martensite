//! `RwMeshTriangulationReader`: filling a triangulation from deferred storage.
use super::{RwMeshCoordinateSystemConverter, RwMeshTriangulationSource};
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::message::Message;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::osd::FileSystemHandle;
use martensite_tkernel::standard::{Failure, SharedState, StandardTransient, Transient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkmath::poly::{Triangle, Triangulation};

/// The nodes and triangles expected and loaded; `RwMeshLoadingStatistic::default()`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RwMeshLoadingStatistic {
    /// The nodes expected.
    pub expected_nodes_nb: i32,
    /// The nodes loaded.
    pub loaded_nodes_nb: i32,
    /// The triangles expected.
    pub expected_triangles_nb: i32,
    /// The degenerated triangles skipped.
    pub degenerated_triangles_nb: i32,
    /// The triangles loaded.
    pub loaded_triangles_nb: i32,
}

impl RwMeshLoadingStatistic {
    /// Clears the counters; `statistic.reset()`.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    /// The disagreement of expected and loaded amounts, empty when they agree;
    /// `statistic.text("[file]. ")`.
    pub fn text(&self, prefix: &str) -> String {
        let mut nodes = String::new();
        if self.expected_nodes_nb != self.loaded_nodes_nb {
            nodes = format!(
                "Nodes: {} expected / {} loaded.",
                self.expected_nodes_nb, self.loaded_nodes_nb
            );
        }
        let triangles = self.triangles_text();
        if !nodes.is_empty() && !triangles.is_empty() {
            nodes.push(' ');
        }
        if nodes.is_empty() && triangles.is_empty() {
            return String::new();
        }
        format!(
            "{prefix}Disconformity of the expected number of nodes/triangles for deferred mesh to the loaded amount. {nodes}{triangles}"
        )
    }
    fn triangles_text(&self) -> String {
        if self.expected_triangles_nb == self.loaded_triangles_nb {
            return String::new();
        }
        let skipped = match self.degenerated_triangles_nb {
            0 => String::new(),
            count => format!("{count} skipped degenerated / "),
        };
        format!(
            "Triangles: {} expected / {skipped}{} loaded.",
            self.expected_triangles_nb, self.loaded_triangles_nb
        )
    }
    /// Warns of a disagreement; `statistic.print_statistic("[file]. ")`.
    pub fn print_statistic(&self, prefix: &str) {
        let text = self.text(prefix);
        if !text.is_empty() {
            Message::send_warning_text(&AsciiString::new(&text));
        }
    }
}

/// The state of a reader; `RwMeshTriangulationReaderBase::default()`.
#[derive(Default)]
pub struct RwMeshTriangulationReaderBase {
    /// The axis and unit conversion.
    pub coord_sys_converter: RwMeshCoordinateSystemConverter,
    /// The file name for messages.
    pub file_name: AsciiString,
    /// The statistic being accumulated.
    pub loading_statistic: SharedState<Option<RwMeshLoadingStatistic>>,
    /// Whether the triangulation is filled with double precision.
    pub is_double_precision: bool,
    /// Whether degenerated triangles are skipped, by their indices.
    pub to_skip_degenerate_tris: bool,
    /// Whether debug information is printed.
    pub to_print_debug_messages: bool,
}

impl_transient_interface!(
    dyn RwMeshTriangulationReader,
    "RwMeshTriangulationReader",
    StandardTransient
);

/// Loads a source's data; a format implements [`Self::load_mesh`].
pub trait RwMeshTriangulationReader: Transient {
    /// The reader state; `reader.reader_base()`.
    fn reader_base(&self) -> &RwMeshTriangulationReaderBase;
    /// The mutable reader state; `reader.reader_base_mut()`.
    fn reader_base_mut(&mut self) -> &mut RwMeshTriangulationReaderBase;
    /// Fills the destination, the part a format supplies;
    /// `reader.load_mesh(&source, &destination, None)`.
    fn load_mesh(
        &self,
        source: &RwMeshTriangulationSource,
        destination: &Triangulation,
        file_system: Option<&FileSystemHandle>,
    ) -> bool;

    /// The file name for messages; `reader.file_name()`.
    fn file_name(&self) -> &AsciiString {
        &self.reader_base().file_name
    }
    /// Sets the file name for messages; `reader.set_file_name(&file)`.
    fn set_file_name(&mut self, file: &AsciiString) {
        self.reader_base_mut().file_name = file.clone();
    }
    /// Sets the axis and unit conversion for loading; `reader.set_coordinate_system_converter(&converter)`.
    fn set_coordinate_system_converter(&mut self, converter: &RwMeshCoordinateSystemConverter) {
        self.reader_base_mut().coord_sys_converter = converter.clone();
    }
    /// Sets whether double precision is used; `reader.set_double_precision(true)`.
    fn set_double_precision(&mut self, is_double: bool) {
        self.reader_base_mut().is_double_precision = is_double;
    }
    /// Sets whether degenerated triangles are skipped; `reader.set_to_skip_degenerates(true)`.
    fn set_to_skip_degenerates(&mut self, to_skip: bool) {
        self.reader_base_mut().to_skip_degenerate_tris = to_skip;
    }
    /// Sets whether debug information is printed; `reader.set_to_print_debug_messages(true)`.
    fn set_to_print_debug_messages(&mut self, to_print: bool) {
        self.reader_base_mut().to_print_debug_messages = to_print;
    }
    /// The axis and unit conversion for loading; `reader.coordinate_system_converter()`.
    fn coordinate_system_converter(&self) -> &RwMeshCoordinateSystemConverter {
        &self.reader_base().coord_sys_converter
    }
    /// Whether double precision is used, false by default; `reader.is_double_precision()`.
    fn is_double_precision(&self) -> bool {
        self.reader_base().is_double_precision
    }
    /// Whether degenerated triangles are skipped; `reader.to_skip_degenerates()`.
    fn to_skip_degenerates(&self) -> bool {
        self.reader_base().to_skip_degenerate_tris
    }
    /// Whether debug information is printed; `reader.to_print_debug_messages()`.
    fn to_print_debug_messages(&self) -> bool {
        self.reader_base().to_print_debug_messages
    }
    /// Starts or resets the statistic; `reader.start_statistic()`.
    fn start_statistic(&self) {
        *self.reader_base().loading_statistic.write() = Some(RwMeshLoadingStatistic::default());
    }
    /// Stops the statistic; `reader.stop_statistic()`.
    fn stop_statistic(&self) {
        *self.reader_base().loading_statistic.write() = None;
    }
    /// Prints the statistic between start and stop; `reader.print_statistic()`.
    fn print_statistic(&self) {
        if let Some(statistic) = *self.reader_base().loading_statistic.read() {
            statistic.print_statistic(&statistic_prefix(self.file_name()));
        }
    }
    /// Clears the destination and loads it, clearing it again on failure;
    /// `reader.load(&source, &destination, None)`.
    fn load(
        &self,
        source: &RwMeshTriangulationSource,
        destination: &Triangulation,
        file_system: Option<&FileSystemHandle>,
    ) -> bool {
        destination.clear();
        let is_set = destination
            .set_double_precision(self.is_double_precision())
            .is_ok();
        if is_set
            && self.load_mesh(source, destination, file_system)
            && self.finalize_loading(source, destination)
        {
            return true;
        }
        destination.clear();
        false
    }
    /// Finishes a load: the bounds and the statistic; `reader.finalize_loading(&source, &destination)`.
    fn finalize_loading(
        &self,
        source: &RwMeshTriangulationSource,
        destination: &Triangulation,
    ) -> bool {
        if let Some(mesh) = source
            .triangulation()
            .get()
            .filter(|mesh| mesh.has_cached_min_max())
        {
            destination.set_cached_min_max(&mesh.cached_min_max());
        }
        let found = loaded_statistic(source, destination);
        let mut statistic = self.reader_base().loading_statistic.write();
        if let Some(total) = statistic.as_mut() {
            total.expected_nodes_nb += found.expected_nodes_nb;
            total.expected_triangles_nb += found.expected_triangles_nb;
            total.degenerated_triangles_nb += found.degenerated_triangles_nb;
            total.loaded_nodes_nb += found.loaded_nodes_nb;
            total.loaded_triangles_nb += found.loaded_triangles_nb;
        } else if self.to_print_debug_messages() {
            let text = found.text(&statistic_prefix(self.file_name()));
            Message::send_trace_text(&AsciiString::new(&text));
        }
        true
    }
    /// Resizes the nodes; false for no nodes; `reader.set_nb_position_nodes(&mesh, 8, false)?`.
    fn set_nb_position_nodes(
        &self,
        mesh: &Triangulation,
        nb_nodes: i32,
        to_copy: bool,
    ) -> Result<bool, Failure> {
        if nb_nodes <= 0 {
            return Ok(false);
        }
        mesh.resize_nodes(nb_nodes, to_copy)?;
        Ok(true)
    }
    /// Sets a node, from index 1; `reader.set_node_position(&mesh, 1, point)?`.
    fn set_node_position(
        &self,
        mesh: &Triangulation,
        index: i32,
        point: Point3,
    ) -> Result<(), Failure> {
        mesh.set_node(index, point)
    }
    /// Adds UV nodes when the count matches the nodes; `reader.set_nb_uv_nodes(&mesh, 8)?`.
    fn set_nb_uv_nodes(&self, mesh: &Triangulation, nb_nodes: i32) -> Result<bool, Failure> {
        if nb_nodes <= 0 || mesh.nb_nodes() != nb_nodes {
            return Ok(false);
        }
        mesh.add_uv_nodes()?;
        Ok(true)
    }
    /// Sets a node's UV, from index 1; `reader.set_node_uv(&mesh, 1, uv)?`.
    fn set_node_uv(&self, mesh: &Triangulation, index: i32, uv: Point2) -> Result<(), Failure> {
        mesh.set_uv_node(index, uv)
    }
    /// Adds normals when the count matches the nodes; `reader.set_nb_normal_nodes(&mesh, 8)?`.
    fn set_nb_normal_nodes(&self, mesh: &Triangulation, nb_nodes: i32) -> Result<bool, Failure> {
        if nb_nodes <= 0 || mesh.nb_nodes() != nb_nodes {
            return Ok(false);
        }
        mesh.add_normals()?;
        Ok(true)
    }
    /// Sets a node's normal, from index 1; `reader.set_node_normal(&mesh, 1, normal)?`.
    fn set_node_normal(
        &self,
        mesh: &Triangulation,
        index: i32,
        normal: Vec3<f32>,
    ) -> Result<(), Failure> {
        mesh.set_normal_vec3(index, normal)
    }
    /// Resizes the triangles; false for none; `reader.set_nb_triangles(&mesh, 12, false)?`.
    fn set_nb_triangles(
        &self,
        mesh: &Triangulation,
        nb_tris: i32,
        to_copy: bool,
    ) -> Result<bool, Failure> {
        if nb_tris < 1 {
            return Ok(false);
        }
        mesh.resize_triangles(nb_tris, to_copy)?;
        Ok(true)
    }
    /// Sets a triangle: 0 for nodes out of range, -1 for a skipped degenerate, 1 when set;
    /// `reader.set_triangle(&mesh, 1, triangle)?`.
    fn set_triangle(
        &self,
        mesh: &Triangulation,
        index: i32,
        triangle: Triangle,
    ) -> Result<i32, Failure> {
        let (n1, n2, n3) = triangle.get();
        let range = 1..=mesh.nb_nodes();
        if !(range.contains(&n1) && range.contains(&n2) && range.contains(&n3)) {
            return Ok(0);
        }
        if self.to_skip_degenerates() && (n1 == n2 || n1 == n3 || n2 == n3) {
            return Ok(-1);
        }
        mesh.set_triangle(index, triangle)?;
        Ok(1)
    }
    /// Resizes the edges of a source; a plain mesh has none and warns;
    /// `reader.set_nb_edges(Some(&source), 4, false)?`.
    fn set_nb_edges(
        &self,
        source: Option<&RwMeshTriangulationSource>,
        nb_edges: i32,
        to_copy: bool,
    ) -> Result<bool, Failure> {
        let Some(source) = source else {
            warn_only_triangulation();
            return Ok(false);
        };
        if nb_edges < 1 {
            return Ok(false);
        }
        source.resize_edges(nb_edges, to_copy)?;
        Ok(true)
    }
    /// Sets an edge of a source: 0 for a plain mesh or a node out of range, 1 when set;
    /// `reader.set_edge(Some(&source), 1, 4)?`.
    fn set_edge(
        &self,
        source: Option<&RwMeshTriangulationSource>,
        index: i32,
        edge: i32,
    ) -> Result<i32, Failure> {
        let Some(source) = source else {
            warn_only_triangulation();
            return Ok(0);
        };
        let nodes = source
            .triangulation()
            .get()
            .map_or(0, Triangulation::nb_nodes);
        if !(1..=nodes).contains(&edge) {
            return Ok(0);
        }
        source.set_edge(index, edge)?;
        Ok(1)
    }
}

fn warn_only_triangulation() {
    Message::send_warning_text(&AsciiString::new(
        "Only triangulation loading is supported.",
    ));
}

fn statistic_prefix(file: &AsciiString) -> String {
    format!(
        "[Mesh reader. File '{}']. ",
        String::from_utf8_lossy(file.as_bytes())
    )
}

fn loaded_statistic(
    source: &RwMeshTriangulationSource,
    destination: &Triangulation,
) -> RwMeshLoadingStatistic {
    RwMeshLoadingStatistic {
        expected_nodes_nb: source.nb_deferred_nodes(),
        loaded_nodes_nb: destination.nb_nodes(),
        expected_triangles_nb: source.nb_deferred_triangles(),
        degenerated_triangles_nb: source.degenerated_tri_nb(),
        loaded_triangles_nb: destination.nb_triangles(),
    }
}
