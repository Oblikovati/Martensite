//! IgesDefsGenericData.
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
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGES Generic Data, Type `406` Form `27`
/// Used to communicate information defined by the system
/// operator while creating the model. The information is
/// system specific and does not map into one of the
/// predefined properties or associativities. Properties
/// and property values can be defined by multiple
/// instances of this property.
IgesDefsGenericData, IgesDefsGenericDataState, "IgesDefsGenericData", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_name: Handle<HAsciiString>,
the_types: Handle<HArray1<i32>>,
the_values: Handle<HArray1<Handle<StandardTransient>>>,
}
}

impl IgesDefsGenericData {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get name = the_name: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// GenericData
    /// - nbPropVal : Number of property values
    /// - aName     : Property Name
    /// - allTypes  : Property Types
    /// - allValues : Property Values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        nb_prop_val: i32,
        a_name: &Handle<HAsciiString>,
        all_types: &Handle<HArray1<i32>>,
        all_values: &Handle<HArray1<Handle<StandardTransient>>>,
    ) -> Result<(), Failure> {
        if ((!all_types.is_null()) && (!all_values.is_null()))
            && (((iges_array1_lower(all_values, "IgesDefsGenericData::init")? != 1)
                || (iges_array1_lower(all_types, "IgesDefsGenericData::init")? != 1))
                || (iges_array1_length(all_types, "IgesDefsGenericData::init")?
                    != iges_array1_length(all_values, "IgesDefsGenericData::init")?))
        {
            return Err(exceptions::dimension_mismatch("IgesDefsGenericData: Init"));
        }
        {
            let mut state = self.state.write();
            state.the_nb_property_values = nb_prop_val;
            state.the_name = a_name.clone();
            state.the_types = all_types.clone();
            state.the_values = all_values.clone();
        }
        self.init_type_and_form(406, 27);
        Ok(())
    }
    /// returns the number of TYPE/VALUE pairs
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_type_value_pairs(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_types,
            "IgesDefsGenericData::nb_type_value_pairs",
        )
    }
    /// returns the Index'th property value data type
    /// raises exception if Index at most 0 or Index &gt; NbTypeValuePairs()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn type_(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_types,
            index,
            "IgesDefsGenericData::type_",
        )
    }
    /// HArray1OfInteger (length 1), HArray1OfReal (length 1) for
    /// Integer, Real, Boolean (= Integer 0/1),
    /// HAsciiString for String (the value itself),
    /// entity for Entity (the value itself)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn value(&self, index: i32) -> Result<Handle<StandardTransient>, Failure> {
        iges_array1_item(
            &self.state.read().the_values,
            index,
            "IgesDefsGenericData::value",
        )
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as an Integer
    /// Error if Index out of Range, or not an Integer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn value_as_integer(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &iges_array1_item(
                &self.state.read().the_values,
                index,
                "IgesDefsGenericData::value_as_integer",
            )?
            .cast::<HArray1<i32>>(),
            1,
            "IgesDefsGenericData::value_as_integer",
        )
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as a Real
    /// Error if Index out of Range, or not a Real
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn value_as_real(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &iges_array1_item(
                &self.state.read().the_values,
                index,
                "IgesDefsGenericData::value_as_real",
            )?
            .cast::<HArray1<f64>>(),
            1,
            "IgesDefsGenericData::value_as_real",
        )
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as an Integer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn value_as_string(&self, index: i32) -> Result<Handle<HAsciiString>, Failure> {
        Ok(iges_array1_item(
            &self.state.read().the_values,
            index,
            "IgesDefsGenericData::value_as_string",
        )?
        .cast::<HAsciiString>())
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as an Entity
    /// Error if Index out of Range, or not a Entity
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn value_as_entity(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        Ok(iges_array1_item(
            &self.state.read().the_values,
            index,
            "IgesDefsGenericData::value_as_entity",
        )?
        .cast::<IgesDataIgesEntity>())
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as a Boolean
    /// Error if Index out of Range, or not a Logical
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn value_as_logical(&self, index: i32) -> Result<bool, Failure> {
        Ok((iges_array1_item(
            &iges_array1_item(
                &self.state.read().the_values,
                index,
                "IgesDefsGenericData::value_as_logical",
            )?
            .cast::<HArray1<i32>>(),
            1,
            "IgesDefsGenericData::value_as_logical",
        )?) != 0)
    }
}
