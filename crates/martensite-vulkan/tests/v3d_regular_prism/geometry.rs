use super::*;
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
const POLES: [[f64; 3]; 9] = [
    [77.34874687409, 77.303696496535, 0.],
    [-37.404350826922, 66.469283013615, 0.],
    [-38.129049645989, 51.427809605917, 0.],
    [45.003598352348, 23.760586819334, 0.],
    [-76.009618710498, -14.499612221562, 0.],
    [44.396611605217, -43.851734118626, 0.],
    [119.71153838454, 27.656796734959, 0.],
    [38.244406969565, 24.98300747794, 0.],
    [68.787902964874, 60.998473938995, 0.],
];
const KNOTS: [f64; 9] = [
    0.,
    73.198335334976,
    109.22594821708,
    168.29694729401,
    244.58155163942,
    307.53411471698,
    344.2978168401,
    379.98768527731,
    399.75469301329,
];

fn periodic_profile() -> Result<Handle<GeomBSplineCurve>, BoxError> {
    let poles = POLES.map(|[x, y, z]| Point3::new(x, y, z));
    Ok(Handle::new(GeomBSplineCurve::with_weights(
        &Array1::from_shared(&poles, 1),
        &Array1::from_shared(&[1.; 9], 1),
        &Array1::from_shared(&KNOTS, 1),
        &Array1::from_shared(&[2, 1, 1, 1, 1, 1, 1, 1, 2], 1),
        3,
        true,
    )?))
}

pub(super) fn actual_prism() -> Result<Shape, BoxError> {
    let curve = periodic_profile()?;
    let edge = BRepBuilderAPIMakeEdge::from_curve(&curve.cast())?.edge()?;
    let wire = BRepBuilderAPIMakeWire::from_edge(&edge)?.wire()?;
    let face = BRepBuilderAPIMakeFace::from_wire(&wire, false)?.face()?;
    let mut maker =
        BRepPrimAPIMakePrism::new(&face.into_shape(), Vector3::new(0., 0., 100.), false, true)?;
    Ok(maker.shape()?.clone())
}

pub(super) fn tessellated_first_face(shape: &Shape) -> Result<Shape, BoxError> {
    use martensite_tkbrep::{topexp::Explorer, topods};
    use martensite_tkg3d::top_abs::ShapeEnum;
    let face = Iterator::next(&mut Explorer::new(
        shape,
        ShapeEnum::Face,
        ShapeEnum::Shape,
    )?)
    .ok_or("explode result F: face count=0; expected result_1")?;
    Ok(crate::regular_surface_tessellation::from_face(&topods::face(&face)?, 50, 50)?.into_shape())
}
