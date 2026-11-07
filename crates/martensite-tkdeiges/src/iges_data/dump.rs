//! The pieces the dumps of entities share: coordinates, with their transformed value at a high
//! level, and lists of values, more or less complete according to the level.
//!
//! A list item is a closure from its index, which fails as the accessor of the entity does.
use super::IgesDataDumpResult;
use martensite_tkernel::{
    standard::{Failure, Handle},
    storage::format_general,
    tcollection::HAsciiString,
};
use martensite_tkmath::gp::{GeneralTransform3, TransformForm, Xy, Xyz};
use std::io::Write;

/// A real as a default text stream writes it: six digits at most.
///
/// ```
/// use martensite_tkdeiges::iges_data::iges_real_text;
/// assert_eq!(iges_real_text(0.5), "0.5");
/// ```
pub fn iges_real_text(value: f64) -> String {
    format_general(value, 6)
}

/// A real or an integer of a dump.
pub trait IgesDataDumpValue {
    /// Its text.
    fn dump_text(&self) -> String;
}

impl IgesDataDumpValue for f64 {
    fn dump_text(&self) -> String {
        iges_real_text(*self)
    }
}

impl IgesDataDumpValue for i32 {
    fn dump_text(&self) -> String {
        self.to_string()
    }
}

impl IgesDataDumpValue for bool {
    fn dump_text(&self) -> String {
        i32::from(*self).to_string()
    }
}

/// Writes a text between quotes, or `(undefined)` for null.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_string(s: &mut dyn Write, text: &Handle<HAsciiString>) -> IgesDataDumpResult {
    match text.get() {
        None => write!(s, "(undefined)")?,
        Some(live) => {
            write!(s, "\"")?;
            s.write_all(live.string().as_bytes())?;
            write!(s, "\"")?;
        }
    }
    Ok(())
}

/// Writes two coordinates.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_xy(s: &mut dyn Write, xy: &Xy) -> IgesDataDumpResult {
    write!(
        s,
        " ({},{})",
        iges_real_text(xy.x()),
        iges_real_text(xy.y())
    )?;
    Ok(())
}

/// Writes three coordinates.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_xyz(s: &mut dyn Write, xyz: &Xyz) -> IgesDataDumpResult {
    write!(
        s,
        " ({},{},{})",
        iges_real_text(xyz.x()),
        iges_real_text(xyz.y()),
        iges_real_text(xyz.z())
    )?;
    Ok(())
}

/// Writes two coordinates once transformed; the third is 0.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_xy_t(s: &mut dyn Write, xy: &Xy, trsf: &GeneralTransform3) -> IgesDataDumpResult {
    let moved = trsf.apply(Xyz::new(xy.x(), xy.y(), 0.0));
    iges_dump_xy(s, &Xy::new(moved.x(), moved.y()))
}

/// Writes two coordinates and a third once transformed.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_xy_tz(
    s: &mut dyn Write,
    xy: &Xy,
    trsf: &GeneralTransform3,
    z: f64,
) -> IgesDataDumpResult {
    iges_dump_xyz(s, &trsf.apply(Xyz::new(xy.x(), xy.y(), z)))
}

/// Writes three coordinates once transformed.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_xyz_t(
    s: &mut dyn Write,
    xyz: &Xyz,
    trsf: &GeneralTransform3,
) -> IgesDataDumpResult {
    iges_dump_xyz(s, &trsf.apply(*xyz))
}

fn shows_transformed(level: i32, trsf: &GeneralTransform3) -> bool {
    level > 5 && trsf.form() != TransformForm::Identity
}

/// Writes two coordinates, and from level 6 their transformed value unless it is the same.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_xy_l(
    s: &mut dyn Write,
    level: i32,
    xy: &Xy,
    trsf: &GeneralTransform3,
) -> IgesDataDumpResult {
    iges_dump_xy(s, xy)?;
    if shows_transformed(level, trsf) {
        write!(s, "  Transformed :")?;
        iges_dump_xy_t(s, xy, trsf)?;
    }
    Ok(())
}

/// [`iges_dump_xy_l`], with a third coordinate in the transformed value.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_xy_lz(
    s: &mut dyn Write,
    level: i32,
    xy: &Xy,
    trsf: &GeneralTransform3,
    z: f64,
) -> IgesDataDumpResult {
    iges_dump_xy(s, xy)?;
    if shows_transformed(level, trsf) {
        write!(s, "  Transformed :")?;
        iges_dump_xy_tz(s, xy, trsf, z)?;
    }
    Ok(())
}

/// Writes three coordinates, and from level 6 their transformed value unless it is the same.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_xyz_l(
    s: &mut dyn Write,
    level: i32,
    xyz: &Xyz,
    trsf: &GeneralTransform3,
) -> IgesDataDumpResult {
    iges_dump_xyz(s, xyz)?;
    if shows_transformed(level, trsf) {
        write!(s, "  Transformed :")?;
        iges_dump_xyz_t(s, xyz, trsf)?;
    }
    Ok(())
}

/// Writes how long a list is.
///
/// # Errors
/// A failure of the stream.
pub fn iges_dump_list_header(s: &mut dyn Write, lower: i32, upper: i32) -> IgesDataDumpResult {
    if lower > upper {
        write!(s, " (Empty List)")?;
    } else if lower == 1 {
        write!(s, " (Count : {upper})")?;
    } else {
        write!(s, " ({lower} - {upper})")?;
    }
    Ok(())
}

pub(super) fn each_of(
    lower: i32,
    upper: i32,
    mut write_one: impl FnMut(i32) -> IgesDataDumpResult,
) -> IgesDataDumpResult {
    for index in lower..=upper {
        write_one(index)?;
    }
    Ok(())
}

/// Writes a whole list of values, whatever the level.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_list_val<V: IgesDataDumpValue>(
    s: &mut dyn Write,
    lower: i32,
    upper: i32,
    mut item: impl FnMut(i32) -> Result<V, Failure>,
) -> IgesDataDumpResult {
    iges_dump_list_header(s, lower, upper)?;
    write!(s, " :")?;
    each_of(lower, upper, |index| {
        write!(s, " {}", item(index)?.dump_text())?;
        Ok(())
    })
}

/// Writes a whole list of couples of coordinates, whatever the level.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_list_xy(
    s: &mut dyn Write,
    lower: i32,
    upper: i32,
    mut item: impl FnMut(i32) -> Result<Xy, Failure>,
) -> IgesDataDumpResult {
    iges_dump_list_header(s, lower, upper)?;
    write!(s, " :")?;
    each_of(lower, upper, |index| iges_dump_xy(s, &item(index)?))
}

/// Writes a whole list of triples of coordinates, whatever the level.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_list_xyz(
    s: &mut dyn Write,
    lower: i32,
    upper: i32,
    mut item: impl FnMut(i32) -> Result<Xyz, Failure>,
) -> IgesDataDumpResult {
    iges_dump_list_header(s, lower, upper)?;
    write!(s, " :")?;
    each_of(lower, upper, |index| iges_dump_xyz(s, &item(index)?))
}
