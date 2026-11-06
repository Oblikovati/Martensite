//! IgesDefsAttributeTable.
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
use crate::iges_basic::entity_shape::{
    iges_array2_lower_col, iges_array2_lower_row, iges_array2_upper_col, iges_array2_upper_row,
};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_array2_item, iges_live};
use crate::iges_defs::IgesDefsAttributeDef;
use crate::iges_entity;
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGES Attribute Table, Type `422` Form &lt;0, 1&gt;
/// This class is used to represent an occurrence of
/// Attribute Table. This Class may be independent
/// or dependent or pointed at by other Entities.
IgesDefsAttributeTable, IgesDefsAttributeTableState, "IgesDefsAttributeTable", parent IgesDataIgesEntity, {
the_attributes: Handle<HArray2<Handle<StandardTransient>>>,
}
}

impl IgesDefsAttributeTable {
    /// This method is used to set the fields of the class
    /// AttributeTable
    /// - attributes : Attribute instances, created as
    ///   (1,NbAttributes,1,NbRows)
    /// - NbRows = 1 is a particular case (Form 0)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        attributes: &Handle<HArray2<Handle<StandardTransient>>>,
    ) -> Result<(), Failure> {
        if (iges_array2_lower_col(attributes, "IgesDefsAttributeTable::init")? != 1)
            || (iges_array2_lower_row(attributes, "IgesDefsAttributeTable::init")? != 1)
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDefsAttributeTable : Init",
            ));
        }
        self.state.write().the_attributes = attributes.clone();
        let mut fn_: i32 = self.form_number();
        if iges_array2_upper_col(attributes, "IgesDefsAttributeTable::init")? > 1 {
            fn_ = 1;
        } else {
            if (fn_ < 0) || (fn_ > 1) {
                fn_ = 0;
            }
        }
        self.init_type_and_form(422, fn_);
        Ok(())
    }
    /// Sets a Definition as Structure information
    /// (works by calling InitMisc)
    pub fn set_definition(&self, def: &Handle<IgesDefsAttributeDef>) {
        self.init_misc(
            &def.cast(),
            &self.label_display(),
            self.line_weight_number(),
        );
    }
    /// Return the Structure information in Directory Entry,
    /// casted as an AttributeDef
    pub fn definition(&self) -> Handle<IgesDefsAttributeDef> {
        self.structure().cast::<IgesDefsAttributeDef>()
    }
    /// returns Number of Rows. Remark that it is always 1 if Form = 0
    /// It means that the list of Attributes (by their number, and for each
    /// one its type and ValueCount) is repeated `NbRows` times
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_rows(&self) -> Result<i32, Failure> {
        iges_array2_upper_col(
            &self.state.read().the_attributes,
            "IgesDefsAttributeTable::nb_rows",
        )
    }
    /// returns Number of Attributes
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_attributes(&self) -> Result<i32, Failure> {
        iges_array2_upper_row(
            &self.state.read().the_attributes,
            "IgesDefsAttributeTable::nb_attributes",
        )
    }
    /// returns the Type of an Attribute, given its No. : it is read in the
    /// Definition.
    /// (1 : Integer, 2 : Real, 3 : String, 4 : Entity, 6 : Logical)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn data_type(&self, atnum: i32) -> Result<i32, Failure> {
        iges_live(&self.definition(), "IgesDefsAttributeTable::data_type")?.attribute_type(atnum)
    }
    /// returns the Count of Value for an Attribute, given its No. :
    /// it is read in the Definition.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn value_count(&self, atnum: i32) -> Result<i32, Failure> {
        iges_live(&self.definition(), "IgesDefsAttributeTable::value_count")?
            .attribute_value_count(atnum)
    }
    /// `attribute_list`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_list(
        &self,
        atnum: i32,
        rownum: i32,
    ) -> Result<Handle<StandardTransient>, Failure> {
        iges_array2_item(
            &self.state.read().the_attributes,
            atnum,
            rownum,
            "IgesDefsAttributeTable::attribute_list",
        )
    }
    /// Returns Attribute Value &lt;AtNum, Rownum, rank ValNum&gt; as an Integer
    /// Error if Indices out of Range, or no Value defined, or not an Integer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_integer(
        &self,
        atnum: i32,
        rownum: i32,
        valuenum: i32,
    ) -> Result<i32, Failure> {
        iges_array1_item(
            &iges_array2_item(
                &self.state.read().the_attributes,
                atnum,
                rownum,
                "IgesDefsAttributeTable::attribute_as_integer",
            )?
            .cast::<HArray1<i32>>(),
            valuenum,
            "IgesDefsAttributeTable::attribute_as_integer",
        )
    }
    /// Returns Attribute Value &lt;AtNum, Rownum, rank ValNum&gt; as a Real
    /// Error if Indices out of Range, or no Value defined, or not a Real
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_real(
        &self,
        atnum: i32,
        rownum: i32,
        valuenum: i32,
    ) -> Result<f64, Failure> {
        iges_array1_item(
            &iges_array2_item(
                &self.state.read().the_attributes,
                atnum,
                rownum,
                "IgesDefsAttributeTable::attribute_as_real",
            )?
            .cast::<HArray1<f64>>(),
            valuenum,
            "IgesDefsAttributeTable::attribute_as_real",
        )
    }
    /// Returns Attribute Value &lt;AtNum, Rownum, rank ValNum&gt; as an Integer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_string(
        &self,
        atnum: i32,
        rownum: i32,
        valuenum: i32,
    ) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &iges_array2_item(
                &self.state.read().the_attributes,
                atnum,
                rownum,
                "IgesDefsAttributeTable::attribute_as_string",
            )?
            .cast::<HArray1<Handle<HAsciiString>>>(),
            valuenum,
            "IgesDefsAttributeTable::attribute_as_string",
        )
    }
    /// Returns Attribute Value &lt;AtNum, Rownum, rank ValNum&gt; as an Entity
    /// Error if Indices out of Range, or no Value defined, or not an Entity
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_entity(
        &self,
        atnum: i32,
        rownum: i32,
        valuenum: i32,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &iges_array2_item(
                &self.state.read().the_attributes,
                atnum,
                rownum,
                "IgesDefsAttributeTable::attribute_as_entity",
            )?
            .cast::<HArray1<Handle<IgesDataIgesEntity>>>(),
            valuenum,
            "IgesDefsAttributeTable::attribute_as_entity",
        )
    }
    /// Returns Attribute Value &lt;AtNum, Rownum, rank ValNum&gt; as a Boolean
    /// Error if Indices out of Range, or no Value defined, or not a Logical
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn attribute_as_logical(
        &self,
        atnum: i32,
        rownum: i32,
        valuenum: i32,
    ) -> Result<bool, Failure> {
        Ok(self.attribute_as_integer(atnum, rownum, valuenum)? != 0)
    }
}
