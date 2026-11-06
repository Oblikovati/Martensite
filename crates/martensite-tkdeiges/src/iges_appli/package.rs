//! The protocol, the modules and the package class of IgesAppli.
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
use crate::iges_appli::{
    IgesAppliDrilledHole, IgesAppliElementResults, IgesAppliFiniteElement, IgesAppliFlow,
    IgesAppliFlowLineSpec, IgesAppliLevelFunction, IgesAppliLevelToPWBLayerMap,
    IgesAppliLineWidening, IgesAppliNodalConstraint, IgesAppliNodalDisplAndRot,
    IgesAppliNodalResults, IgesAppliNode, IgesAppliPWBArtworkStackup, IgesAppliPWBDrilledHole,
    IgesAppliPartNumber, IgesAppliPinNumber, IgesAppliPipingFlow, IgesAppliReferenceDesignator,
    IgesAppliRegionRestriction, IgesAppliToolDrilledHole, IgesAppliToolElementResults,
    IgesAppliToolFiniteElement, IgesAppliToolFlow, IgesAppliToolFlowLineSpec,
    IgesAppliToolLevelFunction, IgesAppliToolLevelToPWBLayerMap, IgesAppliToolLineWidening,
    IgesAppliToolNodalConstraint, IgesAppliToolNodalDisplAndRot, IgesAppliToolNodalResults,
    IgesAppliToolNode, IgesAppliToolPWBArtworkStackup, IgesAppliToolPWBDrilledHole,
    IgesAppliToolPartNumber, IgesAppliToolPinNumber, IgesAppliToolPipingFlow,
    IgesAppliToolReferenceDesignator, IgesAppliToolRegionRestriction,
};
use crate::iges_defs::IgesDefs;
use crate::iges_draw::IgesDraw;
use crate::iges_package_tables;
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::interface::InterfaceCategory;

fn case_iges(typenum: i32, formnum: i32) -> i32 {
    match typenum {
        134 => {
            return 12;
        }
        136 => {
            return 3;
        }
        138 => {
            return 10;
        }
        146 => {
            return 11;
        }
        148 => {
            return 2;
        }
        402 => match formnum {
            18 => {
                return 4;
            }
            20 => {
                return 17;
            }
            _ => {}
        },
        406 => match formnum {
            2 => {
                return 19;
            }
            3 => {
                return 6;
            }
            5 => {
                return 8;
            }
            6 => {
                return 1;
            }
            7 => {
                return 18;
            }
            8 => {
                return 16;
            }
            9 => {
                return 15;
            }
            14 => {
                return 5;
            }
            24 => {
                return 7;
            }
            25 => {
                return 13;
            }
            26 => {
                return 14;
            }
            _ => {}
        },
        418 => {
            return 9;
        }
        _ => {}
    }
    return 0;
}

fn category(cn: i32, _ent: &Handle<martensite_tkernel::standard::StandardTransient>) -> i32 {
    if ((cn == 4) || (cn == 5)) || (cn == 17) {
        return InterfaceCategory::number("Piping");
    }
    if ((cn == 2) || (cn == 3)) || ((cn >= 9) && (cn <= 12)) {
        return InterfaceCategory::number("FEA");
    }
    return InterfaceCategory::number("Professional");
}

iges_package_tables! {
    package: IgesAppli,
    protocol: IgesAppliProtocol "IgesAppliProtocol",
    read_write: IgesAppliReadWriteModule "IgesAppliReadWriteModule",
    general: IgesAppliGeneralModule "IgesAppliGeneralModule",
    specific: IgesAppliSpecificModule "IgesAppliSpecificModule",
    resources: [1 => IgesDefs::protocol(), 2 => IgesDraw::protocol()],
    prior_init: [IgesDefs::try_init(), IgesDraw::try_init()],
    case_iges: case_iges,
    category: category,
    cases: [1 => IgesAppliDrilledHole, IgesAppliToolDrilledHole;
2 => IgesAppliElementResults, IgesAppliToolElementResults;
3 => IgesAppliFiniteElement, IgesAppliToolFiniteElement;
4 => IgesAppliFlow, IgesAppliToolFlow;
5 => IgesAppliFlowLineSpec, IgesAppliToolFlowLineSpec;
6 => IgesAppliLevelFunction, IgesAppliToolLevelFunction;
7 => IgesAppliLevelToPWBLayerMap, IgesAppliToolLevelToPWBLayerMap;
8 => IgesAppliLineWidening, IgesAppliToolLineWidening;
9 => IgesAppliNodalConstraint, IgesAppliToolNodalConstraint;
10 => IgesAppliNodalDisplAndRot, IgesAppliToolNodalDisplAndRot;
11 => IgesAppliNodalResults, IgesAppliToolNodalResults;
12 => IgesAppliNode, IgesAppliToolNode;
13 => IgesAppliPWBArtworkStackup, IgesAppliToolPWBArtworkStackup;
14 => IgesAppliPWBDrilledHole, IgesAppliToolPWBDrilledHole;
15 => IgesAppliPartNumber, IgesAppliToolPartNumber;
16 => IgesAppliPinNumber, IgesAppliToolPinNumber;
17 => IgesAppliPipingFlow, IgesAppliToolPipingFlow;
18 => IgesAppliReferenceDesignator, IgesAppliToolReferenceDesignator;
19 => IgesAppliRegionRestriction, IgesAppliToolRegionRestriction;],
    corrected: [1 => IgesAppliDrilledHole, IgesAppliToolDrilledHole;
4 => IgesAppliFlow, IgesAppliToolFlow;
6 => IgesAppliLevelFunction, IgesAppliToolLevelFunction;
8 => IgesAppliLineWidening, IgesAppliToolLineWidening;
14 => IgesAppliPWBDrilledHole, IgesAppliToolPWBDrilledHole;
15 => IgesAppliPartNumber, IgesAppliToolPartNumber;
16 => IgesAppliPinNumber, IgesAppliToolPinNumber;
17 => IgesAppliPipingFlow, IgesAppliToolPipingFlow;
18 => IgesAppliReferenceDesignator, IgesAppliToolReferenceDesignator;
19 => IgesAppliRegionRestriction, IgesAppliToolRegionRestriction;],
    new_void: [1 => Handle::new(IgesAppliDrilledHole::new()).cast();
2 => Handle::new(IgesAppliElementResults::new()).cast();
3 => Handle::new(IgesAppliFiniteElement::new()).cast();
4 => Handle::new(IgesAppliFlow::new()).cast();
5 => Handle::new(IgesAppliFlowLineSpec::new()).cast();
6 => Handle::new(IgesAppliLevelFunction::new()).cast();
7 => Handle::new(IgesAppliLevelToPWBLayerMap::new()).cast();
8 => Handle::new(IgesAppliLineWidening::new()).cast();
9 => Handle::new(IgesAppliNodalConstraint::new()).cast();
10 => Handle::new(IgesAppliNodalDisplAndRot::new()).cast();
11 => Handle::new(IgesAppliNodalResults::new()).cast();
12 => Handle::new(IgesAppliNode::new()).cast();
13 => Handle::new(IgesAppliPWBArtworkStackup::new()).cast();
14 => Handle::new(IgesAppliPWBDrilledHole::new()).cast();
15 => Handle::new(IgesAppliPartNumber::new()).cast();
16 => Handle::new(IgesAppliPinNumber::new()).cast();
17 => Handle::new(IgesAppliPipingFlow::new()).cast();
18 => Handle::new(IgesAppliReferenceDesignator::new()).cast();
19 => Handle::new(IgesAppliRegionRestriction::new()).cast();],
    general_extra: {  }
}
