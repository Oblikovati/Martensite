//! IgesAppliElementResults.
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
use crate::iges_appli::IgesAppliFiniteElement;
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower, iges_array1_upper};
use crate::iges_basic::{IgesBasicHArray1OfHArray1OfInteger, IgesBasicHArray1OfHArray1OfReal};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_live};
use crate::iges_dimen::IgesDimenGeneralNote;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines ElementResults, Type `148`
/// Used to find the results of FEM analysis
IgesAppliElementResults, IgesAppliElementResultsState, "IgesAppliElementResults", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_subcase_number: i32,
the_time: f64,
the_nb_result_values: i32,
the_result_report_flag: i32,
the_element_identifiers: Handle<HArray1<i32>>,
the_elements: Handle<HArray1<Handle<IgesAppliFiniteElement>>>,
the_element_topology_types: Handle<HArray1<i32>>,
the_nb_layers: Handle<HArray1<i32>>,
the_data_layer_flags: Handle<HArray1<i32>>,
the_nb_result_data_locs: Handle<HArray1<i32>>,
the_result_data_locs: Handle<IgesBasicHArray1OfHArray1OfInteger>,
the_result_data: Handle<IgesBasicHArray1OfHArray1OfReal>,
}
}

impl IgesAppliElementResults {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    get sub_case_number = the_subcase_number: i32;
    get time = the_time: f64;
    get nb_result_values = the_nb_result_values: i32;
    get result_report_flag = the_result_report_flag: i32;
    }
    /// This method is used to set the fields of the class
    /// ElementResults
    /// - aNote             : GeneralNote Entity describing analysis
    /// - aSubCase          : Analysis Subcase number
    /// - aTime             : Analysis time value
    /// - nbResults         : Number of result values per FEM
    /// - aResRepFlag       : Results Reporting Flag
    /// - allElementIdents  : FEM element number for elements
    /// - allFiniteElems    : FEM element
    /// - allTopTypes       : Element Topology Types
    /// - nbLayers          : Number of layers per result data location
    /// - allDataLayerFlags : Data Layer Flags
    /// - allnbResDataLocs  : Number of result data report locations
    /// - allResDataLocs    : Result Data Report Locations
    /// - allResults        : List of Result data values of FEM analysis
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        a_sub_case: i32,
        a_time: f64,
        nb_results: i32,
        a_res_rep_flag: i32,
        all_element_idents: &Handle<HArray1<i32>>,
        all_finite_elems: &Handle<HArray1<Handle<IgesAppliFiniteElement>>>,
        all_top_types: &Handle<HArray1<i32>>,
        nb_layers: &Handle<HArray1<i32>>,
        all_data_layer_flags: &Handle<HArray1<i32>>,
        allnb_res_data_locs: &Handle<HArray1<i32>>,
        all_res_data_locs: &Handle<IgesBasicHArray1OfHArray1OfInteger>,
        all_results: &Handle<IgesBasicHArray1OfHArray1OfReal>,
    ) -> Result<(), Failure> {
        let mut num: i32 = iges_array1_length(all_element_idents, "IgesAppliElementResults::init")?;
        if ((((((((((((((iges_array1_lower(
            all_element_idents,
            "IgesAppliElementResults::init",
        )? != 1)
            || (iges_array1_lower(all_finite_elems, "IgesAppliElementResults::init")?
                != 1))
            || (iges_array1_upper(all_finite_elems, "IgesAppliElementResults::init")?
                != num))
            || (iges_array1_lower(all_top_types, "IgesAppliElementResults::init")? != 1))
            || (iges_array1_upper(all_top_types, "IgesAppliElementResults::init")? != num))
            || (iges_array1_lower(nb_layers, "IgesAppliElementResults::init")? != 1))
            || (iges_array1_upper(nb_layers, "IgesAppliElementResults::init")? != num))
            || (iges_array1_lower(all_data_layer_flags, "IgesAppliElementResults::init")?
                != 1))
            || (iges_array1_upper(all_data_layer_flags, "IgesAppliElementResults::init")?
                != num))
            || (iges_array1_lower(allnb_res_data_locs, "IgesAppliElementResults::init")? != 1))
            || (iges_array1_upper(allnb_res_data_locs, "IgesAppliElementResults::init")? != num))
            || ((iges_live(all_res_data_locs, "IgesAppliElementResults::init")?.lower()) != 1))
            || ((iges_live(all_res_data_locs, "IgesAppliElementResults::init")?.upper()) != num))
            || ((iges_live(all_results, "IgesAppliElementResults::init")?.lower()) != 1))
            || ((iges_live(all_results, "IgesAppliElementResults::init")?.upper()) != num)
        {
            return Err(exceptions::dimension_mismatch(
                "IgesAppliElementsResults : Init",
            ));
        }
        for i in 1..=num {
            let mut nl: i32 = iges_array1_item(nb_layers, i, "IgesAppliElementResults::init")?;
            let mut nrl: i32 =
                iges_array1_item(allnb_res_data_locs, i, "IgesAppliElementResults::init")?;
            if ((iges_array1_lower(
                &iges_live(all_res_data_locs, "IgesAppliElementResults::init")?.value(i)?,
                "IgesAppliElementResults::init",
            )?) != 1)
                || ((iges_array1_upper(
                    &iges_live(all_res_data_locs, "IgesAppliElementResults::init")?.value(i)?,
                    "IgesAppliElementResults::init",
                )?) != nrl)
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesAppliElementsResults : DataLoc",
                ));
            }
            if ((iges_array1_lower(
                &iges_live(all_results, "IgesAppliElementResults::init")?.value(i)?,
                "IgesAppliElementResults::init",
            )?) != 1)
                || ((iges_array1_upper(
                    &iges_live(all_results, "IgesAppliElementResults::init")?.value(i)?,
                    "IgesAppliElementResults::init",
                )?) != ((nl * nrl) * nb_results))
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesAppliElementsResults : Results",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_subcase_number = a_sub_case;
            state.the_time = a_time;
            state.the_nb_result_values = nb_results;
            state.the_result_report_flag = a_res_rep_flag;
            state.the_element_identifiers = all_element_idents.clone();
            state.the_elements = all_finite_elems.clone();
            state.the_element_topology_types = all_top_types.clone();
            state.the_nb_layers = nb_layers.clone();
            state.the_data_layer_flags = all_data_layer_flags.clone();
            state.the_nb_result_data_locs = allnb_res_data_locs.clone();
            state.the_result_data_locs = all_res_data_locs.clone();
            state.the_result_data = all_results.clone();
        }
        self.init_type_and_form(148, self.form_number());
        Ok(())
    }
    /// Changes the FormNumber (which indicates Type of Result)
    /// Error if not in range \[0-34\]
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if (form < 0) || (form > 34) {
            return Err(exceptions::out_of_range(
                "IgesAppliElementResults : SetFormNumber",
            ));
        }
        self.init_type_and_form(148, form);
        Ok(())
    }
    /// returns number of FEM elements
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_elements(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_elements,
            "IgesAppliElementResults::nb_elements",
        )
    }
    /// returns FEM element number for elements
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn element_identifier(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_element_identifiers,
            index,
            "IgesAppliElementResults::element_identifier",
        )
    }
    /// returns FEM element
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn element(&self, index: i32) -> Result<Handle<IgesAppliFiniteElement>, Failure> {
        iges_array1_item(
            &self.state.read().the_elements,
            index,
            "IgesAppliElementResults::element",
        )
    }
    /// returns element Topology Types
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn element_topology_type(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_element_topology_types,
            index,
            "IgesAppliElementResults::element_topology_type",
        )
    }
    /// returns number of layers per result data location
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_layers(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_nb_layers,
            index,
            "IgesAppliElementResults::nb_layers",
        )
    }
    /// returns Data Layer Flags
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn data_layer_flag(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_data_layer_flags,
            index,
            "IgesAppliElementResults::data_layer_flag",
        )
    }
    /// returns number of result data report locations
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_result_data_locs(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_nb_result_data_locs,
            index,
            "IgesAppliElementResults::nb_result_data_locs",
        )
    }
    /// returns Result Data Report Locations
    /// UNFINISHED
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn result_data_loc(&self, n_elem: i32, n_loc: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &iges_live(
                &self.state.read().the_result_data_locs,
                "IgesAppliElementResults::result_data_loc",
            )?
            .value(n_elem)?,
            n_loc,
            "IgesAppliElementResults::result_data_loc",
        )
    }
    /// returns total number of results
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_results(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_length(
            &iges_live(
                &self.state.read().the_result_data,
                "IgesAppliElementResults::nb_results",
            )?
            .value(index)?,
            "IgesAppliElementResults::nb_results",
        )
    }
    /// returns Result data value for an Element, given its
    /// order between 1 and &lt;NbResults(NElem)&gt; (direct access)
    /// For a more comprehensive access, see below
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn result_data(&self, n_elem: i32, num: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &iges_live(
                &self.state.read().the_result_data,
                "IgesAppliElementResults::result_data",
            )?
            .value(n_elem)?,
            num,
            "IgesAppliElementResults::result_data",
        )
    }
    /// Computes, for a given Element `NElem`, the rank of a
    /// individual Result Data, given `NVal`,`NLay`,`NLoc`
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn result_rank(
        &self,
        n_elem: i32,
        n_val: i32,
        n_lay: i32,
        n_loc: i32,
    ) -> Result<i32, Failure> {
        let mut num: i32 = n_val
            + ((self.state.read().the_nb_result_values)
                * (n_lay
                    + ((iges_array1_item(
                        &self.state.read().the_nb_layers,
                        n_elem,
                        "IgesAppliElementResults::result_rank",
                    )?) * n_loc)));
        Ok(num)
    }
    /// returns Result data values of FEM analysis, according this
    /// definition :
    /// - `NElem` : n0 of the Element to be considered
    /// - `NVal` : n0 of the Value between 1 and NbResultValues
    /// - `NLay` : n0 of the Layer for this Element
    /// - `NLoc` : n0 of the Data Location for this Element
    ///   This gives for each Element, the corresponding rank
    ///   computed by ResultRank, in which the leftmost subscript
    ///   changes most rapidly
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn result_data_int(
        &self,
        n_elem: i32,
        n_val: i32,
        n_lay: i32,
        n_loc: i32,
    ) -> Result<f64, Failure> {
        iges_array1_item(
            &iges_live(
                &self.state.read().the_result_data,
                "IgesAppliElementResults::result_data",
            )?
            .value(n_elem)?,
            self.result_rank(n_elem, n_val, n_lay, n_loc)?,
            "IgesAppliElementResults::result_data",
        )
    }
    /// Returns in once the entire list of data for an Element,
    /// addressed as by ResultRank (See above)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn result_list(&self, n_elem: i32) -> Result<Handle<HArray1<f64>>, Failure> {
        iges_live(
            &self.state.read().the_result_data,
            "IgesAppliElementResults::result_list",
        )?
        .value(n_elem)
    }
}
