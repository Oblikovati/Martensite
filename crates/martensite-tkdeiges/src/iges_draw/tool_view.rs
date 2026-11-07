//! IgesDrawToolView.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_live, iges_real_text,
};
use crate::iges_draw::IgesDrawView;
use crate::iges_geom::IgesGeomPlane;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolView,
    "The own-parameter work on `IgesDrawView`."
);

impl IgesDrawToolView {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawView>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_view_number: i32 = Default::default();
        let mut temp_scale_factor: f64 = Default::default();
        let mut temp_left_plane: Handle<IgesGeomPlane> = Default::default();
        let mut temp_top_plane: Handle<IgesGeomPlane> = Default::default();
        let mut temp_right_plane: Handle<IgesGeomPlane> = Default::default();
        let mut temp_bottom_plane: Handle<IgesGeomPlane> = Default::default();
        let mut temp_back_plane: Handle<IgesGeomPlane> = Default::default();
        let mut temp_front_plane: Handle<IgesGeomPlane> = Default::default();
        pr.read_integer_text(&pr.current(), "View Number", &mut temp_view_number)?;
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Scale Factor", &mut temp_scale_factor)?;
        } else {
            temp_scale_factor = 1.0;
        }
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Left Side Of View Volume",
            <IgesGeomPlane>::static_type(),
            &mut temp_left_plane,
            true,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Top Side Of View Volume",
            <IgesGeomPlane>::static_type(),
            &mut temp_top_plane,
            true,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Right Side Of View Volume",
            <IgesGeomPlane>::static_type(),
            &mut temp_right_plane,
            true,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Bottom Side Of View Volume",
            <IgesGeomPlane>::static_type(),
            &mut temp_bottom_plane,
            true,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Back Side Of View Volume",
            <IgesGeomPlane>::static_type(),
            &mut temp_back_plane,
            true,
        )?;
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "Front Side Of View Volume",
            <IgesGeomPlane>::static_type(),
            &mut temp_front_plane,
            true,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_view_number,
            temp_scale_factor,
            &temp_left_plane,
            &temp_top_plane,
            &temp_right_plane,
            &temp_bottom_plane,
            &temp_back_plane,
            &temp_front_plane,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawView>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.view_number())?;
        iw.send(ent.scale_factor())?;
        iw.send_entity(&ent.left_plane(), false)?;
        iw.send_entity(&ent.top_plane(), false)?;
        iw.send_entity(&ent.right_plane(), false)?;
        iw.send_entity(&ent.bottom_plane(), false)?;
        iw.send_entity(&ent.back_plane(), false)?;
        iw.send_entity(&ent.front_plane(), false)?;
        Ok(())
    }
    /// Lists the Entities shared by a View `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawView>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.left_plane().cast());
        iter.get_one_item(&ent.top_plane().cast());
        iter.get_one_item(&ent.right_plane().cast());
        iter.get_one_item(&ent.bottom_plane().cast());
        iter.get_one_item(&ent.back_plane().cast());
        iter.get_one_item(&ent.front_plane().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDrawView>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(410, 0);
        crate::iges_shared_blocks::iges_block_0016!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawView>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.has_transf() {
            if (iges_live(&ent.transf(), "IgesDrawToolView::own_check")?
                .iges_entity()
                .form_number())
                != 0
            {
                ach.add_fail_text("Associated Matrix has not Form Number 0", "");
            }
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDrawView>,
        ent: &Handle<IgesDrawView>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_view_number: i32 = another.view_number();
        let mut temp_scale_factor: f64 = another.scale_factor();
        let mut temp_left_plane: Handle<IgesGeomPlane> =
            tc.transferred(&another.left_plane().cast())?.cast();
        let mut temp_top_plane: Handle<IgesGeomPlane> =
            tc.transferred(&another.top_plane().cast())?.cast();
        let mut temp_right_plane: Handle<IgesGeomPlane> =
            tc.transferred(&another.right_plane().cast())?.cast();
        let mut temp_bottom_plane: Handle<IgesGeomPlane> =
            tc.transferred(&another.bottom_plane().cast())?.cast();
        let mut temp_back_plane: Handle<IgesGeomPlane> =
            tc.transferred(&another.back_plane().cast())?.cast();
        let mut temp_front_plane: Handle<IgesGeomPlane> =
            tc.transferred(&another.front_plane().cast())?.cast();
        ent.init(
            temp_view_number,
            temp_scale_factor,
            &temp_left_plane,
            &temp_top_plane,
            &temp_right_plane,
            &temp_bottom_plane,
            &temp_back_plane,
            &temp_front_plane,
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawView>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        write!(
            s,
            "IgesDrawView\nView Number  : {}\nScale Factor : {}\nLeft Plane Of View Volume   : ",
            ent.view_number(),
            iges_real_text(ent.scale_factor())
        )?;
        dumper.dump(&ent.left_plane(), s, temp_sub_level, -1)?;
        writeln!(s)?;
        write!(s, "Top Plane Of View Volume    : ")?;
        dumper.dump(&ent.top_plane(), s, temp_sub_level, -1)?;
        writeln!(s)?;
        write!(s, "Right Plane Of View Volume  : ")?;
        dumper.dump(&ent.right_plane(), s, temp_sub_level, -1)?;
        writeln!(s)?;
        write!(s, "Bottom Plane Of View Volume : ")?;
        dumper.dump(&ent.bottom_plane(), s, temp_sub_level, -1)?;
        writeln!(s)?;
        write!(s, "Back Plane Of View Volume   : ")?;
        dumper.dump(&ent.back_plane(), s, temp_sub_level, -1)?;
        writeln!(s)?;
        write!(s, "Front Plane Of View Volume  : ")?;
        dumper.dump(&ent.front_plane(), s, temp_sub_level, -1)?;
        writeln!(s)?;
        Ok(())
    }
}
