//! The lists of a dump that depend on its level.
use super::dump::each_of;
use super::{
    IgesDataDumpError, IgesDataDumpResult, IgesDataDumpValue, IgesDataIgesDumper,
    iges_dump_list_header, iges_dump_xy, iges_dump_xy_t, iges_dump_xy_tz, iges_dump_xyz,
    iges_dump_xyz_t,
};
use martensite_tkernel::{
    standard::{Failure, Handle, RuntimeTyped},
    tcollection::HAsciiString,
};
use martensite_tkmath::gp::{GeneralTransform3, TransformForm, Xy, Xyz};
use martensite_tkxsbase::interface::InterfaceMsg;
use std::io::Write;

const ASK_LEVEL_CONTENT: &str = " [content : ask level > 4]";
const ASK_LEVEL_TRANSFORMED: &str = " [content : ask level > 4, transformed : level > 5]";

/// Writes the head of a list that depends on the level; true when the items are to be written.
fn list_head(
    s: &mut dyn Write,
    level: i32,
    lower: i32,
    upper: i32,
    ask: &str,
) -> Result<bool, IgesDataDumpError> {
    iges_dump_list_header(s, lower, upper)?;
    if lower > upper {
        return Ok(false);
    }
    if level == 4 || level == -4 {
        write!(s, "{ask}")?;
        return Ok(false);
    }
    Ok(level > 0)
}

/// Writes a list of values according to the level: its limits at 4, its items above.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_vals<V: IgesDataDumpValue>(
    s: &mut dyn Write,
    level: i32,
    lower: i32,
    upper: i32,
    mut item: impl FnMut(i32) -> Result<V, Failure>,
) -> IgesDataDumpResult {
    if !list_head(s, level, lower, upper, ASK_LEVEL_CONTENT)? {
        return Ok(());
    }
    write!(s, " :")?;
    each_of(lower, upper, |index| {
        write!(s, " {}", item(index)?.dump_text())?;
        Ok(())
    })
}

/// Writes a list of coordinates according to the level, and its transformed items from level 6
/// when `transformed_item` writes them.
fn dump_points(
    s: &mut dyn Write,
    level: i32,
    lower: i32,
    upper: i32,
    trsf: &GeneralTransform3,
    mut direct_item: impl FnMut(&mut dyn Write, i32) -> IgesDataDumpResult,
    mut transformed_item: impl FnMut(&mut dyn Write, i32) -> IgesDataDumpResult,
) -> IgesDataDumpResult {
    if !list_head(s, level, lower, upper, ASK_LEVEL_TRANSFORMED)? {
        return Ok(());
    }
    write!(s, " :")?;
    for index in lower..=upper {
        direct_item(&mut *s, index)?;
    }
    if trsf.form() == TransformForm::Identity {
        return Ok(());
    }
    write!(s, "\n Transformed :")?;
    if level == 5 {
        write!(s, " [ask level > 5]")?;
        return Ok(());
    }
    for index in lower..=upper {
        transformed_item(&mut *s, index)?;
    }
    Ok(())
}

/// Writes a list of couples of coordinates according to the level.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_list_xy_l(
    s: &mut dyn Write,
    level: i32,
    lower: i32,
    upper: i32,
    item: impl Fn(i32) -> Result<Xy, Failure>,
    trsf: &GeneralTransform3,
) -> IgesDataDumpResult {
    dump_points(
        s,
        level,
        lower,
        upper,
        trsf,
        |out, index| iges_dump_xy(out, &item(index)?),
        |out, index| iges_dump_xy_t(out, &item(index)?, trsf),
    )
}

/// Writes a list of couples of coordinates, with a third one in their transformed value.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_list_xy_lz(
    s: &mut dyn Write,
    level: i32,
    lower: i32,
    upper: i32,
    item: impl Fn(i32) -> Result<Xy, Failure>,
    trsf: &GeneralTransform3,
    z: f64,
) -> IgesDataDumpResult {
    dump_points(
        s,
        level,
        lower,
        upper,
        trsf,
        |out, index| iges_dump_xy(out, &item(index)?),
        |out, index| iges_dump_xy_tz(out, &item(index)?, trsf, z),
    )
}

/// Writes a list of triples of coordinates according to the level.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_list_xyz_l(
    s: &mut dyn Write,
    level: i32,
    lower: i32,
    upper: i32,
    item: impl Fn(i32) -> Result<Xyz, Failure>,
    trsf: &GeneralTransform3,
) -> IgesDataDumpResult {
    dump_points(
        s,
        level,
        lower,
        upper,
        trsf,
        |out, index| iges_dump_xyz(out, &item(index)?),
        |out, index| iges_dump_xyz_t(out, &item(index)?, trsf),
    )
}

/// Writes a list of texts according to the level, one per line.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_strings(
    s: &mut dyn Write,
    level: i32,
    lower: i32,
    upper: i32,
    mut item: impl FnMut(i32) -> Result<Handle<HAsciiString>, Failure>,
) -> IgesDataDumpResult {
    if !list_head(s, level, lower, upper, ASK_LEVEL_CONTENT)? {
        return Ok(());
    }
    write!(s, " :")?;
    for index in lower..=upper {
        write!(
            s,
            "\n[{}{index}]:\"",
            InterfaceMsg::blanks_for_integer(index, 3)
        )?;
        if let Some(text) = item(index)?.get() {
            s.write_all(text.string().as_bytes())?;
        }
        write!(s, "\"")?;
    }
    writeln!(s)?;
    Ok(())
}

/// Writes a list of entities according to the level: by their numbers at 5, longer above.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_entities<T: ?Sized + RuntimeTyped>(
    s: &mut dyn Write,
    dumper: &IgesDataIgesDumper,
    level: i32,
    lower: i32,
    upper: i32,
    mut item: impl FnMut(i32) -> Result<Handle<T>, Failure>,
) -> IgesDataDumpResult {
    if !list_head(s, level, lower, upper, ASK_LEVEL_CONTENT)? {
        return Ok(());
    }
    write!(s, " :")?;
    for index in lower..=upper {
        if level == 5 {
            write!(s, " ")?;
            dumper.print_d_num(&item(index)?, s)?;
        } else {
            write!(
                s,
                "\n[{}{index}]:",
                InterfaceMsg::blanks_for_integer(index, 3)
            )?;
            dumper.print_short(&item(index)?, s)?;
        }
    }
    Ok(())
}

/// Writes a rectangular table according to the level, row by row.
///
/// # Errors
/// A failure of the stream or of an item.
pub fn iges_dump_rect_vals<V: IgesDataDumpValue>(
    s: &mut dyn Write,
    level: i32,
    (low_col, up_col): (i32, i32),
    (low_row, up_row): (i32, i32),
    mut item: impl FnMut(i32, i32) -> Result<V, Failure>,
) -> IgesDataDumpResult {
    write!(
        s,
        " (Row :{low_row} - {up_row} ; Col :{low_col} - {up_col})"
    )?;
    if low_col > up_col || low_row > up_row {
        return Ok(());
    }
    if level == 4 || level == -4 {
        write!(s, "{ASK_LEVEL_CONTENT}")?;
        return Ok(());
    }
    if level <= 0 {
        return Ok(());
    }
    writeln!(s)?;
    for row in low_row..=up_row {
        write!(s, "Row {row}:[")?;
        for col in low_col..=up_col {
            write!(s, " {}", item(col, row)?.dump_text())?;
        }
        writeln!(s, " ]")?;
    }
    Ok(())
}
