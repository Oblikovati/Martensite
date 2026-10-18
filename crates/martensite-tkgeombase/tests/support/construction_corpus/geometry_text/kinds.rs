//! The three geometry families of GeomTools behind one interface, so the probe cases are
//! written once, as the C++ templates over GtCurves, GtCurves2d and GtSurfaces are.
use super::super::common::{Reader, attempt, pnt, pnt2d};
use super::super::geometry_record_fixtures as build;
use super::console::Context;
use super::console::attempt_captured;
use super::console::captured;
use super::text;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::storage::IoStream;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkgeombase::geom_tools::{self, Curve2dSet, CurveSet, SurfaceSet};

/// One geometry family: its handle, its set class and the GeomTools members on them.
pub trait GtKind {
    type Geometry: ?Sized + martensite_tkernel::standard::RuntimeTyped;
    type Set;
    fn build(r: &mut Reader<'_>) -> Result<Handle<Self::Geometry>, Failure>;
    fn dump(g: &Handle<Self::Geometry>, s: &mut IoStream) -> Result<(), Failure>;
    fn write(g: &Handle<Self::Geometry>, s: &mut IoStream) -> Result<(), Failure>;
    fn read(s: &mut IoStream) -> Handle<Self::Geometry>;
    /// The bits of the point at (0.3) or (0.3, 0.2).
    fn value(ctx: &Context, g: &Handle<Self::Geometry>) -> Result<(), Failure>;
    fn new_set(ctx: &Context) -> Self::Set;
    fn add(set: &mut Self::Set, g: &Handle<Self::Geometry>) -> i32;
    fn index(set: &Self::Set, g: &Handle<Self::Geometry>) -> i32;
    fn get(set: &Self::Set, index: i32) -> Handle<Self::Geometry>;
    fn dump_set(set: &Self::Set, s: &mut IoStream) -> Result<(), Failure>;
    fn write_set(set: &Self::Set, s: &mut IoStream, p: &ProgressRange<'_>) -> Result<(), Failure>;
    fn read_set(set: &mut Self::Set, s: &mut IoStream, p: &ProgressRange<'_>);
    fn clear(set: &mut Self::Set);
}

/// `gt_summary`: type name (or `null`), compact text at precision 17 with its console
/// output, and a point.
pub fn summary<K: GtKind>(ctx: &Context, g: &Handle<K::Geometry>) {
    let Some(kind) = g.dynamic_type() else {
        ctx.output.write(format_args!("null "));
        return;
    };
    ctx.output.write(format_args!("{} ", kind.name()));
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(17);
    let console = captured(ctx, || attempt_captured(ctx, || K::write(g, &mut stream)));
    text(ctx, &super::written_bytes(stream));
    text(ctx, &console);
    attempt(ctx.output, || K::value(ctx, g));
}

macro_rules! set_members {
    ($set:ty, $geometry:ty, $get:ident) => {
        type Set = $set;
        fn new_set(ctx: &Context) -> $set {
            <$set>::with_console(ctx.console())
        }
        fn add(set: &mut $set, g: &Handle<$geometry>) -> i32 {
            set.add(g)
        }
        fn index(set: &$set, g: &Handle<$geometry>) -> i32 {
            set.index(g)
        }
        fn get(set: &$set, index: i32) -> Handle<$geometry> {
            set.$get(index)
        }
        fn dump_set(set: &$set, s: &mut IoStream) -> Result<(), Failure> {
            set.dump(s)
        }
        fn write_set(set: &$set, s: &mut IoStream, p: &ProgressRange<'_>) -> Result<(), Failure> {
            set.write(s, p)
        }
        fn read_set(set: &mut $set, s: &mut IoStream, p: &ProgressRange<'_>) {
            set.read(s, p)
        }
        fn clear(set: &mut $set) {
            set.clear()
        }
    };
}

pub struct Curves;
impl GtKind for Curves {
    type Geometry = dyn GeomCurve;
    set_members!(CurveSet, dyn GeomCurve, curve);
    fn build(r: &mut Reader<'_>) -> Result<Handle<dyn GeomCurve>, Failure> {
        build::curve(r)
    }
    fn dump(g: &Handle<dyn GeomCurve>, s: &mut IoStream) -> Result<(), Failure> {
        geom_tools::dump_curve(g, s)
    }
    fn write(g: &Handle<dyn GeomCurve>, s: &mut IoStream) -> Result<(), Failure> {
        geom_tools::write_curve(g, s)
    }
    fn read(s: &mut IoStream) -> Handle<dyn GeomCurve> {
        geom_tools::read_curve(s)
    }
    fn value(ctx: &Context, g: &Handle<dyn GeomCurve>) -> Result<(), Failure> {
        pnt(ctx.output, g.value(0.3)?);
        Ok(())
    }
}

pub struct Curves2d;
impl GtKind for Curves2d {
    type Geometry = dyn Geom2dCurve;
    set_members!(Curve2dSet, dyn Geom2dCurve, curve2d);
    fn build(r: &mut Reader<'_>) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        build::curve2d(r)
    }
    fn dump(g: &Handle<dyn Geom2dCurve>, s: &mut IoStream) -> Result<(), Failure> {
        geom_tools::dump_curve2d(g, s)
    }
    fn write(g: &Handle<dyn Geom2dCurve>, s: &mut IoStream) -> Result<(), Failure> {
        geom_tools::write_curve2d(g, s)
    }
    fn read(s: &mut IoStream) -> Handle<dyn Geom2dCurve> {
        geom_tools::read_curve2d(s)
    }
    fn value(ctx: &Context, g: &Handle<dyn Geom2dCurve>) -> Result<(), Failure> {
        pnt2d(ctx.output, g.value(0.3)?);
        Ok(())
    }
}

pub struct Surfaces;
impl GtKind for Surfaces {
    type Geometry = dyn GeomSurface;
    set_members!(SurfaceSet, dyn GeomSurface, surface);
    fn build(r: &mut Reader<'_>) -> Result<Handle<dyn GeomSurface>, Failure> {
        build::surface(r)
    }
    fn dump(g: &Handle<dyn GeomSurface>, s: &mut IoStream) -> Result<(), Failure> {
        geom_tools::dump_surface(g, s)
    }
    fn write(g: &Handle<dyn GeomSurface>, s: &mut IoStream) -> Result<(), Failure> {
        geom_tools::write_surface(g, s)
    }
    fn read(s: &mut IoStream) -> Handle<dyn GeomSurface> {
        geom_tools::read_surface(s)
    }
    fn value(ctx: &Context, g: &Handle<dyn GeomSurface>) -> Result<(), Failure> {
        pnt(ctx.output, g.value(0.3, 0.2)?);
        Ok(())
    }
}
