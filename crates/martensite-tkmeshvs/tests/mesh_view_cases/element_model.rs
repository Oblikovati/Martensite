// SPDX-License-Identifier: GPL-2.0-only
//! The test model of the cases: five volumes (two pyramids, two hexahedra and a prism) on sixteen
//! nodes. It holds no face normals, so a volume asks for its vector in vain.
use martensite_tkernel::ncollection::{Array1, HArray1, Sequence};
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkmeshvs::flags::{MESHVS_ET_NODE, MESHVS_ET_VOLUME};
use martensite_tkmeshvs::{
    MeshVSDataSource, MeshVSEntityType, MeshVSPackedMap, MeshVSTopologyHandle,
};

/// The coordinates of the nodes, id 1 first.
const NODES: [[f64; 3]; 16] = [
    [5., 5., 20.],
    [0., 10., 10.],
    [10., 0., 10.],
    [0., 0., 10.],
    [-10., 0., 10.],
    [-10., 10., 10.],
    [-10., 10., 0.],
    [-10., 0., 0.],
    [0., 0., 0.],
    [0., 10., 0.],
    [0., -10., 10.],
    [10., -10., 10.],
    [10., -10., 0.],
    [0., -10., 0.],
    [10., 0., 0.],
    [5., 5., -10.],
];

/// The node ids of the elements, id 1 first.
const ELEMENTS: [&[i32]; 5] = [
    &[1, 2, 3, 4],
    &[2, 4, 5, 6, 7, 8, 9, 10],
    &[2, 3, 4, 10, 15, 9],
    &[4, 3, 12, 11, 14, 13, 15, 9],
    &[16, 15, 10, 9],
];

/// The model as a mesh data source; `ElementModelSource::default()`.
#[derive(Default)]
pub struct ElementModelSource {
    transient: StandardTransient,
}
martensite_tkernel::impl_transient!(ElementModelSource, "ElementModelSource", dyn MeshVSDataSource, transient,
    interfaces: [dyn MeshVSDataSource]);

fn ids_up_to(last: i32) -> MeshVSPackedMap {
    let mut map = MeshVSPackedMap::default();
    for id in 1..=last {
        map.add(id);
    }
    map
}

fn node(id: i32) -> Option<&'static [f64; 3]> {
    NODES.get(usize::try_from(id - 1).ok()?)
}

fn element(id: i32) -> Option<&'static [i32]> {
    ELEMENTS.get(usize::try_from(id - 1).ok()?).copied()
}

/// Writes the coordinates of the nodes one after the other, from index 1 as the model does.
fn write_nodes(coords: &mut Array1<'_, f64>, ids: &[i32]) {
    for (rank, id) in ids.iter().enumerate() {
        let position = node(*id).expect("the elements name nodes of the model");
        for (axis, value) in position.iter().enumerate() {
            coords[(3 * rank + axis + 1) as i32] = *value;
        }
    }
}

/// The faces of a volume, as zero-based node ranks.
fn faces(faces: &[&[i32]]) -> MeshVSTopologyHandle {
    let lists = HArray1::<Sequence<i32>>::with_bounds(1, faces.len() as i32).unwrap();
    {
        let mut slots = lists.change_array1();
        for (rank, face) in faces.iter().enumerate() {
            for node in *face {
                slots[rank as i32 + 1].append(*node);
            }
        }
    }
    Handle::new(lists)
}

/// A pyramid of four triangles (nodes taken three at a time around the four).
fn pyramid_faces() -> MeshVSTopologyHandle {
    faces(&[&[0, 1, 2], &[1, 2, 3], &[2, 3, 0], &[3, 0, 1]])
}

/// Six quadrangles over eight nodes: four around, then the two the stepping leaves out.
fn hexahedron_faces() -> MeshVSTopologyHandle {
    faces(&[
        &[0, 1, 2, 3],
        &[2, 3, 4, 5],
        &[4, 5, 6, 7],
        &[6, 7, 0, 1],
        &[0, 3, 4, 7],
        &[1, 2, 5, 6],
    ])
}

/// Two triangles and the three quadrangles between them.
fn prism_faces() -> MeshVSTopologyHandle {
    faces(&[
        &[0, 1, 2],
        &[3, 4, 5],
        &[0, 1, 4, 3],
        &[1, 2, 5, 4],
        &[2, 0, 3, 5],
    ])
}

impl MeshVSDataSource for ElementModelSource {
    fn get_geom(
        &self,
        id: i32,
        is_element: bool,
        coords: &mut Array1<'_, f64>,
        nb_nodes: &mut i32,
        kind: &mut MeshVSEntityType,
    ) -> bool {
        if is_element {
            let Some(nodes) = element(id) else {
                return false;
            };
            (*kind, *nb_nodes) = (MESHVS_ET_VOLUME, nodes.len() as i32);
            write_nodes(coords, nodes);
            return true;
        }
        if node(id).is_none() {
            return false;
        }
        (*kind, *nb_nodes) = (MESHVS_ET_NODE, 1);
        write_nodes(coords, &[id]);
        true
    }

    fn get_3d_geom(&self, id: i32, nb_nodes: &mut i32, data: &mut MeshVSTopologyHandle) -> bool {
        let (count, topology) = match id {
            1 | 5 => (4, pyramid_faces()),
            2 | 4 => (8, hexahedron_faces()),
            3 => (6, prism_faces()),
            _ => return false,
        };
        (*nb_nodes, *data) = (count, topology);
        true
    }

    fn get_geom_type(&self, id: i32, is_element: bool, kind: &mut MeshVSEntityType) -> bool {
        let known = if is_element {
            element(id).is_some()
        } else {
            node(id).is_some()
        };
        if known {
            *kind = if is_element {
                MESHVS_ET_VOLUME
            } else {
                MESHVS_ET_NODE
            };
        }
        known
    }

    fn get_addr(&self, _id: i32, _is_element: bool) -> usize {
        0
    }

    fn get_nodes_by_element(
        &self,
        id: i32,
        node_ids: &mut Array1<'_, i32>,
        nb_nodes: &mut i32,
    ) -> bool {
        let Some(nodes) = element(id) else {
            return false;
        };
        *nb_nodes = nodes.len() as i32;
        let low = node_ids.lower();
        for (rank, value) in nodes.iter().enumerate() {
            node_ids[low + rank as i32] = *value;
        }
        true
    }

    fn get_all_nodes(&self) -> MeshVSPackedMap {
        ids_up_to(NODES.len() as i32)
    }

    fn get_all_elements(&self) -> MeshVSPackedMap {
        ids_up_to(ELEMENTS.len() as i32)
    }

    /// The model has no face normals.
    fn get_normal(&self, _id: i32, _max: i32, _nx: &mut f64, _ny: &mut f64, _nz: &mut f64) -> bool {
        false
    }
}
