//! The `DumpJson` shape shared by the four analytic eval curves: the class name,
//! the Geom2dCurve base, `&myPosition`, then the numeric parameters in order.
use crate::geom2d::{dump_base, dump_class_name, dump_curve, dump_field};
use martensite_tkernel::standard::dump::{self, DumpStream};
use martensite_tkmath::gp::Axis2d;
use std::io;

/// Write `className`, the curve base, `"Position"` and each `(name, value)` field;
/// `dump_analytic_curve(&mut s, -1, "Geom2dEvalSineWaveCurve", axis, &[("Amplitude", 2.0)])?`.
pub(crate) fn dump_analytic_curve(
    stream: &mut DumpStream,
    depth: i32,
    class_name: &str,
    position: Axis2d,
    fields: &[(&str, f64)],
) -> io::Result<()> {
    dump_class_name(stream, class_name)?;
    dump_base(stream, depth, dump_curve)?;
    dump_field(stream, depth, "Position", |nested, level| {
        position.dump_json(nested, level)
    })?;
    for &(name, value) in fields {
        dump::dump_real(stream, name, value)?;
    }
    Ok(())
}
