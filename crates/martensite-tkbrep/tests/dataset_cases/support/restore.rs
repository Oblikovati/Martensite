//! `restore` and `binrestore` on the Rust API: the variable-file reader, the
//! geometry and `DBRep_DrawableShape` `Restore` factories and the binary shape reader.
//!
//! `restore` reads the first word of the file as the drawable type and lets that type's
//! factory read the rest; an unknown type rereads the whole file as a
//! `DBRep_DrawableShape`, which is what a header-less `.brep` file gets.
use super::dataset::located_bytes;
use martensite_tkbrep::{bintools, brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, standard::Handle, storage::IoStream};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkgeombase::geom_tools;

/// A restored case variable.
#[derive(Clone)]
pub enum Restored {
    Shape(Shape),
    Curve(Handle<dyn GeomCurve>),
    Curve2d(Handle<dyn Geom2dCurve>),
    Surface(Handle<dyn GeomSurface>),
}

impl From<Shape> for Restored {
    fn from(shape: Shape) -> Self {
        Restored::Shape(shape)
    }
}

impl From<Handle<dyn GeomCurve>> for Restored {
    fn from(curve: Handle<dyn GeomCurve>) -> Self {
        Restored::Curve(curve)
    }
}

impl From<Handle<dyn Geom2dCurve>> for Restored {
    fn from(curve: Handle<dyn Geom2dCurve>) -> Self {
        Restored::Curve2d(curve)
    }
}

impl From<Handle<dyn GeomSurface>> for Restored {
    fn from(surface: Handle<dyn GeomSurface>) -> Self {
        Restored::Surface(surface)
    }
}

/// `restore name var`, with the variable set in `session`.
pub fn restore_as(session: &mut dyn super::session::Session, name: &str, var: &str) -> Restored {
    let restored = restore(name);
    session.set(var, &restored);
    restored
}

impl Restored {
    /// The restored shape; panics with the restored kind otherwise.
    pub fn shape(self) -> Shape {
        match self {
            Restored::Shape(shape) => shape,
            other => panic!("restored {}; expected a shape", other.kind()),
        }
    }

    /// The restored 3D curve.
    pub fn curve(self) -> Handle<dyn GeomCurve> {
        match self {
            Restored::Curve(curve) => curve,
            other => panic!("restored {}; expected a 3d curve", other.kind()),
        }
    }

    /// The restored 2D curve.
    pub fn curve2d(self) -> Handle<dyn Geom2dCurve> {
        match self {
            Restored::Curve2d(curve) => curve,
            other => panic!("restored {}; expected a 2d curve", other.kind()),
        }
    }

    /// The restored surface.
    pub fn surface(self) -> Handle<dyn GeomSurface> {
        match self {
            Restored::Surface(surface) => surface,
            other => panic!("restored {}; expected a surface", other.kind()),
        }
    }

    fn kind(&self) -> &'static str {
        match self {
            Restored::Shape(_) => "a shape",
            Restored::Curve(_) => "a 3d curve",
            Restored::Curve2d(_) => "a 2d curve",
            Restored::Surface(_) => "a surface",
        }
    }
}

/// `restore name var`.
pub fn restore(name: &str) -> Restored {
    let bytes = located_bytes(name);
    let (kind, rest) = first_word(&bytes);
    let mut stream = IoStream::from_bytes(rest.to_vec());
    match kind {
        "DrawTrSurf_Curve" | "DrawTrSurf_BSplineCurve" | "DrawTrSurf_BezierCurve" => {
            Restored::Curve(geom_tools::read_curve(&mut stream))
        }
        "DrawTrSurf_Curve2d" | "DrawTrSurf_BSplineCurve2d" | "DrawTrSurf_BezierCurve2d" => {
            Restored::Curve2d(geom_tools::read_curve2d(&mut stream))
        }
        "DrawTrSurf_Surface" | "DrawTrSurf_BSplineSurface" | "DrawTrSurf_BezierSurface" => {
            Restored::Surface(geom_tools::read_surface(&mut stream))
        }
        "DBRep_DrawableShape" => Restored::Shape(drawable_shape(&mut stream)),
        _ => Restored::Shape(drawable_shape(&mut IoStream::from_bytes(bytes))),
    }
}

/// `binrestore name var`: `BinTools::Read` of the whole file.
pub fn binrestore(name: &str) -> Shape {
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(located_bytes(name));
    bintools::read(&mut shape, &mut stream, &ProgressRange::default())
        .unwrap_or_else(|failure| panic!("binrestore {name}: {failure:?}"));
    shape
}

/// `DBRep_DrawableShape::Restore`: a BRepTools shape set, then the shape reference.
fn drawable_shape(stream: &mut IoStream) -> Shape {
    let mut shape = Shape::default();
    brep_tools::read(
        &mut shape,
        stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap_or_else(|failure| panic!("restore of a DBRep_DrawableShape: {failure:?}"));
    shape
}

/// `*aStream >> aType`: the first whitespace-delimited word and the bytes after it.
fn first_word(bytes: &[u8]) -> (&str, &[u8]) {
    let start = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let end = bytes[start..]
        .iter()
        .position(u8::is_ascii_whitespace)
        .map_or(bytes.len(), |length| start + length);
    let word = std::str::from_utf8(&bytes[start..end]).unwrap_or("");
    (word, &bytes[end..])
}
