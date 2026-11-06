//! The protocol, the modules and the package class of IgesGraph.
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
use crate::iges_basic::IgesBasic;
use crate::iges_graph::{
    IgesGraphColor, IgesGraphDefinitionLevel, IgesGraphDrawingSize, IgesGraphDrawingUnits,
    IgesGraphHighLight, IgesGraphIntercharacterSpacing, IgesGraphLineFontDefPattern,
    IgesGraphLineFontDefTemplate, IgesGraphLineFontPredefined, IgesGraphNominalSize, IgesGraphPick,
    IgesGraphTextDisplayTemplate, IgesGraphTextFontDef, IgesGraphToolColor,
    IgesGraphToolDefinitionLevel, IgesGraphToolDrawingSize, IgesGraphToolDrawingUnits,
    IgesGraphToolHighLight, IgesGraphToolIntercharacterSpacing, IgesGraphToolLineFontDefPattern,
    IgesGraphToolLineFontDefTemplate, IgesGraphToolLineFontPredefined, IgesGraphToolNominalSize,
    IgesGraphToolPick, IgesGraphToolTextDisplayTemplate, IgesGraphToolTextFontDef,
    IgesGraphToolUniformRectGrid, IgesGraphUniformRectGrid,
};
use crate::iges_package_tables;
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::interface::InterfaceCategory;

fn case_iges(typenum: i32, formnum: i32) -> i32 {
    match typenum {
        304 => {
            if formnum == 1 {
                return 9;
            }

            if formnum == 2 {
                return 7;
            }
        }
        310 => {
            return 13;
        }
        312 => {
            return 12;
        }
        314 => {
            return 1;
        }
        406 => match formnum {
            1 => {
                return 2;
            }
            13 => {
                return 10;
            }
            16 => {
                return 3;
            }
            17 => {
                return 4;
            }
            18 => {
                return 6;
            }
            19 => {
                return 8;
            }
            20 => {
                return 5;
            }
            21 => {
                return 11;
            }
            22 => {
                return 14;
            }
            _ => {}
        },
        _ => {}
    }
    return 0;
}

fn category(_cn: i32, _ent: &Handle<martensite_tkernel::standard::StandardTransient>) -> i32 {
    return InterfaceCategory::number("Drawing");
}

iges_package_tables! {
    package: IgesGraph,
    protocol: IgesGraphProtocol "IgesGraphProtocol",
    read_write: IgesGraphReadWriteModule "IgesGraphReadWriteModule",
    general: IgesGraphGeneralModule "IgesGraphGeneralModule",
    specific: IgesGraphSpecificModule "IgesGraphSpecificModule",
    resources: [1 => IgesBasic::protocol()],
    prior_init: [IgesBasic::try_init()],
    case_iges: case_iges,
    category: category,
    cases: [1 => IgesGraphColor, IgesGraphToolColor;
2 => IgesGraphDefinitionLevel, IgesGraphToolDefinitionLevel;
3 => IgesGraphDrawingSize, IgesGraphToolDrawingSize;
4 => IgesGraphDrawingUnits, IgesGraphToolDrawingUnits;
5 => IgesGraphHighLight, IgesGraphToolHighLight;
6 => IgesGraphIntercharacterSpacing, IgesGraphToolIntercharacterSpacing;
7 => IgesGraphLineFontDefPattern, IgesGraphToolLineFontDefPattern;
8 => IgesGraphLineFontPredefined, IgesGraphToolLineFontPredefined;
9 => IgesGraphLineFontDefTemplate, IgesGraphToolLineFontDefTemplate;
10 => IgesGraphNominalSize, IgesGraphToolNominalSize;
11 => IgesGraphPick, IgesGraphToolPick;
12 => IgesGraphTextDisplayTemplate, IgesGraphToolTextDisplayTemplate;
13 => IgesGraphTextFontDef, IgesGraphToolTextFontDef;
14 => IgesGraphUniformRectGrid, IgesGraphToolUniformRectGrid;],
    corrected: [3 => IgesGraphDrawingSize, IgesGraphToolDrawingSize;
4 => IgesGraphDrawingUnits, IgesGraphToolDrawingUnits;
5 => IgesGraphHighLight, IgesGraphToolHighLight;
6 => IgesGraphIntercharacterSpacing, IgesGraphToolIntercharacterSpacing;
8 => IgesGraphLineFontPredefined, IgesGraphToolLineFontPredefined;
10 => IgesGraphNominalSize, IgesGraphToolNominalSize;
11 => IgesGraphPick, IgesGraphToolPick;
14 => IgesGraphUniformRectGrid, IgesGraphToolUniformRectGrid;],
    new_void: [1 => Handle::new(IgesGraphColor::new()).cast();
2 => Handle::new(IgesGraphDefinitionLevel::new()).cast();
3 => Handle::new(IgesGraphDrawingSize::new()).cast();
4 => Handle::new(IgesGraphDrawingUnits::new()).cast();
5 => Handle::new(IgesGraphHighLight::new()).cast();
6 => Handle::new(IgesGraphIntercharacterSpacing::new()).cast();
7 => Handle::new(IgesGraphLineFontDefPattern::new()).cast();
8 => Handle::new(IgesGraphLineFontPredefined::new()).cast();
9 => Handle::new(IgesGraphLineFontDefTemplate::new()).cast();
10 => Handle::new(IgesGraphNominalSize::new()).cast();
11 => Handle::new(IgesGraphPick::new()).cast();
12 => Handle::new(IgesGraphTextDisplayTemplate::new()).cast();
13 => Handle::new(IgesGraphTextFontDef::new()).cast();
14 => Handle::new(IgesGraphUniformRectGrid::new()).cast();],
    general_extra: {  }
}
