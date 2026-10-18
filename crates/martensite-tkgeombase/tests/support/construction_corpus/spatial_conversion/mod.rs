//! The GeomConvert probe; the counterpart of the implementation
//! (conversions, printers) and the implementation (recognizers). The
//! printers are shared with the GeomLib probe.
mod ana;
mod convert;
pub use ana::*;
pub use convert::*;

use super::common::{ax1, ax3, circ, circ2d, cone, cylinder, dir, elips, elips2d, flag};
use super::common::{hypr, hypr2d, int, lin, lin2d, parab, parab2d, pln, pnt, real};
use super::geom2d_convert::{bezier as bezier2d, bspline as bspline2d};
use super::{
    fixture_input::GeometryFixtureInput, geometry_record_fixtures as build, print::Output,
};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse,
    Geom2dHyperbola, Geom2dLine, Geom2dOffsetCurve, Geom2dParabola, Geom2dTrimmedCurve,
};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBSplineSurface, GeomBezierCurve, GeomBezierSurface, GeomCircle,
    GeomConicalSurface, GeomCurve, GeomCylindricalSurface, GeomEllipse, GeomHyperbola, GeomLine,
    GeomOffsetCurve, GeomOffsetSurface, GeomParabola, GeomPlane, GeomRectangularTrimmedSurface,
    GeomSphericalSurface, GeomSurface, GeomSurfaceOfLinearExtrusion, GeomSurfaceOfRevolution,
    GeomSweptSurface, GeomToroidalSurface, GeomTrimmedCurve,
};
use martensite_tkmath::gp::{Sphere3, Torus3};

/// gb_gcv_bspline.
pub fn bspline(output: &Output, c: &Handle<GeomBSplineCurve>) {
    let Some(c) = c.get() else {
        return int(output, -1);
    };
    int(output, c.degree());
    flag(output, c.is_periodic());
    flag(output, c.is_rational());
    int(output, c.nb_poles());
    for i in 1..=c.nb_poles() {
        pnt(output, c.pole(i).unwrap());
        real(output, c.weight(i).unwrap());
    }
    int(output, c.nb_knots());
    for i in 1..=c.nb_knots() {
        real(output, c.knot(i).unwrap());
        int(output, c.multiplicity(i).unwrap());
    }
}

/// gb_gcv_bezier.
pub fn bezier(output: &Output, c: &Handle<GeomBezierCurve>) {
    let Some(c) = c.get() else {
        return int(output, -1);
    };
    int(output, c.degree());
    flag(output, c.is_rational());
    int(output, c.nb_poles());
    for i in 1..=c.nb_poles() {
        pnt(output, c.pole(i).unwrap());
        real(output, c.weight(i).unwrap());
    }
}

/// gb_gcv_bspline_surface.
pub fn bspline_surface(output: &Output, s: &Handle<GeomBSplineSurface>) {
    let Some(s) = s.get() else {
        return int(output, -1);
    };
    int(output, s.u_degree());
    int(output, s.v_degree());
    flag(output, s.is_u_periodic());
    flag(output, s.is_v_periodic());
    flag(output, s.is_u_rational());
    flag(output, s.is_v_rational());
    int(output, s.nb_u_poles());
    int(output, s.nb_v_poles());
    for i in 1..=s.nb_u_poles() {
        for j in 1..=s.nb_v_poles() {
            pnt(output, s.pole(i, j).unwrap());
            real(output, s.weight(i, j).unwrap());
        }
    }
    int(output, s.nb_u_knots());
    for i in 1..=s.nb_u_knots() {
        real(output, s.u_knot(i).unwrap());
        int(output, s.u_multiplicity(i).unwrap());
    }
    int(output, s.nb_v_knots());
    for i in 1..=s.nb_v_knots() {
        real(output, s.v_knot(i).unwrap());
        int(output, s.v_multiplicity(i).unwrap());
    }
}

/// gb_gcv_bezier_surface.
pub fn bezier_surface(output: &Output, s: &Handle<GeomBezierSurface>) {
    let Some(s) = s.get() else {
        return int(output, -1);
    };
    int(output, s.u_degree());
    int(output, s.v_degree());
    flag(output, s.is_u_rational());
    flag(output, s.is_v_rational());
    for i in 1..=s.nb_u_poles() {
        for j in 1..=s.nb_v_poles() {
            pnt(output, s.pole(i, j).unwrap());
            real(output, s.weight(i, j).unwrap());
        }
    }
}

fn sphere(output: &Output, s: Sphere3) {
    ax3(output, s.position());
    real(output, s.radius());
}

fn torus(output: &Output, t: Torus3) {
    ax3(output, t.position());
    real(output, t.major_radius());
    real(output, t.minor_radius());
}

/// gb_gcv_curve: the class name, then the data of its kind.
pub fn curve(output: &Output, c: &Handle<dyn GeomCurve>) {
    name(output, c);
    if let Some(t) = c.cast::<GeomTrimmedCurve>().get() {
        real(output, t.first_parameter());
        real(output, t.last_parameter());
        return curve(output, &t.basis_curve());
    }
    if let Some(x) = c.cast::<GeomLine>().get() {
        return lin(output, x.lin());
    }
    if let Some(x) = c.cast::<GeomCircle>().get() {
        return circ(output, x.circ().unwrap());
    }
    if let Some(x) = c.cast::<GeomEllipse>().get() {
        return elips(output, x.elips().unwrap());
    }
    if let Some(x) = c.cast::<GeomHyperbola>().get() {
        return hypr(output, x.hypr().unwrap());
    }
    if let Some(x) = c.cast::<GeomParabola>().get() {
        return parab(output, x.parab().unwrap());
    }
    curve_poles(output, c);
}

fn curve_poles(output: &Output, c: &Handle<dyn GeomCurve>) {
    let b: Handle<GeomBSplineCurve> = c.cast();
    if b.is_some() {
        return bspline(output, &b);
    }
    let z: Handle<GeomBezierCurve> = c.cast();
    if z.is_some() {
        return bezier(output, &z);
    }
    if let Some(x) = c.cast::<GeomOffsetCurve>().get() {
        real(output, x.offset());
        dir(output, x.direction());
        curve(output, &x.basis_curve());
    }
}

/// gb_gcv_curve2d.
pub fn curve2d(output: &Output, c: &Handle<dyn Geom2dCurve>) {
    name(output, c);
    if let Some(t) = c.cast::<Geom2dTrimmedCurve>().get() {
        real(output, t.first_parameter());
        real(output, t.last_parameter());
        return curve2d(output, &t.basis_curve());
    }
    if let Some(x) = c.cast::<Geom2dLine>().get() {
        return lin2d(output, x.lin2d());
    }
    if let Some(x) = c.cast::<Geom2dCircle>().get() {
        return circ2d(output, x.circ2d().unwrap());
    }
    if let Some(x) = c.cast::<Geom2dEllipse>().get() {
        return elips2d(output, x.elips2d().unwrap());
    }
    if let Some(x) = c.cast::<Geom2dHyperbola>().get() {
        return hypr2d(output, x.hypr2d().unwrap());
    }
    if let Some(x) = c.cast::<Geom2dParabola>().get() {
        return parab2d(output, x.parab2d().unwrap());
    }
    curve2d_poles(output, c);
}

fn curve2d_poles(output: &Output, c: &Handle<dyn Geom2dCurve>) {
    let b: Handle<Geom2dBSplineCurve> = c.cast();
    if b.is_some() {
        return bspline2d(output, &b);
    }
    let z: Handle<Geom2dBezierCurve> = c.cast();
    if z.is_some() {
        return bezier2d(output, &z);
    }
    if let Some(x) = c.cast::<Geom2dOffsetCurve>().get() {
        real(output, x.offset());
        curve2d(output, &x.basis_curve());
    }
}

/// gb_gcv_surface: the class name, then the data of its kind.
pub fn surface(output: &Output, s: &Handle<dyn GeomSurface>) {
    name(output, s);
    if let Some(t) = s.cast::<GeomRectangularTrimmedSurface>().get() {
        let (u1, u2, v1, v2) = t.bounds();
        [u1, u2, v1, v2]
            .into_iter()
            .for_each(|value| real(output, value));
        return surface(output, &t.basis_surface());
    }
    if let Some(x) = s.cast::<GeomPlane>().get() {
        return pln(output, x.pln());
    }
    if let Some(x) = s.cast::<GeomCylindricalSurface>().get() {
        return cylinder(output, x.cylinder().unwrap());
    }
    if let Some(x) = s.cast::<GeomConicalSurface>().get() {
        return cone(output, x.cone().unwrap());
    }
    if let Some(x) = s.cast::<GeomSphericalSurface>().get() {
        return sphere(output, x.sphere().unwrap());
    }
    if let Some(x) = s.cast::<GeomToroidalSurface>().get() {
        return torus(output, x.torus().unwrap());
    }
    surface_poles(output, s);
}

fn surface_poles(output: &Output, s: &Handle<dyn GeomSurface>) {
    let b: Handle<GeomBSplineSurface> = s.cast();
    if b.is_some() {
        return bspline_surface(output, &b);
    }
    let z: Handle<GeomBezierSurface> = s.cast();
    if z.is_some() {
        return bezier_surface(output, &z);
    }
    if let Some(x) = s.cast::<GeomSurfaceOfRevolution>().get() {
        ax1(output, x.axis());
        return curve(output, &x.basis_curve());
    }
    if let Some(x) = s.cast::<GeomSurfaceOfLinearExtrusion>().get() {
        dir(output, x.direction());
        return curve(output, &x.basis_curve());
    }
    if let Some(x) = s.cast::<GeomOffsetSurface>().get() {
        real(output, x.offset());
        surface(output, &x.basis_surface());
    }
}

/// gb_gcv_read_bspline.
pub fn read_bspline(
    r: &mut impl GeometryFixtureInput,
) -> Result<Handle<GeomBSplineCurve>, Failure> {
    Ok(build::curve(r)?.cast())
}

/// gb_gcv_read_bspline_surface.
pub fn read_bspline_surface(
    r: &mut impl GeometryFixtureInput,
) -> Result<Handle<GeomBSplineSurface>, Failure> {
    Ok(build::surface(r)?.cast())
}

pub use super::numeric_protocol::UNSET;

pub fn name<T: ?Sized + RuntimeTyped>(output: &Output, handle: &Handle<T>) {
    match handle.dynamic_type() {
        Some(kind) => output.write(format_args!("{} ", kind.name())),
        None => output.write(format_args!("null ")),
    }
}
