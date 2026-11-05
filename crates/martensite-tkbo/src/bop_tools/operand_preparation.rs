//! BOP operand preparation used by source regressions that repair missing p-curves and
//! normalize tolerances before an operation.
//!
//! The steps implement the geometry-only effects of the legacy preparation commands.
use martensite_tkbrep::brep::access::{edge_tshape, face_tshape, vertex_tshape};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Shape, Vertex};
use martensite_tkbrep::toptools::{IndexedDataMapOfShapeListOfShape, IndexedMapOfShape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tktopalgo::brep_lib::same_parameter_shape;

/// Reduces edge, face, and vertex tolerances then enforces same-parameter geometry.
///
/// # Examples
/// ```ignore
/// prepare_boolean_operand(&shape)?;
/// ```
pub fn prepare_boolean_operand(shape: &Shape) -> Result<(), Failure> {
    add_missing_pcurves(shape)?;
    reduce_tolerances(shape, 1.0e-7)?;
    same_parameter_shape(shape, 1.0e-7, true)
}

/// Reduces the edge, face, and vertex tolerances of `shape` to their geometric bounds.
///
/// # Examples
/// ```ignore
/// reduce_tolerances(&shape, 1.0e-7)?;
/// ```
pub fn reduce_tolerances(shape: &Shape, edge_minimum: f64) -> Result<(), Failure> {
    reduce_edge_tolerance(shape, edge_minimum)?;
    reduce_face_tolerance(shape)?;
    reduce_vertex_tolerances(shape)
}

/// `ReduceEdgeTolerance(aS, aTolTreshold)`.
fn reduce_edge_tolerance(shape: &Shape, threshold: f64) -> Result<(), Failure> {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges)?;
    for i in 1..=edges.extent() {
        process_edge(edges.find_key(i)?, threshold)?;
    }
    Ok(())
}

/// `ReduceFaceTolerance(aS)`: each face takes the smallest tolerance of its edges, at
/// least 1e-7.
fn reduce_face_tolerance(shape: &Shape) -> Result<(), Failure> {
    let tol_e_min = 1.0e-7;
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Face, &mut faces)?;
    // The edge map is not cleared between faces, as natively.
    let mut edges = IndexedMapOfShape::default();
    for i in 1..=faces.extent() {
        let mut tolx = 1.0e6;
        let face = faces.find_key(i)?;
        let tface = face_tshape(face, "BRep_TFace")?;
        topexp::map_shapes_of_type(face, ShapeEnum::Edge, &mut edges)?;
        for j in 1..=edges.extent() {
            let tol_e = BRepTool::edge_tolerance(&topods::edge(edges.find_key(j)?)?)?;
            if tol_e < tolx {
                tolx = tol_e;
            }
        }
        tface.set_tolerance(if tolx > tol_e_min { tolx } else { tol_e_min });
    }
    Ok(())
}

/// `ReduceVertexTolerance(aS)`.
pub fn reduce_vertex_tolerances(shape: &Shape) -> Result<(), Failure> {
    let mut ve_map = IndexedDataMapOfShapeListOfShape::default();
    let mut vf_map = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_unique_ancestors(
        shape,
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        &mut ve_map,
        false,
    )?;
    topexp::map_shapes_and_unique_ancestors(
        shape,
        ShapeEnum::Vertex,
        ShapeEnum::Face,
        &mut vf_map,
        false,
    )?;
    for i in 1..=ve_map.extent() {
        let v = topods::vertex(ve_map.find_key(i)?)?;
        let le: Vec<Shape> = ve_map.find_from_index(i)?.iter().cloned().collect();
        let lf: Vec<Shape> = vf_map.find_from_key(v.as_ref())?.iter().cloned().collect();
        process_vertex(&v, &le, &lf)?;
    }
    Ok(())
}

/// `ProcessEdge(aE, aTolTreshold)`: the tolerance of the edge set to the largest
/// distance between its 3D curve and its curves on surface at 24 parameters (at least
/// the threshold).
fn process_edge(edge_shape: &Shape, threshold: f64) -> Result<(), Failure> {
    let nb = 23;
    let edge = topods::edge(edge_shape)?;
    let (c3d, t1, t2) = BRepTool::curve(&edge)?;
    let Some(c3d) = c3d.get() else {
        return Ok(());
    };
    let dt = (t2 - t1) / f64::from(nb);
    let tedge = edge_tshape(edge_shape, "BRep_TEdge")?;
    let eloc = edge.location().clone();
    let mut tol_max2: f64 = -1.0e6;
    for cr in tedge.curves() {
        let Some(cr) = cr.get() else {
            continue;
        };
        let l = (&eloc * &cr.location())?;
        // 3D-Curve
        if cr.is_curve3d() {
            continue;
        }
        // 2D-Curve
        if cr.is_curve_on_surface() {
            let c2d = cr.pcurve()?;
            let Some(c2d) = c2d.get() else {
                continue;
            };
            // Surface
            let surface = cr.surface()?;
            let Some(s) = surface.get() else {
                continue;
            };
            // 2D-point treatment
            for i in 0..=nb {
                let t = if i == nb { t2 } else { t1 + f64::from(i) * dt };
                let pc3d = c3d.value(t)?;
                let pc2d = c2d.value(t)?;
                let p3d = s.d0(pc2d.x(), pc2d.y())?.transformed(l.transformation());
                let d2 = pc3d.square_distance(&p3d);
                if d2 > tol_max2 {
                    tol_max2 = d2;
                }
            }
        }
    }
    if tol_max2 < 0.0 {
        return Ok(());
    }
    let tol_max = tol_max2.sqrt();
    tedge.set_tolerance(if tol_max > threshold {
        tol_max
    } else {
        threshold
    });
    Ok(())
}

/// `ProcessVertex(aV, aLE, aLF)`: the tolerance of the vertex reduced to the largest
/// distance to its edges' curves, not below the tolerances of its edges and faces.
fn process_vertex(v: &Vertex, le: &[Shape], lf: &[Shape]) -> Result<(), Failure> {
    let mut tol_max2: f64 = -1.0e6;
    let tv = vertex_tshape(v.as_ref(), "BRep_TVertex")?;
    let pv3d = tv.pnt();
    let tol = BRepTool::vertex_tolerance(v)?;
    for edge_shape in le {
        let edge = topods::edge(edge_shape)?;
        let tedge = edge_tshape(edge_shape, "BRep_TEdge")?;
        let eloc = edge.location().clone();
        let mut v_exp = Explorer::new(edge_shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
        while v_exp.more() {
            let vx = topods::vertex(v_exp.current())?;
            v_exp.next()?;
            if !vx.as_ref().is_same(v.as_ref()) {
                continue;
            }
            let or_v = vx.orientation();
            if or_v != Orientation::Forward && or_v != Orientation::Reversed {
                continue;
            }
            for cr in tedge.curves() {
                let Some(cr) = cr.get() else {
                    continue;
                };
                let l = (&eloc * &cr.location())?.predivided(v.location())?;
                if cr.is_curve3d() {
                    // 3D-Curve
                    let c3d = cr.curve3d()?;
                    let Some(c3d) = c3d.get() else {
                        continue;
                    };
                    // 3D-point treatment
                    let param = BRepTool::parameter(&vx, &edge)?;
                    let pc3d = c3d.value(param)?.transformed(l.transformation());
                    let d2 = pv3d.square_distance(&pc3d);
                    if d2 > tol_max2 {
                        tol_max2 = d2;
                    }
                } else if cr.is_curve_on_surface() {
                    // 2D-Curve
                    let c2d = cr.pcurve()?;
                    let Some(c2d) = c2d.get() else {
                        continue;
                    };
                    let surface: Handle<dyn GeomSurface> = cr.surface()?;
                    let Some(s) = surface.get() else {
                        continue;
                    };
                    // 2D-point treatment
                    let param = BRepTool::parameter_on_surface(&vx, &edge, &surface, &l)?;
                    let pc2d = c2d.value(param)?;
                    let pc3d = s.d0(pc2d.x(), pc2d.y())?.transformed(l.transformation());
                    let d2 = pv3d.square_distance(&pc3d);
                    if d2 > tol_max2 {
                        tol_max2 = d2;
                    }
                }
            }
        }
    }
    // Reducing
    if tol_max2 < 0.0 {
        return Ok(());
    }
    let mut tol_max = tol_max2.sqrt();
    if tol_max > tol {
        return Ok(());
    }
    for edge in le {
        let tol_e = BRepTool::edge_tolerance(&topods::edge(edge)?)?;
        if tol_max < tol_e {
            tol_max = tol_e;
        }
    }
    for face in lf {
        let tol_f = BRepTool::face_tolerance(&topods::face(face)?)?;
        if tol_max < tol_f {
            tol_max = tol_f;
        }
    }
    if tol_max > tol {
        return Ok(());
    }
    // Update Tolerance
    tv.set_tolerance(tol_max);
    Ok(())
}

/// Adds missing p-curves to edges on their incident faces.
///
/// # Examples
/// ```ignore
/// add_missing_pcurves(&shape)?;
/// ```
pub fn add_missing_pcurves(shape: &Shape) -> Result<(), Failure> {
    let mut ef_map = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_ancestors(shape, ShapeEnum::Edge, ShapeEnum::Face, &mut ef_map)?;
    let builder = BRepBuilder::default();
    for i in 1..=ef_map.extent() {
        let edge_shape = ef_map.find_key(i)?.clone();
        let edge = topods::edge(&edge_shape)?;
        if BRepTool::degenerated(&edge)? {
            continue;
        }
        let (c3d, _, _) = BRepTool::curve(&edge)?;
        if c3d.is_null() {
            continue;
        }
        let tol_e = BRepTool::edge_tolerance(&edge)?;
        let faces: Vec<Shape> = ef_map.find_from_index(i)?.iter().cloned().collect();
        for face_shape in &faces {
            let face = topods::face(face_shape)?;
            // Map of surfaces on which the edge lays.
            let tedge = edge_tshape(&edge_shape, "BRep_TEdge")?;
            let mut scr_map: Vec<Handle<dyn GeomSurface>> = Vec::new();
            for cr in tedge.curves() {
                if let Some(cr) = cr.get()
                    && cr.is_curve_on_surface()
                {
                    let surface = cr.surface()?;
                    if !scr_map.contains(&surface) {
                        scr_map.push(surface);
                    }
                }
            }
            let (surface, location) = BRepTool::surface_with_location(&face)?;
            if !scr_map.contains(&surface) {
                // try to obtain 2D curve
                let pc = BRepTool::curve_on_surface(&edge, &surface, &location)?;
                if pc.pcurve.is_null() {
                    continue;
                }
                builder.update_edge_pcurve_on_face(&edge, &pc.pcurve, &face, tol_e)?;
            }
        }
    }
    Ok(())
}
