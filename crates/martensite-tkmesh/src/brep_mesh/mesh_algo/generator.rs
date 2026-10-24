//! Build constrained connectivity before collecting a face triangulation.
use super::base::{BaseMeshAlgoData, BaseMeshAlgoHooks, ConstrainedBaseMeshAlgoHooks};
use crate::brep_mesh::{BRepMeshDelaun, BRepMeshMeshTool};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use std::marker::PhantomData;

/// The `BaseAlgo` template argument of the node-insertion algorithms: how the base
/// triangulation is generated (`generateMesh`) and what the base `postProcessMesh`
/// does.
pub trait MeshGenerator {
    /// `MeshGenerator::generate_mesh` on the full algorithm.
    fn generate_mesh<A: ConstrainedBaseMeshAlgoHooks + ?Sized>(
        algo: &mut A,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure>;
    /// `MeshGenerator::post_process_mesh`: nothing for the constrained base.
    fn post_process_mesh<A: ConstrainedBaseMeshAlgoHooks + ?Sized>(
        _algo: &mut A,
        _mesher: &mut BRepMeshDelaun,
        _range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

/// `BRepMeshDelaunayBaseMeshAlgo` as a generator: the Watson triangulation of every
/// registered node in index order.
pub struct DelaunayGenerator;

impl MeshGenerator for DelaunayGenerator {
    /// Generate triangulation from the registered boundary constraints.
    fn generate_mesh<A: ConstrainedBaseMeshAlgoHooks + ?Sized>(
        algo: &mut A,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let structure = algo.base_data().get_structure().clone();
        let count = algo.base_data().get_nodes_map().len() as i32;
        let mut order: Vec<i32> = (1..=count).collect();
        let cells = algo.get_cells_count(order.len() as i32)?;
        let mut mesher = BRepMeshDelaun::with_indices(&structure, &mut order, cells)?;
        BRepMeshMeshTool::new(&structure).erase_free_links()?;
        if !range.more() {
            return Ok(());
        }
        algo.post_process_mesh(&mut mesher, range)
    }
}

/// `CustomBaseTriangulation::build_base_triangulation`: a custom
/// triangulation of the registered nodes into the structure.
pub trait CustomBaseTriangulation {
    /// Builds the base triangulation; `T::build_base_triangulation(&mut algo)?`.
    fn build_base_triangulation<A: ConstrainedBaseMeshAlgoHooks + ?Sized>(
        algo: &mut A,
    ) -> Result<(), Failure>;
}

/// `CustomBaseMeshAlgo` as a generator over a custom base triangulation.
pub struct CustomGenerator<T: CustomBaseTriangulation>(PhantomData<T>);

impl<T: CustomBaseTriangulation> MeshGenerator for CustomGenerator<T> {
    /// `MeshGenerator::generate_mesh`: the custom triangulation, the
    /// constraints, the removal of the auxiliary nodes it added and of free links.
    fn generate_mesh<A: ConstrainedBaseMeshAlgoHooks + ?Sized>(
        algo: &mut A,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let structure = algo.base_data().get_structure().clone();
        let nodes_nb = structure.nb_nodes();
        T::build_base_triangulation(algo)?;
        let cells = algo.get_cells_count(structure.nb_nodes())?;
        let mut mesher = BRepMeshDelaun::from_mesh(&structure, cells.0, cells.1, false)?;
        let new_nodes_nb = structure.nb_nodes();
        let remove_aux = new_nodes_nb > nodes_nb;
        if remove_aux {
            let mut aux = martensite_tkernel::ncollection::DynamicArray::with_increment(
                (new_nodes_nb - nodes_nb) as usize,
            );
            for node in nodes_nb + 1..=new_nodes_nb {
                aux.append(node);
            }
            mesher.set_aux_vertices(&aux);
        }
        mesher.process_constraints()?;
        if remove_aux {
            mesher.remove_aux_elements()?;
        }
        BRepMeshMeshTool::new(&structure).erase_free_links()?;
        algo.post_process_mesh(&mut mesher, range)
    }
}

/// Fill the mesher's circle tool before the configured postprocessing stage.
pub struct CustomDelaunayGenerator<G: MeshGenerator>(PhantomData<G>);

impl<G: MeshGenerator> MeshGenerator for CustomDelaunayGenerator<G> {
    fn generate_mesh<A: ConstrainedBaseMeshAlgoHooks + ?Sized>(
        algo: &mut A,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        G::generate_mesh(algo, range)
    }
    /// Initialize circle bounds before delegating mesh postprocessing.
    fn post_process_mesh<A: ConstrainedBaseMeshAlgoHooks + ?Sized>(
        algo: &mut A,
        mesher: &mut BRepMeshDelaun,
        range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let nodes_nb = algo.base_data().get_structure().nb_nodes();
        let cells = algo.get_cells_count(nodes_nb)?;
        mesher.init_circles_tool(cells.0, cells.1)?;
        G::post_process_mesh(algo, mesher, range)
    }
}

/// `BRepMeshDelaunayBaseMeshAlgo`: the plain Watson algorithm (planes and cylinders
/// without internal vertices); `DelaunayBaseMeshAlgoCore::default()`.
#[derive(Default)]
pub struct DelaunayBaseMeshAlgoCore {
    data: BaseMeshAlgoData,
}

impl BaseMeshAlgoHooks for DelaunayBaseMeshAlgoCore {
    fn base_data(&self) -> &BaseMeshAlgoData {
        &self.data
    }
    fn base_data_mut(&mut self) -> &mut BaseMeshAlgoData {
        &mut self.data
    }
    fn generate_mesh(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        DelaunayGenerator::generate_mesh(self, range)
    }
}

impl ConstrainedBaseMeshAlgoHooks for DelaunayBaseMeshAlgoCore {}
