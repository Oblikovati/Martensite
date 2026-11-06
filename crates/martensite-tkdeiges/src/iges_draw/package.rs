//! The protocol, the modules and the package class of IgesDraw.
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
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_dimen::IgesDimen;
use crate::iges_draw::{
    IgesDrawCircArraySubfigure, IgesDrawConnectPoint, IgesDrawDrawing, IgesDrawDrawingWithRotation,
    IgesDrawLabelDisplay, IgesDrawNetworkSubfigure, IgesDrawNetworkSubfigureDef,
    IgesDrawPerspectiveView, IgesDrawPlanar, IgesDrawRectArraySubfigure,
    IgesDrawSegmentedViewsVisible, IgesDrawToolCircArraySubfigure, IgesDrawToolConnectPoint,
    IgesDrawToolDrawing, IgesDrawToolDrawingWithRotation, IgesDrawToolLabelDisplay,
    IgesDrawToolNetworkSubfigure, IgesDrawToolNetworkSubfigureDef, IgesDrawToolPerspectiveView,
    IgesDrawToolPlanar, IgesDrawToolRectArraySubfigure, IgesDrawToolSegmentedViewsVisible,
    IgesDrawToolView, IgesDrawToolViewsVisible, IgesDrawToolViewsVisibleWithAttr, IgesDrawView,
    IgesDrawViewsVisible, IgesDrawViewsVisibleWithAttr,
};
use crate::{iges_case_dispatch, iges_package_tables};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceCategory, InterfaceCopyTool, InterfaceEntityIterator,
};

fn case_iges(typenum: i32, formnum: i32) -> i32 {
    match typenum {
        132 => {
            return 2;
        }
        320 => {
            return 7;
        }
        402 => match formnum {
            3 => {
                return 13;
            }
            4 => {
                return 14;
            }
            5 => {
                return 5;
            }
            16 => {
                return 9;
            }
            19 => {
                return 11;
            }
            _ => {}
        },
        404 => {
            return formnum + 3;
        }
        410 => {
            if formnum == 0 {
                return 12;
            }

            if formnum == 1 {
                return 8;
            }
        }
        412 => {
            return 10;
        }
        414 => {
            return 1;
        }
        420 => {
            return 6;
        }
        _ => {}
    }
    return 0;
}

fn category(cn: i32, _ent: &Handle<martensite_tkernel::standard::StandardTransient>) -> i32 {
    if cn == 9 {
        return InterfaceCategory::number("Auxiliary");
    }
    if ((cn == 1) || (cn == 2)) || (cn == 10) {
        return InterfaceCategory::number("Structure");
    }
    return InterfaceCategory::number("Drawing");
}

iges_package_tables! {
    package: IgesDraw,
    protocol: IgesDrawProtocol "IgesDrawProtocol",
    read_write: IgesDrawReadWriteModule "IgesDrawReadWriteModule",
    general: IgesDrawGeneralModule "IgesDrawGeneralModule",
    specific: IgesDrawSpecificModule "IgesDrawSpecificModule",
    resources: [1 => IgesDimen::protocol()],
    prior_init: [IgesDimen::try_init()],
    case_iges: case_iges,
    category: category,
    cases: [1 => IgesDrawCircArraySubfigure, IgesDrawToolCircArraySubfigure;
2 => IgesDrawConnectPoint, IgesDrawToolConnectPoint;
3 => IgesDrawDrawing, IgesDrawToolDrawing;
4 => IgesDrawDrawingWithRotation, IgesDrawToolDrawingWithRotation;
5 => IgesDrawLabelDisplay, IgesDrawToolLabelDisplay;
6 => IgesDrawNetworkSubfigure, IgesDrawToolNetworkSubfigure;
7 => IgesDrawNetworkSubfigureDef, IgesDrawToolNetworkSubfigureDef;
8 => IgesDrawPerspectiveView, IgesDrawToolPerspectiveView;
9 => IgesDrawPlanar, IgesDrawToolPlanar;
10 => IgesDrawRectArraySubfigure, IgesDrawToolRectArraySubfigure;
11 => IgesDrawSegmentedViewsVisible, IgesDrawToolSegmentedViewsVisible;
12 => IgesDrawView, IgesDrawToolView;
13 => IgesDrawViewsVisible, IgesDrawToolViewsVisible;
14 => IgesDrawViewsVisibleWithAttr, IgesDrawToolViewsVisibleWithAttr;],
    corrected: [3 => IgesDrawDrawing, IgesDrawToolDrawing;
4 => IgesDrawDrawingWithRotation, IgesDrawToolDrawingWithRotation;
9 => IgesDrawPlanar, IgesDrawToolPlanar;
13 => IgesDrawViewsVisible, IgesDrawToolViewsVisible;
14 => IgesDrawViewsVisibleWithAttr, IgesDrawToolViewsVisibleWithAttr;],
    new_void: [1 => Handle::new(IgesDrawCircArraySubfigure::new()).cast();
2 => Handle::new(IgesDrawConnectPoint::new()).cast();
3 => Handle::new(IgesDrawDrawing::new()).cast();
4 => Handle::new(IgesDrawDrawingWithRotation::new()).cast();
5 => Handle::new(IgesDrawLabelDisplay::new()).cast();
6 => Handle::new(IgesDrawNetworkSubfigure::new()).cast();
7 => Handle::new(IgesDrawNetworkSubfigureDef::new()).cast();
8 => IgesDrawPerspectiveView::new_bound().cast();
9 => Handle::new(IgesDrawPlanar::new()).cast();
10 => Handle::new(IgesDrawRectArraySubfigure::new()).cast();
11 => Handle::new(IgesDrawSegmentedViewsVisible::new()).cast();
12 => IgesDrawView::new_bound().cast();
13 => Handle::new(IgesDrawViewsVisible::new()).cast();
14 => Handle::new(IgesDrawViewsVisibleWithAttr::new()).cast();],
    general_extra: { fn own_implied_case(
    &self,
    cn: i32,
    ent: &Handle<IgesDataIgesEntity>,
    iter: &mut InterfaceEntityIterator,
) -> Result<(), Failure> {
    iges_case_dispatch!(cn, ent, Ok(()), Ok(()), [
        13 => IgesDrawViewsVisible, IgesDrawToolViewsVisible;
        14 => IgesDrawViewsVisibleWithAttr, IgesDrawToolViewsVisibleWithAttr;
    ], |tool, typed| tool.own_implied(&typed, iter))
}
fn own_renew_case(
    &self,
    cn: i32,
    entfrom: &Handle<IgesDataIgesEntity>,
    entto: &Handle<IgesDataIgesEntity>,
    tc: &InterfaceCopyTool,
) -> Result<(), Failure> {
    iges_case_dispatch!(cn, entfrom, Ok(()), Ok(()), [
        13 => IgesDrawViewsVisible, IgesDrawToolViewsVisible;
        14 => IgesDrawViewsVisibleWithAttr, IgesDrawToolViewsVisibleWithAttr;
    ], |tool, typed| tool.own_renew(&typed, &entto.cast(), tc))
}
fn own_delete_case(&self, cn: i32, ent: &Handle<IgesDataIgesEntity>) {
    let forget = || -> Result<(), Failure> {
        iges_case_dispatch!(cn, ent, Ok(()), Ok(()), [
            13 => IgesDrawViewsVisible, IgesDrawToolViewsVisible;
            14 => IgesDrawViewsVisibleWithAttr, IgesDrawToolViewsVisibleWithAttr;
        ], |tool, typed| tool.own_when_delete(&typed))
    };
    // A deletion cannot report: what it could not clear is released with the entity.
    drop(forget());
} }
}
