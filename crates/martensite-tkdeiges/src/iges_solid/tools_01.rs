#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{IgesSolidBlock, IgesSolidBooleanTree, IgesSolidConeFrustum};
use crate::iges_basic::entity_shape::{iges_array1_fill, iges_tool};
use crate::iges_basic::shared_runs::{
    iges_shared_002, iges_shared_004, iges_shared_010, iges_shared_017, iges_shared_025,
    iges_shared_039, iges_shared_040, iges_shared_043, iges_shared_046,
};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_xyz, iges_dump_xyz_l, iges_live, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesSolidToolBlock,
    "The own-parameter work on `IgesSolidBlock`."
);
impl IgesSolidToolBlock {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidBlock>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolBlock::read_own_params")?;
        iges_shared_040!(temp_corner, temp_size, temp_x_axis, temp_z_axis);
        let mut tempreal: f64 = 0.0;
        pr.read_xyz(&pr.current_list(1, 3), "Size of Block", &mut temp_size)?;
        iges_shared_025!(
            ent,
            pr,
            self,
            temp_corner,
            temp_x_axis,
            temp_z_axis,
            tempreal
        );
        ent_live.init(temp_size, temp_corner, temp_x_axis, temp_z_axis);
        iges_shared_017!(ent_live, eps, pr, temp_x_axis, temp_z_axis);
        Ok(())
    }
}
impl IgesSolidToolBlock {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidBlock>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolBlock::write_own_params")?;
        iges_shared_046!(ent_live, iw);
        iges_shared_004!(ent_live, iw);
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesSolidBlock>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidBlock>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(150, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc
    }
}
impl IgesSolidToolBlock {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidBlock>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolBlock::own_check")?;
        iges_shared_043!(ach, ent_live, eps, prosca);
        if ent_live.size().x() <= 0.0 || ent_live.size().y() <= 0.0 || ent_live.size().z() <= 0.0 {
            ach.add_fail_text("Size : Not positive lengths", "");
        }
        Ok(())
    }
}
impl IgesSolidToolBlock {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidBlock>,
        ent: &Handle<IgesSolidBlock>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolBlock::own_copy")?;
        let another_live = iges_live(another, "IgesSolidToolBlock::own_copy")?;
        ent_live.init(
            another_live.size(),
            Xyz::from(another_live.corner()),
            Xyz::from(another_live.x_axis()?),
            Xyz::from(another_live.z_axis()?),
        );
        Ok(())
    }
}
impl IgesSolidToolBlock {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidBlock>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolBlock::own_dump")?;
        write!(s, "IgesSolidBlock\nSize   : ")?;
        iges_dump_xyz(s, &ent_live.size())?;
        write!(s, "\nCorner : ")?;
        iges_dump_xyz_l(
            s,
            level,
            &Xyz::from(ent_live.corner()),
            &ent_live.location()?,
        )?;
        iges_shared_010!(ent_live, level, s);
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolBooleanTree,
    "The own-parameter work on `IgesSolidBooleanTree`."
);
impl IgesSolidToolBooleanTree {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidBooleanTree>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolBooleanTree::read_own_params")?;
        let mut length: i32 = 0;
        let mut intvalue: i32 = 0;
        let mut entvalue: Handle<IgesDataIgesEntity> = Handle::default();
        let mut temp_operations: Handle<HArray1<i32>> = Handle::default();
        let mut temp_operands: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        let st: bool =
            pr.read_integer_text(&pr.current(), "Length of post-order notation", &mut length)?;
        if st && length > 0 {
            temp_operations = iges_new_array1::<i32>(1, length)?;
            iges_array1_fill(
                &temp_operations,
                0,
                "IgesSolidToolBooleanTree::read_own_params",
            )?;
            temp_operands = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, length)?;
            if pr.read_entity(ir, &pr.current(), "Operand 1", &mut entvalue, false)? {
                iges_array1_set(
                    &temp_operands,
                    1,
                    entvalue.clone(),
                    "IgesSolidToolBooleanTree::read_own_params",
                )?;
            }
            if pr.read_entity(ir, &pr.current(), "Operand 2", &mut entvalue, false)? {
                iges_array1_set(
                    &temp_operands,
                    2,
                    entvalue.clone(),
                    "IgesSolidToolBooleanTree::read_own_params",
                )?;
            }
            for i in 3..length {
                let curnum: i32 = pr.current_number();
                pr.read_integer_text(&pr.current(), "Operation code", &mut intvalue)?;
                if intvalue < 0 {
                    entvalue = pr.param_entity(ir, curnum)?;
                    if entvalue.is_null() {
                        pr.add_fail("Operand : incorrect reference", "");
                    } else {
                        iges_array1_set(
                            &temp_operands,
                            i,
                            entvalue.clone(),
                            "IgesSolidToolBooleanTree::read_own_params",
                        )?;
                    }
                } else {
                    iges_array1_set(
                        &temp_operations,
                        i,
                        intvalue,
                        "IgesSolidToolBooleanTree::read_own_params",
                    )?;
                }
            }
            if pr.read_integer_text(&pr.current(), "Operation code", &mut intvalue)? {
                iges_array1_set(
                    &temp_operations,
                    length,
                    intvalue,
                    "IgesSolidToolBooleanTree::read_own_params",
                )?;
            }
        } else {
            pr.add_fail("Length of post-order : Not Positive", "");
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(&temp_operands, &temp_operations)?;
        Ok(())
    }
}
impl IgesSolidToolBooleanTree {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesSolidBooleanTree>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolBooleanTree::write_own_params")?;
        let length: i32 = ent_live.length()?;
        iw.send(length)?;
        for i in 1..=length {
            if ent_live.is_operand(i)? {
                iw.send_entity(&ent_live.operand(i)?, false)?;
            } else {
                iw.send(ent_live.operation(i)?)?;
            }
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesSolidBooleanTree>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolBooleanTree::own_shared")?;
        let length: i32 = ent_live.length()?;
        for i in 1..=length {
            if ent_live.is_operand(i)? {
                iter.get_one_item(&ent_live.operand(i)?.cast::<StandardTransient>());
            }
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesSolidBooleanTree>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(180, 0);
        dc.structure(IgesDataDefType::DefVoid);
        dc.line_font(IgesDataDefType::DefAny);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(0);
        dc.graphics_ignored(1);
        dc
    }
}
impl IgesSolidToolBooleanTree {
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesSolidBooleanTree>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolBooleanTree::own_check")?;
        let length: i32 = ent_live.length()?;
        if length <= 2 {
            ach.add_fail_text("Length of post-order notation : Less than three", "");
        } else {
            if !ent_live.is_operand(1)? {
                ach.add_fail_text("First Item is not an Operand", "");
            }
            if !ent_live.is_operand(2)? {
                ach.add_fail_text("Second Item is not an Operand", "");
            }
            if ent_live.is_operand(length)? {
                ach.add_fail_text("Last Item is not an Operation", "");
            }
        }
        for i in 1..=length {
            if !ent_live.operand(i)?.is_null() {
                continue;
            }
            if ent_live.operation(i)? < 1 || ent_live.operation(i)? > 3 {
                let mut mess = String::new();
                mess = format!("Item no. {} Incorrect", i);
                ach.add_fail_text(&mess, "");
            }
        }
        Ok(())
    }
}
impl IgesSolidToolBooleanTree {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesSolidBooleanTree>,
        ent: &Handle<IgesSolidBooleanTree>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesSolidToolBooleanTree::own_copy")?;
        let ent_live = iges_live(ent, "IgesSolidToolBooleanTree::own_copy")?;
        let length: i32 = another_live.length()?;
        let temp_operations: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, length)?;
        let temp_operands: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, length)?;
        for i in 1..=length {
            if another_live.is_operand(i)? {
                let new_ent = tc
                    .transferred(&another_live.operand(i)?.cast::<StandardTransient>())?
                    .cast::<IgesDataIgesEntity>();
                iges_array1_set(
                    &temp_operands,
                    i,
                    new_ent.clone(),
                    "IgesSolidToolBooleanTree::own_copy",
                )?;
            } else {
                iges_array1_set(
                    &temp_operations,
                    i,
                    another_live.operation(i)?,
                    "IgesSolidToolBooleanTree::own_copy",
                )?;
            }
        }
        ent_live.init(&temp_operands, &temp_operations)?;
        Ok(())
    }
}
impl IgesSolidToolBooleanTree {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesSolidBooleanTree>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesSolidToolBooleanTree::own_dump")?;
        let length: i32 = ent_live.length()?;
        write!(
            s,
            "IgesSolidBooleanTree\nLength of the post-order notation :{}\n",
            length
        )?;
        if level > 4 {
            writeln!(s, "Post-order notation of the Boolean Tree :")?;
            for i in 1..=length {
                if ent_live.is_operand(i)? {
                    write!(s, "[{}] Operand : ", i)?;
                    dumper.dump(&ent_live.operand(i)?, s, 1, -1)?;
                    writeln!(s)?;
                } else {
                    let opcode: i32 = ent_live.operation(i)?;
                    write!(s, "[{}] Operator : {}", i, opcode)?;
                    if opcode == 1 {
                        write!(s, " (Union)")?;
                    } else if opcode == 2 {
                        writeln!(s, " (Intersection)")?;
                    } else if opcode == 3 {
                        writeln!(s, " (Difference)")?;
                    } else {
                        writeln!(s, " (incorrect value)")?;
                    }
                }
            }
        }
        Ok(())
    }
}
iges_tool!(
    IgesSolidToolConeFrustum,
    "The own-parameter work on `IgesSolidConeFrustum`."
);
impl IgesSolidToolConeFrustum {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesSolidConeFrustum>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesSolidToolConeFrustum::read_own_params")?;
        let mut temp_height: f64 = 0.0;
        let mut temp_r1: f64 = 0.0;
        let mut temp_r2: f64 = 0.0;
        iges_shared_039!(pr, temp_axis, temp_center, temp_height, tempreal);
        pr.read_real_text(&pr.current(), "Larger face radius", &mut temp_r1)?;
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Smaller face radius", &mut temp_r2)?;
        } else {
            temp_r2 = 0.0;
        }
        iges_shared_002!(ent, pr, self, temp_axis, temp_center, tempreal);
        ent_live.init(temp_height, temp_r1, temp_r2, temp_center, temp_axis);
        let eps: f64 = 1.0e-05;
        if !temp_axis.is_equal(&Xyz::from(ent_live.axis()?), eps) {
            pr.add_warning("Axis poorly unitary, normalized", "");
        }
        Ok(())
    }
}
