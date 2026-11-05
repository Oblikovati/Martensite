// SPDX-License-Identifier: GPL-2.0-only
//! The steps of the cases: creating the mesh, hiding and showing, vectors, labels and the
//! deformed mesh.
use crate::element_model::ElementModelSource;
use crate::view_host::ViewHost;
use martensite_tkernel::ncollection::{Array1, DataMap};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkernel::tcol_std::HPackedMapOfInteger;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Vector3;
use martensite_tkmeshvs::flags::*;
use martensite_tkmeshvs::selection_policy::MESHVS_BP_VECTOR;
use martensite_tkmeshvs::*;
use martensite_tkv3d::ais::AisInteractiveObject;

use MeshVSDrawerAttribute as Key;

/// A mesh on the view, under the two types the calls need it as.
pub struct DisplayedMesh {
    pub mesh: Handle<MeshVSMesh>,
    pub object: Handle<dyn AisInteractiveObject>,
    pub parent: Handle<dyn MeshVSMeshSource>,
}

/// The model with yellow edges and hidden nodes, displayed without selection modes
/// and fitted. Example: `let m = create_element_mesh(&host);`.
pub fn create_element_mesh(host: &ViewHost) -> DisplayedMesh {
    let source: Handle<dyn MeshVSDataSource> = Handle::new(ElementModelSource::default()).cast();
    let mesh = MeshVSMesh::new(false).unwrap();
    mesh.set_data_source(&source);
    let parent: Handle<dyn MeshVSMeshSource> = mesh.clone().cast();
    let builder = MeshVSMeshPrsBuilder::with_defaults(&parent);
    mesh.add_builder(&Handle::new(builder).cast(), true);
    let yellow = Color::named(NamedColor::YELLOW);
    mesh.get_drawer().set_color(Key::EdgeColor as i32, &yellow);
    let nodes = Handle::new(HPackedMapOfInteger::from_map(source.get_all_nodes()));
    mesh.set_hidden_nodes(&nodes);
    mesh.set_selectable_nodes(&nodes);
    let object: Handle<dyn AisInteractiveObject> = mesh.clone().cast();
    host.context.display(&object, true).unwrap();
    host.context.deactivate(&object).unwrap();
    host.view.fit_all(0.01, true).unwrap();
    DisplayedMesh {
        mesh,
        object,
        parent,
    }
}

/// Erases the mesh from the view; `hide_mesh(&host, &m)`.
pub fn hide_mesh(host: &ViewHost, mesh: &DisplayedMesh) {
    host.context.erase(&mesh.object, true).unwrap();
}

/// The owners selected in the context, as (is it a node, its id).
fn selected_entities(host: &ViewHost) -> Vec<(bool, i32)> {
    let mut entities = Vec::new();
    host.context.init_selected();
    while host.context.more_selected() {
        let owner = host.context.selected_owner().unwrap();
        let owner = owner.cast::<MeshVSMeshEntityOwner>();
        if let Some(owner) = owner.get() {
            entities.push((owner.kind() == MESHVS_ET_NODE, owner.id()));
        }
        host.context.next_selected();
    }
    entities
}

/// The ids of a hidden map in increasing order; `hidden_ids(&m.mesh.get_hidden_elems())`.
pub fn hidden_ids(map: &Handle<HPackedMapOfInteger>) -> Vec<i32> {
    let mut ids: Vec<i32> = map_or_empty(map.clone()).iter().collect();
    ids.sort_unstable();
    ids
}

fn map_or_empty(map: Handle<HPackedMapOfInteger>) -> MeshVSPackedMap {
    map.get().map(|map| map.map().clone()).unwrap_or_default()
}

fn store_hidden(mesh: &DisplayedMesh, nodes: MeshVSPackedMap, elements: MeshVSPackedMap) {
    mesh.mesh
        .set_hidden_nodes(&Handle::new(HPackedMapOfInteger::from_map(nodes)));
    mesh.mesh
        .set_hidden_elems(&Handle::new(HPackedMapOfInteger::from_map(elements)));
}

/// The selected entities join the hidden ones and the selection clears.
/// Example: `hide_selected(&host, &m)`.
pub fn hide_selected(host: &ViewHost, mesh: &DisplayedMesh) {
    let mut nodes = map_or_empty(mesh.mesh.get_hidden_nodes());
    let mut elements = map_or_empty(mesh.mesh.get_hidden_elems());
    for (is_node, id) in selected_entities(host) {
        let hidden = if is_node { &mut nodes } else { &mut elements };
        hidden.add(id);
    }
    host.context.clear_selected(false).unwrap();
    store_hidden(mesh, nodes, elements);
    host.context.redisplay(&mesh.object, true, false).unwrap();
}

/// Only the selected entities stay; the selection stays too.
/// Example: `show_only_selected(&host, &m)`.
pub fn show_only_selected(host: &ViewHost, mesh: &DisplayedMesh) {
    let source = mesh.mesh.get_data_source();
    let mut nodes = source.get_all_nodes();
    let mut elements = source.get_all_elements();
    for (is_node, id) in selected_entities(host) {
        let hidden = if is_node { &mut nodes } else { &mut elements };
        hidden.remove(id);
    }
    store_hidden(mesh, nodes, elements);
    host.context.redisplay(&mesh.object, true, false).unwrap();
}

/// Nothing hidden; `show_all(&host, &m)`.
pub fn show_all(host: &ViewHost, mesh: &DisplayedMesh) {
    store_hidden(mesh, MeshVSPackedMap::default(), MeshVSPackedMap::default());
    host.context.redisplay(&mesh.object, true, false).unwrap();
}

/// Which entities carry a vector.
#[derive(Clone, Copy, PartialEq)]
pub enum VectorMode {
    Element,
    Nodal,
}

/// The vectors a case asks for.
pub struct VectorRequest {
    pub mode: VectorMode,
    pub max_length: f64,
    pub color: NamedColor,
    pub arrow_part: f64,
    pub is_simple: bool,
}

/// The vector of one entity: an element's face normal when it has one, a node's position, and
/// the Z axis when neither gives a direction.
fn entity_vector(source: &dyn MeshVSDataSource, id: i32, mode: VectorMode) -> Vector3 {
    let mut coords = [0.; 3];
    let valid = match mode {
        VectorMode::Element => face_normal(source, id, &mut coords),
        VectorMode::Nodal => node_position(source, id, &mut coords),
    };
    let vector = Vector3::new(coords[0], coords[1], coords[2]);
    if valid && vector.magnitude() >= martensite_tkmath::gp::resolution() {
        return vector.normalized().unwrap();
    }
    Vector3::new(0., 0., 1.)
}

fn face_normal(source: &dyn MeshVSDataSource, id: i32, coords: &mut [f64; 3]) -> bool {
    let mut kind = MeshVSEntityType::default();
    source.get_geom_type(id, true, &mut kind);
    if kind != MESHVS_ET_FACE {
        return false;
    }
    let [x, y, z] = coords;
    source.get_normal(id, 3, x, y, z)
}

fn node_position(source: &dyn MeshVSDataSource, id: i32, coords: &mut [f64; 3]) -> bool {
    let mut array = Array1::<f64>::with_bounds(1, 3).unwrap();
    let (mut count, mut kind) = (0, MeshVSEntityType::default());
    let valid = source.get_geom(id, false, &mut array, &mut count, &mut kind);
    for axis in 0..3 {
        coords[axis] = array[axis as i32 + 1];
    }
    valid
}

/// Replaces the vector builder with one holding a vector per entity.
/// Example: `show_vectors(&host, &m, &request)`.
pub fn show_vectors(host: &ViewHost, mesh: &DisplayedMesh, request: &VectorRequest) {
    let vectors = MeshVSVectorPrsBuilder::static_type();
    let previous = mesh.mesh.find_builder(vectors);
    if let Some(previous) = previous.get() {
        mesh.mesh.remove_builder_by_id(previous.get_id());
    }
    let builder = new_vector_builder(mesh, request);
    let source = mesh.mesh.get_data_source();
    let source = source.get().unwrap();
    let is_element = request.mode == VectorMode::Element;
    let ids = if is_element {
        source.get_all_elements()
    } else {
        source.get_all_nodes()
    };
    for id in ids.iter() {
        builder.set_vector(is_element, id, &entity_vector(source, id, request.mode));
    }
    mesh.mesh.add_builder(&Handle::new(builder).cast(), false);
    mesh.mesh
        .get_drawer()
        .set_double(Key::VectorArrowPart as i32, request.arrow_part);
    host.context.redisplay(&mesh.object, true, false).unwrap();
}

fn new_vector_builder(mesh: &DisplayedMesh, request: &VectorRequest) -> MeshVSVectorPrsBuilder {
    MeshVSVectorPrsBuilder::new(
        &mesh.parent,
        request.max_length,
        &Color::named(request.color),
        MESHVS_DMF_VECTOR_DATA_PRS,
        &Handle::default(),
        -1,
        MESHVS_BP_VECTOR,
        request.is_simple,
    )
}

/// Yellow labels of height 20 with the element ids; no redisplay.
/// Example: `add_element_labels(&m)`.
pub fn add_element_labels(mesh: &DisplayedMesh) {
    let count = mesh.mesh.get_data_source().get_all_elements().extent();
    let mut labels = DataMap::default();
    for id in 1..=count {
        labels.bind(id, AsciiString::from(id.to_string().as_str()));
    }
    let yellow = Color::named(NamedColor::YELLOW);
    let builder = MeshVSTextPrsBuilder::with_defaults(&mesh.parent, 20., &yellow);
    builder.set_texts(true, &labels);
    mesh.mesh.add_builder(&Handle::new(builder).cast(), false);
}

/// Moves each node along its own position, or along Z from
/// the origin, then fits. Example: `deform_mesh(&host, &m, 20.)`.
pub fn deform_mesh(host: &ViewHost, mesh: &DisplayedMesh, scale: f64) {
    let source = mesh.mesh.get_data_source();
    let deformed = MeshVSDeformedDataSource::new(&source, scale);
    let source = source.get().unwrap();
    for id in source.get_all_nodes().iter() {
        let mut coords = [0.; 3];
        node_position(source, id, &mut coords);
        let vector = Vector3::new(coords[0], coords[1], coords[2]);
        let direction = if vector.magnitude() == 0. {
            Vector3::new(0., 0., 1.)
        } else {
            vector
        };
        deformed.set_vector(id, &direction.normalized().unwrap());
    }
    mesh.mesh.set_data_source(&Handle::new(deformed).cast());
    host.context.redisplay(&mesh.object, false, false).unwrap();
    host.view.fit_all(0.01, true).unwrap();
}
