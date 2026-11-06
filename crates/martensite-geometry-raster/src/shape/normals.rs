//

//! The normal segments of a shape: at the triangulation nodes, or on
//! a grid of surface parameters.
use super::TopologyRendering;
use martensite_raster_core::RasterReports;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::brep_tools::uv_bounds;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Face, Shape, face as to_face};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkmath::{
    poly::{Triangulation, mesh_purpose},
    toploc::Location,
};

/// A normal as a segment: `[from, to]`.
pub type NormalSegment = (Point3, Point3);

/// The normals of each face (`NCollectionDataMap<TopoDSFace, ...>`, keyed with
/// `IsEqual`), in first-seen order.
pub type MeshNormals = Vec<(Face, Vec<NormalSegment>)>;

/// The length below which a normal counts as null.
const NULL_NORMAL: f64 = 1.0e-10;

impl TopologyRendering {
    /// `addMeshNormals(theNormals, theFace, theLength)`: a segment of `length` along the
    /// normal at each triangulation node (stored normals, else the surface normal at
    /// the UV node); false without a triangulation or without normals and UV nodes.
    /// Native quirk kept: stored normals are not moved by the face location.
    pub fn add_mesh_normals(
        normals: &mut Vec<NormalSegment>,
        face: &Face,
        length: f64,
        out: &dyn RasterReports,
    ) -> Result<bool, Failure> {
        let record = BRepTool::triangulation(face, mesh_purpose::NONE)?;
        add_mesh_record(normals, face, (length, out), record)
    }

    /// `addMeshNormals(theNormals, theShape, theLength)`: the mesh normals of each face.
    pub fn add_shape_mesh_normals(
        normals: &mut MeshNormals,
        shape: &Shape,
        length: f64,
        out: &dyn RasterReports,
    ) -> Result<(), Failure> {
        for face in faces_of(shape)? {
            let segments = entry(normals, &face);
            Self::add_mesh_normals(segments, &face, length, out)?;
        }
        Ok(())
    }

    /// `addSurfaceNormals(theNormals, theFace, theLength, theNbAlongU, theNbAlongV)`: a
    /// segment of `length` along the surface normal at `nb_u` x `nb_v` parameters
    /// spanning the UV bounds (the middle for a count of 1); false without surface.
    pub fn add_surface_normals(
        normals: &mut Vec<NormalSegment>,
        face: &Face,
        length: f64,
        (nb_u, nb_v): (i32, i32),
        out: &dyn RasterReports,
    ) -> Result<bool, Failure> {
        if BRepTool::surface_with_location(face)?.0.is_null() {
            return Ok(false);
        }
        let grid = SurfaceNormalGrid::new(face, (nb_u, nb_v))?;
        let surface = BRepAdaptorSurface::with_face(face, true)?;
        grid.append(normals, &surface, length, out)?;
        Ok(true)
    }

    /// `addSurfaceNormals(theNormals, theShape, ...)`: the surface normals of each face.
    pub fn add_shape_surface_normals(
        normals: &mut MeshNormals,
        shape: &Shape,
        length: f64,
        counts: (i32, i32),
        out: &dyn RasterReports,
    ) -> Result<(), Failure> {
        for face in faces_of(shape)? {
            let segments = entry(normals, &face);
            Self::add_surface_normals(segments, &face, length, counts, out)?;
        }
        Ok(())
    }
}

/// The normal scaled to `length`; a null one becomes `(|n| / 2, 0, 0)` and is reported.
fn scaled_normal(normal: Vector3, length: f64, report: impl FnOnce()) -> Vector3 {
    let magnitude = normal.magnitude();
    if magnitude > NULL_NORMAL {
        return normal * (length / magnitude);
    }
    report();
    Vector3::new(magnitude / 2.0, 0.0, 0.0)
}

/// The faces of `shape` in exploration order.
fn faces_of(shape: &Shape) -> Result<Vec<Face>, Failure> {
    let mut faces = Vec::new();
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while explorer.more() {
        faces.push(to_face(explorer.current())?);
        explorer.next()?;
    }
    Ok(faces)
}

/// The segments of `face` in the map, bound to an empty list on first use.
fn entry<'a>(normals: &'a mut MeshNormals, face: &Face) -> &'a mut Vec<NormalSegment> {
    let index = match normals.iter().position(|(key, _)| key.is_equal(face)) {
        Some(index) => index,
        None => {
            normals.push((face.clone(), Vec::new()));
            normals.len() - 1
        }
    };
    &mut normals[index].1
}

fn add_mesh_record(
    normals: &mut Vec<NormalSegment>,
    face: &Face,
    (length, out): (f64, &dyn RasterReports),
    (triangulation, location): (Handle<Triangulation>, Location),
) -> Result<bool, Failure> {
    let Some(mesh) = triangulation.get() else {
        return Ok(false);
    };
    let has_normals = mesh.has_normals();
    if !has_normals && !mesh.has_uv_nodes() {
        return Ok(false);
    }
    append_mesh_nodes(normals, face, (mesh, &location, has_normals), (length, out))?;
    Ok(true)
}

fn append_mesh_nodes(
    normals: &mut Vec<NormalSegment>,
    face: &Face,
    (mesh, location, has_normals): (&Triangulation, &Location, bool),
    report: (f64, &dyn RasterReports),
) -> Result<(), Failure> {
    let surface = BRepAdaptorSurface::with_face(face, true)?;
    for node in 1..=mesh.nb_nodes() {
        let segment = mesh_node_segment((mesh, location, &surface), (node, has_normals), report)?;
        normals.push(segment);
    }
    Ok(())
}

fn mesh_node_segment(
    (mesh, location, surface): (&Triangulation, &Location, &BRepAdaptorSurface),
    (node, has_normals): (i32, bool),
    (length, out): (f64, &dyn RasterReports),
) -> Result<NormalSegment, Failure> {
    let mut start = mesh.node(node)?;
    if !location.is_identity() {
        start = start.transformed(location.transformation());
    }
    let normal = mesh_node_direction(mesh, surface, node, has_normals)?;
    let normal = scaled_normal(normal, length, || report_null_mesh_node(start, out));
    Ok((start, start.translated(normal)))
}

fn mesh_node_direction(
    mesh: &Triangulation,
    surface: &BRepAdaptorSurface,
    node: i32,
    has_normals: bool,
) -> Result<Vector3, Failure> {
    if has_normals {
        return Ok(Vector3::from(mesh.normal(node)?));
    }
    let uv = mesh.uv_node(node)?;
    let d1 = surface.d1(uv.x(), uv.y())?;
    Ok(d1.d1u.crossed(d1.d1v))
}

fn report_null_mesh_node(start: Point3, out: &dyn RasterReports) {
    let (x, y, z) = (
        format_general(start.x(), 6),
        format_general(start.y(), 6),
        format_general(start.z(), 6),
    );
    out.write_out(&format!("Null normal at node X = {x}, Y = {y}, Z = {z}\n"));
}

struct SurfaceNormalGrid {
    u_min: f64,
    v_min: f64,
    du: f64,
    dv: f64,
    mid_u: bool,
    mid_v: bool,
    nb_u: i32,
    nb_v: i32,
}
impl SurfaceNormalGrid {
    fn new(face: &Face, (nb_u, nb_v): (i32, i32)) -> Result<Self, Failure> {
        let (u_min, u_max, v_min, v_max) = uv_bounds(face)?;
        let (mid_u, mid_v) = (nb_u == 1, nb_v == 1);
        let du = (u_max - u_min) / f64::from(if mid_u { 2 } else { nb_u - 1 });
        let dv = (v_max - v_min) / f64::from(if mid_v { 2 } else { nb_v - 1 });
        Ok(Self {
            u_min,
            v_min,
            du,
            dv,
            mid_u,
            mid_v,
            nb_u,
            nb_v,
        })
    }

    fn append(
        &self,
        normals: &mut Vec<NormalSegment>,
        surface: &BRepAdaptorSurface,
        length: f64,
        out: &dyn RasterReports,
    ) -> Result<(), Failure> {
        for i in 0..self.nb_u {
            let u = self.u_min + f64::from(if self.mid_u { 1 } else { i }) * self.du;
            self.append_row(normals, surface, u, length, out)?;
        }
        Ok(())
    }

    fn append_row(
        &self,
        normals: &mut Vec<NormalSegment>,
        surface: &BRepAdaptorSurface,
        u: f64,
        length: f64,
        out: &dyn RasterReports,
    ) -> Result<(), Failure> {
        for j in 0..self.nb_v {
            let v = self.v_min + f64::from(if self.mid_v { 1 } else { j }) * self.dv;
            normals.push(surface_normal_segment(surface, (u, v), length, out)?);
        }
        Ok(())
    }
}

fn surface_normal_segment(
    surface: &BRepAdaptorSurface,
    (u, v): (f64, f64),
    length: f64,
    out: &dyn RasterReports,
) -> Result<NormalSegment, Failure> {
    let d1 = surface.d1(u, v)?;
    let normal = scaled_normal(d1.d1u.crossed(d1.d1v), length, || {
        report_null_surface_normal(u, v, out)
    });
    Ok((d1.point, d1.point.translated(normal)))
}

fn report_null_surface_normal(u: f64, v: f64, out: &dyn RasterReports) {
    let (u, v) = (format_general(u, 6), format_general(v, 6));
    out.write_out(&format!("Null normal at U = {u}, V = {v}\n"));
}
