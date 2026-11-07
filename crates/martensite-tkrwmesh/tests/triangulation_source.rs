//! Deferred data through a source and a reader.
use martensite_tkernel::impl_transient;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::osd::FileSystemHandle;
use martensite_tkernel::standard::{StandardTransient, Transient as _};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkmath::poly::{Triangle, Triangulation};
use martensite_tkrwmesh::rw_mesh::{
    RwMeshLoadingStatistic, RwMeshTriangulationReader, RwMeshTriangulationReaderBase,
    RwMeshTriangulationSource,
};
use std::sync::Arc;

/// A reader of one triangle, which can also fail.
#[derive(Default)]
struct OneTriangleReader {
    transient: StandardTransient,
    base: RwMeshTriangulationReaderBase,
    to_fail: bool,
}

impl_transient!(OneTriangleReader, "OneTriangleReader", dyn RwMeshTriangulationReader, transient, interfaces: [dyn RwMeshTriangulationReader]);

impl RwMeshTriangulationReader for OneTriangleReader {
    fn reader_base(&self) -> &RwMeshTriangulationReaderBase {
        &self.base
    }
    fn reader_base_mut(&mut self) -> &mut RwMeshTriangulationReaderBase {
        &mut self.base
    }
    fn load_mesh(
        &self,
        _source: &RwMeshTriangulationSource,
        destination: &Triangulation,
        _file_system: Option<&FileSystemHandle>,
    ) -> bool {
        if self.to_fail {
            return false;
        }
        self.set_nb_position_nodes(destination, 3, false).unwrap()
            && self.set_nb_triangles(destination, 1, false).unwrap()
            && {
                for (index, x) in [(1, 0.0), (2, 1.0), (3, 2.0)] {
                    self.set_node_position(destination, index, Point3::new(x, 0.0, 0.0))
                        .unwrap();
                }
                self.set_triangle(destination, 1, Triangle::new(1, 2, 3))
                    .unwrap()
                    == 1
            }
    }
}

#[test]
fn a_source_holds_edges_and_its_counts() {
    let source = RwMeshTriangulationSource::new();
    assert!(source.reader().is_none());
    assert!(!source.has_geometry());
    source.resize_edges(2, false).unwrap();
    source.set_edge(1, 4).unwrap();
    assert_eq!((source.nb_edges(), source.edge(1).unwrap()), (2, 4));
    assert!(source.set_edge(3, 4).is_err() && source.edge(3).is_err());
    assert_eq!(source.internal_edges().len(), 2);
    source.set_nb_deferred_nodes(3);
    source.set_nb_deferred_triangles(1);
    source.set_degenerated_tri_nb(2);
    assert_eq!(
        (
            source.nb_deferred_nodes(),
            source.nb_deferred_triangles(),
            source.degenerated_tri_nb()
        ),
        (3, 1, 2)
    );
    let mesh = source.triangulation().get().unwrap();
    assert_eq!(
        (mesh.nb_deferred_nodes(), mesh.nb_deferred_triangles()),
        (3, 1)
    );
    assert!(mesh.has_deferred_data());
    mesh.resize_nodes(1, false).unwrap();
    assert!(source.has_geometry(), "nodes with edges are geometry");
}

#[test]
fn a_mesh_loads_its_deferred_data_through_the_reader() {
    let source = RwMeshTriangulationSource::new();
    assert!(
        !source.triangulation().get().unwrap().load_deferred_data(),
        "no reader, nothing to load"
    );
    source.set_reader(Some(Arc::new(OneTriangleReader::default())));
    source.set_nb_deferred_nodes(3);
    source.set_nb_deferred_triangles(1);
    let mesh = source.triangulation().get().unwrap();
    assert!(mesh.load_deferred_data());
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (3, 1));
    assert_eq!(mesh.node(3).unwrap().x(), 2.0);
}

#[test]
fn a_failed_load_clears_the_destination_and_a_statistic_adds_up() {
    let source = RwMeshTriangulationSource::new();
    source.set_nb_deferred_nodes(5);
    source.set_nb_deferred_triangles(2);
    let destination = Triangulation::with_sizes(2, 0, false, false).unwrap();
    let failing = OneTriangleReader {
        to_fail: true,
        ..Default::default()
    };
    assert!(!failing.load(&source, &destination, None));
    assert_eq!(destination.nb_nodes(), 0);
    let mut reader = OneTriangleReader::default();
    reader.set_file_name(&AsciiString::new("a.bin"));
    reader.set_double_precision(true);
    reader.set_to_skip_degenerates(true);
    reader.set_to_print_debug_messages(true);
    assert!(
        reader.is_double_precision()
            && reader.to_skip_degenerates()
            && reader.to_print_debug_messages()
    );
    assert_eq!(reader.file_name().as_bytes(), b"a.bin");
    assert!(reader.coordinate_system_converter().is_empty());
    let converter = reader.coordinate_system_converter().clone();
    reader.set_coordinate_system_converter(&converter);
    assert!(
        reader.load(&source, &destination, None),
        "debug printing path"
    );
    reader.start_statistic();
    assert!(reader.load(&source, &destination, None));
    assert!(reader.load(&source, &destination, None));
    let statistic = reader.base.loading_statistic.read().unwrap();
    assert_eq!(statistic.loaded_nodes_nb, 6);
    assert_eq!(statistic.expected_nodes_nb, 10);
    reader.print_statistic();
    reader.stop_statistic();
    assert!(reader.base.loading_statistic.read().is_none());
}

#[test]
fn statistic_text_says_what_disagrees() {
    let mut statistic = RwMeshLoadingStatistic {
        expected_nodes_nb: 5,
        loaded_nodes_nb: 3,
        ..Default::default()
    };
    assert!(statistic.text("p ").starts_with("p Disconformity"));
    assert!(
        statistic
            .text("")
            .ends_with("Nodes: 5 expected / 3 loaded.")
    );
    statistic.expected_triangles_nb = 4;
    statistic.degenerated_triangles_nb = 1;
    statistic.loaded_triangles_nb = 3;
    assert!(statistic.text("").ends_with(
        "Nodes: 5 expected / 3 loaded. Triangles: 4 expected / 1 skipped degenerated / 3 loaded."
    ));
    statistic.print_statistic("p ");
    statistic.reset();
    assert_eq!(statistic.text(""), "");
}

#[test]
fn filling_helpers_check_their_counts_and_indices() {
    let mut reader = OneTriangleReader::default();
    let mesh = Triangulation::with_sizes(2, 1, false, false).unwrap();
    assert!(!reader.set_nb_position_nodes(&mesh, 0, false).unwrap());
    assert!(
        !reader.set_nb_uv_nodes(&mesh, 3).unwrap() && reader.set_nb_uv_nodes(&mesh, 2).unwrap()
    );
    reader.set_node_uv(&mesh, 1, Point2::new(0.5, 0.5)).unwrap();
    assert!(
        !reader.set_nb_normal_nodes(&mesh, 3).unwrap()
            && reader.set_nb_normal_nodes(&mesh, 2).unwrap()
    );
    reader
        .set_node_normal(&mesh, 1, Vec3::new(0.0, 0.0, 1.0))
        .unwrap();
    assert!(!reader.set_nb_triangles(&mesh, 0, false).unwrap());
    assert_eq!(
        reader
            .set_triangle(&mesh, 1, Triangle::new(1, 2, 3))
            .unwrap(),
        0
    );
    assert_eq!(
        reader
            .set_triangle(&mesh, 1, Triangle::new(1, 2, 2))
            .unwrap(),
        1
    );
    reader.set_to_skip_degenerates(true);
    assert_eq!(
        reader
            .set_triangle(&mesh, 1, Triangle::new(1, 2, 2))
            .unwrap(),
        -1
    );
    assert!(!reader.set_nb_edges(None, 1, false).unwrap());
    assert_eq!(reader.set_edge(None, 1, 1).unwrap(), 0);
    let source = RwMeshTriangulationSource::new();
    source
        .triangulation()
        .get()
        .unwrap()
        .resize_nodes(2, false)
        .unwrap();
    assert!(!reader.set_nb_edges(Some(&source), 0, false).unwrap());
    assert!(reader.set_nb_edges(Some(&source), 1, false).unwrap());
    assert_eq!(reader.set_edge(Some(&source), 1, 3).unwrap(), 0);
    assert_eq!(reader.set_edge(Some(&source), 1, 2).unwrap(), 1);
}

#[test]
fn a_source_says_its_type_and_its_parent() {
    let source = RwMeshTriangulationSource::new();
    let found = source.dynamic_type();
    assert_eq!(found.name(), "RwMeshTriangulationSource");
    assert_eq!(found.parent().unwrap().name(), "Poly_Triangulation");
}
