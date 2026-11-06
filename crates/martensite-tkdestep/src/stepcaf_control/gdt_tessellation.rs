//! The tessellation of `STEPCAFControlGDTProperty::get_tessellation`.
use crate::step_entity::{
    Array1, DynamicArray, HSequence, ObjectHandle, step_array1_set, step_array2_set, step_live,
    step_new_array1, step_new_array2,
};
use crate::step_visual::{
    StepVisualComplexTriangulatedSurfaceSet, StepVisualCoordinatesList,
    StepVisualTessellatedCurveSet, StepVisualTessellatedGeometricSet, StepVisualTessellatedItem,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomLine};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Transform3, Xyz};
use martensite_tkmath::poly::{Triangulation, mesh_purpose};
use martensite_tkshhealing::shape_construct::ShapeConstructCurve;

const WHERE: &str = "STEPCAFControlGDTProperty::GetTessellation";

fn empty_name() -> Handle<HAsciiString> {
    Handle::new(HAsciiString::default())
}

/// `GenerateCoordinateList`: the nodes of a triangulation, transformed.
fn coordinate_list(
    triangulation: &Triangulation,
    transformation: &Transform3,
) -> Result<Handle<StepVisualCoordinatesList>, Failure> {
    let points = step_new_array1::<Xyz>(1, triangulation.nb_nodes())?;
    for index in 1..=triangulation.nb_nodes() {
        step_array1_set(
            &points,
            index,
            triangulation
                .node(index)?
                .transformed(*transformation)
                .xyz(),
            WHERE,
        )?;
    }
    let list = Handle::new(StepVisualCoordinatesList::new());
    list.init(&empty_name(), &points);
    Ok(list)
}

/// `count_normals`: 0 without normals, 1 when they are all equal, else one per node. A triangulation
/// holds no normals or one per node, so one normal in a file was read as one on every node.
fn count_normals(triangulation: &Triangulation) -> Result<i32, Failure> {
    if !triangulation.has_normals() {
        return Ok(0);
    }
    let equal = |first: f64, second: f64| (first - second).abs() < precision::confusion();
    let reference = triangulation.normal(1)?;
    for index in 1..=triangulation.nb_nodes() {
        let current = triangulation.normal(index)?;
        if !equal(reference.x(), current.x())
            || !equal(reference.y(), current.y())
            || !equal(reference.z(), current.z())
        {
            return Ok(triangulation.nb_nodes());
        }
    }
    Ok(1)
}

/// `GenerateNormalsArray`: the normals transformed; null without normals.
fn normals_array(
    triangulation: &Triangulation,
    transformation: &Transform3,
) -> Result<Handle<HArray2<f64>>, Failure> {
    let count = count_normals(triangulation)?;
    if count == 0 {
        return Ok(Handle::default());
    }
    let normals = step_new_array2::<f64>(1, count, 1, 3)?;
    for index in 1..=count {
        let normal = triangulation.normal(index)?.transformed(*transformation);
        step_array2_set(&normals, index, 1, normal.x(), WHERE)?;
        step_array2_set(&normals, index, 2, normal.y(), WHERE)?;
        step_array2_set(&normals, index, 3, normal.z(), WHERE)?;
    }
    Ok(normals)
}

/// `GenerateTriangleStrips`: a triangulation has no strips or fans, so each triangle is one strip.
fn triangle_strips(
    triangulation: &Triangulation,
) -> Result<Handle<HArray1<Handle<StandardTransient>>>, Failure> {
    let strips = step_new_array1::<Handle<StandardTransient>>(1, triangulation.nb_triangles())?;
    for index in 1..=triangulation.nb_triangles() {
        let triangle = triangulation.triangle(index)?;
        let strip = step_new_array1::<i32>(1, 3)?;
        for corner in 1..=3 {
            step_array1_set(&strip, corner, triangle.value(corner)?, WHERE)?;
        }
        step_array1_set(&strips, index, strip.cast(), WHERE)?;
    }
    Ok(strips)
}

/// `GenerateComplexTriangulatedSurfaceSet`: null for a face without triangulation. pnindex is empty:
/// "0: no indirection" (Recommended Practices for 3D Tessellated Geometry, Release 1.1).
fn surface_set(face: &Face) -> Result<Handle<StepVisualComplexTriangulatedSurfaceSet>, Failure> {
    let (triangulation, location) = BRepTool::triangulation(face, mesh_purpose::NONE)?;
    let Some(triangulation) = triangulation.get() else {
        return Ok(Handle::default());
    };
    let transformation = location.transformation();
    let set = Handle::new(StepVisualComplexTriangulatedSurfaceSet::new());
    set.init(
        &empty_name(),
        &coordinate_list(triangulation, &transformation)?,
        triangulation.nb_nodes(),
        &normals_array(triangulation, &transformation)?,
        &Handle::new(HArray1::<i32>::default()),
        &triangle_strips(triangulation)?,
        // All triangles were already written as triangle strips.
        &Handle::new(HArray1::<Handle<StandardTransient>>::default()),
    );
    Ok(set)
}

/// The points of one edge into `points`, as a strip; `None` for an edge without a curve.
fn edge_strip(
    edge: &Shape,
    points: &mut Vec<Xyz>,
) -> Result<Option<Handle<HSequence<i32>>>, Failure> {
    let (curve, first, last) = BRepTool::curve(&topods::edge(edge)?)?;
    if curve.is_null() {
        return Ok(None);
    }
    let strip = Handle::new(HSequence::<i32>::default());
    let live = step_live(&strip, WHERE)?;
    if curve.cast::<GeomLine>().is_some() {
        let mut vertices = Explorer::new(edge, ShapeEnum::Vertex, ShapeEnum::Shape)?;
        while vertices.more() {
            points.push(BRepTool::pnt(&topods::vertex(vertices.current())?)?.xyz());
            live.append(points.len() as i32);
            vertices.next()?;
        }
        return Ok(Some(strip));
    }
    let spline: Handle<GeomBSplineCurve> =
        ShapeConstructCurve.convert_to_bspline(&curve, first, last, precision::confusion())?;
    let spline = step_live(&spline, WHERE)?;
    for pole in 1..=spline.nb_poles() {
        points.push(spline.pole(pole)?.xyz());
        live.append(points.len() as i32);
    }
    Ok(Some(strip))
}

/// `GenerateTessellatedCurveSet`: null when no valid curves were found.
fn curve_set(shape: &Shape) -> Result<Handle<StepVisualTessellatedCurveSet>, Failure> {
    let mut strips = DynamicArray::default();
    let mut points = Vec::new();
    let mut edges = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while edges.more() {
        if let Some(strip) = edge_strip(edges.current(), &mut points)? {
            strips.append(strip);
        }
        edges.next()?;
    }
    if points.is_empty() {
        return Ok(Handle::default());
    }
    let coordinates = Handle::new(StepVisualCoordinatesList::new());
    coordinates.init(&empty_name(), &crate::geom_to_step::shared_array1(points));
    let set = Handle::new(StepVisualTessellatedCurveSet::new());
    set.init(&empty_name(), &coordinates, &ObjectHandle::new(strips));
    Ok(set)
}

/// The set: its first item is the curve set, then one surface set per face; any may be null.
pub(super) fn tessellation_of(
    shape: &Shape,
) -> Result<Handle<StepVisualTessellatedGeometricSet>, Failure> {
    let mut items: Vec<Handle<StepVisualTessellatedItem>> = vec![curve_set(shape)?.cast()];
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while faces.more() {
        items.push(surface_set(&topods::face(faces.current())?)?.cast());
        faces.next()?;
    }
    let set = Handle::new(StepVisualTessellatedGeometricSet::new());
    let items: Array1<'static, Handle<StepVisualTessellatedItem>> =
        crate::geom_to_step::array1_of(items);
    set.init(&empty_name(), &ObjectHandle::new(items));
    Ok(set)
}
