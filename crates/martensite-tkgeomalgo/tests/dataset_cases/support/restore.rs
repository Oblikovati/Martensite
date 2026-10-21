//! The `restore` and `save` on the Rust API (`restore`/`save`,
//! the geometry and shape variable save and restore).
//!
//! `restore` reads the first word of the file as the drawable type and lets that type's
//! factory read the rest; an unknown type rereads the whole file as a
//! a shape, which is what a header-less `.brep` file gets.
use super::dataset::located_bytes;
use super::script::{Reply, Script, ScriptStep};
use super::value::Value;
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
    storage::IoStream,
};
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkgeombase::geom_tools;
use std::io::{Read, Seek, SeekFrom};

/// `restore name var`: the pinned dataset member `name` as `var`.
pub fn restore(d: &mut Script, name: &str, var: &str) -> ScriptStep {
    let bytes = located_bytes(name);
    d.command(&["restore", name, var], |d| {
        let value = restored(&bytes)?;
        if generic_drawable(&bytes) {
            d.set_generic(var, value);
        } else {
            d.set(var, value);
        }
        Ok(Reply::ok(var))
    })?;
    Ok(())
}

/// Whether the file names the generic `DrawTrSurf_Curve`, `DrawTrSurf_Curve2d` or
/// `DrawTrSurf_Surface` drawable, which draws any geometry without its poles and knots
/// (setting a variable picks the B-spline or Bezier drawable from the geometry instead).
fn generic_drawable(bytes: &[u8]) -> bool {
    matches!(
        first_word(bytes).0,
        "DrawTrSurf_Curve" | "DrawTrSurf_Curve2d" | "DrawTrSurf_Surface"
    )
}

/// `save curve file` then `restore file`: the restored variable is named after the file
/// (`restore` with one argument); the file text exists only in memory.
pub fn save_and_restore(d: &mut Script, curve: &str, file: &str) -> ScriptStep {
    d.command(&["bsave", curve, file], |_| Ok(Reply::ok(curve)))?;
    let saved = saved_curve(&d.curve(curve), "DrawTrSurf_BSplineCurve")
        .map_err(|failure| super::script::ScriptError(format!("save {curve}: {failure}")))?;
    d.command(&["brestore", file, file], |d| {
        d.set(file, restored(&saved)?);
        Ok(Reply::ok(file))
    })?;
    Ok(())
}

/// The drawable a `save` file holds.
pub fn restored(bytes: &[u8]) -> Result<Value, Failure> {
    let (kind, rest) = first_word(bytes);
    let mut stream = IoStream::from_bytes(rest.to_vec());
    Ok(match kind {
        "DrawTrSurf_Curve" | "DrawTrSurf_BSplineCurve" | "DrawTrSurf_BezierCurve" => {
            Value::Curve(geom_tools::read_curve(&mut stream))
        }
        "DrawTrSurf_Curve2d" | "DrawTrSurf_BSplineCurve2d" | "DrawTrSurf_BezierCurve2d" => {
            Value::Curve2d(geom_tools::read_curve2d(&mut stream))
        }
        "DrawTrSurf_Surface" | "DrawTrSurf_BSplineSurface" | "DrawTrSurf_BezierSurface" => {
            Value::Surface(geom_tools::read_surface(&mut stream))
        }
        "DBRep_DrawableShape" => Value::Shape(drawable_shape(&mut stream)?),
        _ => Value::Shape(drawable_shape(&mut IoStream::from_bytes(bytes.to_vec()))?),
    })
}

/// `save curve file` of a 3D curve variable: the drawable type, `GeomTools` text at
/// precision 15, then `\n0\n\n`. `drawable` is the saved variable's the case harness type name.
pub fn saved_curve(curve: &Handle<dyn GeomCurve>, drawable: &str) -> Result<Vec<u8>, Failure> {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    stream.write(format!("{drawable}\n").as_bytes());
    geom_tools::write_curve(curve, &mut stream)?;
    stream.write(b"\n0\n\n");
    let mut device = stream.into_device().expect("in-memory save stream");
    device
        .seek(SeekFrom::Start(0))
        .expect("rewind the save stream");
    let mut bytes = Vec::new();
    device.read_to_end(&mut bytes).expect("saved bytes");
    Ok(bytes)
}

/// The shape variable restore: a BRepTools shape set, then the shape reference.
fn drawable_shape(stream: &mut IoStream) -> Result<Shape, Failure> {
    let mut shape = Shape::default();
    brep_tools::read(
        &mut shape,
        stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )?;
    Ok(shape)
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
