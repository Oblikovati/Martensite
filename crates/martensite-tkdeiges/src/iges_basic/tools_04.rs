#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{
    IgesBasicGroupWithoutBackP, IgesBasicHierarchy, IgesBasicName, IgesBasicOrderedGroup,
    IgesBasicToolGroupWithoutBackP,
};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::shared_runs::{iges_shared_005, iges_shared_048};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_dump_string, iges_live,
    iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesBasicToolGroupWithoutBackP {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicGroupWithoutBackP>,
        ent: &Handle<IgesBasicGroupWithoutBackP>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolGroupWithoutBackP::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolGroupWithoutBackP::own_copy")?;
        iges_shared_048!(another_live, ent_array, lower, upper);
        for i in lower..=upper {
            let myentity = tc
                .transferred(&another_live.entity(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &ent_array,
                i,
                myentity.clone(),
                "IgesBasicToolGroupWithoutBackP::own_copy",
            )?;
        }
        ent_live.init(&ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolGroupWithoutBackP {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicGroupWithoutBackP>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolGroupWithoutBackP::own_dump")?;
        write!(s, "IgesBasicGroupWithoutBackP\nEntries in the Group : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_entities(), |first| {
            ent_live.entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolHierarchy,
    "The own-parameter work on `IgesBasicHierarchy`."
);
impl IgesBasicToolHierarchy {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicHierarchy>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolHierarchy::read_own_params")?;
        let mut temp_nb_property_values: i32 = 0;
        let mut temp_line_font: i32 = 0;
        let mut temp_view: i32 = 0;
        let mut temp_entity_level: i32 = 0;
        let mut temp_blank_status: i32 = 0;
        let mut temp_line_weight: i32 = 0;
        let mut temp_color_num: i32 = 0;
        pr.read_integer_text(
            &pr.current(),
            "No. of Property values",
            &mut temp_nb_property_values,
        )?;
        pr.read_integer_text(&pr.current(), "LineFont", &mut temp_line_font)?;
        pr.read_integer_text(&pr.current(), "View", &mut temp_view)?;
        pr.read_integer_text(&pr.current(), "Entity level", &mut temp_entity_level)?;
        pr.read_integer_text(&pr.current(), "Blank status", &mut temp_blank_status)?;
        pr.read_integer_text(&pr.current(), "Line weight", &mut temp_line_weight)?;
        pr.read_integer_text(&pr.current(), "Color number", &mut temp_color_num)?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(
            temp_nb_property_values,
            temp_line_font,
            temp_view,
            temp_entity_level,
            temp_blank_status,
            temp_line_weight,
            temp_color_num,
        );
        Ok(())
    }
}
impl IgesBasicToolHierarchy {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicHierarchy>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolHierarchy::write_own_params")?;
        iw.send(ent_live.nb_property_values())?;
        iw.send(ent_live.new_line_font())?;
        iw.send(ent_live.new_view())?;
        iw.send(ent_live.new_entity_level())?;
        iw.send(ent_live.new_blank_status())?;
        iw.send(ent_live.new_line_weight())?;
        iw.send(ent_live.new_color_num())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesBasicHierarchy>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicHierarchy>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(406, 10);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.use_flag_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesBasicToolHierarchy {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesBasicHierarchy>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolHierarchy::own_correct")?;
        let res: bool = ent_live.nb_property_values() != 6;
        if res {
            ent_live.init(
                6,
                ent_live.new_line_font(),
                ent_live.new_view(),
                ent_live.new_entity_level(),
                ent_live.new_blank_status(),
                ent_live.new_line_weight(),
                ent_live.new_color_num(),
            );
        }
        Ok(res)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicHierarchy>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolHierarchy::own_check")?;
        if ent_live.nb_property_values() != 6 {
            ach.add_fail_text("Number of Property Values != 6", "");
        }
        if ent_live.new_line_font() != 0 && ent_live.new_line_font() != 1 {
            ach.add_fail_text("InCorrect LineFont", "");
        }
        if ent_live.new_view() != 0 && ent_live.new_view() != 1 {
            ach.add_fail_text("InCorrect View", "");
        }
        if ent_live.new_entity_level() != 0 && ent_live.new_entity_level() != 1 {
            ach.add_fail_text("InCorrect EntityLevel", "");
        }
        if ent_live.new_blank_status() != 0 && ent_live.new_blank_status() != 1 {
            if ent_live.new_line_weight() != 0 && ent_live.new_line_weight() != 1 {
                ach.add_fail_text("InCorrect LineWeight", "");
            }
        }
        if ent_live.new_color_num() != 0 && ent_live.new_color_num() != 1 {
            ach.add_fail_text("InCorrect ColorNum", "");
        }
        Ok(())
    }
}
impl IgesBasicToolHierarchy {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicHierarchy>,
        ent: &Handle<IgesBasicHierarchy>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolHierarchy::own_copy")?;
        let another_live = iges_live(another, "IgesBasicToolHierarchy::own_copy")?;
        ent_live.init(
            6,
            another_live.new_line_font(),
            another_live.new_view(),
            another_live.new_entity_level(),
            another_live.new_blank_status(),
            another_live.new_line_weight(),
            another_live.new_color_num(),
        );
        Ok(())
    }
}
impl IgesBasicToolHierarchy {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicHierarchy>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolHierarchy::own_dump")?;
        write!(
            s,
            "IgesBasicHierarchy\nNumber of property values : {}\nLine Font    : {}\nView Number  : {}\nEntity level : {}\nBlank status : {}\nLine weight  : {}\nColor number : {}\n",
            ent_live.nb_property_values(),
            ent_live.new_line_font(),
            ent_live.new_view(),
            ent_live.new_entity_level(),
            ent_live.new_blank_status(),
            ent_live.new_line_weight(),
            ent_live.new_color_num()
        )?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolName,
    "The own-parameter work on `IgesBasicName`."
);
impl IgesBasicToolName {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicName>,
        _ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolName::read_own_params")?;
        let mut temp_nb_property_values: i32 = 0;
        let mut temp_name: Handle<HAsciiString> = Handle::default();
        pr.read_integer_text(
            &pr.current(),
            "Number of property values",
            &mut temp_nb_property_values,
        )?;
        pr.read_text(&pr.current(), "Name", &mut temp_name)?;
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(temp_nb_property_values, &temp_name);
        Ok(())
    }
}
impl IgesBasicToolName {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicName>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolName::write_own_params")?;
        iw.send(ent_live.nb_property_values())?;
        iw.send(ent_live.value())?;
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        _ent: &Handle<IgesBasicName>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicName>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(406, 15);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.use_flag_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesBasicToolName {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesBasicName>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolName::own_correct")?;
        let res: bool = ent_live.nb_property_values() != 1;
        if res {
            ent_live.init(1, &ent_live.value());
        }
        Ok(res)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicName>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolName::own_check")?;
        if ent_live.nb_property_values() != 1 {
            ach.add_fail_text("Number of Property Values != 1", "");
        }
        Ok(())
    }
}
impl IgesBasicToolName {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicName>,
        ent: &Handle<IgesBasicName>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolName::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolName::own_copy")?;
        let mut a_nb_property_values: i32 = 0;
        let a_name: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another_live.value(),
            "IgesBasicToolName::own_copy",
        )?));
        a_nb_property_values = another_live.nb_property_values();
        ent_live.init(a_nb_property_values, &a_name);
        Ok(())
    }
}
impl IgesBasicToolName {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicName>,
        _dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        _own: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolName::own_dump")?;
        write!(
            s,
            "IgesBasicName\nNumber of property values : {}\nName : ",
            ent_live.nb_property_values()
        )?;
        iges_dump_string(s, &ent_live.value())?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolOrderedGroup,
    "The own-parameter work on `IgesBasicOrderedGroup`."
);
impl IgesBasicToolOrderedGroup {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicOrderedGroup>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroup::read_own_params")?;
        iges_shared_005!(ent, ent_array, ent_live, ir, nbval, pr, self);
        Ok(())
    }
}
impl IgesBasicToolOrderedGroup {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicOrderedGroup>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroup::write_own_params")?;
        let upper: i32 = ent_live.nb_entities();
        iw.send(upper)?;
        for i in 1..=upper {
            iw.send_entity(&ent_live.entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesBasicOrderedGroup>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroup::own_shared")?;
        let upper: i32 = ent_live.nb_entities();
        for i in 1..=upper {
            iter.get_one_item(&ent_live.entity(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicOrderedGroup>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(402, 14);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
