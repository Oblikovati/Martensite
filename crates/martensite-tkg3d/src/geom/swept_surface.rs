//! `GeomSweptSurface`: surfaces swept by a basis curve along or around a
//! direction (the retained protected `basisCurve`, `direction` and `smooth`).
use super::curve::GeomCurve;
use super::geometry::GeomGeometry;
use super::surface::{GeomSurface, dump_surface};
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::{self, DumpStream};
use martensite_tkg2d::geom2d::{dump_base, dump_class_name, dump_field};
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::Direction3;
use std::io;

/// A swept surface; `extrusion.basis_curve()`.
pub trait GeomSweptSurface: GeomSurface {
    /// The sweep direction (`Direction`); `surface.direction()`.
    fn direction(&self) -> Direction3;
    /// The swept curve (`BasisCurve`); `surface.basis_curve()`.
    fn basis_curve(&self) -> Handle<dyn GeomCurve>;
}
impl_transient_interface!(dyn GeomSweptSurface, "Geom_SweptSurface", dyn GeomSurface);

/// `GeomSweptSurface::DumpJson`; `dump_swept_surface(&mut stream, -1, &basis, dir, smooth)?`.
pub fn dump_swept_surface(
    stream: &mut DumpStream,
    depth: i32,
    basis: &Handle<dyn GeomCurve>,
    direction: Direction3,
    smooth: Shape,
) -> io::Result<()> {
    dump_class_name(stream, "Geom_SweptSurface")?;
    dump_base(stream, depth, dump_surface)?;
    if let Some(curve) = basis.get() {
        dump_field(stream, depth, "basisCurve", |nested, level| {
            GeomGeometry::dump_json(curve, nested, level)
        })?;
    }
    dump_field(stream, depth, "direction", |nested, _| {
        direction.dump_json(nested)
    })?;
    dump::dump_integer(stream, "smooth", smooth as i32)
}
