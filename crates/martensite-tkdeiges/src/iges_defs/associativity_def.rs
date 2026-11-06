//! IgesDefsAssociativityDef.
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
use crate::iges_basic::IgesBasicHArray1OfHArray1OfInteger;
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_live};
use crate::iges_entity;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines IGES Associativity Definition Entity, Type `302`
/// This class permits the preprocessor to define an
/// associativity schema. i.e., by using it preprocessor
/// defines the type of relationship.
IgesDefsAssociativityDef, IgesDefsAssociativityDefState, "IgesDefsAssociativityDef", parent IgesDataIgesEntity, {
the_back_pointer_reqs: Handle<HArray1<i32>>,
the_class_orders: Handle<HArray1<i32>>,
the_nb_items_per_class: Handle<HArray1<i32>>,
the_items: Handle<IgesBasicHArray1OfHArray1OfInteger>,
}
}

impl IgesDefsAssociativityDef {
    /// This method is used to set the fields of the class
    /// AssociativityDef
    /// - requirements : Back Pointers requirements
    /// - orders       : Class Orders
    /// - numItems     : Number of Items per Class
    /// - items        : Items in each class
    ///   raises exception if lengths of the arrays are not the same.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        requirements: &Handle<HArray1<i32>>,
        orders: &Handle<HArray1<i32>>,
        num_items: &Handle<HArray1<i32>>,
        items: &Handle<IgesBasicHArray1OfHArray1OfInteger>,
    ) -> Result<(), Failure> {
        let mut len: i32 = iges_array1_length(requirements, "IgesDefsAssociativityDef::init")?;
        if (((iges_array1_lower(requirements, "IgesDefsAssociativityDef::init")? != 1)
            || ((iges_array1_lower(orders, "IgesDefsAssociativityDef::init")? != 1)
                || (iges_array1_length(orders, "IgesDefsAssociativityDef::init")? != len)))
            || ((iges_array1_lower(num_items, "IgesDefsAssociativityDef::init")? != 1)
                || (iges_array1_length(num_items, "IgesDefsAssociativityDef::init")? != len)))
            || (((iges_live(items, "IgesDefsAssociativityDef::init")?.lower()) != 1)
                || ((iges_live(items, "IgesDefsAssociativityDef::init")?.length()) != len))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDefsAssociativityDef : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_back_pointer_reqs = requirements.clone();
            state.the_class_orders = orders.clone();
            state.the_nb_items_per_class = num_items.clone();
            state.the_items = items.clone();
        }
        self.init_type_and_form(302, self.form_number());
        Ok(())
    }
    /// `set_form_number`.
    pub fn set_form_number(&self, form: i32) {
        self.init_type_and_form(302, form);
    }
    /// returns the Number of class definitions
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_class_defs(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_back_pointer_reqs,
            "IgesDefsAssociativityDef::nb_class_defs",
        )
    }
    /// returns 1 if the theBackPointerReqs(ClassNum) = 1
    /// returns 0 if the theBackPointerReqs(ClassNum) = 2
    /// raises exception if ClassNum at most 0 or ClassNum &gt; NbClassDefs()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_back_pointer_req(&self, class_num: i32) -> Result<bool, Failure> {
        Ok((iges_array1_item(
            &self.state.read().the_back_pointer_reqs,
            class_num,
            "IgesDefsAssociativityDef::is_back_pointer_req",
        )?) == 1)
    }
    /// returns 1 or 2
    /// raises exception if ClassNum at most 0 or ClassNum &gt; NbClassDefs()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn back_pointer_req(&self, class_num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_back_pointer_reqs,
            class_num,
            "IgesDefsAssociativityDef::back_pointer_req",
        )
    }
    /// returns 1 if theClassOrders(ClassNum) = 1 (ordered class)
    /// returns 0 if theClassOrders(ClassNum) = 2 (unordered class)
    /// raises exception if ClassNum at most 0 or ClassNum &gt; NbClassDefs()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_ordered(&self, class_num: i32) -> Result<bool, Failure> {
        Ok((iges_array1_item(
            &self.state.read().the_class_orders,
            class_num,
            "IgesDefsAssociativityDef::is_ordered",
        )?) == 1)
    }
    /// returns 1 or 2
    /// raises exception if ClassNum at most 0 or ClassNum &gt; NbClassDefs()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn class_order(&self, class_num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_class_orders,
            class_num,
            "IgesDefsAssociativityDef::class_order",
        )
    }
    /// returns no. of items per class entry
    /// raises exception if ClassNum at most 0 or ClassNum &gt; NbClassDefs()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_items_per_class(&self, class_num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_nb_items_per_class,
            class_num,
            "IgesDefsAssociativityDef::nb_items_per_class",
        )
    }
    /// returns ItemNum'th Item of ClassNum'th Class
    /// raises exception if
    /// ClassNum at most 0 or ClassNum &gt; NbClassDefs()
    /// ItemNum at most 0 or ItemNum &gt; NbItemsPerClass(ClassNum)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn item(&self, class_num: i32, item_num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &iges_live(
                &self.state.read().the_items,
                "IgesDefsAssociativityDef::item",
            )?
            .value(class_num)?,
            item_num,
            "IgesDefsAssociativityDef::item",
        )
    }
}
