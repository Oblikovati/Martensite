//! The tessellated entities of one triangulation, and the items a solid gathers from its shells.
use super::TopoDSToStepTool;
use crate::geom_to_step::{empty_step_name, shared_array1};
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::{step_array2_set, step_live, step_new_array1, step_new_array2};
use crate::step_shape::StepShapeManifoldSolidBrep;
use crate::step_visual::{
    StepVisualCoordinatesList, StepVisualFaceOrSurface, StepVisualTessellatedItem,
    StepVisualTessellatedShell, StepVisualTessellatedSolid, StepVisualTessellatedStructuredItem,
    StepVisualTriangulatedFace, StepVisualTriangulatedSurfaceSet,
};
use martensite_tkbrep::topods::Face;
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;
use martensite_tkmath::poly::{self, Triangulation};

const WHERE: &str = "TopoDSToStepMakeTessellatedItem";

/// The fields of a tessellated entity, from `init_triangulation`.
struct TriangulationFields {
    coordinates: Handle<StepVisualCoordinatesList>,
    normals: Handle<HArray2<f64>>,
    indices: Handle<HArray1<i32>>,
    triangles: Handle<HArray2<i32>>,
    nb_nodes: i32,
}

/// `init_triangulation`: get parameters from a `Triangulation`.
fn triangulation_fields(
    mesh: &Triangulation,
    factors: &StepDataFactors,
) -> Result<TriangulationFields, Failure> {
    let nb_nodes = mesh.nb_nodes();
    let factor = factors.length_factor();
    let mut points = Vec::new();
    let mut indices = Vec::new();
    for index in 1..=nb_nodes {
        points.push(mesh.node(index)?.xyz() / factor);
        indices.push(index);
    }
    let coordinates = Handle::new(StepVisualCoordinatesList::new());
    coordinates.init(&empty_step_name(), &shared_array1::<Xyz>(points));
    if !mesh.has_normals() {
        poly::compute_normals(mesh)?;
    }
    let normals = step_new_array2::<f64>(1, nb_nodes, 1, 3)?;
    for index in 1..=nb_nodes {
        let normal = mesh.normal(index)?;
        step_array2_set(&normals, index, 1, normal.x(), WHERE)?;
        step_array2_set(&normals, index, 2, normal.y(), WHERE)?;
        step_array2_set(&normals, index, 3, normal.z(), WHERE)?;
    }
    let triangles = step_new_array2::<i32>(1, mesh.nb_triangles(), 1, 3)?;
    for index in 1..=mesh.nb_triangles() {
        let triangle = mesh.triangle(index)?;
        for corner in 1..=3 {
            step_array2_set(&triangles, index, corner, triangle.value(corner)?, WHERE)?;
        }
    }
    Ok(TriangulationFields {
        coordinates,
        normals,
        indices: shared_array1(indices),
        triangles,
        nb_nodes,
    })
}

/// The triangulated_face on the fields, linked to its geometry when given.
fn triangulated_face(
    fields: &TriangulationFields,
    has_link: bool,
    link: &StepVisualFaceOrSurface,
) -> Handle<StepVisualTessellatedItem> {
    let entity = Handle::new(StepVisualTriangulatedFace::new());
    entity.init(
        &empty_step_name(),
        &fields.coordinates,
        fields.nb_nodes,
        &fields.normals,
        has_link,
        link,
        &fields.indices,
        &fields.triangles,
    );
    entity.cast()
}

/// The item of a face's triangulation.
pub(super) fn face_triangulation_item(
    mesh: &Handle<Triangulation>,
    face: &Face,
    tool: &TopoDSToStepTool,
    to_prefer_surface_set: bool,
    factors: &StepDataFactors,
) -> Result<Handle<StepVisualTessellatedItem>, Failure> {
    let fields = triangulation_fields(step_live(mesh, WHERE)?, factors)?;
    let mut link = StepVisualFaceOrSurface::default();
    if tool.is_bound(face) {
        link.set_value(&tool.find(face)?.cast())?;
        return Ok(triangulated_face(&fields, true, &link));
    }
    if !to_prefer_surface_set {
        return Ok(triangulated_face(&fields, false, &link));
    }
    let entity = Handle::new(StepVisualTriangulatedSurfaceSet::new());
    entity.init(
        &empty_step_name(),
        &fields.coordinates,
        fields.nb_nodes,
        &fields.normals,
        &fields.indices,
        &fields.triangles,
    );
    Ok(entity.cast())
}

/// The items of the tessellated_shells a solid gathers, in their order; a null item is the recorded
/// null dereference.
fn items_of_shells(
    shells: &[Handle<StepVisualTessellatedItem>],
) -> Result<Vec<Handle<StepVisualTessellatedStructuredItem>>, Failure> {
    let mut items = Vec::new();
    for shell in shells {
        let cast: Handle<StepVisualTessellatedShell> = shell.cast();
        let live = step_live(&cast, WHERE)?;
        for rank in 1..=live.nb_items() {
            items.push(live.items_value(rank)?);
        }
    }
    Ok(items)
}

/// The tessellated_solid of a solid's shells, linked to its brep when there is one.
pub(super) fn tessellated_solid(
    shells: &[Handle<StepVisualTessellatedItem>],
    brep: &Handle<StepShapeManifoldSolidBrep>,
) -> Result<Handle<StepVisualTessellatedItem>, Failure> {
    let solid = Handle::new(StepVisualTessellatedSolid::new());
    let items = items_of_shells(shells)?;
    let array = if items.is_empty() {
        step_new_array1::<Handle<StepVisualTessellatedStructuredItem>>(1, 0)?
    } else {
        shared_array1(items)
    };
    solid.init(&empty_step_name(), &array, brep.is_some(), brep);
    Ok(solid.cast())
}
