//! Shared original adaptor fixture constructors; both numeric reader identities use one builder.
use super::fixture_input::GeometryFixtureInput;
use super::geometry_record_fixtures::{curve, curve2d, surface};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::{adaptor2d::Adaptor2dCurve2d, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface},
};
use martensite_tkmath::geom_abs::Shape;
pub fn adapted_curve(
    r: &mut impl GeometryFixtureInput,
) -> Result<Handle<dyn Adaptor3dCurve>, Failure> {
    let kind = r.integer();
    let range = if kind == 1 {
        Some((r.real(), r.real()))
    } else {
        None
    };
    let c = curve(r)?;
    Ok(match range {
        Some((u1, u2)) => Handle::new(GeomAdaptorCurve::with_range(&c, u1, u2)?).cast(),
        None => Handle::new(GeomAdaptorCurve::with_curve(&c)?).cast(),
    })
}

pub fn adapted_curve2d(
    r: &mut impl GeometryFixtureInput,
) -> Result<Handle<dyn Adaptor2dCurve2d>, Failure> {
    Ok(Handle::new(Geom2dAdaptorCurve::with_curve(&curve2d(r)?)?).cast())
}

pub fn adapted_surface(
    r: &mut impl GeometryFixtureInput,
) -> Result<Handle<dyn Adaptor3dSurface>, Failure> {
    Ok(Handle::new(GeomAdaptorSurface::with_surface(&surface(r)?)?).cast())
}

/// `Shape(code)`: the retained enumeration order.
pub fn shape(code: i32) -> Shape {
    [
        Shape::C0,
        Shape::G1,
        Shape::C1,
        Shape::G2,
        Shape::C2,
        Shape::C3,
        Shape::CN,
    ][code as usize]
}
