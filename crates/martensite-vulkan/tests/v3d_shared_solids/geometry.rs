use super::*;
use crate::primitive_shapes::box_sized;
use martensite_tkbrep::topods::ShapeIterator;

pub(super) struct SharedSolidsSource {
    pub boxes: [Shape; 2],
    pub result: Shape,
    pub parts: [Shape; 2],
}

pub(super) fn actual_shared_solids() -> Result<SharedSolidsSource, BoxError> {
    let boxes = [box_sized([20., 10., 10.])?, box_sized([10.; 3])?];
    let result = crate::common_operation::general_fuse(&boxes[0], &boxes[1]);
    let members: Vec<Shape> = ShapeIterator::new(&result, true, true).collect();
    let parts: [Shape; 2] = members.try_into().map_err(|members: Vec<Shape>| {
        format!("explode r count={}; expected r_1/r_2", members.len())
    })?;
    Ok(SharedSolidsSource {
        boxes,
        result,
        parts,
    })
}

pub(super) fn sequential_mesh_checks(parts: &[Shape; 2]) -> Result<(), BoxError> {
    mesh_shape(&parts[0], &MeshRequest::new(0.1))?;
    crate::mesh_topology_check::report_mesh_validity(&parts[0])?;
    mesh_shape(&parts[1], &MeshRequest::new(0.1))?;
    crate::mesh_topology_check::report_mesh_validity(&parts[1])?;
    crate::mesh_topology_check::report_mesh_validity(&parts[0])?;
    Ok(())
}
