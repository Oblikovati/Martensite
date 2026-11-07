//! IgesDefsUnitsData.
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
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::iges_entity;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGES UnitsData Entity, Type `316` Form `0`
/// This class stores data about a model's fundamental units.
IgesDefsUnitsData, IgesDefsUnitsDataState, "IgesDefsUnitsData", parent IgesDataIgesEntity, {
the_unit_types: Handle<HArray1<Handle<HAsciiString>>>,
the_unit_values: Handle<HArray1<Handle<HAsciiString>>>,
the_unit_scales: Handle<HArray1<f64>>,
}
}

impl IgesDefsUnitsData {
    /// This method is used to set the fields of the class
    /// UnitsData
    /// - unitTypes  : Types of the units being defined
    /// - unitValues : Unit Values of the units
    /// - unitScales : Multiplicative Scale Factors
    ///   raises exception if lengths of unitTypes, unitValues and
    ///   unitScale are not same
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        unit_types: &Handle<HArray1<Handle<HAsciiString>>>,
        unit_values: &Handle<HArray1<Handle<HAsciiString>>>,
        unit_scales: &Handle<HArray1<f64>>,
    ) -> Result<(), Failure> {
        let mut length: i32 = iges_array1_length(unit_types, "IgesDefsUnitsData::init")?;
        if ((iges_array1_lower(unit_types, "IgesDefsUnitsData::init")? != 1)
            || ((iges_array1_lower(unit_values, "IgesDefsUnitsData::init")? != 1)
                || (iges_array1_length(unit_values, "IgesDefsUnitsData::init")? != length)))
            || ((iges_array1_lower(unit_scales, "IgesDefsUnitsData::init")? != 1)
                || (iges_array1_length(unit_scales, "IgesDefsUnitsData::init")? != length))
        {
            return Err(exceptions::dimension_mismatch("IgesDefsUnitsData : Init"));
        }
        {
            let mut state = self.state.write();
            state.the_unit_types = unit_types.clone();
            state.the_unit_values = unit_values.clone();
            state.the_unit_scales = unit_scales.clone();
        }
        self.init_type_and_form(316, 0);
        Ok(())
    }
    /// returns the Number of units defined by this entity
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_units(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_unit_types,
            "IgesDefsUnitsData::nb_units",
        )
    }
    /// returns the Type of the UnitNum'th unit being defined
    /// raises exception if UnitNum at most 0 or UnitNum &gt; NbUnits()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn unit_type(&self, unit_num: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_unit_types,
            unit_num,
            "IgesDefsUnitsData::unit_type",
        )
    }
    /// returns the Units of the UnitNum'th unit being defined
    /// raises exception if UnitNum at most 0 or UnitNum &gt; NbUnits()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn unit_value(&self, unit_num: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_unit_values,
            unit_num,
            "IgesDefsUnitsData::unit_value",
        )
    }
    /// returns the multiplicative scale factor to be applied to the
    /// UnitNum'th unit being defined
    /// raises exception if UnitNum at most 0 or UnitNum &gt; NbUnits()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn scale_factor(&self, unit_num: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_unit_scales,
            unit_num,
            "IgesDefsUnitsData::scale_factor",
        )
    }
}
