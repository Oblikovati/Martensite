//! IgesDimenToolCenterLine.
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
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataLineFontEntity, IgesDataParamReader,
    iges_array1_set, iges_dump_list_xy_lz, iges_new_array1, iges_real_text,
};
use crate::iges_dimen::IgesDimenCenterLine;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point3, Xy};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolCenterLine,
    "The own-parameter work on `IgesDimenCenterLine`."
);

impl IgesDimenToolCenterLine {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenCenterLine>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0008!(
            data_points,
            datatype,
            nbval,
            pr,
            st,
            z_displacement
        );
        if !data_points.is_null() {
            for i in 1..=nbval {
                let mut temp_xy: Xy = Default::default();
                if pr.read_xy(&pr.current_list(1, 2), "Data Points", &mut temp_xy)? {
                    iges_array1_set(
                        &data_points,
                        i,
                        temp_xy,
                        "IgesDimenToolCenterLine::read_own_params",
                    )?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(datatype, z_displacement, &data_points)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenCenterLine>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0032!(ent, i, iw, upper);
        Ok(())
    }
    /// Lists the Entities shared by a CenterLine `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDimenCenterLine>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a CenterLine
    /// (LineFont forced to Rank = 1, DataType forced to 1)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDimenCenterLine>) -> Result<bool, Failure> {
        crate::iges_shared_blocks::iges_block_0042!(ent, nb, nulfont, res);
        if nb == 0 {
            return Ok(res);
        }
        let mut pts: Handle<HArray1<Xy>> = iges_new_array1::<Xy>(1, nb)?;
        for i in 1..=nb {
            iges_array1_set(
                &pts,
                i,
                Xy::new(ent.point(i)?.x(), ent.point(i)?.y()),
                "IgesDimenToolCenterLine::own_correct",
            )?;
        }
        ent.init(1, ent.z_displacement(), &pts)?;
        return Ok(true);
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenCenterLine>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(106, 20, 21);
        crate::iges_shared_blocks::iges_block_0025!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenCenterLine>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0053!(ach, ent);
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenCenterLine>,
        ent: &Handle<IgesDimenCenterLine>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0044!(
            another,
            data_points,
            datatype,
            nbval,
            z_displacement
        );
        for i in 1..=nbval {
            let mut temp_pnt: Point3 = another.point(i)?;
            let mut temp_pnt2d: Xy = Xy::new(temp_pnt.x(), temp_pnt.y());
            iges_array1_set(
                &data_points,
                i,
                temp_pnt2d,
                "IgesDimenToolCenterLine::own_copy",
            )?;
        }
        ent.init(datatype, z_displacement, &data_points)?;
        ent.set_cross_hair(another.is_cross_hair());
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenCenterLine>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        writeln!(s, "IgesDimenCenterLine")?;
        if ent.is_cross_hair() {
            writeln!(s, "Cross Hair")?;
        } else {
            writeln!(s, "Through Circle Centers")?;
        }
        writeln!(
            s,
            "Data Type : {}  Number of Data Points : {}  Common Z displacement : {}  Data Points :",
            ent.datatype(),
            ent.nb_points()?,
            iges_real_text(ent.z_displacement())
        )?;
        crate::iges_shared_blocks::iges_block_0067!(ent, first, level, point, s);
        Ok(())
    }
}
