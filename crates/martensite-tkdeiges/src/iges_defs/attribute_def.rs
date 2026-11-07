//! IgesDefsAttributeDef.
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
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_live};
use crate::iges_defs::IgesDefsHArray1OfHArray1OfTextDisplayTemplate;
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGES Attribute Table Definition Entity,
/// This is class is used to support the concept of well
/// defined collection of attributes, whether it is a table
/// or a single row of attributes.
IgesDefsAttributeDef, IgesDefsAttributeDefState, "IgesDefsAttributeDef", parent IgesDataIgesEntity, {
the_name: Handle<HAsciiString>,
the_list_type: i32,
the_attr_types: Handle<HArray1<i32>>,
the_attr_value_data_types: Handle<HArray1<i32>>,
the_attr_value_counts: Handle<HArray1<i32>>,
the_attr_values: Handle<HArray1<Handle<StandardTransient>>>,
the_attr_value_pointers: Handle<IgesDefsHArray1OfHArray1OfTextDisplayTemplate>,
}
}

impl IgesDefsAttributeDef {
    iges_methods! {
    get table_name = the_name: Handle<HAsciiString>;
    get list_type = the_list_type: i32;
    }
    /// `init`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_list_type: i32,
        attr_types: &Handle<HArray1<i32>>,
        attr_value_data_types: &Handle<HArray1<i32>>,
        attr_value_counts: &Handle<HArray1<i32>>,
        attr_values: &Handle<HArray1<Handle<StandardTransient>>>,
        attr_value_pointers: &Handle<IgesDefsHArray1OfHArray1OfTextDisplayTemplate>,
    ) -> Result<(), Failure> {
        let mut nb: i32 = iges_array1_length(attr_types, "IgesDefsAttributeDef::init")?;
        if ((((iges_array1_lower(attr_types, "IgesDefsAttributeDef::init")? != 1)
            || (iges_array1_lower(attr_value_data_types, "IgesDefsAttributeDef::init")? != 1))
            || (iges_array1_length(attr_value_data_types, "IgesDefsAttributeDef::init")? != nb))
            || (iges_array1_lower(attr_value_counts, "IgesDefsAttributeDef::init")? != 1))
            || (iges_array1_length(attr_value_counts, "IgesDefsAttributeDef::init")? != nb)
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDefsAttributeDef : Init",
            ));
        }
        if self.form_number() >= 1 {
            if (iges_array1_lower(attr_values, "IgesDefsAttributeDef::init")? != 1)
                || (iges_array1_length(attr_values, "IgesDefsAttributeDef::init")? != nb)
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesDefsAttributeDef : Init",
                ));
            }
        }
        if self.form_number() == 2 {
            if ((iges_live(attr_value_pointers, "IgesDefsAttributeDef::init")?.lower()) != 1)
                || ((iges_live(attr_value_pointers, "IgesDefsAttributeDef::init")?.length()) != nb)
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesDefsAttributeDef : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_name = a_name.clone();
            state.the_list_type = a_list_type;
            state.the_attr_types = attr_types.clone();
            state.the_attr_value_data_types = attr_value_data_types.clone();
            state.the_attr_value_counts = attr_value_counts.clone();
            state.the_attr_values = attr_values.clone();
            state.the_attr_value_pointers = attr_value_pointers.clone();
        }
        if attr_values.is_null() {
            self.init_type_and_form(322, 0);
        } else {
            if attr_value_pointers.is_null() {
                self.init_type_and_form(322, 1);
            } else {
                self.init_type_and_form(322, 2);
            }
        }
        Ok(())
    }
    /// Returns True if a Table Name is defined
    pub fn has_table_name(&self) -> bool {
        !(self.state.read().the_name.is_null())
    }
    /// returns the Number of Attributes
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_attributes(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_attr_types,
            "IgesDefsAttributeDef::nb_attributes",
        )
    }
    /// returns the num'th Attribute Type
    /// raises exception if num at most 0 or num &gt; NbAttributes()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_type(&self, num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_attr_types,
            num,
            "IgesDefsAttributeDef::attribute_type",
        )
    }
    /// returns the num'th Attribute value data type
    /// raises exception if num at most 0 or num &gt; NbAttributes()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_value_data_type(&self, num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_attr_value_data_types,
            num,
            "IgesDefsAttributeDef::attribute_value_data_type",
        )
    }
    /// returns the num'th Attribute value count
    /// raises exception if num at most 0 or num &gt; NbAttributes()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_value_count(&self, num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_attr_value_counts,
            num,
            "IgesDefsAttributeDef::attribute_value_count",
        )
    }
    /// returns false if Values are defined (i.e. for Form = 1 or 2)
    pub fn has_values(&self) -> bool {
        !(self.state.read().the_attr_values.is_null())
    }
    /// returns false if TextDisplays are defined (i.e. for Form = 2)
    pub fn has_text_display(&self) -> bool {
        !(self.state.read().the_attr_value_pointers.is_null())
    }
    /// `attribute_text_display`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_text_display(
        &self,
        attr_num: i32,
        pointer_num: i32,
    ) -> Result<Handle<IgesGraphTextDisplayTemplate>, Failure> {
        let mut res: Handle<IgesGraphTextDisplayTemplate> = Default::default();
        if self.has_text_display() {
            res = iges_array1_item(
                &iges_live(
                    &self.state.read().the_attr_value_pointers,
                    "IgesDefsAttributeDef::attribute_text_display",
                )?
                .value(attr_num)?,
                pointer_num,
                "IgesDefsAttributeDef::attribute_text_display",
            )?;
        }
        Ok(res)
    }
    /// Returns the List of Attributes `AttrNum`, as a Transient.
    /// Its effective Type depends of the Type of Attribute :
    /// HArray1OfInteger for Integer, Logical(0-1),
    /// HArray1OfReal for Real, HArray1OfHSaciiString for String,
    /// HArray1Ofentity for Entity (Pointer)
    /// See methods AttributeAs... for an accurate access
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_list(&self, attr_num: i32) -> Result<Handle<StandardTransient>, Failure> {
        let mut nulres: Handle<StandardTransient> = Default::default();
        if !self.has_values() {
            return Ok(nulres);
        }
        return iges_array1_item(
            &self.state.read().the_attr_values,
            attr_num,
            "IgesDefsAttributeDef::attribute_list",
        );
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as an Integer
    /// Error if Indices out of Range, or no Value defined, or not an Integer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_integer(&self, attr_num: i32, value_num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &iges_array1_item(
                &self.state.read().the_attr_values,
                attr_num,
                "IgesDefsAttributeDef::attribute_as_integer",
            )?
            .cast::<HArray1<i32>>(),
            value_num,
            "IgesDefsAttributeDef::attribute_as_integer",
        )
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as a Real
    /// Error if Indices out of Range, or no Value defined, or not a Real
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_real(&self, attr_num: i32, value_num: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &iges_array1_item(
                &self.state.read().the_attr_values,
                attr_num,
                "IgesDefsAttributeDef::attribute_as_real",
            )?
            .cast::<HArray1<f64>>(),
            value_num,
            "IgesDefsAttributeDef::attribute_as_real",
        )
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as an Integer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_string(
        &self,
        attr_num: i32,
        value_num: i32,
    ) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &iges_array1_item(
                &self.state.read().the_attr_values,
                attr_num,
                "IgesDefsAttributeDef::attribute_as_string",
            )?
            .cast::<HArray1<Handle<HAsciiString>>>(),
            value_num,
            "IgesDefsAttributeDef::attribute_as_string",
        )
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as an Entity
    /// Error if Indices out of Range, or no Value defined, or not a Entity
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_entity(
        &self,
        attr_num: i32,
        value_num: i32,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &iges_array1_item(
                &self.state.read().the_attr_values,
                attr_num,
                "IgesDefsAttributeDef::attribute_as_entity",
            )?
            .cast::<HArray1<Handle<IgesDataIgesEntity>>>(),
            value_num,
            "IgesDefsAttributeDef::attribute_as_entity",
        )
    }
    /// Returns Attribute Value &lt;AttrNum, rank ValueNum&gt; as a Boolean
    /// Error if Indices out of Range, or no Value defined, or not a Logical
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_logical(&self, attr_num: i32, value_num: i32) -> Result<bool, Failure> {
        Ok((iges_array1_item(
            &iges_array1_item(
                &self.state.read().the_attr_values,
                attr_num,
                "IgesDefsAttributeDef::attribute_as_logical",
            )?
            .cast::<HArray1<i32>>(),
            value_num,
            "IgesDefsAttributeDef::attribute_as_logical",
        )?) != 0)
    }
}
