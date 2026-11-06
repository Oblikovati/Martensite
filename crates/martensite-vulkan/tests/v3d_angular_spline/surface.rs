use super::*;
use martensite_tkernel::{ncollection::Array1, precision, standard::Handle};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomCurve, GeomSurface, GeomSurfaceOfLinearExtrusion,
};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeHalfSpace;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
const POLES: [[f64; 3]; 14] = [
    [-500.0, -798.578274581199, 755.284518447357],
    [-500.0, -646.205376770376, 627.428476092882],
    [-500.0, -493.832478959552, 499.572433738407],
    [-500.0, -341.459581148729, 371.716391383932],
    [-500.0, -189.086683337906, 243.860349029457],
    [-500.0, -35.5842111132817, 115.056481200973],
    [-500.0, -33.7993197286247, 114.2995541724],
    [-500.0, -31.778022626919, 114.565662984205],
    [-500.0, -30.2498570721059, 115.75876223351],
    [-500.0, 84.6852511720001, 279.903107977019],
    [-500.0, 198.774589601206, 442.839569245217],
    [-500.0, 312.863928030413, 605.776030513415],
    [-500.0, 426.953266459619, 768.712491781612],
    [-500.0, 541.042604888826, 931.64895304981],
];
const WEIGHTS: [f64; 14] = [
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    0.957903314642061,
    0.95790331464206,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
];
const KNOTS: [f64; 4] = [-494.543457494654, 500.0, 507.372773368102, 1501.91623086297];

fn case_profile_curve() -> Result<Handle<dyn GeomCurve>, BoxError> {
    let poles = POLES.map(|[x, y, z]| Point3::new(x, y, z));
    let curve = Handle::new(GeomBSplineCurve::with_weights(
        &Array1::from_shared(&poles, 1),
        &Array1::from_shared(&WEIGHTS, 1),
        &Array1::from_shared(&KNOTS, 1),
        &Array1::from_shared(&[6, 4, 4, 6], 1),
        5,
        false,
    )?)
    .cast();
    Ok(curve)
}

fn case_extruded_surface() -> Result<Handle<dyn GeomSurface>, BoxError> {
    let curve = case_profile_curve()?;
    Ok(Handle::new(GeomSurfaceOfLinearExtrusion::new(
        &curve,
        Direction3::new(1., 0., 0.)?,
    )?)
    .cast())
}

pub(super) fn case_half_space() -> Result<Shape, BoxError> {
    let surface = case_extruded_surface()?;
    let face = BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        (KNOTS[0], KNOTS[3], 0., 1000.),
        precision::confusion(),
    )?
    .face()?;
    let maker = BRepPrimAPIMakeHalfSpace::with_face(&face, Point3::new(0., 0., 500.))?;
    Ok(maker.solid()?.clone().into_shape())
}
