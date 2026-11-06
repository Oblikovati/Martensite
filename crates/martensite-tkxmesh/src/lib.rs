// SPDX-License-Identifier: GPL-2.0-only
//! The meshing factory named "XBRepMesh": an alternative entry to the incremental mesher in the
//! registry of named meshing algorithms.
//!
//! Rust runs nothing when a library loads, so an application asks for the registration with
//! [`XBRepMeshFactory::new`]. The registry's own default stays first, because this factory does
//! not ask to be preferred.
//!
//! ```
//! use martensite_tkmesh::brep_mesh::find_factory;
//! use martensite_tkxmesh::XBRepMeshFactory;
//! let _factory = XBRepMeshFactory::new();
//! assert!(!find_factory("XBRepMesh").is_null());
//! ```
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkmesh::brep_mesh::{
    BRepMeshDiscretAlgoFactory, BRepMeshDiscretRoot, BRepMeshIncrementalMesh, register_factory,
};

/// The name this factory registers under, compared without case by the registry.
pub const XBREPMESH_FACTORY_NAME: &str = "XBRepMesh";

/// A meshing factory making incremental meshers; `XBRepMeshFactory::new()`.
#[derive(Default)]
pub struct XBRepMeshFactory {
    transient: StandardTransient,
}
impl_transient!(XBRepMeshFactory, "XBRepMeshFactory", dyn BRepMeshDiscretAlgoFactory, transient,
    interfaces: [dyn BRepMeshDiscretAlgoFactory]);

impl XBRepMeshFactory {
    /// A factory registered behind the existing ones; a factory of this name already registered
    /// keeps its place. Example: `let factory = XBRepMeshFactory::new();`.
    pub fn new() -> Handle<Self> {
        let factory = Handle::new(Self::default());
        register_factory(&factory.clone().cast(), false);
        factory
    }
}

impl BRepMeshDiscretAlgoFactory for XBRepMeshFactory {
    fn name(&self) -> &str {
        XBREPMESH_FACTORY_NAME
    }

    /// An incremental mesher with these deflections and a shape, not performed.
    /// Unlike the registry's default factory it leaves the parallel flag at the parameters'
    /// default instead of the process-wide one.
    fn create_algorithm(
        &self,
        shape: &Shape,
        lin_deflection: f64,
        ang_deflection: f64,
    ) -> Handle<dyn BRepMeshDiscretRoot> {
        let algorithm = BRepMeshIncrementalMesh::new();
        {
            let mut parameters = algorithm.change_parameters();
            parameters.deflection = lin_deflection;
            parameters.angle = ang_deflection;
        }
        algorithm.set_shape(shape);
        Handle::new(algorithm).cast()
    }
}
