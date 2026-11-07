use super::{
    IgesGeomBSplineCurve, IgesGeomBSplineSurface, IgesGeomBoundary, IgesGeomBoundedSurface,
    IgesGeomCircularArc, IgesGeomCompositeCurve, IgesGeomConicArc, IgesGeomCopiousData,
    IgesGeomCurveOnSurface, IgesGeomDirection, IgesGeomFlash, IgesGeomLine, IgesGeomOffsetCurve,
    IgesGeomOffsetSurface, IgesGeomPlane, IgesGeomPoint, IgesGeomRuledSurface, IgesGeomSplineCurve,
    IgesGeomSplineSurface, IgesGeomSurfaceOfRevolution, IgesGeomTabulatedCylinder,
    IgesGeomToolBSplineCurve, IgesGeomToolBSplineSurface, IgesGeomToolBoundary,
    IgesGeomToolBoundedSurface, IgesGeomToolCircularArc, IgesGeomToolCompositeCurve,
    IgesGeomToolConicArc, IgesGeomToolCopiousData, IgesGeomToolCurveOnSurface,
    IgesGeomToolDirection, IgesGeomToolFlash, IgesGeomToolLine, IgesGeomToolOffsetCurve,
    IgesGeomToolOffsetSurface, IgesGeomToolPlane, IgesGeomToolPoint, IgesGeomToolRuledSurface,
    IgesGeomToolSplineCurve, IgesGeomToolSplineSurface, IgesGeomToolSurfaceOfRevolution,
    IgesGeomToolTabulatedCylinder, IgesGeomToolTransformationMatrix, IgesGeomToolTrimmedSurface,
    IgesGeomTransformationMatrix, IgesGeomTrimmedSurface,
};
use crate::iges_basic::IgesBasic;
use crate::iges_package_tables;
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkxsbase::interface::InterfaceCategory;

/// The case of a type and form, 0 when not recognised.
fn iges_geom_case_iges(typenum: i32, formnum: i32) -> i32 {
    match (typenum, formnum) {
        (100, _) => 5,
        (102, _) => 6,
        (104, _) => 7,
        (106, form) if !(20..=40).contains(&form) => 8,
        (108, _) => 15,
        (110, _) => 12,
        (112, _) => 18,
        (114, _) => 19,
        (116, _) => 16,
        (118, _) => 17,
        (120, _) => 20,
        (122, _) => 21,
        (123, _) => 10,
        (124, _) => 22,
        (125, _) => 11,
        (126, _) => 1,
        (128, _) => 2,
        (130, _) => 13,
        (140, _) => 14,
        (141, _) => 3,
        (142, _) => 9,
        (143, _) => 4,
        (144, _) => 23,
        _ => 0,
    }
}

/// The category of a case.
fn iges_geom_category(cn: i32, ent: &Handle<StandardTransient>) -> i32 {
    if cn == 11 {
        return InterfaceCategory::number("Drawing");
    }
    if cn == 15
        && ent
            .cast::<IgesGeomPlane>()
            .get()
            .is_some_and(IgesGeomPlane::has_symbol_attach)
    {
        return InterfaceCategory::number("Drawing");
    }
    if cn == 16
        && ent
            .cast::<IgesGeomPoint>()
            .get()
            .is_some_and(IgesGeomPoint::has_display_symbol)
    {
        return InterfaceCategory::number("Drawing");
    }
    if cn == 22 {
        return InterfaceCategory::number("Auxiliary");
    }
    InterfaceCategory::number("Shape")
}

iges_package_tables! {
package: IgesGeom,
protocol: IgesGeomProtocol "IgesGeomProtocol",
read_write: IgesGeomReadWriteModule "IgesGeomReadWriteModule",
general: IgesGeomGeneralModule "IgesGeomGeneralModule",
specific: IgesGeomSpecificModule "IgesGeomSpecificModule",
resources: [1 => IgesBasic::protocol()],
prior_init: [IgesBasic::try_init()],
case_iges: iges_geom_case_iges,
category: iges_geom_category,
cases: [
1 => IgesGeomBSplineCurve, IgesGeomToolBSplineCurve;
2 => IgesGeomBSplineSurface, IgesGeomToolBSplineSurface;
3 => IgesGeomBoundary, IgesGeomToolBoundary;
4 => IgesGeomBoundedSurface, IgesGeomToolBoundedSurface;
5 => IgesGeomCircularArc, IgesGeomToolCircularArc;
6 => IgesGeomCompositeCurve, IgesGeomToolCompositeCurve;
7 => IgesGeomConicArc, IgesGeomToolConicArc;
8 => IgesGeomCopiousData, IgesGeomToolCopiousData;
9 => IgesGeomCurveOnSurface, IgesGeomToolCurveOnSurface;
10 => IgesGeomDirection, IgesGeomToolDirection;
11 => IgesGeomFlash, IgesGeomToolFlash;
12 => IgesGeomLine, IgesGeomToolLine;
13 => IgesGeomOffsetCurve, IgesGeomToolOffsetCurve;
14 => IgesGeomOffsetSurface, IgesGeomToolOffsetSurface;
15 => IgesGeomPlane, IgesGeomToolPlane;
16 => IgesGeomPoint, IgesGeomToolPoint;
17 => IgesGeomRuledSurface, IgesGeomToolRuledSurface;
18 => IgesGeomSplineCurve, IgesGeomToolSplineCurve;
19 => IgesGeomSplineSurface, IgesGeomToolSplineSurface;
20 => IgesGeomSurfaceOfRevolution, IgesGeomToolSurfaceOfRevolution;
21 => IgesGeomTabulatedCylinder, IgesGeomToolTabulatedCylinder;
22 => IgesGeomTransformationMatrix, IgesGeomToolTransformationMatrix;
23 => IgesGeomTrimmedSurface, IgesGeomToolTrimmedSurface;
],
corrected: [
3 => IgesGeomBoundary, IgesGeomToolBoundary;
7 => IgesGeomConicArc, IgesGeomToolConicArc;
9 => IgesGeomBoundary, IgesGeomToolBoundary;
11 => IgesGeomFlash, IgesGeomToolFlash;
13 => IgesGeomOffsetCurve, IgesGeomToolOffsetCurve;
22 => IgesGeomTransformationMatrix, IgesGeomToolTransformationMatrix;
],
new_void: [
1 => Handle::new(IgesGeomBSplineCurve::new()).cast();
2 => Handle::new(IgesGeomBSplineSurface::new()).cast();
3 => Handle::new(IgesGeomBoundary::new()).cast();
4 => Handle::new(IgesGeomBoundedSurface::new()).cast();
5 => Handle::new(IgesGeomCircularArc::new()).cast();
6 => Handle::new(IgesGeomCompositeCurve::new()).cast();
7 => Handle::new(IgesGeomConicArc::new()).cast();
8 => Handle::new(IgesGeomCopiousData::new()).cast();
9 => Handle::new(IgesGeomCurveOnSurface::new()).cast();
10 => Handle::new(IgesGeomDirection::new()).cast();
11 => Handle::new(IgesGeomFlash::new()).cast();
12 => Handle::new(IgesGeomLine::new()).cast();
13 => Handle::new(IgesGeomOffsetCurve::new()).cast();
14 => Handle::new(IgesGeomOffsetSurface::new()).cast();
15 => Handle::new(IgesGeomPlane::new()).cast();
16 => Handle::new(IgesGeomPoint::new()).cast();
17 => Handle::new(IgesGeomRuledSurface::new()).cast();
18 => Handle::new(IgesGeomSplineCurve::new()).cast();
19 => Handle::new(IgesGeomSplineSurface::new()).cast();
20 => Handle::new(IgesGeomSurfaceOfRevolution::new()).cast();
21 => Handle::new(IgesGeomTabulatedCylinder::new()).cast();
22 => Handle::new(IgesGeomTransformationMatrix::new()).cast();
23 => Handle::new(IgesGeomTrimmedSurface::new()).cast();
],
general_extra: {}
}
