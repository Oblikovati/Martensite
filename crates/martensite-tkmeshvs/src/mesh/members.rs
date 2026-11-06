// SPDX-License-Identifier: GPL-2.0-only
//! The non-virtual members of `MeshVSMesh`: data source, drawer, owner maps and selection
//! method. The builder members are in builders.rs, the hidden and selectable maps in
//! visibility.rs.
use super::{MeshVSMeshSource, MeshVSOwnerMap, builders, visibility};
use crate::data_source::MeshVSDataSource;
use crate::drawer::MeshVSDrawerSource;
use crate::prs_builder::MeshVSPrsBuilder;
use crate::selection_policy::MeshVSMeshSelectionMethod;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Handle, RuntimeType};
use martensite_tkernel::tcol_std::HPackedMapOfInteger;

/// The non-virtual members of `MeshVSMesh`.
pub trait MeshVSMeshMembers: MeshVSMeshSource {
    /// `get_builders_count`; `mesh.get_builders_count()`.
    fn get_builders_count(&self) -> i32 {
        self.mesh_base().fields.read().builders.len() as i32
    }
    /// `get_builder`: the builder of a one-based index, null outside; `mesh.get_builder(1)`.
    fn get_builder(&self, index: i32) -> Handle<dyn MeshVSPrsBuilder> {
        builders::builder_at(self.mesh_base(), index)
    }
    /// `get_builder_by_id`: the first builder of an id, null without; `mesh.get_builder_by_id(0)`.
    fn get_builder_by_id(&self, id: i32) -> Handle<dyn MeshVSPrsBuilder> {
        builders::builder_by_id(self.mesh_base(), id)
    }
    /// `get_free_id`: the smallest non-negative id of no builder; `mesh.get_free_id()`.
    fn get_free_id(&self) -> i32 {
        builders::free_id(self.mesh_base())
    }
    /// `add_builder`: inserts before the first builder of a lower priority; a null builder is
    /// ignored. Example: `mesh.add_builder(&builder, true)`.
    fn add_builder(&self, builder: &Handle<dyn MeshVSPrsBuilder>, treat_as_hilighter: bool) {
        builders::add_builder(self.mesh_base(), builder, treat_as_hilighter);
    }
    /// `mesh.set_hilighter(&builder)`.
    fn set_hilighter(&self, builder: &Handle<dyn MeshVSPrsBuilder>) {
        self.mesh_base().fields.write().hilighter = builder.clone();
    }
    /// The builder of an index, false when null; `mesh.set_hilighter_at(1)`.
    fn set_hilighter_at(&self, index: i32) -> bool {
        let builder = self.get_builder(index);
        let found = !builder.is_null();
        if found {
            self.set_hilighter(&builder);
        }
        found
    }
    /// `set_hilighter_by_id`; `mesh.set_hilighter_by_id(0)`.
    fn set_hilighter_by_id(&self, id: i32) -> bool {
        let builder = self.get_builder_by_id(id);
        let found = !builder.is_null();
        if found {
            self.set_hilighter(&builder);
        }
        found
    }
    /// `get_hilighter`; `mesh.get_hilighter()`.
    fn get_hilighter(&self) -> Handle<dyn MeshVSPrsBuilder> {
        self.mesh_base().fields.read().hilighter.clone()
    }
    /// `remove_builder`: removes the builder of an index, forgetting it as highlighter.
    fn remove_builder(&self, index: i32) {
        builders::remove_builder(self.mesh_base(), index);
    }
    /// `remove_builder_by_id`: removes the first builder of an id.
    fn remove_builder_by_id(&self, id: i32) {
        builders::remove_builder_by_id(self.mesh_base(), id);
    }
    /// The first builder of a kind, null without.
    fn find_builder(&self, kind: &RuntimeType) -> Handle<dyn MeshVSPrsBuilder> {
        builders::find_builder(self.mesh_base(), kind)
    }
    /// `get_owner_maps`: the mesh owners with advanced selection, else the element owners
    /// (merged on first use) or the node owners. Example: `mesh.get_owner_maps(true)`.
    fn get_owner_maps(&self, is_element: bool) -> MeshVSOwnerMap {
        visibility::owner_maps(self, is_element)
    }
    /// `get_data_source`; `mesh.get_data_source()`.
    fn get_data_source(&self) -> Handle<dyn MeshVSDataSource> {
        self.mesh_base().fields.read().data_source.clone()
    }
    /// `set_data_source`; `mesh.set_data_source(&source)`.
    fn set_data_source(&self, data_source: &Handle<dyn MeshVSDataSource>) {
        self.mesh_base().fields.write().data_source = data_source.clone();
    }
    /// `get_drawer`; `mesh.get_drawer()`.
    fn get_drawer(&self) -> Handle<dyn MeshVSDrawerSource> {
        self.mesh_base().fields.read().current_drawer.clone()
    }
    /// `set_drawer`; `mesh.set_drawer(&drawer)`.
    fn set_drawer(&self, drawer: &Handle<dyn MeshVSDrawerSource>) {
        self.mesh_base().fields.write().current_drawer = drawer.clone();
    }
    /// `is_hidden_elem`: listed in the hidden elements; `mesh.is_hidden_elem(3)`.
    fn is_hidden_elem(&self, id: i32) -> bool {
        let hidden = self.get_hidden_elems();
        hidden.get().is_some_and(|map| map.map().contains(id))
    }
    /// `is_hidden_node`: listed in the hidden nodes; without that map every node is hidden.
    fn is_hidden_node(&self, id: i32) -> bool {
        let hidden = self.get_hidden_nodes();
        hidden.get().is_none_or(|map| map.map().contains(id))
    }
    /// `is_selectable_elem`: not hidden; `mesh.is_selectable_elem(3)`.
    fn is_selectable_elem(&self, id: i32) -> bool {
        !self.is_hidden_elem(id)
    }
    /// `is_selectable_node`: in the selectable nodes, else not hidden; `mesh.is_selectable_node(3)`.
    fn is_selectable_node(&self, id: i32) -> bool {
        let selectable = self.get_selectable_nodes();
        match selectable.get() {
            Some(map) => map.map().contains(id),
            None => !self.is_hidden_node(id),
        }
    }
    /// `get_hidden_nodes`; `mesh.get_hidden_nodes()`.
    fn get_hidden_nodes(&self) -> Handle<HPackedMapOfInteger> {
        self.mesh_base().fields.read().hidden_nodes.clone()
    }
    /// `set_hidden_nodes`: then the selectable nodes when the drawer asks `MeshVSDrawerAttribute::SelectableAuto`.
    fn set_hidden_nodes(&self, ids: &Handle<HPackedMapOfInteger>) {
        self.mesh_base().fields.write().hidden_nodes = ids.clone();
        visibility::auto_update_selectable(self);
    }
    /// `get_hidden_elems`; `mesh.get_hidden_elems()`.
    fn get_hidden_elems(&self) -> Handle<HPackedMapOfInteger> {
        self.mesh_base().fields.read().hidden_elements.clone()
    }
    /// `set_hidden_elems`: then the selectable nodes when the drawer asks `MeshVSDrawerAttribute::SelectableAuto`.
    fn set_hidden_elems(&self, ids: &Handle<HPackedMapOfInteger>) {
        self.mesh_base().fields.write().hidden_elements = ids.clone();
        visibility::auto_update_selectable(self);
    }
    /// `get_selectable_nodes`; `mesh.get_selectable_nodes()`.
    fn get_selectable_nodes(&self) -> Handle<HPackedMapOfInteger> {
        self.mesh_base().fields.read().selectable_nodes.clone()
    }
    /// `set_selectable_nodes`; `mesh.set_selectable_nodes(&ids)`.
    fn set_selectable_nodes(&self, ids: &Handle<HPackedMapOfInteger>) {
        self.mesh_base().fields.write().selectable_nodes = ids.clone();
    }
    /// `update_selectable_nodes`: the visible nodes and the nodes of the visible elements.
    fn update_selectable_nodes(&self) {
        visibility::update_selectable_nodes(self);
    }
    /// `get_mesh_sel_method`; `mesh.get_mesh_sel_method()`.
    fn get_mesh_sel_method(&self) -> MeshVSMeshSelectionMethod {
        self.mesh_base().fields.read().selection_method
    }
    /// `set_mesh_sel_method`; `mesh.set_mesh_sel_method(MeshVSMeshSelectionMethod::Box)`.
    fn set_mesh_sel_method(&self, method: MeshVSMeshSelectionMethod) {
        self.mesh_base().fields.write().selection_method = method;
    }
}
impl<T: MeshVSMeshSource + ?Sized> MeshVSMeshMembers for T {}

/// The `add_to_map` helper: binds or replaces every owner of `addition` in `result`.
pub(super) fn add_to_map(result: &mut MeshVSOwnerMap, addition: &MeshVSOwnerMap) {
    for (key, value) in addition.items() {
        match result.change_find(key) {
            Ok(slot) => *slot = value.clone(),
            Err(_) => {
                result.bind(*key, value.clone());
            }
        }
    }
}

/// A copy of an owner map (the const reference, read once).
pub(super) fn copy_map(map: &MeshVSOwnerMap) -> MeshVSOwnerMap {
    let mut copy = DataMap::default();
    copy.assign(map);
    copy
}
