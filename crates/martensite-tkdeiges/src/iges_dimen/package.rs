//! The protocol, the modules and the package class of IgesDimen.
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
use crate::iges_dimen::{
    IgesDimenAngularDimension, IgesDimenBasicDimension, IgesDimenCenterLine,
    IgesDimenCurveDimension, IgesDimenDiameterDimension, IgesDimenDimensionDisplayData,
    IgesDimenDimensionTolerance, IgesDimenDimensionUnits, IgesDimenDimensionedGeometry,
    IgesDimenFlagNote, IgesDimenGeneralLabel, IgesDimenGeneralNote, IgesDimenGeneralSymbol,
    IgesDimenLeaderArrow, IgesDimenLinearDimension, IgesDimenNewDimensionedGeometry,
    IgesDimenNewGeneralNote, IgesDimenOrdinateDimension, IgesDimenPointDimension,
    IgesDimenRadiusDimension, IgesDimenSection, IgesDimenSectionedArea,
    IgesDimenToolAngularDimension, IgesDimenToolBasicDimension, IgesDimenToolCenterLine,
    IgesDimenToolCurveDimension, IgesDimenToolDiameterDimension, IgesDimenToolDimensionDisplayData,
    IgesDimenToolDimensionTolerance, IgesDimenToolDimensionUnits, IgesDimenToolDimensionedGeometry,
    IgesDimenToolFlagNote, IgesDimenToolGeneralLabel, IgesDimenToolGeneralNote,
    IgesDimenToolGeneralSymbol, IgesDimenToolLeaderArrow, IgesDimenToolLinearDimension,
    IgesDimenToolNewDimensionedGeometry, IgesDimenToolNewGeneralNote,
    IgesDimenToolOrdinateDimension, IgesDimenToolPointDimension, IgesDimenToolRadiusDimension,
    IgesDimenToolSection, IgesDimenToolSectionedArea, IgesDimenToolWitnessLine,
    IgesDimenWitnessLine,
};
use crate::iges_geom::IgesGeom;
use crate::iges_graph::IgesGraph;
use crate::iges_package_tables;
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::interface::InterfaceCategory;

fn case_iges(typenum: i32, formnum: i32) -> i32 {
    match typenum {
        106 => {
            if (formnum == 20) || (formnum == 21) {
                return 3;
            } else {
                if (formnum > 30) && (formnum < 40) {
                    return 21;
                } else {
                    if formnum == 40 {
                        return 23;
                    }
                }
            }
        }
        202 => {
            return 1;
        }
        204 => {
            return 4;
        }
        206 => {
            return 5;
        }
        208 => {
            return 10;
        }
        210 => {
            return 11;
        }
        212 => {
            return 12;
        }
        213 => {
            return 17;
        }
        214 => {
            return 14;
        }
        216 => {
            return 15;
        }
        218 => {
            return 18;
        }
        220 => {
            return 19;
        }
        222 => {
            return 20;
        }
        228 => {
            return 13;
        }
        230 => {
            return 22;
        }
        402 => match formnum {
            13 => {
                return 9;
            }
            21 => {
                return 16;
            }
            _ => {}
        },
        406 => match formnum {
            28 => {
                return 8;
            }
            29 => {
                return 7;
            }
            30 => {
                return 6;
            }
            31 => {
                return 2;
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
    package: IgesDimen,
    protocol: IgesDimenProtocol "IgesDimenProtocol",
    read_write: IgesDimenReadWriteModule "IgesDimenReadWriteModule",
    general: IgesDimenGeneralModule "IgesDimenGeneralModule",
    specific: IgesDimenSpecificModule "IgesDimenSpecificModule",
    resources: [1 => IgesGraph::protocol(), 2 => IgesGeom::protocol()],
    prior_init: [IgesGeom::try_init(), IgesGraph::try_init()],
    case_iges: case_iges,
    category: category,
    cases: [1 => IgesDimenAngularDimension, IgesDimenToolAngularDimension;
2 => IgesDimenBasicDimension, IgesDimenToolBasicDimension;
3 => IgesDimenCenterLine, IgesDimenToolCenterLine;
4 => IgesDimenCurveDimension, IgesDimenToolCurveDimension;
5 => IgesDimenDiameterDimension, IgesDimenToolDiameterDimension;
6 => IgesDimenDimensionDisplayData, IgesDimenToolDimensionDisplayData;
7 => IgesDimenDimensionTolerance, IgesDimenToolDimensionTolerance;
8 => IgesDimenDimensionUnits, IgesDimenToolDimensionUnits;
9 => IgesDimenDimensionedGeometry, IgesDimenToolDimensionedGeometry;
10 => IgesDimenFlagNote, IgesDimenToolFlagNote;
11 => IgesDimenGeneralLabel, IgesDimenToolGeneralLabel;
12 => IgesDimenGeneralNote, IgesDimenToolGeneralNote;
13 => IgesDimenGeneralSymbol, IgesDimenToolGeneralSymbol;
14 => IgesDimenLeaderArrow, IgesDimenToolLeaderArrow;
15 => IgesDimenLinearDimension, IgesDimenToolLinearDimension;
16 => IgesDimenNewDimensionedGeometry, IgesDimenToolNewDimensionedGeometry;
17 => IgesDimenNewGeneralNote, IgesDimenToolNewGeneralNote;
18 => IgesDimenOrdinateDimension, IgesDimenToolOrdinateDimension;
19 => IgesDimenPointDimension, IgesDimenToolPointDimension;
20 => IgesDimenRadiusDimension, IgesDimenToolRadiusDimension;
21 => IgesDimenSection, IgesDimenToolSection;
22 => IgesDimenSectionedArea, IgesDimenToolSectionedArea;
23 => IgesDimenWitnessLine, IgesDimenToolWitnessLine;],
    corrected: [2 => IgesDimenBasicDimension, IgesDimenToolBasicDimension;
3 => IgesDimenCenterLine, IgesDimenToolCenterLine;
6 => IgesDimenDimensionDisplayData, IgesDimenToolDimensionDisplayData;
7 => IgesDimenDimensionTolerance, IgesDimenToolDimensionTolerance;
8 => IgesDimenDimensionUnits, IgesDimenToolDimensionUnits;
9 => IgesDimenDimensionedGeometry, IgesDimenToolDimensionedGeometry;
16 => IgesDimenNewDimensionedGeometry, IgesDimenToolNewDimensionedGeometry;
21 => IgesDimenSection, IgesDimenToolSection;
23 => IgesDimenWitnessLine, IgesDimenToolWitnessLine;],
    new_void: [1 => Handle::new(IgesDimenAngularDimension::new()).cast();
2 => Handle::new(IgesDimenBasicDimension::new()).cast();
3 => Handle::new(IgesDimenCenterLine::new()).cast();
4 => Handle::new(IgesDimenCurveDimension::new()).cast();
5 => Handle::new(IgesDimenDiameterDimension::new()).cast();
6 => Handle::new(IgesDimenDimensionDisplayData::new()).cast();
7 => Handle::new(IgesDimenDimensionTolerance::new()).cast();
8 => Handle::new(IgesDimenDimensionUnits::new()).cast();
9 => Handle::new(IgesDimenDimensionedGeometry::new()).cast();
10 => Handle::new(IgesDimenFlagNote::new()).cast();
11 => Handle::new(IgesDimenGeneralLabel::new()).cast();
12 => Handle::new(IgesDimenGeneralNote::new()).cast();
13 => Handle::new(IgesDimenGeneralSymbol::new()).cast();
14 => Handle::new(IgesDimenLeaderArrow::new()).cast();
15 => Handle::new(IgesDimenLinearDimension::new()).cast();
16 => Handle::new(IgesDimenNewDimensionedGeometry::new()).cast();
17 => Handle::new(IgesDimenNewGeneralNote::new()).cast();
18 => Handle::new(IgesDimenOrdinateDimension::new()).cast();
19 => Handle::new(IgesDimenPointDimension::new()).cast();
20 => Handle::new(IgesDimenRadiusDimension::new()).cast();
21 => Handle::new(IgesDimenSection::new()).cast();
22 => Handle::new(IgesDimenSectionedArea::new()).cast();
23 => Handle::new(IgesDimenWitnessLine::new()).cast();],
    general_extra: {  }
}
