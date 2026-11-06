//! IgesGraphToolTextFontDef.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_basic::IgesBasicHArray1OfHArray1OfInteger;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::iges_read_integer_item;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_set,
    iges_dump_string, iges_dump_vals, iges_live, iges_new_array1,
};
use crate::iges_graph::IgesGraphTextFontDef;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xy;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolTextFontDef,
    "The own-parameter work on `IgesGraphTextFontDef`."
);

impl IgesGraphToolTextFontDef {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphTextFontDef>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        let mut font_code: i32 = Default::default();
        let mut font_name: Handle<HAsciiString> = Default::default();
        let mut superseded_font: i32 = Default::default();
        crate::iges_shared_blocks::iges_block_0005!(
            a_scii_codes,
            move_pen_x,
            move_pen_y,
            next_char_x,
            next_char_y,
            pen_flags,
            pen_motions,
            scale,
            superseded_entity
        );
        let mut temp_motion: i32 = Default::default();
        pr.read_integer_text(&pr.current(), "Font Code", &mut font_code)?;
        pr.read_text(&pr.current(), "Font Name", &mut font_name)?;
        if pr.is_param_entity(pr.current_number())? {
            superseded_font = -1;
            pr.read_entity_typed(
                ir,
                &pr.current(),
                "Text Definition Entity",
                <IgesGraphTextFontDef>::static_type(),
                &mut superseded_entity,
                false,
            )?;
        } else {
            pr.read_integer_text(
                &pr.current(),
                "No. of superseded font",
                &mut superseded_font,
            )?;
        }
        pr.read_integer_text(
            &pr.current(),
            "Grid units eqvt to one text height",
            &mut scale,
        )?;
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "No. of characters in this defn", &mut nbval)?;
        if st && (nbval > 0) {
            a_scii_codes = iges_new_array1::<i32>(1, nbval)?;
            next_char_x = iges_new_array1::<i32>(1, nbval)?;
            next_char_y = iges_new_array1::<i32>(1, nbval)?;
            pen_motions = iges_new_array1::<i32>(1, nbval)?;
            pen_flags = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbval)?);
            move_pen_x = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbval)?);
            move_pen_y = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbval)?);
            for i in 1..=nbval {
                iges_read_integer_item(
                    pr,
                    "array aSCIICodes",
                    &a_scii_codes,
                    i,
                    "IgesGraphToolTextFontDef::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "array nextChar X",
                    &next_char_x,
                    i,
                    "IgesGraphToolTextFontDef::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "array nextChar Y",
                    &next_char_y,
                    i,
                    "IgesGraphToolTextFontDef::read_own_params",
                )?;
                if pr.read_integer_text(&pr.current(), "array penMotions", &mut temp_motion)? {
                    iges_array1_set(
                        &pen_motions,
                        i,
                        temp_motion,
                        "IgesGraphToolTextFontDef::read_own_params",
                    )?;
                    if temp_motion > 0 {
                        let mut intarray: Handle<HArray1<i32>> = Default::default();
                        let mut xarray: Handle<HArray1<i32>> = Default::default();
                        let mut yarray: Handle<HArray1<i32>> = Default::default();
                        intarray = iges_new_array1::<i32>(1, temp_motion)?;
                        xarray = iges_new_array1::<i32>(1, temp_motion)?;
                        yarray = iges_new_array1::<i32>(1, temp_motion)?;
                        for j in 1..=temp_motion {
                            if pr.defined_else_skip()? {
                                iges_read_integer_item(
                                    pr,
                                    "array penFlags",
                                    &intarray,
                                    j,
                                    "IgesGraphToolTextFontDef::read_own_params",
                                )?;
                            } else {
                                iges_array1_set(
                                    &intarray,
                                    j,
                                    0,
                                    "IgesGraphToolTextFontDef::read_own_params",
                                )?;
                            }
                            iges_read_integer_item(
                                pr,
                                "array movePenTo X",
                                &xarray,
                                j,
                                "IgesGraphToolTextFontDef::read_own_params",
                            )?;
                            iges_read_integer_item(
                                pr,
                                "array movePenTo Y",
                                &yarray,
                                j,
                                "IgesGraphToolTextFontDef::read_own_params",
                            )?;
                        }
                        iges_live(&pen_flags, "IgesGraphToolTextFontDef::read_own_params")?
                            .set_value(i, &intarray)?;
                        iges_live(&move_pen_x, "IgesGraphToolTextFontDef::read_own_params")?
                            .set_value(i, &xarray)?;
                        iges_live(&move_pen_y, "IgesGraphToolTextFontDef::read_own_params")?
                            .set_value(i, &yarray)?;
                    } else {
                        pr.add_fail("Count of Pen motions : Not Positive", "");
                    }
                }
            }
        } else {
            pr.add_fail("Count of characters in this defn : Not Positive", "");
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            font_code,
            &font_name,
            superseded_font,
            &superseded_entity,
            scale,
            &a_scii_codes,
            &next_char_x,
            &next_char_y,
            &pen_motions,
            &pen_flags,
            &move_pen_x,
            &move_pen_y,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphTextFontDef>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut ix: i32 = Default::default();
        let mut iy: i32 = Default::default();
        iw.send(ent.font_code())?;
        iw.send(ent.font_name())?;
        if ent.is_superseded_font_entity() {
            iw.send_entity(&ent.superseded_font_entity(), true)?;
        } else {
            iw.send(ent.superseded_font_code())?;
        }
        iw.send(ent.scale())?;
        let mut up: i32 = ent.nb_characters()?;
        iw.send(up)?;
        for i in 1..=up {
            iw.send(ent.ascii_code(i)?)?;
            ent.next_char_origin(i, &mut ix, &mut iy)?;
            iw.send(ix)?;
            iw.send(iy)?;
            iw.send(ent.nb_pen_motions(i)?)?;
            {
                let mut j: i32 = 1;
                while j <= ent.nb_pen_motions(i)? {
                    iw.send_boolean(ent.is_pen_up(i, j)?)?;
                    ent.next_pen_position(i, j, &mut ix, &mut iy)?;
                    iw.send(ix)?;
                    iw.send(iy)?;
                    j += 1;
                }
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a TextFontDef `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGraphTextFontDef>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        if ent.is_superseded_font_entity() {
            iter.get_one_item(&ent.superseded_font_entity().cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesGraphTextFontDef>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(310, 0);
        crate::iges_shared_blocks::iges_block_0014!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesGraphTextFontDef>,
        _unused_1: &InterfaceShareTool,
        _unused_2: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesGraphTextFontDef>,
        ent: &Handle<IgesGraphTextFontDef>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nbval: i32 = Default::default();
        let mut font_code: i32 = Default::default();
        let mut font_name: Handle<HAsciiString> = Default::default();
        let mut superseded_font: i32 = 0;
        crate::iges_shared_blocks::iges_block_0005!(
            a_scii_codes,
            move_pen_x,
            move_pen_y,
            next_char_x,
            next_char_y,
            pen_flags,
            pen_motions,
            scale,
            superseded_entity
        );
        let mut temp_motion: i32 = Default::default();
        let mut intarray: Handle<HArray1<i32>> = Default::default();
        let mut xarray: Handle<HArray1<i32>> = Default::default();
        let mut yarray: Handle<HArray1<i32>> = Default::default();
        nbval = another.nb_characters()?;
        a_scii_codes = iges_new_array1::<i32>(1, nbval)?;
        next_char_x = iges_new_array1::<i32>(1, nbval)?;
        next_char_y = iges_new_array1::<i32>(1, nbval)?;
        pen_motions = iges_new_array1::<i32>(1, nbval)?;
        pen_flags = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbval)?);
        move_pen_x = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbval)?);
        move_pen_y = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, nbval)?);
        font_code = another.font_code();
        font_name = Handle::new(HAsciiString::copied(iges_live(
            &another.font_name(),
            "IgesGraphToolTextFontDef::own_copy",
        )?));
        if another.is_superseded_font_entity() {
            superseded_entity = tc
                .transferred(&another.superseded_font_entity().cast())?
                .cast::<IgesGraphTextFontDef>();
        } else {
            superseded_font = another.superseded_font_code();
        }
        scale = another.scale();
        let mut j: i32 = Default::default();
        let mut ix: i32 = Default::default();
        let mut iy: i32 = Default::default();
        for i in 1..=nbval {
            iges_array1_set(
                &a_scii_codes,
                i,
                another.ascii_code(i)?,
                "IgesGraphToolTextFontDef::own_copy",
            )?;
            another.next_char_origin(i, &mut ix, &mut iy)?;
            iges_array1_set(&next_char_x, i, ix, "IgesGraphToolTextFontDef::own_copy")?;
            iges_array1_set(&next_char_y, i, iy, "IgesGraphToolTextFontDef::own_copy")?;
            temp_motion = another.nb_pen_motions(i)?;
            iges_array1_set(
                &pen_motions,
                i,
                temp_motion,
                "IgesGraphToolTextFontDef::own_copy",
            )?;
            intarray = iges_new_array1::<i32>(1, temp_motion)?;
            xarray = iges_new_array1::<i32>(1, temp_motion)?;
            yarray = iges_new_array1::<i32>(1, temp_motion)?;
            for j in 1..=temp_motion {
                if another.is_pen_up(i, j)? {
                    iges_array1_set(&intarray, j, 1, "IgesGraphToolTextFontDef::own_copy")?;
                } else {
                    iges_array1_set(&intarray, j, 0, "IgesGraphToolTextFontDef::own_copy")?;
                }
                another.next_pen_position(i, j, &mut ix, &mut iy)?;
                iges_array1_set(&xarray, j, ix, "IgesGraphToolTextFontDef::own_copy")?;
                iges_array1_set(&yarray, j, iy, "IgesGraphToolTextFontDef::own_copy")?;
            }
            iges_live(&pen_flags, "IgesGraphToolTextFontDef::own_copy")?.set_value(i, &intarray)?;
            iges_live(&move_pen_x, "IgesGraphToolTextFontDef::own_copy")?.set_value(i, &xarray)?;
            iges_live(&move_pen_y, "IgesGraphToolTextFontDef::own_copy")?.set_value(i, &yarray)?;
        }
        ent.init(
            font_code,
            &font_name,
            superseded_font,
            &superseded_entity,
            scale,
            &a_scii_codes,
            &next_char_x,
            &next_char_y,
            &pen_motions,
            &pen_flags,
            &move_pen_x,
            &move_pen_y,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphTextFontDef>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        let mut nbchars: i32 = ent.nb_characters()?;
        write!(
            s,
            "IgesGraphTextFontDef\nFont Code : {}\nFont Name : ",
            ent.font_code()
        )?;
        iges_dump_string(s, &ent.font_name())?;
        writeln!(s)?;
        if ent.is_superseded_font_entity() {
            write!(s, "Text Definition Entity : ")?;
            dumper.dump(&ent.superseded_font_entity(), s, sublevel, -1)?;
        } else {
            write!(
                s,
                "Superseding Font Number : {}",
                ent.superseded_font_code()
            )?;
        }
        writeln!(
            s,
            "\nNo. of Grid Units eqvt to 1 Text Height : {}\nASCII Codes                              :\nGrid Locations of next character origins :\nPen Motions                              :\nPen Positions                            :\nGrid Locations the pen moves to          : Count = {}",
            ent.scale(),
            nbchars
        )?;
        iges_dump_vals(s, -level, 1, nbchars, |first| ent.ascii_code(first))?;
        writeln!(s)?;
        if level > 4 {
            let mut arr_xy: Handle<HArray1<Xy>> = Default::default();
            let mut i: i32 = Default::default();
            let mut j: i32 = Default::default();
            let mut nbmotions: i32 = Default::default();
            for i in 1..=nbchars {
                let mut ix: i32 = Default::default();
                let mut iy: i32 = Default::default();
                write!(
                    s,
                    "[{}]: ASCII Code : {}\nGrid Location of next character's origin : ",
                    i,
                    ent.ascii_code(i)?
                )?;
                ent.next_char_origin(i, &mut ix, &mut iy)?;
                write!(s, "X={} Y={}", ix, iy)?;
                nbmotions = ent.nb_pen_motions(i)?;
                write!(s, "  No. of Pen Motions : {}", nbmotions)?;
                if level <= 5 {
                    writeln!(s, " [ ask level > 5 for Details ]")?;
                } else {
                    writeln!(s)?;
                    for j in 1..=nbmotions {
                        write!(
                            s,
                            "Pen up(1) / down(0) flag : {} Next Pen Position : ",
                            i32::from(ent.is_pen_up(i, j)?)
                        )?;
                        ent.next_pen_position(i, j, &mut ix, &mut iy)?;
                        writeln!(s, " X={} Y={}", ix, iy)?;
                    }
                }
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
