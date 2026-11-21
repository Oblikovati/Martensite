//! Build the actual discrete BRep model after native bounding-box validation.
use super::{BRepMeshShapeVisitor, ShapeTool};
use crate::{
    brep_mesh_data::DiscreteModel,
    imesh_data::MeshModel,
    imesh_tools::{MeshParameters, ModelBuilder, ShapeExplorer},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    impl_transient,
    message::{AlgorithmSource, AlgorithmState, Status},
    standard::{Failure, Handle, RuntimeTyped},
};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
/// Concrete native model construction; `BRepMeshModelBuilder::new().perform(&shape, &parameters)`.
pub struct BRepMeshModelBuilder {
    algorithm: AlgorithmState,
}
impl_transient!(BRepMeshModelBuilder,"BRepMesh_ModelBuilder",dyn ModelBuilder,interfaces:[dyn ModelBuilder, dyn AlgorithmSource]);
impl Default for BRepMeshModelBuilder {
    fn default() -> Self {
        Self {
            algorithm: AlgorithmState::with_runtime_type(Self::static_type()),
        }
    }
}
impl BRepMeshModelBuilder {
    /// Empty algorithm state; `BRepMeshModelBuilder::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    fn model_size(bounds: &BoundingBox3, parameters: &MeshParameters) -> Result<f64, Failure> {
        let mut size = if parameters.deflection < parameters.deflection_interior {
            parameters.deflection_interior
        } else {
            parameters.deflection
        };
        if parameters.relative {
            ShapeTool::box_max_dimension(bounds, &mut size)?;
        }
        Ok(size)
    }
}
impl std::ops::Deref for BRepMeshModelBuilder {
    type Target = AlgorithmState;
    fn deref(&self) -> &Self::Target {
        &self.algorithm
    }
}
impl ModelBuilder for BRepMeshModelBuilder {
    fn perform_internal(
        &self,
        shape: &Shape,
        parameters: &MeshParameters,
    ) -> Result<Handle<dyn MeshModel>, Failure> {
        let mut bounds = BoundingBox3::default();
        BRepBndLib::add(shape, &mut bounds, false)?;
        if bounds.is_void() {
            self.set_status(Status::FAIL1);
            return Ok(Handle::default());
        }
        let model = Handle::new(DiscreteModel::new(shape));
        model.set_max_size(Self::model_size(&bounds, parameters)?);
        let visitor = Handle::new(BRepMeshShapeVisitor::new(&model.upcast()));
        ShapeExplorer::new(shape).accept(&visitor.into_upcast())?;
        self.set_status(Status::DONE1);
        Ok(model.into_upcast())
    }
}

impl AlgorithmSource for BRepMeshModelBuilder {
    fn algorithm_state(&self) -> &AlgorithmState {
        &self.algorithm
    }
}
