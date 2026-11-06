//! IGES definition entities: attributes, macros, tabular data, units.
mod associativity_def;
mod attribute_def;
mod attribute_table;
#[cfg(test)]
mod class_tests_01;
mod generic_data;
mod macro_def;
mod tabular_data;
mod tool_associativity_def;
mod tool_attribute_def;
mod tool_attribute_def_2;
mod tool_attribute_def_3;
mod tool_attribute_def_4;
mod tool_attribute_table;
mod tool_attribute_table_2;
mod tool_attribute_table_3;
mod tool_generic_data;
mod tool_macro_def;
mod tool_tabular_data;
mod tool_units_data;
mod units_data;
pub use associativity_def::IgesDefsAssociativityDef;
pub use attribute_def::IgesDefsAttributeDef;
pub use attribute_table::IgesDefsAttributeTable;
pub use generic_data::IgesDefsGenericData;
pub use macro_def::IgesDefsMacroDef;
pub use tabular_data::IgesDefsTabularData;
pub use tool_associativity_def::IgesDefsToolAssociativityDef;
pub use tool_attribute_def::IgesDefsToolAttributeDef;
pub use tool_attribute_table::IgesDefsToolAttributeTable;
pub use tool_generic_data::IgesDefsToolGenericData;
pub use tool_macro_def::IgesDefsToolMacroDef;
pub use tool_tabular_data::IgesDefsToolTabularData;
pub use tool_units_data::IgesDefsToolUnitsData;
pub use units_data::IgesDefsUnitsData;
#[cfg(test)]
mod attribute_form_tests;
mod text_template_lists;
pub use text_template_lists::IgesDefsHArray1OfHArray1OfTextDisplayTemplate;
mod package;
pub use package::{
    IgesDefs, IgesDefsGeneralModule, IgesDefsProtocol, IgesDefsReadWriteModule,
    IgesDefsSpecificModule,
};
