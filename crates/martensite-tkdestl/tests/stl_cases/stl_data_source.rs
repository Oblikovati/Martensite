//! A mesh data source over a triangulation, as the mesh display commands use: nodes and triangles
//! by identifier, and each triangle's normal.
use martensite_tkernel::{
    impl_transient,
    ncollection::Array1,
    standard::{Handle, StandardTransient},
};
use martensite_tkmath::{gp::Vector3, poly::Triangulation};
use martensite_tkmeshvs::{MeshVSDataSource, MeshVSEntityType, MeshVSPackedMap, flags::*};

/// The tolerance of the normal: `Precision::square_confusion()`.
const SQUARE_CONFUSION: f64 = 1.0e-14;

/// A triangulation's nodes, triangles and normals; `StlDataSource::new(&mesh)`.
pub struct StlDataSource {
    transient: StandardTransient,
    nodes: MeshVSPackedMap,
    elements: MeshVSPackedMap,
    node_coords: Vec<[f64; 3]>,
    element_nodes: Vec<[i32; 3]>,
    element_normals: Vec<[f64; 3]>,
}
impl_transient!(StlDataSource, "StlDataSource", dyn MeshVSDataSource, transient, interfaces: [dyn MeshVSDataSource]);

impl StlDataSource {
    /// Copies the mesh; the identifiers run from 1 as its indices do.
    pub fn new(mesh: &Handle<Triangulation>) -> Self {
        let mesh = mesh.get().expect("a read mesh");
        let mut source = Self {
            transient: StandardTransient::default(),
            nodes: MeshVSPackedMap::default(),
            elements: MeshVSPackedMap::default(),
            node_coords: Vec::new(),
            element_nodes: Vec::new(),
            element_normals: Vec::new(),
        };
        for index in 1..=mesh.nb_nodes() {
            source.nodes.add(index);
            let point = mesh.node(index).unwrap();
            source.node_coords.push([point.x(), point.y(), point.z()]);
        }
        for index in 1..=mesh.nb_triangles() {
            source.elements.add(index);
            let (first, second, third) = mesh.triangle(index).unwrap().get();
            source.element_nodes.push([first, second, third]);
            source
                .element_normals
                .push(triangle_normal(mesh, [first, second, third]));
        }
        source
    }
}

fn triangle_normal(mesh: &Triangulation, nodes: [i32; 3]) -> [f64; 3] {
    let [a, b, c] = nodes.map(|node| mesh.node(node).unwrap());
    let mut normal = Vector3::between(a, b).crossed(Vector3::between(b, c));
    if normal.square_magnitude() > SQUARE_CONFUSION {
        normal.normalize().unwrap();
    } else {
        normal.set_coordinates(0., 0., 0.);
    }
    [normal.x(), normal.y(), normal.z()]
}

impl MeshVSDataSource for StlDataSource {
    fn get_geom(
        &self,
        id: i32,
        is_element: bool,
        coords: &mut Array1<'_, f64>,
        nb_nodes: &mut i32,
        kind: &mut MeshVSEntityType,
    ) -> bool {
        let lower = coords.lower();
        let index = id as usize;
        if is_element {
            let Some(nodes) = index
                .checked_sub(1)
                .and_then(|at| self.element_nodes.get(at))
            else {
                return false;
            };
            *kind = MESHVS_ET_FACE;
            *nb_nodes = 3;
            for (slot, value) in nodes
                .iter()
                .flat_map(|node| self.node_coords[*node as usize - 1])
                .enumerate()
            {
                coords.set_value(lower + slot as i32, value).unwrap();
            }
            return true;
        }
        let Some(point) = index.checked_sub(1).and_then(|at| self.node_coords.get(at)) else {
            return false;
        };
        *kind = MESHVS_ET_NODE;
        *nb_nodes = 1;
        for (slot, value) in point.iter().enumerate() {
            coords.set_value(lower + slot as i32, *value).unwrap();
        }
        true
    }
    fn get_geom_type(&self, _id: i32, is_element: bool, kind: &mut MeshVSEntityType) -> bool {
        *kind = if is_element {
            MESHVS_ET_FACE
        } else {
            MESHVS_ET_NODE
        };
        true
    }
    fn get_addr(&self, _id: i32, _is_element: bool) -> usize {
        0
    }
    fn get_nodes_by_element(
        &self,
        id: i32,
        node_ids: &mut Array1<'_, i32>,
        _nb_nodes: &mut i32,
    ) -> bool {
        let lower = node_ids.lower();
        let Some(nodes) = (id as usize)
            .checked_sub(1)
            .and_then(|at| self.element_nodes.get(at))
        else {
            return false;
        };
        if node_ids.length() < 3 {
            return false;
        }
        for (slot, node) in nodes.iter().enumerate() {
            node_ids.set_value(lower + slot as i32, *node).unwrap();
        }
        true
    }
    fn get_all_nodes(&self) -> MeshVSPackedMap {
        self.nodes.clone()
    }
    fn get_all_elements(&self) -> MeshVSPackedMap {
        self.elements.clone()
    }
    fn get_normal(&self, id: i32, max: i32, nx: &mut f64, ny: &mut f64, nz: &mut f64) -> bool {
        let Some(normal) = (id as usize)
            .checked_sub(1)
            .and_then(|at| self.element_normals.get(at))
        else {
            return false;
        };
        if max < 3 {
            return false;
        }
        (*nx, *ny, *nz) = (normal[0], normal[1], normal[2]);
        true
    }
}
