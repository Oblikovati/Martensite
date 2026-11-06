//! IgesAppliToolElementResults.
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
use crate::iges_appli::{IgesAppliElementResults, IgesAppliFiniteElement};
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_basic::{IgesBasicHArray1OfHArray1OfInteger, IgesBasicHArray1OfHArray1OfReal};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesReaderData, IgesDataIgesWriter,
    IgesDataParamReader, iges_array1_set, iges_live, iges_new_array1,
};
use crate::iges_dimen::IgesDimenGeneralNote;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::InterfaceEntityIterator;

iges_tool!(
    IgesAppliToolElementResults,
    "The own-parameter work on `IgesAppliElementResults`."
);

impl IgesAppliToolElementResults {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesAppliElementResults>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        let mut a_note: Handle<IgesDimenGeneralNote> = Default::default();
        let mut a_subcase_number: i32 = 0;
        let mut a_time: f64 = 0.0;
        let mut nv: i32 = 0;
        let mut a_result_report_flag: i32 = 0;
        crate::iges_shared_blocks::iges_block_0009!(
            all_data_layer_flag,
            all_element_identifiers,
            all_element_topology_type,
            all_elements,
            all_nb_layers,
            all_nb_result_data_locs,
            all_result_data,
            all_result_data_loc
        );
        pr.read_entity_typed(
            ir,
            &pr.current(),
            "General Note",
            <IgesDimenGeneralNote>::static_type(),
            &mut a_note,
            false,
        )?;
        pr.read_integer_text(&pr.current(), "Subcase Number", &mut a_subcase_number)?;
        pr.read_real_text(&pr.current(), "Time", &mut a_time)?;
        pr.read_integer_text(&pr.current(), "Number of Result Values", &mut nv)?;
        pr.read_integer_text(
            &pr.current(),
            "Result Report Flag",
            &mut a_result_report_flag,
        )?;
        pr.read_integer_text(&pr.current(), "Number of Finite Elements", &mut num)?;
        if num > 0 {
            all_element_identifiers = iges_new_array1::<i32>(1, num)?;
            all_elements = iges_new_array1::<Handle<IgesAppliFiniteElement>>(1, num)?;
            all_element_topology_type = iges_new_array1::<i32>(1, num)?;
            all_nb_layers = iges_new_array1::<i32>(1, num)?;
            all_data_layer_flag = iges_new_array1::<i32>(1, num)?;
            all_nb_result_data_locs = iges_new_array1::<i32>(1, num)?;
            all_result_data_loc =
                Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, num)?);
            all_result_data = Handle::new(IgesBasicHArray1OfHArray1OfReal::with_bounds(1, num)?);
        } else {
            pr.add_fail("Number of Finite Elements: Not Positive", "");
        }
        for nume in 1..=num {
            let mut ival: i32 = Default::default();
            let mut nl: i32 = Default::default();
            let mut nrl: i32 = Default::default();
            let mut numv: i32 = Default::default();
            numv = 0;
            nrl = numv;
            nl = nrl;
            ival = nl;
            let mut a_fem: Handle<IgesAppliFiniteElement> = Default::default();
            let mut rdrl: Handle<HArray1<i32>> = Default::default();
            let mut vres: Handle<HArray1<f64>> = Default::default();
            if pr.read_integer_text(&pr.current(), "FEM Element Identifier", &mut ival)? {
                iges_array1_set(
                    &all_element_identifiers,
                    nume,
                    ival,
                    "IgesAppliToolElementResults::read_own_params",
                )?;
            }
            if pr.read_entity_typed(
                ir,
                &pr.current(),
                "FEM Element Entity",
                <IgesAppliFiniteElement>::static_type(),
                &mut a_fem,
                false,
            )? {
                iges_array1_set(
                    &all_elements,
                    nume,
                    a_fem.clone(),
                    "IgesAppliToolElementResults::read_own_params",
                )?;
            }
            if pr.read_integer_text(&pr.current(), "FEM Element Topology Type", &mut ival)? {
                iges_array1_set(
                    &all_element_topology_type,
                    nume,
                    ival,
                    "IgesAppliToolElementResults::read_own_params",
                )?;
            }
            if pr.read_integer_text(&pr.current(), "Nb. of layers", &mut nl)? {
                iges_array1_set(
                    &all_nb_layers,
                    nume,
                    nl,
                    "IgesAppliToolElementResults::read_own_params",
                )?;
            }
            if pr.read_integer_text(&pr.current(), "Data Layer Flag", &mut ival)? {
                iges_array1_set(
                    &all_data_layer_flag,
                    nume,
                    ival,
                    "IgesAppliToolElementResults::read_own_params",
                )?;
            }
            if pr.read_integer_text(&pr.current(), "Nb. of result data locations", &mut nrl)? {
                iges_array1_set(
                    &all_nb_result_data_locs,
                    nume,
                    nrl,
                    "IgesAppliToolElementResults::read_own_params",
                )?;
            }
            if pr.read_ints(
                &pr.current_list(nrl, 1),
                "Result data locations",
                &mut rdrl,
                1,
            )? {
                iges_live(
                    &all_result_data_loc,
                    "IgesAppliToolElementResults::read_own_params",
                )?
                .set_value(nume, &rdrl)?;
            }
            pr.read_integer_text(&pr.current(), "Nb. of result data locations", &mut numv)?;
            if pr.read_reals(&pr.current_list(numv, 1), "Result Data", &mut vres, 1)? {
                iges_live(
                    &all_result_data,
                    "IgesAppliToolElementResults::read_own_params",
                )?
                .set_value(nume, &vres)?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &a_note,
            a_subcase_number,
            a_time,
            nv,
            a_result_report_flag,
            &all_element_identifiers,
            &all_elements,
            &all_element_topology_type,
            &all_nb_layers,
            &all_data_layer_flag,
            &all_nb_result_data_locs,
            &all_result_data_loc,
            &all_result_data,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesAppliElementResults>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut j: i32 = Default::default();
        let mut num: i32 = ent.nb_elements()?;
        iw.send_entity(&ent.note(), false)?;
        iw.send(ent.sub_case_number())?;
        iw.send(ent.time())?;
        iw.send(ent.nb_result_values())?;
        iw.send(ent.result_report_flag())?;
        iw.send(num)?;
        for i in 1..=num {
            let mut nl: i32 = ent.nb_layers(i)?;
            let mut nrl: i32 = ent.nb_result_data_locs(i)?;
            let mut numv: i32 = ent.nb_results(i)?;
            iw.send(ent.element_identifier(i)?)?;
            iw.send_entity(&ent.element(i)?, false)?;
            iw.send(ent.element_topology_type(i)?)?;
            iw.send(nl)?;
            iw.send(ent.data_layer_flag(i)?)?;
            iw.send(nrl)?;
            for j in 1..=nrl {
                iw.send(ent.result_data_loc(i, j)?)?;
            }
            iw.send(numv)?;
            for j in 1..=numv {
                iw.send(ent.result_data(i, j)?)?;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a ElementResults `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesAppliElementResults>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = ent.nb_elements()?;
        iter.get_one_item(&ent.note().cast());
        for i in 1..=num {
            iter.get_one_item(&ent.element(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesAppliElementResults>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(148, 0, 34);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        crate::iges_shared_blocks::iges_block_0027!(dc);
        dc
    }
}
