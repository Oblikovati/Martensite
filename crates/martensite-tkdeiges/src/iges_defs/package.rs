//! The protocol, the modules and the package class of IgesDefs.
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
use crate::iges_defs::{
    IgesDefsAssociativityDef, IgesDefsAttributeDef, IgesDefsAttributeTable, IgesDefsGenericData,
    IgesDefsMacroDef, IgesDefsTabularData, IgesDefsToolAssociativityDef, IgesDefsToolAttributeDef,
    IgesDefsToolAttributeTable, IgesDefsToolGenericData, IgesDefsToolMacroDef,
    IgesDefsToolTabularData, IgesDefsToolUnitsData, IgesDefsUnitsData,
};
use crate::iges_graph::IgesGraph;
use crate::iges_package_tables;
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::interface::InterfaceCategory;

fn case_iges(typenum: i32, formnum: i32) -> i32 {
    match typenum {
        302 => {
            return 1;
        }
        306 => {
            return 5;
        }
        316 => {
            return 7;
        }
        322 => {
            return 2;
        }
        406 => match formnum {
            11 => {
                return 6;
            }
            27 => {
                return 4;
            }
            _ => {}
        },
        422 => {
            return 3;
        }
        _ => {}
    }
    return 0;
}

fn category(_cn: i32, _ent: &Handle<martensite_tkernel::standard::StandardTransient>) -> i32 {
    return InterfaceCategory::number("Auxiliary");
}

iges_package_tables! {
    package: IgesDefs,
    protocol: IgesDefsProtocol "IgesDefsProtocol",
    read_write: IgesDefsReadWriteModule "IgesDefsReadWriteModule",
    general: IgesDefsGeneralModule "IgesDefsGeneralModule",
    specific: IgesDefsSpecificModule "IgesDefsSpecificModule",
    resources: [1 => IgesGraph::protocol()],
    prior_init: [IgesGraph::try_init()],
    case_iges: case_iges,
    category: category,
    cases: [1 => IgesDefsAssociativityDef, IgesDefsToolAssociativityDef;
2 => IgesDefsAttributeDef, IgesDefsToolAttributeDef;
3 => IgesDefsAttributeTable, IgesDefsToolAttributeTable;
4 => IgesDefsGenericData, IgesDefsToolGenericData;
5 => IgesDefsMacroDef, IgesDefsToolMacroDef;
6 => IgesDefsTabularData, IgesDefsToolTabularData;
7 => IgesDefsUnitsData, IgesDefsToolUnitsData;],
    corrected: [],
    new_void: [1 => Handle::new(IgesDefsAssociativityDef::new()).cast();
2 => Handle::new(IgesDefsAttributeDef::new()).cast();
3 => Handle::new(IgesDefsAttributeTable::new()).cast();
4 => Handle::new(IgesDefsGenericData::new()).cast();
5 => Handle::new(IgesDefsMacroDef::new()).cast();
6 => Handle::new(IgesDefsTabularData::new()).cast();
7 => Handle::new(IgesDefsUnitsData::new()).cast();],
    general_extra: {  }
}
