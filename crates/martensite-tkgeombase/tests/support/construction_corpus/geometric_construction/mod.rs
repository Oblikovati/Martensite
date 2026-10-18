//! The GC package probe; the counterpart of the implementation, split
//! like its curve and surface files.
mod curves;
mod surfaces;
pub use curves::*;
pub use surfaces::*;

use super::common::{self as p, Reader};
use super::print::Output;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg2d::geom2d::{
    Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dHyperbola, Geom2dLine, Geom2dParabola,
    Geom2dTransformation, Geom2dTrimmedCurve,
};
use martensite_tkg3d::geom::{
    GeomCircle, GeomConicalSurface, GeomCurve, GeomCylindricalSurface, GeomEllipse, GeomHyperbola,
    GeomLine, GeomParabola, GeomPlane, GeomRectangularTrimmedSurface, GeomSurface,
    GeomTransformation, GeomTrimmedCurve,
};
use martensite_tkmath::gp::{
    Circle2, Circle3, Ellipse2, Ellipse3, Hyperbola2, Hyperbola3, Parabola2, Parabola3,
};

/// The class name of a handle, `null` for a null one (gb_name).
pub fn name<T: ?Sized + RuntimeTyped>(output: &Output, handle: &Handle<T>) {
    match handle.dynamic_type() {
        Some(kind) => output.write(format_args!("{} ", kind.name())),
        None => output.write(format_args!("null ")),
    }
}

/// gb_geom_curve: the class, then the trimmed bounds and basis or the gp element.
pub fn geom_curve<T: ?Sized + RuntimeTyped>(output: &Output, handle: &Handle<T>) {
    let curve: Handle<dyn GeomCurve> = handle.cast();
    name(output, &curve);
    if let Some(trimmed) = curve.cast::<GeomTrimmedCurve>().get() {
        p::real(output, trimmed.first_parameter());
        p::real(output, trimmed.last_parameter());
        return geom_curve(output, &trimmed.basis_curve());
    }
    if let Some(line) = curve.cast::<GeomLine>().get() {
        return p::lin(output, line.lin());
    }
    geom_conic(output, &curve);
}

fn geom_conic(output: &Output, curve: &Handle<dyn GeomCurve>) {
    if let Some(x) = curve.cast::<GeomCircle>().get() {
        p::circ(output, x.circ().unwrap());
    } else if let Some(x) = curve.cast::<GeomEllipse>().get() {
        p::elips(output, x.elips().unwrap());
    } else if let Some(x) = curve.cast::<GeomHyperbola>().get() {
        p::hypr(output, x.hypr().unwrap());
    } else if let Some(x) = curve.cast::<GeomParabola>().get() {
        p::parab(output, x.parab().unwrap());
    }
}

/// gb_geom2d_curve: the class, then the trimmed bounds and basis or the gp element.
pub fn geom2d_curve<T: ?Sized + RuntimeTyped>(output: &Output, handle: &Handle<T>) {
    let curve: Handle<dyn Geom2dCurve> = handle.cast();
    name(output, &curve);
    if let Some(trimmed) = curve.cast::<Geom2dTrimmedCurve>().get() {
        p::real(output, trimmed.first_parameter());
        p::real(output, trimmed.last_parameter());
        return geom2d_curve(output, &trimmed.basis_curve());
    }
    if let Some(line) = curve.cast::<Geom2dLine>().get() {
        return p::lin2d(output, line.lin2d());
    }
    geom2d_conic(output, &curve);
}

fn geom2d_conic(output: &Output, curve: &Handle<dyn Geom2dCurve>) {
    if let Some(x) = curve.cast::<Geom2dCircle>().get() {
        p::circ2d(output, x.circ2d().unwrap());
    } else if let Some(x) = curve.cast::<Geom2dEllipse>().get() {
        p::elips2d(output, x.elips2d().unwrap());
    } else if let Some(x) = curve.cast::<Geom2dHyperbola>().get() {
        p::hypr2d(output, x.hypr2d().unwrap());
    } else if let Some(x) = curve.cast::<Geom2dParabola>().get() {
        p::parab2d(output, x.parab2d().unwrap());
    }
}

/// gb_geom_surface: the class, then the trimmed bounds and basis or the gp element.
pub fn geom_surface<T: ?Sized + RuntimeTyped>(output: &Output, handle: &Handle<T>) {
    let surface: Handle<dyn GeomSurface> = handle.cast();
    name(output, &surface);
    if let Some(trimmed) = surface.cast::<GeomRectangularTrimmedSurface>().get() {
        let (u1, u2, v1, v2) = trimmed.bounds();
        [u1, u2, v1, v2]
            .into_iter()
            .for_each(|value| p::real(output, value));
        return geom_surface(output, &trimmed.basis_surface());
    }
    if let Some(x) = surface.cast::<GeomPlane>().get() {
        p::pln(output, x.pln());
    } else if let Some(x) = surface.cast::<GeomConicalSurface>().get() {
        p::cone(output, x.cone().unwrap());
    } else if let Some(x) = surface.cast::<GeomCylindricalSurface>().get() {
        p::cylinder(output, x.cylinder().unwrap());
    }
}

/// gb_geom_trsf and gb_geom2d_trsf.
pub fn geom_trsf(output: &Output, handle: &Handle<GeomTransformation>) {
    name(output, handle);
    if let Some(t) = handle.get() {
        p::trsf(output, t.trsf());
    }
}
pub fn geom2d_trsf(output: &Output, handle: &Handle<Geom2dTransformation>) {
    name(output, handle);
    if let Some(t) = handle.get() {
        p::trsf2d(output, t.trsf2d());
    }
}

/// The conversion operator of a Root algorithm, typed like its `value()`.
pub fn converted<M, H>(maker: &M, _like: &Result<H, Failure>) -> Result<H, Failure>
where
    for<'a> H: TryFrom<&'a M, Error = Failure>,
{
    H::try_from(maker)
}

/// geometry construction serialization inside fallible result capture: status, IsDone, IsError, Value() and the conversion.
macro_rules! gc_root {
    ($output:expr, $print:path, $maker:expr) => {
        super::super::common::attempt($output, || {
            use martensite_tkgeombase::gc::Root;
            let maker = $maker?;
            super::super::common::int($output, maker.status() as i32);
            super::super::common::flag($output, maker.is_done());
            super::super::common::flag($output, maker.is_error());
            let value = maker.value();
            let conversion = super::converted(&maker, &value);
            super::super::common::show($output, value, |h| $print($output, &h));
            super::super::common::show($output, conversion, |h| $print($output, &h));
            Ok(())
        })
    };
}

/// geometry transformation serialization inside fallible result capture: Value() and the conversion of a transformation maker.
macro_rules! gc_trsf {
    ($output:expr, $print:path, $maker:expr) => {
        super::super::common::attempt($output, || {
            let maker = $maker?;
            $print($output, &maker.value());
            $print($output, &martensite_tkernel::standard::Handle::from(&maker));
            Ok(())
        })
    };
}
pub(super) use {gc_root, gc_trsf};

/// A maker that cannot raise, in the shape of the fallible ones.
pub(super) fn ok<T>(value: T) -> Result<T, Failure> {
    Ok(value)
}

/// The gp readers of the implementation (gb_read_*).
impl Reader<'_> {
    pub fn circ(&mut self) -> Circle3 {
        let f = self.ax2();
        Circle3::new(f, self.real()).unwrap()
    }
    pub fn elips(&mut self) -> Ellipse3 {
        let (f, a) = (self.ax2(), self.real());
        Ellipse3::new(f, a, self.real()).unwrap()
    }
    pub fn hypr(&mut self) -> Hyperbola3 {
        let (f, a) = (self.ax2(), self.real());
        Hyperbola3::new(f, a, self.real()).unwrap()
    }
    pub fn parab(&mut self) -> Parabola3 {
        let f = self.ax2();
        Parabola3::new(f, self.real()).unwrap()
    }
    pub fn circ2d(&mut self) -> Circle2 {
        let f = self.ax22d();
        Circle2::new(f, self.real()).unwrap()
    }
    pub fn elips2d(&mut self) -> Ellipse2 {
        let (f, a) = (self.ax22d(), self.real());
        Ellipse2::new(f, a, self.real()).unwrap()
    }
    pub fn hypr2d(&mut self) -> Hyperbola2 {
        let (f, a) = (self.ax22d(), self.real());
        Hyperbola2::new(f, a, self.real()).unwrap()
    }
    pub fn parab2d(&mut self) -> Parabola2 {
        let f = self.ax22d();
        Parabola2::new(f, self.real()).unwrap()
    }
}
