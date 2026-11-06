//! IgesDefsToolTabularData.
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
use crate::iges_basic::IgesBasicHArray1OfHArray1OfReal;
use crate::iges_basic::entity_shape::iges_array1_length;
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, iges_array1_item,
    iges_array1_set, iges_dump_vals, iges_live, iges_new_array1, iges_real_text,
};
use crate::iges_defs::IgesDefsTabularData;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceParamType,
    InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDefsToolTabularData,
    "The own-parameter work on `IgesDefsTabularData`."
);

impl IgesDefsToolTabularData {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDefsTabularData>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut nb_props: i32 = Default::default();
        let mut prop_type: i32 = Default::default();
        let mut nb_deps: i32 = Default::default();
        let mut nb_indeps: i32 = Default::default();
        let mut types_ind: Handle<HArray1<i32>> = Default::default();
        let mut nb_values_ind: Handle<HArray1<i32>> = Default::default();
        let mut values_ind: Handle<IgesBasicHArray1OfHArray1OfReal> = Default::default();
        let mut values_dep: Handle<IgesBasicHArray1OfHArray1OfReal> = Default::default();
        let mut i: i32 = Default::default();
        pr.read_integer_text(&pr.current(), "Number of Property values", &mut nb_props)?;
        pr.read_integer_text(&pr.current(), "Property type", &mut prop_type)?;
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "No. of dependent variables", &mut nb_deps)?;
        if st && (nb_deps > 0) {
            values_dep = Handle::new(IgesBasicHArray1OfHArray1OfReal::with_bounds(1, nb_deps)?);
        }
        st = pr.read_integer_text(
            &pr.current(),
            "No. of Independent variables",
            &mut nb_indeps,
        )?;
        if st && (nb_indeps > 0) {
            values_ind = Handle::new(IgesBasicHArray1OfHArray1OfReal::with_bounds(1, nb_indeps)?);
            types_ind = iges_new_array1::<i32>(1, nb_indeps)?;
            nb_values_ind = iges_new_array1::<i32>(1, nb_indeps)?;
        }
        pr.read_ints(
            &pr.current_list(nb_indeps, 1),
            "Type of independent variables",
            &mut types_ind,
            1,
        )?;
        pr.read_ints(
            &pr.current_list(nb_indeps, 1),
            "No. of values of independent variables",
            &mut nb_values_ind,
            1,
        )?;
        for i in 1..=nb_indeps {
            let mut tarr: Handle<HArray1<f64>> = Default::default();
            let mut nb: i32 = iges_array1_item(
                &nb_values_ind,
                i,
                "IgesDefsToolTabularData::read_own_params",
            )?;
            let mut j: i32 = Default::default();
            if nb > 0 {
                tarr = iges_new_array1::<f64>(1, nb)?;
                for j in 1..=nb {
                    let mut treal: f64 = Default::default();
                    pr.read_real_text(&pr.current(), "Value of independent variable", &mut treal)?;
                    iges_array1_set(&tarr, j, treal, "IgesDefsToolTabularData::read_own_params")?;
                }
            }
            iges_live(&values_ind, "IgesDefsToolTabularData::read_own_params")?
                .set_value(i, &tarr)?;
        }
        let mut curnum: i32 = pr.current_number();
        let mut nbpars: i32 = pr.nb_params();
        let mut nbd: i32 = 0;
        for i in curnum..=nbpars {
            if pr.param_type(i)? != (InterfaceParamType::ParamReal) {
                break;
            }
            nbd = (i - curnum) + 1;
        }
        let mut somedeps: Handle<HArray1<f64>> = Default::default();
        if nbd > 0 {
            somedeps = iges_new_array1::<f64>(1, nbd)?;
        }
        for i in 1..=nbd {
            let mut treal: f64 = Default::default();
            pr.read_real_text(&pr.current(), "Value of dependent variable", &mut treal)?;
            iges_array1_set(
                &somedeps,
                i,
                treal,
                "IgesDefsToolTabularData::read_own_params",
            )?;
        }
        if nb_deps > 0 {
            iges_live(&values_dep, "IgesDefsToolTabularData::read_own_params")?
                .set_value(1, &somedeps)?;
        } else {
            pr.add_warning("Some Real remain while no dependent value is defined", "");
        }
        nb_props = pr.current_number() - 2;
        pr.add_warning("Don't know exactly how to read dependent values ...", "");
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            nb_props,
            prop_type,
            &types_ind,
            &nb_values_ind,
            &values_ind,
            &values_dep,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDefsTabularData>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut nb_indeps: i32 = ent.nb_independents()?;
        let mut j: i32 = Default::default();
        let mut nb_deps: i32 = ent.nb_dependents()?;
        iw.send(ent.nb_property_values())?;
        iw.send(ent.property_type())?;
        iw.send(nb_deps)?;
        iw.send(nb_indeps)?;
        for i in 1..=nb_indeps {
            iw.send(ent.type_of_independents(i)?)?;
        }
        for i in 1..=nb_indeps {
            iw.send(ent.nb_values(i)?)?;
        }
        for i in 1..=nb_indeps {
            {
                j = 1;
                while j <= ent.nb_values(i)? {
                    iw.send(ent.independent_value(i, j)?)?;
                    j += 1;
                }
            }
        }
        if nb_deps == 0 {
            return Ok(());
        }
        let mut deps: Handle<HArray1<f64>> = ent.dependent_values(1)?;
        for i in 1..=iges_array1_length(&deps, "IgesDefsToolTabularData::write_own_params")? {
            iw.send(iges_array1_item(
                &deps,
                i,
                "IgesDefsToolTabularData::write_own_params",
            )?)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a TabularData `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDefsTabularData>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDefsTabularData>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 11);
        crate::iges_shared_blocks::iges_block_0017!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDefsTabularData>,
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
        another: &Handle<IgesDefsTabularData>,
        ent: &Handle<IgesDefsTabularData>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut nb_props: i32 = another.nb_property_values();
        let mut prop_type: i32 = another.property_type();
        let mut nb_deps: i32 = another.nb_dependents()?;
        let mut nb_indeps: i32 = another.nb_independents()?;
        let mut types_ind: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nb_indeps)?;
        let mut nb_values_ind: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nb_indeps)?;
        let mut values_ind: Handle<IgesBasicHArray1OfHArray1OfReal> =
            Handle::new(IgesBasicHArray1OfHArray1OfReal::with_bounds(1, nb_indeps)?);
        let mut values_dep: Handle<IgesBasicHArray1OfHArray1OfReal> =
            Handle::new(IgesBasicHArray1OfHArray1OfReal::with_bounds(1, nb_deps)?);
        let mut i: i32 = Default::default();
        for i in 1..=nb_indeps {
            let mut j: i32 = Default::default();
            let mut nval: i32 = Default::default();
            iges_array1_set(
                &types_ind,
                i,
                another.type_of_independents(i)?,
                "IgesDefsToolTabularData::own_copy",
            )?;
            nval = another.nb_values(i)?;
            iges_array1_set(&nb_values_ind, i, nval, "IgesDefsToolTabularData::own_copy")?;
            let mut tmparr: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, nval)?;
            for j in 1..=nval {
                iges_array1_set(
                    &tmparr,
                    j,
                    another.independent_value(i, j)?,
                    "IgesDefsToolTabularData::own_copy",
                )?;
            }
            iges_live(&values_ind, "IgesDefsToolTabularData::own_copy")?.set_value(i, &tmparr)?;
        }
        for i in 1..=nb_deps {
            let source = another.dependent_values(i)?;
            if source.is_some() {
                let length = iges_array1_length(&source, "IgesDefsToolTabularData::own_copy")?;
                let list = iges_new_array1::<f64>(1, length)?;
                for j in 1..=length {
                    iges_array1_set(
                        &list,
                        j,
                        iges_array1_item(&source, j, "IgesDefsToolTabularData::own_copy")?,
                        "IgesDefsToolTabularData::own_copy",
                    )?;
                }
                iges_live(&values_dep, "IgesDefsToolTabularData::own_copy")?.set_value(i, &list)?;
            }
        }
        ent.init(
            nb_props,
            prop_type,
            &types_ind,
            &nb_values_ind,
            &values_ind,
            &values_dep,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDefsTabularData>,
        _unused_1: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut nb_indeps: i32 = ent.nb_independents()?;
        let mut nb_deps: i32 = ent.nb_dependents()?;
        write!(
            s,
            "IgesDefsTabularData\nNo. of property values : {}\nProperty type : {}\nNo. of Dependent variables    : {}\nNo. of Independent variables  : {}\nType of independent variables : ",
            ent.nb_property_values(),
            ent.property_type(),
            nb_deps,
            nb_indeps
        )?;
        iges_dump_vals(s, level, 1, nb_indeps, |first| {
            ent.type_of_independents(first)
        })?;
        write!(s, "\nNumber of values of independent variables : ")?;
        iges_dump_vals(s, level, 1, nb_indeps, |first| ent.nb_values(first))?;
        write!(s, "\nValues of the independent variable : ")?;
        if level < 5 {
            write!(s, " [ask level > 4]")?;
        } else {
            for ind in 1..=nb_indeps {
                write!(s, "\n[{}]:", ind)?;
                let mut nbi: i32 = ent.nb_values(ind)?;
                for iv in 1..=nbi {
                    write!(s, " {}", iges_real_text(ent.independent_value(ind, iv)?))?;
                }
            }
        }
        write!(s, "\nValues of the dependent variable : ")?;
        writeln!(s, "  TO BE DONE")?;
        Ok(())
    }
}
