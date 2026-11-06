// SPDX-License-Identifier: GPL-2.0-only
//! `MeshVSMesh`: the interactive object of a mesh data source, drawn by its
//! presentation builders. The overridable members are the [`MeshVSMeshSource`] trait over the
//! embedded [`MeshVSMeshBase`]; the bodies of its AIS overrides are the
//! `meshvs_mesh_*` functions, which a derived mesh may call as `MeshVSMesh::member`.
//!
//! ```
//! use martensite_tkmeshvs::{MeshVSMesh, MeshVSMeshMembers};
//! use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
//! let mesh = MeshVSMesh::new(false).unwrap();
//! assert_eq!(mesh.display_mode(), 1);
//! assert_eq!(mesh.get_builders_count(), 0);
//! assert!(mesh.is_hidden_node(1));
//! ```
mod builders;
mod compute;
mod defaults;
mod highlight;
mod members;
mod selection;
mod selection_elements;
mod selection_whole;
mod visibility;

pub use compute::{meshvs_mesh_accept_display_mode, meshvs_mesh_compute};
pub use highlight::{
    meshvs_mesh_clear_selected, meshvs_mesh_hilight_owner_with_color, meshvs_mesh_hilight_selected,
};
pub use members::MeshVSMeshMembers;
pub use selection::meshvs_mesh_compute_selection;
pub use visibility::scan_faces_for_shared_nodes;

use crate::data_source::MeshVSDataSource;
use crate::drawer::MeshVSDrawerSource;
use crate::prs_builder::MeshVSPrsBuilder;
use crate::selection_policy::MeshVSMeshSelectionMethod;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{
    Failure, Handle, HandleReference, SharedState, StandardTransient,
};
use martensite_tkernel::tcol_std::HPackedMapOfInteger;
use martensite_tkservice::graphic3d::Graphic3dStructureSource;
use martensite_tkv3d::ais::{AisInteractiveObject, AisInteractiveObjectBase};
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject, PrsMgrPresentableObjectBase, PrsMgrPresentationManager,
    PrsMgrTypeOfPresentation3d, prs_mgr_presentable_bind,
};
use martensite_tkv3d::prs3d::Prs3dDrawer;
use martensite_tkv3d::select_mgr::{
    SelectMgrEntityOwnerSource, SelectMgrSelectableObject, SelectMgrSelectableObjectBase,
    SelectMgrSelection,
};

/// The `DataMap<int, Handle<SelectMgrEntityOwner>>` of owners.
pub type MeshVSOwnerMap = DataMap<i32, Handle<dyn SelectMgrEntityOwnerSource>>;

/// The fields of `MeshVSMesh`: the protected owner maps and drawers, then the private ones.
pub struct MeshVSMeshFields {
    /// The node owners.
    pub node_owners: MeshVSOwnerMap,
    /// The element owners.
    pub element_owners: MeshVSOwnerMap,
    /// `my0DOwners`.
    pub owners_0d: MeshVSOwnerMap,
    /// The link owners.
    pub link_owners: MeshVSOwnerMap,
    /// The face owners.
    pub face_owners: MeshVSOwnerMap,
    /// The volume owners.
    pub volume_owners: MeshVSOwnerMap,
    /// The group owners.
    pub group_owners: MeshVSOwnerMap,
    /// The mesh owners.
    pub mesh_owners: MeshVSOwnerMap,
    /// The current drawer.
    pub current_drawer: Handle<dyn MeshVSDrawerSource>,
    /// The selection drawer.
    pub selection_drawer: Handle<dyn MeshVSDrawerSource>,
    /// The highlight drawer.
    pub hilight_drawer: Handle<dyn MeshVSDrawerSource>,
    /// The whole mesh owner.
    pub whole_mesh_owner: Handle<dyn SelectMgrEntityOwnerSource>,
    /// `builders`, ordered by decreasing priority.
    pub builders: Vec<Handle<dyn MeshVSPrsBuilder>>,
    /// The highlighter.
    pub hilighter: Handle<dyn MeshVSPrsBuilder>,
    /// The hidden elements.
    pub hidden_elements: Handle<HPackedMapOfInteger>,
    /// The hidden nodes.
    pub hidden_nodes: Handle<HPackedMapOfInteger>,
    /// The selectable nodes.
    pub selectable_nodes: Handle<HPackedMapOfInteger>,
    /// The data source.
    pub data_source: Handle<dyn MeshVSDataSource>,
    /// The selection method.
    pub selection_method: MeshVSMeshSelectionMethod,
}

impl MeshVSMeshFields {
    /// The constructor state: no owner, drawer, builder, hidden map nor data source.
    fn empty() -> Self {
        Self {
            node_owners: DataMap::default(),
            element_owners: DataMap::default(),
            owners_0d: DataMap::default(),
            link_owners: DataMap::default(),
            face_owners: DataMap::default(),
            volume_owners: DataMap::default(),
            group_owners: DataMap::default(),
            mesh_owners: DataMap::default(),
            current_drawer: Handle::default(),
            selection_drawer: Handle::default(),
            hilight_drawer: Handle::default(),
            whole_mesh_owner: Handle::default(),
            builders: Vec::new(),
            hilighter: Handle::default(),
            hidden_elements: Handle::default(),
            hidden_nodes: Handle::default(),
            selectable_nodes: Handle::default(),
            data_source: Handle::default(),
            selection_method: MeshVSMeshSelectionMethod::Nodes,
        }
    }
}

/// The state every mesh embeds: the AIS base and the mesh fields.
pub struct MeshVSMeshBase {
    /// The inherited interactive state.
    pub interactive: AisInteractiveObjectBase,
    /// The mesh fields; `base.fields.read().builders.len()`.
    pub fields: SharedState<MeshVSMeshFields>,
}

impl MeshVSMeshBase {
    /// The AIS base of every view and the default selection method `MeshVSMeshSelectionMethod::Nodes`; the
    /// drawers and the appearance come from [`meshvs_mesh_initialize`] once the object exists.
    /// Example: `MeshVSMeshBase::new()`.
    pub fn new() -> Self {
        Self {
            interactive: AisInteractiveObjectBase::new(PrsMgrTypeOfPresentation3d::AllView),
            fields: SharedState::new(MeshVSMeshFields::empty()),
        }
    }
}

impl Default for MeshVSMeshBase {
    fn default() -> Self {
        Self::new()
    }
}

pub use defaults::meshvs_mesh_initialize;

/// A mesh interactive object; the default bodies are the standard behavior.
pub trait MeshVSMeshSource: AisInteractiveObject {
    /// The embedded base; `mesh.mesh_base()`.
    fn mesh_base(&self) -> &MeshVSMeshBase;
    /// `is_whole_mesh_owner`: the owner is the whole-mesh owner of the mode `MESHVS_SMF_MESH`.
    fn is_whole_mesh_owner(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> bool {
        let whole = self
            .mesh_base()
            .fields
            .read()
            .whole_mesh_owner
            .object_address();
        !owner.is_null() && owner.object_address() == whole
    }
}
martensite_tkernel::impl_transient_interface!(
    dyn MeshVSMeshSource,
    "MeshVSMesh",
    dyn AisInteractiveObject
);

/// The concrete `MeshVSMesh`; `MeshVSMesh::new(false)?`.
pub struct MeshVSMesh {
    transient: StandardTransient,
    base: MeshVSMeshBase,
}
martensite_tkernel::impl_transient!(MeshVSMesh, "MeshVSMesh", dyn AisInteractiveObject, transient,
    interfaces: [dyn MeshVSMeshSource, dyn AisInteractiveObject, dyn SelectMgrSelectableObject, dyn PrsMgrPresentableObject]);

impl MeshVSMesh {
    /// The drawers and appearance.
    /// Example: `MeshVSMesh::new(false)?`.
    pub fn new(allow_overlapped: bool) -> Result<Handle<Self>, Failure> {
        let mesh = Handle::new(Self {
            transient: StandardTransient::default(),
            base: MeshVSMeshBase::new(),
        });
        prs_mgr_presentable_bind(&mesh.clone().cast());
        meshvs_mesh_initialize(&*mesh, allow_overlapped)?;
        Ok(mesh)
    }
}

impl MeshVSMeshSource for MeshVSMesh {
    fn mesh_base(&self) -> &MeshVSMeshBase {
        &self.base
    }
}

impl AisInteractiveObject for MeshVSMesh {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base.interactive
    }
}

impl PrsMgrPresentableObject for MeshVSMesh {
    fn presentable_base(&self) -> &PrsMgrPresentableObjectBase {
        &self.base.interactive.presentable
    }
    fn compute(
        &self,
        manager: &dyn HandleReference<PrsMgrPresentationManager>,
        prs: &dyn HandleReference<dyn Graphic3dStructureSource>,
        mode: i32,
    ) -> Result<(), Failure> {
        meshvs_mesh_compute(self, &manager.observe(), &prs.observe(), mode)
    }
    fn accept_display_mode(&self, mode: i32) -> bool {
        meshvs_mesh_accept_display_mode(self, mode)
    }
    martensite_tkv3d::ais_presentable_overrides!();
}

impl SelectMgrSelectableObject for MeshVSMesh {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        &self.base.interactive.selectable
    }
    fn compute_selection(
        &self,
        selection: &dyn HandleReference<SelectMgrSelection>,
        mode: i32,
    ) -> Result<(), Failure> {
        meshvs_mesh_compute_selection(self, &selection.observe(), mode)
    }
    fn hilight_selected(
        &self,
        manager: &Handle<PrsMgrPresentationManager>,
        owners: &[Handle<dyn SelectMgrEntityOwnerSource>],
    ) -> Result<(), Failure> {
        meshvs_mesh_hilight_selected(self, manager, owners)
    }
    fn hilight_owner_with_color(
        &self,
        manager: &Handle<PrsMgrPresentationManager>,
        style: &Handle<Prs3dDrawer>,
        owner: &Handle<dyn SelectMgrEntityOwnerSource>,
    ) -> Result<(), Failure> {
        meshvs_mesh_hilight_owner_with_color(self, manager, style, owner)
    }
    fn clear_selected(&self) {
        meshvs_mesh_clear_selected(self);
    }
}
