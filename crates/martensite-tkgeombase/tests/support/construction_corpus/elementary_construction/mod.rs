//! The gce package probe; the counterpart of the implementation,
//! split by the kind of result.
mod curves;
mod surfaces;
mod transformations;
pub use curves::*;
pub use surfaces::*;
pub use transformations::*;

/// gb_gce_root inside GB_TRY: status, IsDone, IsError, Value() and Operator(), or the
/// exception that the constructor raised.
macro_rules! gce_root {
    ($output:expr, $print:path, $maker:expr) => {
        super::super::common::attempt($output, || {
            let maker = $maker?;
            super::super::common::int($output, maker.status() as i32);
            super::super::common::flag($output, maker.is_done());
            super::super::common::flag($output, maker.is_error());
            super::super::common::show($output, maker.value(), |value| $print($output, value));
            super::super::common::show($output, maker.operator(), |value| $print($output, value));
            Ok(())
        })
    };
}

/// gb_gce_trsf inside GB_TRY: Value() and Operator() of a transformation maker.
macro_rules! gce_trsf {
    ($output:expr, $print:path, $maker:expr) => {
        super::super::common::attempt($output, || {
            let maker = $maker?;
            $print($output, maker.value());
            $print($output, maker.operator());
            Ok(())
        })
    };
}

/// A maker that cannot raise, in the shape of the fallible ones.
pub(super) fn ok<T>(value: T) -> Result<T, martensite_tkernel::standard::Failure> {
    Ok(value)
}
pub(super) use {gce_root, gce_trsf};
