//! Face triangulation strategies share generators and overridable meshing stages.
//!
//! [`BaseMeshAlgoHooks`], [`ConstrainedBaseMeshAlgoHooks`] and [`NodeInsertionHooks`]
//! define those stages, while [`MeshGenerator`] coordinates their execution.
//! [`BRepMeshDelaunayBaseMeshAlgo`] and [`BRepMeshDelabellaBaseMeshAlgo`] expose
//! the selected runtime strategy through a handle.
mod algo_object;
mod base;
mod collect;
mod deflection_control;
mod deflection_split;
mod delabella_base;
mod delabella_factory;
mod delaunay_insertion;
mod factory;
mod generator;
mod node_insertion;

pub use algo_object::{
    BRepMeshDelaunayBaseMeshAlgo, BaseMeshAlgo, ConstrainedBaseMeshAlgo, CustomBaseMeshAlgo,
    FaceMeshingCore,
};
pub use base::{
    BaseMeshAlgoData, BaseMeshAlgoHooks, ConstrainedBaseMeshAlgoHooks, base_init_data_structure,
    base_perform, base_register_node,
};
pub use deflection_control::DelaunayDeflectionControlMeshAlgo;
pub use delabella_base::{
    BRepMeshDelabellaBaseMeshAlgo, DelabellaBaseMeshAlgoCore, DelabellaTriangulation,
};
pub use delabella_factory::BRepMeshDelabellaMeshAlgoFactory;
pub use delaunay_insertion::{
    DelaunayInsertionHooks, DelaunayNodeInsertionMeshAlgo, delaunay_insertion_cells_count,
    delaunay_insertion_init_data_structure, delaunay_insertion_post_process, insert_nodes,
};
pub use factory::BRepMeshMeshAlgoFactory;
pub use generator::{
    CustomBaseTriangulation, CustomDelaunayGenerator, CustomGenerator, DelaunayBaseMeshAlgoCore,
    DelaunayGenerator, MeshGenerator,
};
pub use node_insertion::{
    NodeInsertionData, NodeInsertionHooks, NodeInsertionMeshAlgo,
    node_insertion_add_node_to_structure, node_insertion_init_data_structure,
    node_insertion_perform, node_insertion_point2d,
};
