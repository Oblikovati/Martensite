//! More of IgesAppliToolElementResults.
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
use crate::iges_appli::{
    IgesAppliElementResults, IgesAppliFiniteElement, IgesAppliToolElementResults,
};
use crate::iges_basic::{IgesBasicHArray1OfHArray1OfInteger, IgesBasicHArray1OfHArray1OfReal};
use crate::iges_data::{iges_array1_set, iges_live, iges_new_array1};
use crate::iges_dimen::IgesDimenGeneralNote;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceCopyTool, InterfaceShareTool};

impl IgesAppliToolElementResults {
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesAppliElementResults>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut rrf: i32 = ent.result_report_flag();
        if (rrf < 0) || (rrf > 3) {
            ach.add_fail_text("Result Report Flag not in [0-3]", "");
        }
        let mut nv: i32 = ent.nb_result_values();
        let mut ok: bool = true;
        match ent.form_number() {
            0 => {
                if nv < 0 {
                    ok = false;
                }
            }
            1 => {
                if nv != 1 {
                    ok = false;
                }
            }
            2 => {
                if nv != 1 {
                    ok = false;
                }
            }
            3 => {
                if nv != 3 {
                    ok = false;
                }
            }
            4 => {
                if nv != 6 {
                    ok = false;
                }
            }
            5 => {
                if nv != 3 {
                    ok = false;
                }
            }
            6 => {
                if nv != 3 {
                    ok = false;
                }
            }
            7 => {
                if nv != 3 {
                    ok = false;
                }
            }
            8 => {
                if nv != 3 {
                    ok = false;
                }
            }
            9 => {
                if nv != 3 {
                    ok = false;
                }
            }
            10 => {
                if nv != 1 {
                    ok = false;
                }
            }
            11 => {
                if nv != 1 {
                    ok = false;
                }
            }
            12 => {
                if nv != 3 {
                    ok = false;
                }
            }
            13 => {
                if nv != 1 {
                    ok = false;
                }
            }
            14 => {
                if nv != 1 {
                    ok = false;
                }
            }
            15 => {
                if nv != 3 {
                    ok = false;
                }
            }
            16 => {
                if nv != 1 {
                    ok = false;
                }
            }
            17 => {
                if nv != 3 {
                    ok = false;
                }
            }
            18 => {
                if nv != 3 {
                    ok = false;
                }
            }
            19 => {
                if nv != 3 {
                    ok = false;
                }
            }
            20 => {
                if nv != 3 {
                    ok = false;
                }
            }
            21 => {
                if nv != 3 {
                    ok = false;
                }
            }
            22 => {
                if nv != 3 {
                    ok = false;
                }
            }
            23 => {
                if nv != 6 {
                    ok = false;
                }
            }
            24 => {
                if nv != 6 {
                    ok = false;
                }
            }
            25 => {
                if nv != 6 {
                    ok = false;
                }
            }
            26 => {
                if nv != 6 {
                    ok = false;
                }
            }
            27 => {
                if nv != 6 {
                    ok = false;
                }
            }
            28 => {
                if nv != 6 {
                    ok = false;
                }
            }
            29 => {
                if nv != 9 {
                    ok = false;
                }
            }
            30 => {
                if nv != 9 {
                    ok = false;
                }
            }
            31 => {
                if nv != 9 {
                    ok = false;
                }
            }
            32 => {
                if nv != 9 {
                    ok = false;
                }
            }
            33 => {
                if nv != 9 {
                    ok = false;
                }
            }
            34 => {
                if nv != 9 {
                    ok = false;
                }
            }
            _ => {
                ach.add_fail_text("Incorrect Form Number", "");
            }
        }
        if !ok {
            ach.add_fail_text("Incorrect count of real values in array V for FEM node", "");
        }
        let mut ne: i32 = ent.nb_elements()?;
        for i in 1..=ne {
            let mut mess = String::new();
            let mut dlf: i32 = ent.data_layer_flag(i)?;
            let mut nl: i32 = ent.nb_layers(i)?;
            let mut nrl: i32 = ent.nb_result_data_locs(i)?;
            if (dlf < 0) || (dlf > 4) {
                ach.add_fail_text("One of the Data Layer Flags not in [0-4]", "");
            }
            if (dlf < 4) && (ent.nb_layers(i)? != 1) {
                mess = format!(
                    "Nb. of Layers n0.{} not ONE while Data Layer Flag is in [0-3]",
                    i
                );
                ach.add_fail_text(&mess, "");
            }
            if (rrf == 1) || (rrf == 2) {
                if (nrl != 1) || (ent.result_data_loc(i, 1)? != 0) {
                    mess = format!(
                        "Result Data Locs n0.{} incorrect for Result Report = 1 or 2",
                        i
                    );
                    ach.add_fail_text(&mess, "");
                }
            }
            if ent.nb_results(i)? != ((nv * nl) * nrl) {
                mess = format!(
                    "Nb. of results for Element n0.{} incorrect, should be {}",
                    i,
                    (nv * nl) * nrl
                );
                ach.add_fail_text(&mess, "");
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
        another: &Handle<IgesAppliElementResults>,
        ent: &Handle<IgesAppliElementResults>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut j: i32 = Default::default();
        let mut num: i32 = another.nb_elements()?;
        let mut a_note: Handle<IgesDimenGeneralNote> =
            tc.transferred(&another.note().cast())?.cast();
        let mut a_subcase_number: i32 = another.sub_case_number();
        let mut a_time: f64 = another.time();
        let mut nv: i32 = another.nb_result_values();
        let mut a_result_report_flag: i32 = another.result_report_flag();
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
        }
        for i in 1..=num {
            let mut nl: i32 = Default::default();
            let mut nrl: i32 = Default::default();
            let mut numv: i32 = Default::default();
            iges_array1_set(
                &all_element_identifiers,
                i,
                another.element_identifier(i)?,
                "IgesAppliToolElementResults::own_copy",
            )?;
            iges_array1_set(
                &all_elements,
                i,
                tc.transferred(&another.element(i)?.cast())?
                    .cast::<IgesAppliFiniteElement>(),
                "IgesAppliToolElementResults::own_copy",
            )?;
            iges_array1_set(
                &all_element_topology_type,
                i,
                another.element_topology_type(i)?,
                "IgesAppliToolElementResults::own_copy",
            )?;
            nl = another.nb_layers(i)?;
            iges_array1_set(
                &all_nb_layers,
                i,
                nl,
                "IgesAppliToolElementResults::own_copy",
            )?;
            iges_array1_set(
                &all_data_layer_flag,
                i,
                another.data_layer_flag(i)?,
                "IgesAppliToolElementResults::own_copy",
            )?;
            nrl = another.nb_result_data_locs(i)?;
            iges_array1_set(
                &all_nb_result_data_locs,
                i,
                nrl,
                "IgesAppliToolElementResults::own_copy",
            )?;
            let mut rdrl: Handle<HArray1<i32>> = iges_new_array1::<i32>(1, nrl)?;
            iges_live(
                &all_result_data_loc,
                "IgesAppliToolElementResults::own_copy",
            )?
            .set_value(i, &rdrl)?;
            for j in 1..=nrl {
                iges_array1_set(
                    &rdrl,
                    j,
                    another.result_data_loc(i, j)?,
                    "IgesAppliToolElementResults::own_copy",
                )?;
            }
            numv = another.nb_results(i)?;
            let mut vres: Handle<HArray1<f64>> = iges_new_array1::<f64>(1, numv)?;
            for j in 1..=numv {
                iges_array1_set(
                    &vres,
                    j,
                    another.result_data(i, j)?,
                    "IgesAppliToolElementResults::own_copy",
                )?;
            }
            iges_live(&all_result_data, "IgesAppliToolElementResults::own_copy")?
                .set_value(i, &vres)?;
        }
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
        ent.set_form_number(another.form_number())?;
        Ok(())
    }
}
