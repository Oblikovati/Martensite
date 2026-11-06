//! Objects as the viewer's object commands display them: points, circles (an outline circle or
//! arc, or the filled stand-in) through three points, and triangles, segments and polylines
//! of points.
#[path = "viewer_objects/filled_circle.rs"]
mod filled_circle;
pub use filled_circle::FilledCircle;
#[path = "viewer_objects/primitive_array.rs"]
mod primitive_array;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomCircle};
use martensite_tkgeombase::gc::MakeCircle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::{
    Graphic3dArrayOfPolylines, Graphic3dArrayOfPrimitives, Graphic3dArrayOfSegments,
    Graphic3dArrayOfTriangles, Graphic3dAspectsMembers as _,
};
use martensite_tkv3d::ais::{AisCircle, AisInteractiveObject, AisPoint};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
pub use primitive_array::PrimitiveArrayObject;
use std::f64::consts::PI;

/// The circle through three points, as `GC_MakeCircle` builds it.
/// Example: `circle_through([[0., 0., 0.], [50., 50., 0.], [100., 0., 0.]])?`.
pub fn circle_through(points: [[f64; 3]; 3]) -> Result<Handle<GeomCircle>, Failure> {
    let [a, b, c] = points.map(|[x, y, z]| Point3::new(x, y, z));
    MakeCircle::through_points(a, b, c)?.value()
}

/// The displayed circle object: the filled stand-in, or an outline over the range. The range is
/// in degrees, converted as the command does (`deg * PI / 180`); `None` is the whole circle.
/// Example: `circle_object(&circle, true, Some([90., 150.]))`.
pub fn circle_object(
    circle: &Handle<GeomCircle>,
    filled: bool,
    degrees: Option<[f64; 2]>,
) -> Handle<dyn AisInteractiveObject> {
    let [u_start, u_end] = degrees.map_or([0., PI * 2.], |range| range.map(|d| d * PI / 180.));
    if filled {
        return FilledCircle::new(circle, u_start, u_end).cast();
    }
    AisCircle::with_parameters(circle, u_start, u_end, false).cast()
}

/// The point object the point command displays; `point_object([0., 0., 0.])?`.
pub fn point_object([x, y, z]: [f64; 3]) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    let point = Handle::new(GeomCartesianPoint::new(x, y, z));
    Ok(AisPoint::new(&point.cast())?.cast())
}

/// The object of a primitive array filled with `points`.
fn array_object(
    prims: Handle<Graphic3dArrayOfPrimitives>,
    points: &[[f64; 3]],
) -> Result<Handle<PrimitiveArrayObject>, Failure> {
    let array = prims.get().ok_or_else(|| {
        Failure::with_message("Standard_NullObject", "Graphic3d_ArrayOfPrimitives")
    })?;
    for &[x, y, z] in points {
        array.add_vertex_pnt(&Point3::new(x, y, z))?;
    }
    Ok(PrimitiveArrayObject::new(prims))
}

/// The triangle command's object: a triangle array of three points, drawn shaded.
/// Example: `triangle_object([[0., 0., 0.], [50., 0., 0.], [25., 40., 0.]])?`.
pub fn triangle_object(points: [[f64; 3]; 3]) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    let prims = Handle::new(Graphic3dArrayOfTriangles::new(3, 0, 0)?).cast();
    Ok(array_object(prims, &points)?.cast())
}

/// The segment command's object: a segment array of two points, its own shading aspect yellow.
/// Example: `segment_object([[0., 0., 0.], [50., 0., 0.]])?`.
pub fn segment_object(points: [[f64; 3]; 2]) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    let prims = Handle::new(Graphic3dArrayOfSegments::new(2, 0, 0)?).cast();
    let object = array_object(prims, &points)?;
    let drawer = object.attributes();
    drawer.setup_own_shading_aspect(&Handle::default())?;
    if let Some(shading) = drawer.shading_aspect().get() {
        shading
            .aspect()
            .set_interior_color(Color::named(NamedColor::YELLOW));
    }
    Ok(object.cast())
}

/// The primitive array command's object of one polyline through `points` (no bounds), drawn
/// with the object's own aspects; `polyline_object(&[[0., 0., 0.], [10., 0., 0.]])?`.
pub fn polyline_object(points: &[[f64; 3]]) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    let prims = Handle::new(Graphic3dArrayOfPolylines::new(
        points.len() as i32,
        0,
        0,
        0,
    )?)
    .cast();
    Ok(array_object(prims, points)?.cast())
}
