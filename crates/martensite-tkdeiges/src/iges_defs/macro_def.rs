//! IgesDefsMacroDef.
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
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGES Macro Definition Entity, Type `306` Form `0`
/// This Class specifies the action of a specific MACRO.
/// After specification MACRO can be used as necessary
/// by means of MACRO class instance entity.
IgesDefsMacroDef, IgesDefsMacroDefState, "IgesDefsMacroDef", parent IgesDataIgesEntity, {
the_macro: Handle<HAsciiString>,
the_entity_type_id: i32,
the_lang_statements: Handle<HArray1<Handle<HAsciiString>>>,
the_endmacro: Handle<HAsciiString>,
}
}

impl IgesDefsMacroDef {
    iges_methods! {
    get macro_ = the_macro: Handle<HAsciiString>;
    get entity_type_id = the_entity_type_id: i32;
    get endmacro = the_endmacro: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// MacroDef
    /// - macro          : MACRO
    /// - entityTypeID   : Entity Type ID
    /// - langStatements : Language Statements
    /// - endMacro       : END MACRO
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        macro_: &Handle<HAsciiString>,
        entity_type_id: i32,
        lang_statements: &Handle<HArray1<Handle<HAsciiString>>>,
        end_macro: &Handle<HAsciiString>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(lang_statements, "IgesDefsMacroDef::init")? != 1 {
            return Err(exceptions::dimension_mismatch("IgesDefsMacroDef : Init"));
        }
        {
            let mut state = self.state.write();
            state.the_macro = macro_.clone();
            state.the_entity_type_id = entity_type_id;
            state.the_lang_statements = lang_statements.clone();
            state.the_endmacro = end_macro.clone();
        }
        self.init_type_and_form(306, 0);
        Ok(())
    }
    /// returns the number of language statements
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_statements(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_lang_statements,
            "IgesDefsMacroDef::nb_statements",
        )
    }
    /// `language_statement`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn language_statement(&self, stat_num: i32) -> Result<Handle<HAsciiString>, Failure> {
        iges_array1_item(
            &self.state.read().the_lang_statements,
            stat_num,
            "IgesDefsMacroDef::language_statement",
        )
    }
}
