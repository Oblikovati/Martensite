//! The case variable commands the cases call: `copy`,
//! `renamevar`, `dump`, `isdraw` and `whatis`, plus `point` and `dval` arithmetic.
use crate::support::numbers::{e, g17};
use crate::support::script::{Reply, Script, ScriptStep};
use crate::support::value::Value;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::IoStream;
use martensite_tkgeombase::geom_tools;
use martensite_tkmath::gp::Point3;
use std::io::{Read, Seek, SeekFrom};

/// `copy from to`: the variable copy, a deep copy of geometry and the same shape.
pub fn copy(d: &mut Script, from: &str, to: &str) -> ScriptStep {
    d.command(&["copy", from, to], |d| {
        let Some(value) = d.get(from).cloned() else {
            return Ok(Reply::ok(""));
        };
        d.set(to, copied(value)?);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

fn copied(value: Value) -> Result<Value, Failure> {
    Ok(match value {
        Value::Curve(curve) => Value::Curve(curve.get().expect("copy").copy()?.cast()),
        Value::Curve2d(curve) => Value::Curve2d(curve.get().expect("copy").copy()?.cast()),
        Value::Surface(surface) => Value::Surface(surface.get().expect("copy").copy()?.cast()),
        other => other,
    })
}

/// `renamevar from to`.
pub fn renamevar(d: &mut Script, from: &str, to: &str) -> ScriptStep {
    d.command(&["renamevar", from, to], |d| {
        d.rename(from, to);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `isdraw name`: 1 or 0.
pub fn isdraw(d: &mut Script, name: &str) -> ScriptStep<bool> {
    let defined = d.isdraw(name);
    d.command(&["isdraw", name], |_| {
        Ok(Reply::ok(if defined { "1" } else { "0" }))
    })?;
    Ok(defined)
}

/// `whatis name`: `name is a [dtyp name]\n`, where `dtyp` prints
/// the drawable's `Whatis` (nothing for an undefined variable).
pub fn whatis(d: &mut Script, name: &str) -> ScriptStep<String> {
    let text = d.whatis(name);
    let dtyp = d.command(&["dtyp", name], |_| Ok(Reply::ok(text)))?;
    Ok(format!("{name} is a {dtyp}\n"))
}

/// `decho off; dlog reset; dlog on; dump name; set info [dlog get]; dlog reset;
/// dlog off; decho on`: the dump text captured without reaching the test log.
pub fn logged_dump(d: &mut Script, name: &str) -> ScriptStep<String> {
    d.log.set_echo(false);
    d.log.reset();
    d.log.set_logging(true);
    dump(d, &[name])?;
    let info = d.log.buffer().to_string();
    d.log.reset();
    d.log.set_logging(false);
    d.log.set_echo(true);
    Ok(info)
}

/// `point name x y z`: a 3D point variable.
pub fn point(d: &mut Script, name: &str, xyz: [f64; 3]) -> ScriptStep {
    let [x, y, z] = xyz.map(g17);
    d.command(&["point", name, &x, &y, &z], |d| {
        d.set(name, Point3::new(xyz[0], xyz[1], xyz[2]));
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `dump names...`: the banner and the dump of each variable at precision
/// 15.
pub fn dump(d: &mut Script, names: &[&str]) -> ScriptStep<String> {
    let args: Vec<&str> = std::iter::once("dump")
        .chain(names.iter().copied())
        .collect();
    d.command(&args, |d| {
        let mut text = String::new();
        for name in names {
            if let Some(value) = d.get(name) {
                text.push_str(&format!("\n\n*********** Dump of {name} *************\n"));
                text.push_str(&dumped(value)?);
            }
        }
        Ok(Reply::ok(text))
    })
}

/// The dump of one value.
fn dumped(value: &Value) -> Result<String, Failure> {
    Ok(match value {
        Value::Point(p) => format!(
            "Point : {}, {}, {}\n",
            e(p.x(), 15),
            e(p.y(), 15),
            e(p.z(), 15)
        ),
        Value::Point2d(p) => format!("Point 2d : {}, {}\n", e(p.x(), 15), e(p.y(), 15)),
        Value::Number(number) => crate::support::numbers::stream(*number, 15),
        Value::Curve(curve) => printed(|s| geom_tools::dump_curve(curve, s))?,
        Value::Curve2d(curve) => printed(|s| geom_tools::dump_curve2d(curve, s))?,
        Value::Surface(surface) => printed(|s| geom_tools::dump_surface(surface, s))?,
        Value::Shape(_) => panic!("dump of a shape: no case dumps one"),
    })
}

/// The text `print` writes on a stream at precision 15.
fn printed(print: impl FnOnce(&mut IoStream) -> Result<(), Failure>) -> Result<String, Failure> {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    print(&mut stream)?;
    let mut device = stream.into_device().expect("in-memory dump stream");
    device
        .seek(SeekFrom::Start(0))
        .expect("rewind the dump stream");
    let mut body = String::new();
    device.read_to_string(&mut body).expect("dump text");
    Ok(body)
}
