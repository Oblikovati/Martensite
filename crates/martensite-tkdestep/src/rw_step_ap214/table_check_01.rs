use super::tables::*;
use crate::step_rw::*;

/// A part of the table.
pub(super) const TABLE_CHECK_01: &[CheckRow] = &[
    check_row!(
        39,
        StepGeomBSplineCurveWithKnots,
        RwStepGeomRwBSplineCurveWithKnots
    ),
    check_row!(
        41,
        StepGeomBSplineSurfaceWithKnots,
        RwStepGeomRwBSplineSurfaceWithKnots
    ),
    check_row!(52, StepShapeBrepWithVoids, RwStepShapeRwBrepWithVoids),
    check_row!(105, StepGeomDirection, RwStepGeomRwDirection),
    check_row!(116, StepShapeEdgeCurve, RwStepShapeRwEdgeCurve),
    check_row!(117, StepShapeEdgeLoop, RwStepShapeRwEdgeLoop),
    check_row!(119, StepGeomEllipse, RwStepGeomRwEllipse),
    check_row!(
        239,
        StepGeomRationalBSplineCurve,
        RwStepGeomRwRationalBSplineCurve
    ),
    check_row!(
        240,
        StepGeomRationalBSplineSurface,
        RwStepGeomRwRationalBSplineSurface
    ),
    check_row!(305, StepGeomToroidalSurface, RwStepGeomRwToroidalSurface),
    check_row!(313, StepGeomVector, RwStepGeomRwVector),
    check_row!(
        320,
        StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve,
        RwStepGeomRwBSplineCurveWithKnotsAndRationalBSplineCurve
    ),
    check_row!(
        323,
        StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface,
        RwStepGeomRwBSplineSurfaceWithKnotsAndRationalBSplineSurface
    ),
];
