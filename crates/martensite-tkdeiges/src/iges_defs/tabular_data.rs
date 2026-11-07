//! IgesDefsTabularData.
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
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_live};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// Defines IGES Tabular Data, Type `406` Form `11`,
/// This Class is used to provide a Structure to accommodate
/// point form data.
IgesDefsTabularData, IgesDefsTabularDataState, "IgesDefsTabularData", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_property_type: i32,
the_type_of_independent_variables: Handle<HArray1<i32>>,
the_nb_values: Handle<HArray1<i32>>,
the_independent_values: Handle<IgesBasicHArray1OfHArray1OfReal>,
the_dependent_values: Handle<IgesBasicHArray1OfHArray1OfReal>,
}
}

impl IgesDefsTabularData {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get computed_nb_property_values = the_nb_property_values: i32;
    get property_type = the_property_type: i32;
    }
    /// This method is used to set the fields of the class
    /// TabularData
    /// - nbProps     : Number of property values
    /// - propType    : Property Type
    /// - typesInd    : Type of independent variables
    /// - nbValuesInd : Number of values of independent variables
    /// - valuesInd   : Values of independent variables
    /// - valuesDep   : Values of dependent variables
    ///   raises exception if lengths of typeInd and nbValuesInd are not same
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        nb_props: i32,
        prop_type: i32,
        types_ind: &Handle<HArray1<i32>>,
        nb_values_ind: &Handle<HArray1<i32>>,
        values_ind: &Handle<IgesBasicHArray1OfHArray1OfReal>,
        values_dep: &Handle<IgesBasicHArray1OfHArray1OfReal>,
    ) -> Result<(), Failure> {
        let mut num: i32 = iges_array1_length(types_ind, "IgesDefsTabularData::init")?;
        if (((((iges_array1_lower(types_ind, "IgesDefsTabularData::init")? != 1)
            || (iges_array1_lower(nb_values_ind, "IgesDefsTabularData::init")? != 1))
            || (iges_array1_length(nb_values_ind, "IgesDefsTabularData::init")? != num))
            || ((iges_live(values_ind, "IgesDefsTabularData::init")?.lower()) != 1))
            || ((iges_live(values_ind, "IgesDefsTabularData::init")?.length()) != num))
            || ((iges_live(values_dep, "IgesDefsTabularData::init")?.lower()) != 1)
        {
            return Err(exceptions::dimension_mismatch("IgesDefsTabularData : Init"));
        }
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_props;
            state.the_property_type = prop_type;
            state.the_type_of_independent_variables = types_ind.clone();
            state.the_nb_values = nb_values_ind.clone();
            state.the_independent_values = values_ind.clone();
            state.the_dependent_values = values_dep.clone();
        }
        self.init_type_and_form(406, 11);
        Ok(())
    }
    /// checks, and correct as necessary, the number of property
    /// values. Returns True if corrected, False if already OK
    pub fn own_correct(&self) -> bool {
        let mut newnb: i32 = self.computed_nb_property_values();
        if newnb == (self.state.read().the_nb_property_values) {
            return false;
        }
        self.state.write().the_nb_property_values = newnb;
        return true;
    }
    /// returns the number of dependent variables
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_dependents(&self) -> Result<i32, Failure> {
        Ok(iges_live(
            &self.state.read().the_dependent_values,
            "IgesDefsTabularData::nb_dependents",
        )?
        .length())
    }
    /// returns the number of independent variables
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_independents(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_type_of_independent_variables,
            "IgesDefsTabularData::nb_independents",
        )
    }
    /// returns the type of the num'th independent variable
    /// raises exception if num at most 0 or num &gt; NbIndependents()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn type_of_independents(&self, num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_type_of_independent_variables,
            num,
            "IgesDefsTabularData::type_of_independents",
        )
    }
    /// returns the number of different values of the num'th indep. variable
    /// raises exception if num at most 0 or num &gt; NbIndependents()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_values(&self, num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_nb_values,
            num,
            "IgesDefsTabularData::nb_values",
        )
    }
    /// `independent_value`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn independent_value(&self, variablenum: i32, valuenum: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &iges_live(
                &self.state.read().the_independent_values,
                "IgesDefsTabularData::independent_value",
            )?
            .value(variablenum)?,
            valuenum,
            "IgesDefsTabularData::independent_value",
        )
    }
    /// `dependent_values`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn dependent_values(&self, num: i32) -> Result<Handle<HArray1<f64>>, Failure> {
        iges_live(
            &self.state.read().the_dependent_values,
            "IgesDefsTabularData::dependent_values",
        )?
        .value(num)
    }
    /// `dependent_value`.
    pub fn dependent_value(&self, _unused_0: i32, _unused_1: i32) -> f64 {
        let mut val: f64 = 0.0;
        val
    }
}
