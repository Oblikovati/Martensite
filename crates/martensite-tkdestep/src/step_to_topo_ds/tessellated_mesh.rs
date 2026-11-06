//! `StepToTopoDSTranslateFace`'s `create_poly_triangulation`: a tessellated entity to a
//! `Triangulation`.
use crate::step_data::StepDataFactors;
use crate::step_entity::{step_array1_item, step_array1_len, step_array2_item, step_live};
use crate::step_visual::{
    StepVisualComplexTriangulatedFace, StepVisualComplexTriangulatedSurfaceSet,
    StepVisualTessellatedItem, StepVisualTriangulatedFace, StepVisualTriangulatedSurfaceSet,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{Direction3, Point3, Xyz};
use martensite_tkmath::poly::{Triangle, Triangulation};

const WHERE: &str = "StepToTopoDSTranslateFace::createMesh";

/// The elements a tessellated entity gives.
#[derive(Default)]
struct MeshElements {
    nodes: Handle<HArray1<Xyz>>,
    normals: Handle<HArray2<f64>>,
    triangles: Handle<HArray2<i32>>,
    nb_normals: i32,
    nb_triangles: i32,
    pn_indices: Option<Vec<i32>>,
    strips: Handle<HArray1<Handle<StandardTransient>>>,
    fans: Handle<HArray1<Handle<StandardTransient>>>,
    nb_strips: i32,
    nb_fans: i32,
}

/// The fields every tessellated entity has: `theFace->Coordinates()->Points()`, `normals`...
macro_rules! common_elements {
    ($face:expr) => {{
        let mut elements = MeshElements::default();
        elements.nodes = step_live(&$face.coordinates(), WHERE)?.points();
        elements.normals = $face.normals();
        elements.nb_normals = $face.nb_normals();
        let count = $face.nb_pnindex();
        if count > 0 {
            let mut indices = Vec::with_capacity(count as usize);
            for rank in 1..=count {
                indices.push($face.pnindex_value(rank)?);
            }
            elements.pn_indices = Some(indices);
        }
        elements
    }};
}

/// `GetSimpleFaceElements`.
macro_rules! simple_elements {
    ($face:expr) => {{
        let mut elements = common_elements!($face);
        elements.triangles = $face.triangles();
        elements.nb_triangles = $face.nb_triangles();
        elements
    }};
}

/// `GetComplexFaceElements`.
macro_rules! complex_elements {
    ($face:expr) => {{
        let mut elements = common_elements!($face);
        elements.strips = $face.triangle_strips();
        elements.fans = $face.triangle_fans();
        elements.nb_strips = $face.nb_triangle_strips();
        elements.nb_fans = $face.nb_triangle_fans();
        elements
    }};
}

/// The elements of the four triangulated classes; `None` for any other class.
fn elements_of(item: &Handle<StepVisualTessellatedItem>) -> Result<Option<MeshElements>, Failure> {
    if let Some(live) = item.cast::<StepVisualTriangulatedFace>().get() {
        return Ok(Some(simple_elements!(live)));
    }
    if let Some(live) = item.cast::<StepVisualTriangulatedSurfaceSet>().get() {
        return Ok(Some(simple_elements!(live)));
    }
    if let Some(live) = item.cast::<StepVisualComplexTriangulatedFace>().get() {
        return Ok(Some(complex_elements!(live)));
    }
    if let Some(live) = item.cast::<StepVisualComplexTriangulatedSurfaceSet>().get() {
        return Ok(Some(complex_elements!(live)));
    }
    Ok(None)
}

/// The indices of a strip or a fan, 1-based through `values[i - 1]`.
fn index_list(
    list: &Handle<HArray1<Handle<StandardTransient>>>,
    rank: i32,
) -> Result<Vec<i32>, Failure> {
    let item: Handle<HArray1<i32>> = step_array1_item(list, rank, WHERE)?.cast();
    let mut values = Vec::new();
    for index in 1..=step_array1_len(&item) {
        values.push(step_array1_item(&item, index, WHERE)?);
    }
    Ok(values)
}

/// The triangles of a strip, the odd then the even ones; the degenerate ones are left out.
fn strip_triangles(strip: &[i32]) -> Vec<Triangle> {
    let distinct = |i: usize| strip[i] != strip[i - 2] && strip[i] != strip[i - 1];
    let mut triangles = Vec::new();
    for index in (2..strip.len()).step_by(2) {
        if distinct(index) {
            triangles.push(Triangle::new(
                strip[index - 2],
                strip[index],
                strip[index - 1],
            ));
        }
    }
    for index in (3..strip.len()).step_by(2) {
        if distinct(index) {
            triangles.push(Triangle::new(
                strip[index - 2],
                strip[index - 1],
                strip[index],
            ));
        }
    }
    triangles
}

/// The triangles of a fan; the degenerate ones are left out.
fn fan_triangles(fan: &[i32]) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    for index in 2..fan.len() {
        if fan[index] != fan[index - 2] && fan[index - 1] != fan[index - 2] {
            triangles.push(Triangle::new(fan[0], fan[index], fan[index - 1]));
        }
    }
    triangles
}

/// The triangles made from the strips and fans. The count of a fan is `Length - 2`
/// whatever is degenerate, so a degenerate fan leaves the tail of the mesh's triangles unset.
fn complex_triangles(elements: &MeshElements) -> Result<(Vec<Triangle>, i32), Failure> {
    let mut triangles = Vec::new();
    let mut declared = 0;
    for rank in 1..=elements.nb_strips {
        let made = strip_triangles(&index_list(&elements.strips, rank)?);
        declared += made.len() as i32;
        triangles.extend(made);
    }
    for rank in 1..=elements.nb_fans {
        let fan = index_list(&elements.fans, rank)?;
        declared += fan.len() as i32 - 2;
        triangles.extend(fan_triangles(&fan));
    }
    Ok((triangles, declared))
}

/// `set_nodes`.
fn set_nodes(
    mesh: &Triangulation,
    elements: &MeshElements,
    length_factor: f64,
) -> Result<(), Failure> {
    for rank in 1..=mesh.nb_nodes() {
        let source = match &elements.pn_indices {
            Some(indices) => indices[(rank - 1) as usize],
            None => rank,
        };
        let point = step_array1_item(&elements.nodes, source, WHERE)?;
        mesh.set_node(
            rank,
            Point3::new(
                length_factor * point.x(),
                length_factor * point.y(),
                length_factor * point.z(),
            ),
        )?;
    }
    Ok(())
}

/// The normal of the row, as a direction: `set_normal` makes a `gp_Dir` of it.
fn normal_at(normals: &Handle<HArray2<f64>>, row: i32) -> Result<Direction3, Failure> {
    Direction3::new(
        step_array2_item(normals, row, 1, WHERE)?,
        step_array2_item(normals, row, 2, WHERE)?,
        step_array2_item(normals, row, 3, WHERE)?,
    )
}

/// `set_normals`: one normal for all the nodes, or one each, when the rows have three values.
fn set_normals(
    mesh: &Triangulation,
    elements: &MeshElements,
    nb_nodes: i32,
) -> Result<(), Failure> {
    let live = step_live(&elements.normals, WHERE)?;
    if live.array2().row_length() != 3 {
        return Ok(());
    }
    if elements.nb_normals == 1 {
        let normal = normal_at(&elements.normals, 1)?;
        for rank in 1..=nb_nodes {
            mesh.set_normal(rank, normal)?;
        }
    } else if nb_nodes == elements.nb_normals {
        for rank in 1..=elements.nb_normals {
            mesh.set_normal(rank, normal_at(&elements.normals, rank)?)?;
        }
    }
    Ok(())
}

/// `create_poly_triangulation`: null for a null entity or a class it does not know.
pub(super) fn create_poly_triangulation(
    item: &Handle<StepVisualTessellatedItem>,
    factors: &StepDataFactors,
) -> Result<Handle<Triangulation>, Failure> {
    if item.is_null() {
        return Ok(Handle::default());
    }
    let Some(elements) = elements_of(item)? else {
        return Ok(Handle::default());
    };
    let has_normals = elements.nb_normals > 0;
    let nb_nodes = match &elements.pn_indices {
        Some(indices) => indices.len() as i32,
        None => step_array1_len(&elements.nodes),
    };
    let simple = elements.nb_strips == 0 && elements.nb_fans == 0;
    let (made_triangles, nb_triangles) = if simple {
        (Vec::new(), elements.nb_triangles)
    } else {
        complex_triangles(&elements)?
    };
    let mesh = Triangulation::with_sizes(nb_nodes, nb_triangles, false, has_normals)?;
    set_nodes(&mesh, &elements, factors.length_factor())?;
    if has_normals {
        set_normals(&mesh, &elements, nb_nodes)?;
    }
    if simple {
        for rank in 1..=nb_triangles {
            let corner = |column| step_array2_item(&elements.triangles, rank, column, WHERE);
            mesh.set_triangle(rank, Triangle::new(corner(1)?, corner(2)?, corner(3)?))?;
        }
    } else {
        for (index, triangle) in made_triangles.into_iter().enumerate() {
            mesh.set_triangle(index as i32 + 1, triangle)?;
        }
    }
    Ok(Handle::new(mesh))
}
