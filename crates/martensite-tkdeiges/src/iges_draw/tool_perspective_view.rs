//! IgesDrawToolPerspectiveView.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_dump_xyz_l, iges_live,
    iges_real_text,
};
use crate::iges_draw::IgesDrawPerspectiveView;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Xy, Xyz};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolPerspectiveView,
    "The own-parameter work on `IgesDrawPerspectiveView`."
);

impl IgesDrawToolPerspectiveView {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawPerspectiveView>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_top_left: Xy = Default::default();
        let mut temp_bottom_right: Xy = Default::default();
        let mut temp_left: f64 = Default::default();
        let mut temp_right: f64 = Default::default();
        let mut temp_top: f64 = Default::default();
        let mut temp_bottom: f64 = Default::default();
        let mut temp_center_of_projection: Xyz = Default::default();
        let mut temp_view_up_vector: Xyz = Default::default();
        let mut temp_view_normal_vector: Xyz = Default::default();
        let mut temp_view_reference_point: Xyz = Default::default();
        let mut temp_view_number: i32 = Default::default();
        let mut temp_depth_clip: i32 = Default::default();
        let mut temp_scale_factor: f64 = Default::default();
        let mut temp_view_plane_distance: f64 = Default::default();
        let mut temp_back_plane_distance: f64 = Default::default();
        let mut temp_front_plane_distance: f64 = Default::default();
        pr.read_integer_text(&pr.current(), "View Number", &mut temp_view_number)?;
        pr.read_real_text(&pr.current(), "Scale Number", &mut temp_scale_factor)?;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "View Plane Normal Vector",
            &mut temp_view_normal_vector,
        )?;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "View Reference Point",
            &mut temp_view_reference_point,
        )?;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Center Of Projection",
            &mut temp_center_of_projection,
        )?;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "View Up Vector",
            &mut temp_view_up_vector,
        )?;
        pr.read_real_text(
            &pr.current(),
            "View Plane Distance",
            &mut temp_view_plane_distance,
        )?;
        if pr.read_real_text(
            &pr.current(),
            "Left Side Of Clipping Window",
            &mut temp_left,
        )? {
            temp_top_left.set_x(temp_left);
        }
        if pr.read_real_text(
            &pr.current(),
            "Right Side Of Clipping Window",
            &mut temp_right,
        )? {
            temp_bottom_right.set_x(temp_right);
        }
        if pr.read_real_text(&pr.current(), "Bottom Of Clipping Window", &mut temp_bottom)? {
            temp_bottom_right.set_y(temp_bottom);
        }
        if pr.read_real_text(&pr.current(), "Top Of Clipping Window", &mut temp_top)? {
            temp_top_left.set_y(temp_top);
        }
        pr.read_integer_text(
            &pr.current(),
            "Depth Clipping Indicator",
            &mut temp_depth_clip,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Back Plane Distance",
            &mut temp_back_plane_distance,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Front Plane Distance",
            &mut temp_front_plane_distance,
        )?;
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_view_number,
            temp_scale_factor,
            temp_view_normal_vector,
            temp_view_reference_point,
            temp_center_of_projection,
            temp_view_up_vector,
            temp_view_plane_distance,
            temp_top_left,
            temp_bottom_right,
            temp_depth_clip,
            temp_back_plane_distance,
            temp_front_plane_distance,
        );
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawPerspectiveView>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.view_number())?;
        iw.send(ent.scale_factor())?;
        iw.send(ent.view_normal_vector().x())?;
        iw.send(ent.view_normal_vector().y())?;
        iw.send(ent.view_normal_vector().z())?;
        iw.send(ent.view_reference_point().x())?;
        iw.send(ent.view_reference_point().y())?;
        iw.send(ent.view_reference_point().z())?;
        iw.send(ent.center_of_projection().x())?;
        iw.send(ent.center_of_projection().y())?;
        iw.send(ent.center_of_projection().z())?;
        iw.send(ent.view_up_vector().x())?;
        iw.send(ent.view_up_vector().y())?;
        iw.send(ent.view_up_vector().z())?;
        iw.send(ent.view_plane_distance())?;
        iw.send(ent.top_left().x())?;
        iw.send(ent.bottom_right().x())?;
        iw.send(ent.bottom_right().y())?;
        iw.send(ent.top_left().y())?;
        iw.send(ent.depth_clip())?;
        iw.send(ent.back_plane_distance())?;
        iw.send(ent.front_plane_distance())?;
        Ok(())
    }
    /// Lists the Entities shared by a PerspectiveView `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDrawPerspectiveView>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDrawPerspectiveView>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(410, 1);
        crate::iges_shared_blocks::iges_block_0016!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawPerspectiveView>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if (ent.depth_clip() < 0) || (ent.depth_clip() > 3) {
            ach.add_fail_text("DepthClip has invalid value", "");
        }
        if ent.has_transf() {
            if (iges_live(&ent.transf(), "IgesDrawToolPerspectiveView::own_check")?
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
        another: &Handle<IgesDrawPerspectiveView>,
        ent: &Handle<IgesDrawPerspectiveView>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ent.init(
            another.view_number(),
            another.scale_factor(),
            *another.view_normal_vector().xyz(),
            another.view_reference_point().xyz(),
            another.center_of_projection().xyz(),
            *another.view_up_vector().xyz(),
            another.view_plane_distance(),
            another.top_left().xy(),
            another.bottom_right().xy(),
            another.depth_clip(),
            another.back_plane_distance(),
            another.front_plane_distance(),
        );
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawPerspectiveView>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        write!(
            s,
            "IgesDrawPerspectiveView\nView Number  : {}  Scale Factor : {}\nView Plane Normal Vector : ",
            ent.view_number(),
            iges_real_text(ent.scale_factor())
        )?;
        iges_dump_xyz_l(s, level, ent.view_normal_vector().xyz(), &ent.location()?)?;
        write!(s, "\nView Reference Point     : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &ent.view_reference_point().xyz(),
            &ent.location()?,
        )?;
        write!(s, "\nCenter Of Projection     : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &ent.center_of_projection().xyz(),
            &ent.location()?,
        )?;
        write!(s, "\nView Up Vector           : ")?;
        iges_dump_xyz_l(s, level, ent.view_up_vector().xyz(), &ent.location()?)?;
        write!(
            s,
            "\nView Plane Distance      : {}\nLeft   Side Of Clipping Window : {}\nRight  Side Of Clipping Window : {}\nBottom Side Of Clipping Window : {}\nTop    Side Of Clipping Window : {}\nDepth Clipping : {}",
            iges_real_text(ent.view_plane_distance()),
            iges_real_text(ent.top_left().x()),
            iges_real_text(ent.bottom_right().x()),
            iges_real_text(ent.bottom_right().y()),
            iges_real_text(ent.top_left().y()),
            ent.depth_clip()
        )?;
        match ent.depth_clip() {
            0 => {
                writeln!(s, " (No Depth Clipping)")?;
            }
            1 => {
                writeln!(s, " (Back Clipping Plane ON)")?;
            }
            2 => {
                writeln!(s, " (Front Clipping Plane ON)")?;
            }
            3 => {
                writeln!(s, " (Front and Back Clipping Planes ON)")?;
            }
            _ => {
                writeln!(s, " (Invalid Value)")?;
            }
        }
        writeln!(
            s,
            "Back Plane Distance  : {}  Front Plane Distance : {}\n",
            iges_real_text(ent.back_plane_distance()),
            iges_real_text(ent.front_plane_distance())
        )?;
        Ok(())
    }
}
