use super::*;
use crate::primitive_shapes::{box_at, compound_of, sphere, translated};
use crate::shape_mesh::{MeshRequest, mesh_shape};
#[path = "../v3d_state_curve_cases/mesh_information.rs"]
mod mesh_information;

pub(super) fn grid_boxes() -> Result<Vec<Shape>, Failure> {
    let mut boxes = Vec::with_capacity(900);
    for i in 0..30 {
        for j in 0..30 {
            boxes.push(box_at([f64::from(i * 6), f64::from(j * 6), 0.], [5.; 3])?);
        }
    }
    Ok(boxes)
}

pub(super) fn source_scene_compound() -> Result<Shape, BoxError> {
    let mut boxes = grid_boxes()?;
    let sphere = translated(&sphere(30.)?, [90., 90., 0.])?;
    mesh_shape(&sphere, &MeshRequest::new(0.002))?;
    mesh_information::print_information(&sphere)?;
    boxes.push(sphere);
    Ok(compound_of(&boxes)?)
}

pub(super) fn actual_self_overlap_faces(shape: &Shape) -> Result<Shape, BoxError> {
    let start = std::time::Instant::now();
    let mut tool = BRepExtremaSelfIntersection::with_shape(shape, 0.)?;
    let initialization = start.elapsed();
    let start = std::time::Instant::now();
    tool.perform()?;
    let work = start.elapsed();
    if !tool.is_done() {
        return Err("selfintersect done=false; expected completed source shape analysis".into());
    }
    println!(
        "Building data structure (BVH):    {}",
        initialization.as_secs_f64()
    );
    println!("Executing self-intersection test: {}", work.as_secs_f64());
    overlap_face_compound(&tool)
}

pub(super) fn overlap_face_compound(tool: &BRepExtremaSelfIntersection) -> Result<Shape, BoxError> {
    let mut faces = Vec::new();
    for (index, _) in tool.overlap_elements().items() {
        faces.push(tool.get_sub_shape(*index)?.into_shape());
        println!("C_{} ", index + 1);
    }
    println!("Compound of overlapped sub-faces: C_overlapped");
    Ok(compound_of(&faces)?)
}
