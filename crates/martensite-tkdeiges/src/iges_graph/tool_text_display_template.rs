//! IgesGraphToolTextDisplayTemplate.
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
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_xyz_l,
    iges_real_text,
};
use crate::iges_graph::{IgesGraphTextDisplayTemplate, IgesGraphTextFontDef};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesGraphToolTextDisplayTemplate,
    "The own-parameter work on `IgesGraphTextDisplayTemplate`."
);

impl IgesGraphToolTextDisplayTemplate {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesGraphTextDisplayTemplate>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut box_width: f64 = Default::default();
        let mut box_height: f64 = Default::default();
        let mut font_code: i32 = Default::default();
        let mut font_ent: Handle<IgesGraphTextFontDef> = Default::default();
        crate::iges_shared_blocks::iges_block_0036!(
            corner,
            mirror_flag,
            rotate_flag,
            rotation_angle,
            slant_angle
        );
        pr.read_real_text(&pr.current(), "Character box width", &mut box_width)?;
        pr.read_real_text(&pr.current(), "Character box height", &mut box_height)?;
        let mut curnum: i32 = pr.current_number();
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Font Code", &mut font_code)?;
            if font_code < 0 {
                font_ent = pr.param_entity(ir, curnum)?.cast::<IgesGraphTextFontDef>();
                if font_ent.is_null() {
                    pr.add_fail("Font Entity : incorrect reference", "");
                }
            }
        } else {
            font_code = 1;
        }
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Slant Angle", &mut slant_angle)?;
        } else {
            slant_angle = (std::f64::consts::PI) / 2.0;
        }
        pr.read_real_text(&pr.current(), "Rotation Angle", &mut rotation_angle)?;
        pr.read_integer_text(&pr.current(), "Mirror Flag", &mut mirror_flag)?;
        pr.read_integer_text(&pr.current(), "Rotate Flag", &mut rotate_flag)?;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Lower left coordinates/Increments",
            &mut corner,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            box_width,
            box_height,
            font_code,
            &font_ent,
            slant_angle,
            rotation_angle,
            mirror_flag,
            rotate_flag,
            corner,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesGraphTextDisplayTemplate>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.box_width())?;
        iw.send(ent.box_height())?;
        if ent.is_font_entity() {
            iw.send_entity(&ent.font_entity(), true)?;
        } else {
            iw.send(ent.font_code())?;
        }
        iw.send(ent.slant_angle())?;
        iw.send(ent.rotation_angle())?;
        iw.send(ent.mirror_flag())?;
        iw.send(ent.rotate_flag())?;
        iw.send(ent.starting_corner().x())?;
        iw.send(ent.starting_corner().y())?;
        iw.send(ent.starting_corner().z())?;
        Ok(())
    }
    /// Lists the Entities shared by a TextDisplayTemplate `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesGraphTextDisplayTemplate>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        if ent.is_font_entity() {
            iter.get_one_item(&ent.font_entity().cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesGraphTextDisplayTemplate>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(312, 0, 1);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefVoid);
        dc.line_weight(IgesDataDefType::DefVoid);
        dc.color(IgesDataDefType::DefAny);
        dc.subordinate_status_required(0);
        dc.use_flag_required(2);
        dc.hierarchy_status_required(0);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesGraphTextDisplayTemplate>,
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
        another: &Handle<IgesGraphTextDisplayTemplate>,
        ent: &Handle<IgesGraphTextDisplayTemplate>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut box_width: f64 = Default::default();
        let mut box_height: f64 = Default::default();
        let mut font_code: i32 = 0;
        let mut font_entity: Handle<IgesGraphTextFontDef> = Default::default();
        crate::iges_shared_blocks::iges_block_0036!(
            corner,
            mirror_flag,
            rotate_flag,
            rotation_angle,
            slant_angle
        );
        box_width = another.box_width();
        box_height = another.box_height();
        if another.is_font_entity() {
            font_entity = tc
                .transferred(&another.font_entity().cast())?
                .cast::<IgesGraphTextFontDef>();
        } else {
            font_code = another.font_code();
        }
        slant_angle = another.slant_angle();
        rotation_angle = another.rotation_angle();
        mirror_flag = another.mirror_flag();
        rotate_flag = another.rotate_flag();
        corner = another.starting_corner().xyz();
        ent.init(
            box_width,
            box_height,
            font_code,
            &font_entity,
            slant_angle,
            rotation_angle,
            mirror_flag,
            rotate_flag,
            corner,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesGraphTextDisplayTemplate>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        writeln!(
            s,
            "IgesGraphTextDisplayTemplate\nCharacter box width  : {}  Character box height : {}",
            iges_real_text(ent.box_width()),
            iges_real_text(ent.box_height())
        )?;
        if ent.is_font_entity() {
            write!(s, "Font Entity : ")?;
            dumper.dump(&ent.font_entity(), s, sublevel, -1)?;
        } else {
            write!(s, "Font code : {}", ent.font_code())?;
        }
        writeln!(
            s,
            "\nSlant angle    : {}  Rotation angle : {}\nMirror flag    : {}  Rotate flag    : {}",
            iges_real_text(ent.slant_angle()),
            iges_real_text(ent.rotation_angle()),
            ent.mirror_flag(),
            ent.rotate_flag()
        )?;
        if ent.form_number() == 0 {
            write!(s, "Lower Left Corner coordinates : ")?;
        } else {
            write!(s, "Increments from coordinates : ")?;
        }
        iges_dump_xyz_l(s, level, &ent.starting_corner().xyz(), &ent.location()?)?;
        writeln!(s)?;
        Ok(())
    }
}
