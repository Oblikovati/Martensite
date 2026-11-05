//! The numeric forms of the primitive, curve and `restore` commands.
use crate::script::command_stopped as stopped;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools,
    topexp::Explorer,
    topods::{self, Shape, Solid},
    toptools::MapOfShape,
};
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::Array1,
    standard::{Failure, Handle},
    storage::IoStream,
};
use martensite_tkg3d::{
    geom::{GeomBSplineCurve, GeomBezierCurve, GeomCurve},
    top_abs::ShapeEnum,
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Vector3, dz};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis,
    BRepPrimAPIMakeSphere, BRepPrimAPIMakeTorus, BRepPrimAPIMakeWedge,
};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::f64::consts::PI;

pub fn box_solid(v: &[f64]) -> Result<Solid, Failure> {
    let mut maker = match v {
        [x, y, z] => BRepPrimAPIMakeBox::at_point(Point3::default(), *x, *y, *z)?,
        [x, y, z, dx, dy, dz] => {
            BRepPrimAPIMakeBox::at_point(Point3::new(*x, *y, *z), *dx, *dy, *dz)?
        }
        _ => return Err(stopped(&format!("box takes 3 or 6 values, got {v:?}"))),
    };
    Ok(maker.solid()?.clone())
}

pub fn wedge_solid(v: &[f64]) -> Result<Solid, Failure> {
    let mut maker = match v {
        [dx, dy, dz, ltx] => BRepPrimAPIMakeWedge::with_ltx(*dx, *dy, *dz, *ltx)?,
        [dx, dy, dz, a, b, c, d] => {
            BRepPrimAPIMakeWedge::with_limits(*dx, *dy, *dz, [*a, *b, *c, *d])?
        }
        [x, y, z, zx, zy, zz, xx, xy, xz, dx, dy, dz, ltx] => {
            let axes = axes_of(Point3::new(*x, *y, *z), [*zx, *zy, *zz], [*xx, *xy, *xz])?;
            BRepPrimAPIMakeWedge::with_axes_ltx(axes, *dx, *dy, *dz, *ltx)
        }
        _ => {
            let message = format!("wedge takes 4, 7 or 13 values, got {v:?}");
            return Err(stopped(&message));
        }
    };
    Ok(maker.solid()?.clone())
}

fn axes_of(
    origin: Point3,
    [zx, zy, zz]: [f64; 3],
    [xx, xy, xz]: [f64; 3],
) -> Result<Axis2, Failure> {
    Axis2::with_x_direction(
        origin,
        Direction3::from_vector(Vector3::new(zx, zy, zz))?,
        Direction3::from_vector(Vector3::new(xx, xy, xz))?,
    )
}

pub fn cylinder_solid(v: &[f64]) -> Result<Solid, Failure> {
    let mut maker = match v {
        [r, h] => BRepPrimAPIMakeCylinder::new(*r, *h),
        [r, h, angle] => BRepPrimAPIMakeCylinder::with_angle(*r, *h, angle.to_radians_like())?,
        _ => {
            return Err(stopped(&format!(
                "pcylinder takes 2 or 3 values, got {v:?}"
            )));
        }
    };
    Ok(maker.solid()?.clone())
}

/// `pcylinder name plane R H`, where the plane is `plane pp x y z nx ny nz [xx xy xz]`.
pub fn cylinder_on_plane(plane: &[f64], v: &[f64; 2]) -> Result<Solid, Failure> {
    let (frame, _) = conic_frame(plane, 0)?;
    let mut maker = BRepPrimAPIMakeCylinder::with_axes(frame, v[0], v[1]);
    Ok(maker.solid()?.clone())
}

pub fn cone_solid(v: &[f64]) -> Result<Solid, Failure> {
    let mut maker = match v {
        [r1, r2, h] => BRepPrimAPIMakeCone::new(*r1, *r2, *h)?,
        [r1, r2, h, angle] => {
            BRepPrimAPIMakeCone::with_angle(*r1, *r2, *h, angle.to_radians_like())?
        }
        _ => return Err(stopped(&format!("pcone takes 3 or 4 values, got {v:?}"))),
    };
    Ok(maker.solid()?.clone())
}

pub fn sphere_solid(v: &[f64]) -> Result<Solid, Failure> {
    let mut maker = match v {
        [r] => BRepPrimAPIMakeSphere::new(*r)?,
        [r, a] => BRepPrimAPIMakeSphere::with_angle(*r, a.to_radians_like())?,
        [r, a, b] => {
            BRepPrimAPIMakeSphere::with_latitudes(*r, a.to_radians_like(), b.to_radians_like())?
        }
        [r, a, b, c] => BRepPrimAPIMakeSphere::with_latitudes_angle(
            *r,
            a.to_radians_like(),
            b.to_radians_like(),
            c.to_radians_like(),
        )?,
        _ => return Err(stopped(&format!("psphere takes 1 to 4 values, got {v:?}"))),
    };
    Ok(maker.solid()?.clone())
}

pub fn torus_solid(v: &[f64]) -> Result<Solid, Failure> {
    let mut maker = match v {
        [r1, r2] => BRepPrimAPIMakeTorus::new(*r1, *r2)?,
        [r1, r2, a] => BRepPrimAPIMakeTorus::with_angle(*r1, *r2, a.to_radians_like())?,
        [r1, r2, a, b] => {
            BRepPrimAPIMakeTorus::with_pipe(*r1, *r2, a.to_radians_like(), b.to_radians_like())?
        }
        [r1, r2, a, b, c] => BRepPrimAPIMakeTorus::with_pipe_angle(
            *r1,
            *r2,
            a.to_radians_like(),
            b.to_radians_like(),
            c.to_radians_like(),
        )?,
        _ => return Err(stopped(&format!("ptorus takes 2 to 5 values, got {v:?}"))),
    };
    Ok(maker.solid()?.clone())
}

/// command-host converts with `angle * (M_PI / 180.0)`, which can differ from `f64::to_radians`
/// in the last bit, so the commands' own formula is kept.
trait DrawDegrees {
    fn to_radians_like(&self) -> f64;
}

impl DrawDegrees for f64 {
    fn to_radians_like(&self) -> f64 {
        self * (PI / 180.0)
    }
}

/// The frame of a 3D conic and its `radii` trailing values: origin alone (DZ normal),
/// origin and normal, or origin, normal and X direction.
pub fn conic_frame(values: &[f64], radii: usize) -> Result<(Axis2, &[f64]), Failure> {
    let (frame, radius) = values.split_at(values.len().saturating_sub(radii));
    let dir = |d: &[f64]| Direction3::from_vector(Vector3::new(d[0], d[1], d[2]));
    let at = |p: &[f64]| Point3::new(p[0], p[1], p[2]);
    let axes = match frame.len() {
        3 => Axis2::new(at(frame), dz())?,
        6 => Axis2::new(at(frame), dir(&frame[3..])?)?,
        9 => Axis2::with_x_direction(at(frame), dir(&frame[3..])?, dir(&frame[6..])?)?,
        _ => {
            return Err(stopped(&format!(
                "conic frame needs 3, 6 or 9 values: {values:?}"
            )));
        }
    };
    Ok((axes, radius))
}

/// `beziercurve name nbpole pole [weight] ...`; values start at the pole count.
pub fn bezier_curve(values: &[f64]) -> Result<Handle<dyn GeomCurve>, Failure> {
    let count = values[0] as usize;
    let stride = (values.len() - 1) / count;
    let poles: Vec<Point3> = values[1..]
        .chunks_exact(stride)
        .map(|p| Point3::new(p[0], p[1], p[2]))
        .collect();
    let poles = Array1::from_shared(&poles, 1);
    if stride == 3 {
        return Ok(Handle::new(GeomBezierCurve::new(&poles)?).cast());
    }
    let weights: Vec<f64> = values[1..].chunks_exact(4).map(|p| p[3]).collect();
    let weights = Array1::from_shared(&weights, 1);
    Ok(Handle::new(GeomBezierCurve::with_weights(&poles, &weights)?).cast())
}

/// `bsplinecurve name degree nbknots knot mult ... pole weight ...`.
pub fn bspline_curve(values: &[f64]) -> Result<Handle<dyn GeomCurve>, Failure> {
    let (degree, knot_count) = (values[0] as i32, values[1] as usize);
    let (knot_values, pole_values) = values[2..].split_at(2 * knot_count);
    let knots: Vec<f64> = knot_values.chunks_exact(2).map(|k| k[0]).collect();
    let mults: Vec<i32> = knot_values.chunks_exact(2).map(|k| k[1] as i32).collect();
    let poles: Vec<Point3> = pole_values
        .chunks_exact(4)
        .map(|p| Point3::new(p[0], p[1], p[2]))
        .collect();
    let weights: Vec<f64> = pole_values.chunks_exact(4).map(|p| p[3]).collect();
    let curve = GeomBSplineCurve::with_weights(
        &Array1::from_shared(&poles, 1),
        &Array1::from_shared(&weights, 1),
        &Array1::from_shared(&knots, 1),
        &Array1::from_shared(&mults, 1),
        degree,
        false,
    )?;
    Ok(Handle::new(curve).cast())
}

/// The first word of a file saved from a drawable, before its plain shape text.
const DRAWABLE_HEADER: &[u8] = b"DBRep_DrawableShape";

/// `restore file name` and `brestore file name`, with or without the drawable header.
pub fn restored_shape(path: &str) -> Result<Shape, Failure> {
    let full = format!(
        "{}/../../external/martensite-test-data/dataset-7.9.0/{path}",
        env!("CARGO_MANIFEST_DIR")
    );
    let bytes = std::fs::read(&full)
        .map_err(|error| Failure::with_message("Standard_Failure", &format!("{full}: {error}")))?;
    let text = bytes.strip_prefix(DRAWABLE_HEADER).unwrap_or(&bytes);
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(text.to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )?;
    Ok(shape)
}

/// The `explode` type argument: first letters, any case.
pub fn explode_kind(text: &str) -> Result<ShapeEnum, Failure> {
    Ok(match text.to_ascii_uppercase().as_str() {
        "SO" => ShapeEnum::Solid,
        "SH" => ShapeEnum::Shell,
        "F" => ShapeEnum::Face,
        "W" => ShapeEnum::Wire,
        "E" => ShapeEnum::Edge,
        "V" => ShapeEnum::Vertex,
        _ => {
            return Err(stopped(&format!(
                "explode kind {text:?} is not So/Sh/F/W/E/V"
            )));
        }
    })
}

/// The sub-shapes of a kind, each once by `IsSame`, the shape itself excluded.
pub fn distinct_parts(shape: &Shape, kind: ShapeEnum) -> Result<Vec<Shape>, Failure> {
    let mut seen = MapOfShape::default();
    seen.add(shape.clone());
    let mut parts = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape)?;
    while explorer.more() {
        if seen.add(explorer.current().clone()) {
            parts.push(explorer.current().clone());
        }
        explorer.next()?;
    }
    Ok(parts)
}

/// `X * 999 + Y * 99 + Z * 0.9` of the centre of mass, or of a vertex point.
pub fn numbered_key(shape: &Shape) -> Result<f64, Failure> {
    let point = if shape.shape_type()? == ShapeEnum::Vertex {
        BRepTool::pnt(&topods::vertex(shape)?)?
    } else {
        let mut props = GProps::default();
        BRepGProp::linear_properties(shape, &mut props, false, false)?;
        props.centre_of_mass()
    };
    Ok(point.x() * 999. + point.y() * 99. + point.z() * 0.9)
}

/// The ascending order of the native bubble sort, which only swaps on a strict `>`.
pub fn bubble_order(keys: &[f64]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..keys.len()).collect();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..order.len() {
            if keys[order[i - 1]] > keys[order[i]] {
                order.swap(i - 1, i);
                swapped = true;
            }
        }
    }
    order
}
