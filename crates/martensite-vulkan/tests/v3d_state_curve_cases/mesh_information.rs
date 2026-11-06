//! Query every ordinary `trinfo` field from the actual shape, including edge representations.
use super::*;
use crate::shape_mesh::TriangulationSummary;
use martensite_tkbrep::{
    brep::{BRepTEdge, BRepTool},
    topexp::{self, Explorer},
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::mesh_purpose;

pub(super) struct MeshInformation {
    pub summary: TriangulationSummary,
    pub polygon_representations: usize,
    pub parameters: [f64; 3],
}
fn polygon_representations(shape: &Shape) -> Result<usize, Failure> {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges)?;
    Ok(edges
        .iter()
        .map(|edge| {
            edge.tshape()
                .cast::<BRepTEdge>()
                .curves()
                .iter()
                .filter(|curve| curve.is_polygon_on_triangulation())
                .count()
        })
        .sum())
}
fn face_parameters(face: &topods::Face) -> Result<Option<[f64; 3]>, Failure> {
    let (mesh, _) = BRepTool::triangulation(face, mesh_purpose::NONE)?;
    let Some(mesh) = mesh.get() else {
        return Ok(None);
    };
    let handle = mesh.parameters();
    let Some(params) = handle.get() else {
        return Ok(None);
    };
    Ok(Some([
        params.deflection(),
        params.angle(),
        params.min_size(),
    ]))
}
fn merge_parameters(values: &mut [f64; 3], current: [f64; 3]) {
    for (value, current) in values.iter_mut().zip(current) {
        *value = value.max(current);
    }
}
fn meshing_parameters(shape: &Shape) -> Result<[f64; 3], Failure> {
    let mut values = [-1_f64; 3];
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while faces.more() {
        if let Some(current) = face_parameters(&topods::face(faces.current())?)? {
            merge_parameters(&mut values, current);
        }
        faces.next()?;
    }
    Ok(values)
}
pub(super) fn information(shape: &Shape) -> Result<MeshInformation, Failure> {
    Ok(MeshInformation {
        summary: TriangulationSummary::of(shape)?,
        polygon_representations: polygon_representations(shape)?,
        parameters: meshing_parameters(shape)?,
    })
}
const PARAMETER_LABELS: [&str; 3] = [
    "Meshing deflection",
    "Meshing angular deflection",
    "Meshing min size",
];
fn print_parameters(values: [f64; 3]) {
    let converted = [
        values[0],
        values[1] * 180. / std::f64::consts::PI,
        values[2],
    ];
    for (name, value) in PARAMETER_LABELS.into_iter().zip(converted) {
        if value > 0. {
            println!("{name} {}", format_general(value, 17));
        }
    }
}
pub(super) fn print_information(shape: &Shape) -> Result<(), Failure> {
    let info = information(shape)?;
    let mesh = info.summary;
    println!("\nThis shape contains {} faces.", mesh.faces);
    if mesh.empty_faces > 0 {
        println!("                    {} empty faces.", mesh.empty_faces);
    }
    println!("                    {} triangles.", mesh.triangles);
    println!("                    {} nodes.", mesh.nodes);
    println!(
        "                    {} polygons on triangulation.",
        info.polygon_representations
    );
    println!(
        "Maximal deflection {}",
        format_general(mesh.max_deflection, 17)
    );
    print_parameters(info.parameters);
    Ok(())
}
