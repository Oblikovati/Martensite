use super::tables::*;
use crate::step_rw::*;

/// A part of the table.
pub(super) const TABLE_WRITE_01: &[WriteRow] = &[
    write_row!(1, StepBasicAddress, RwStepBasicRwAddress),
    write_row!(
        2,
        StepShapeAdvancedBrepShapeRepresentation,
        RwStepShapeRwAdvancedBrepShapeRepresentation
    ),
    write_row!(3, StepShapeAdvancedFace, RwStepShapeRwAdvancedFace),
    write_row!(
        4,
        StepVisualAnnotationCurveOccurrence,
        RwStepVisualRwAnnotationCurveOccurrence
    ),
    write_row!(
        5,
        StepVisualAnnotationFillArea,
        RwStepVisualRwAnnotationFillArea
    ),
    write_row!(
        6,
        StepVisualAnnotationFillAreaOccurrence,
        RwStepVisualRwAnnotationFillAreaOccurrence
    ),
    write_row!(
        7,
        StepVisualAnnotationOccurrence,
        RwStepVisualRwAnnotationOccurrence
    ),
    write_row!(11, StepReprMappedItem, RwStepReprRwMappedItem),
    write_row!(12, StepVisualStyledItem, RwStepVisualRwStyledItem),
    write_row!(
        13,
        StepBasicApplicationContext,
        RwStepBasicRwApplicationContext
    ),
    write_row!(
        14,
        StepBasicApplicationContextElement,
        RwStepBasicRwApplicationContextElement
    ),
    write_row!(
        15,
        StepBasicApplicationProtocolDefinition,
        RwStepBasicRwApplicationProtocolDefinition
    ),
    write_row!(16, StepBasicApproval, RwStepBasicRwApproval),
    write_row!(
        18,
        StepBasicApprovalPersonOrganization,
        RwStepBasicRwApprovalPersonOrganization
    ),
    write_row!(
        19,
        StepBasicApprovalRelationship,
        RwStepBasicRwApprovalRelationship
    ),
    write_row!(20, StepBasicApprovalRole, RwStepBasicRwApprovalRole),
    write_row!(21, StepBasicApprovalStatus, RwStepBasicRwApprovalStatus),
    write_row!(22, StepVisualAreaInSet, RwStepVisualRwAreaInSet),
    write_row!(
        23,
        StepAP214AutoDesignActualDateAndTimeAssignment,
        RwStepAP214RwAutoDesignActualDateAndTimeAssignment
    ),
    write_row!(
        24,
        StepAP214AutoDesignActualDateAssignment,
        RwStepAP214RwAutoDesignActualDateAssignment
    ),
    write_row!(
        25,
        StepAP214AutoDesignApprovalAssignment,
        RwStepAP214RwAutoDesignApprovalAssignment
    ),
    write_row!(
        26,
        StepAP214AutoDesignDateAndPersonAssignment,
        RwStepAP214RwAutoDesignDateAndPersonAssignment
    ),
    write_row!(
        27,
        StepAP214AutoDesignGroupAssignment,
        RwStepAP214RwAutoDesignGroupAssignment
    ),
    write_row!(
        28,
        StepAP214AutoDesignNominalDateAndTimeAssignment,
        RwStepAP214RwAutoDesignNominalDateAndTimeAssignment
    ),
    write_row!(
        29,
        StepAP214AutoDesignNominalDateAssignment,
        RwStepAP214RwAutoDesignNominalDateAssignment
    ),
    write_row!(
        30,
        StepAP214AutoDesignOrganizationAssignment,
        RwStepAP214RwAutoDesignOrganizationAssignment
    ),
    write_row!(
        31,
        StepAP214AutoDesignPersonAndOrganizationAssignment,
        RwStepAP214RwAutoDesignPersonAndOrganizationAssignment
    ),
    write_row!(
        32,
        StepAP214AutoDesignPresentedItem,
        RwStepAP214RwAutoDesignPresentedItem
    ),
    write_row!(
        33,
        StepAP214AutoDesignSecurityClassificationAssignment,
        RwStepAP214RwAutoDesignSecurityClassificationAssignment
    ),
    write_row!(35, StepGeomAxis1Placement, RwStepGeomRwAxis1Placement),
    write_row!(36, StepGeomAxis2Placement2d, RwStepGeomRwAxis2Placement2d),
    write_row!(37, StepGeomAxis2Placement3d, RwStepGeomRwAxis2Placement3d),
    write_row!(38, StepGeomBSplineCurve, RwStepGeomRwBSplineCurve),
    write_row!(
        39,
        StepGeomBSplineCurveWithKnots,
        RwStepGeomRwBSplineCurveWithKnots
    ),
    write_row!(40, StepGeomBSplineSurface, RwStepGeomRwBSplineSurface),
    write_row!(
        41,
        StepGeomBSplineSurfaceWithKnots,
        RwStepGeomRwBSplineSurfaceWithKnots
    ),
    write_row!(
        42,
        StepVisualBackgroundColour,
        RwStepVisualRwBackgroundColour
    ),
    write_row!(43, StepGeomBezierCurve, RwStepGeomRwBezierCurve),
    write_row!(44, StepGeomBezierSurface, RwStepGeomRwBezierSurface),
    write_row!(45, StepShapeBlock, RwStepShapeRwBlock),
    write_row!(46, StepShapeBooleanResult, RwStepShapeRwBooleanResult),
    write_row!(47, StepGeomBoundaryCurve, RwStepGeomRwBoundaryCurve),
    write_row!(48, StepGeomBoundedCurve, RwStepGeomRwBoundedCurve),
    write_row!(49, StepGeomBoundedSurface, RwStepGeomRwBoundedSurface),
    write_row!(50, StepShapeBoxDomain, RwStepShapeRwBoxDomain),
    write_row!(51, StepShapeBoxedHalfSpace, RwStepShapeRwBoxedHalfSpace),
    write_row!(52, StepShapeBrepWithVoids, RwStepShapeRwBrepWithVoids),
    write_row!(53, StepBasicCalendarDate, RwStepBasicRwCalendarDate),
    write_row!(54, StepVisualCameraImage, RwStepVisualRwCameraImage),
    write_row!(55, StepVisualCameraModel, RwStepVisualRwCameraModel),
    write_row!(56, StepVisualCameraModelD2, RwStepVisualRwCameraModelD2),
    write_row!(57, StepVisualCameraModelD3, RwStepVisualRwCameraModelD3),
    write_row!(58, StepVisualCameraUsage, RwStepVisualRwCameraUsage),
    write_row!(59, StepGeomCartesianPoint, RwStepGeomRwCartesianPoint),
    write_row!(
        60,
        StepGeomCartesianTransformationOperator,
        RwStepGeomRwCartesianTransformationOperator
    ),
    write_row!(
        61,
        StepGeomCartesianTransformationOperator3d,
        RwStepGeomRwCartesianTransformationOperator3d
    ),
    write_row!(62, StepGeomCircle, RwStepGeomRwCircle),
    write_row!(63, StepShapeClosedShell, RwStepShapeRwClosedShell),
    write_row!(64, StepVisualColour, RwStepVisualRwColour),
    write_row!(65, StepVisualColourRgb, RwStepVisualRwColourRgb),
    write_row!(
        66,
        StepVisualColourSpecification,
        RwStepVisualRwColourSpecification
    ),
    write_row!(67, StepGeomCompositeCurve, RwStepGeomRwCompositeCurve),
    write_row!(
        68,
        StepGeomCompositeCurveOnSurface,
        RwStepGeomRwCompositeCurveOnSurface
    ),
    write_row!(
        69,
        StepGeomCompositeCurveSegment,
        RwStepGeomRwCompositeCurveSegment
    ),
    write_row!(70, StepVisualCompositeText, RwStepVisualRwCompositeText),
    write_row!(
        73,
        StepVisualCompositeTextWithExtent,
        RwStepVisualRwCompositeTextWithExtent
    ),
    write_row!(74, StepGeomConic, RwStepGeomRwConic),
    write_row!(75, StepGeomConicalSurface, RwStepGeomRwConicalSurface),
    write_row!(76, StepShapeConnectedFaceSet, RwStepShapeRwConnectedFaceSet),
    write_row!(
        77,
        StepVisualContextDependentInvisibility,
        RwStepVisualRwContextDependentInvisibility
    ),
    write_row!(
        78,
        StepVisualContextDependentOverRidingStyledItem,
        RwStepVisualRwContextDependentOverRidingStyledItem
    ),
    write_row!(
        79,
        StepBasicConversionBasedUnit,
        RwStepBasicRwConversionBasedUnit
    ),
    write_row!(
        80,
        StepBasicCoordinatedUniversalTimeOffset,
        RwStepBasicRwCoordinatedUniversalTimeOffset
    ),
    write_row!(
        82,
        StepShapeCsgShapeRepresentation,
        RwStepShapeRwCsgShapeRepresentation
    ),
    write_row!(83, StepShapeCsgSolid, RwStepShapeRwCsgSolid),
    write_row!(84, StepGeomCurve, RwStepGeomRwCurve),
    write_row!(
        85,
        StepGeomCurveBoundedSurface,
        RwStepGeomRwCurveBoundedSurface
    ),
    write_row!(86, StepGeomCurveReplica, RwStepGeomRwCurveReplica),
    write_row!(87, StepVisualCurveStyle, RwStepVisualRwCurveStyle),
    write_row!(88, StepVisualCurveStyleFont, RwStepVisualRwCurveStyleFont),
    write_row!(
        89,
        StepVisualCurveStyleFontPattern,
        RwStepVisualRwCurveStyleFontPattern
    ),
    write_row!(
        90,
        StepGeomCylindricalSurface,
        RwStepGeomRwCylindricalSurface
    ),
    write_row!(91, StepBasicDate, RwStepBasicRwDate),
    write_row!(92, StepBasicDateAndTime, RwStepBasicRwDateAndTime),
    write_row!(95, StepBasicDateRole, RwStepBasicRwDateRole),
    write_row!(96, StepBasicDateTimeRole, RwStepBasicRwDateTimeRole),
    write_row!(
        98,
        StepReprDefinitionalRepresentation,
        RwStepReprRwDefinitionalRepresentation
    ),
    write_row!(99, StepGeomDegeneratePcurve, RwStepGeomRwDegeneratePcurve),
    write_row!(
        100,
        StepGeomDegenerateToroidalSurface,
        RwStepGeomRwDegenerateToroidalSurface
    ),
    write_row!(
        101,
        StepReprDescriptiveRepresentationItem,
        RwStepReprRwDescriptiveRepresentationItem
    ),
    write_row!(
        104,
        StepBasicDimensionalExponents,
        RwStepBasicRwDimensionalExponents
    ),
    write_row!(105, StepGeomDirection, RwStepGeomRwDirection),
    write_row!(106, StepVisualStyledItem, RwStepVisualRwStyledItem),
    write_row!(
        107,
        StepVisualDraughtingCallout,
        RwStepVisualRwDraughtingCallout
    ),
    write_row!(
        108,
        StepVisualDraughtingPreDefinedColour,
        RwStepVisualRwDraughtingPreDefinedColour
    ),
    write_row!(
        109,
        StepVisualDraughtingPreDefinedCurveFont,
        RwStepVisualRwDraughtingPreDefinedCurveFont
    ),
    write_row!(115, StepShapeEdge, RwStepShapeRwEdge),
    write_row!(116, StepShapeEdgeCurve, RwStepShapeRwEdgeCurve),
    write_row!(117, StepShapeEdgeLoop, RwStepShapeRwEdgeLoop),
    write_row!(
        118,
        StepGeomElementarySurface,
        RwStepGeomRwElementarySurface
    ),
    write_row!(119, StepGeomEllipse, RwStepGeomRwEllipse),
    write_row!(
        120,
        StepGeomEvaluatedDegeneratePcurve,
        RwStepGeomRwEvaluatedDegeneratePcurve
    ),
    write_row!(121, StepBasicExternalSource, RwStepBasicRwExternalSource),
    write_row!(
        122,
        StepVisualExternallyDefinedCurveFont,
        RwStepVisualRwExternallyDefinedCurveFont
    ),
    write_row!(
        124,
        StepBasicExternallyDefinedItem,
        RwStepBasicRwExternallyDefinedItem
    ),
    write_row!(
        126,
        StepBasicExternallyDefinedItem,
        RwStepBasicRwExternallyDefinedItem
    ),
    write_row!(
        128,
        StepShapeExtrudedAreaSolid,
        RwStepShapeRwExtrudedAreaSolid
    ),
    write_row!(129, StepShapeFace, RwStepShapeRwFace),
    write_row!(131, StepShapeFaceBound, RwStepShapeRwFaceBound),
    write_row!(132, StepShapeFaceOuterBound, RwStepShapeRwFaceOuterBound),
    write_row!(133, StepShapeFaceSurface, RwStepShapeRwFaceSurface),
    write_row!(134, StepShapeFacetedBrep, RwStepShapeRwFacetedBrep),
    write_row!(
        135,
        StepShapeFacetedBrepShapeRepresentation,
        RwStepShapeRwFacetedBrepShapeRepresentation
    ),
    write_row!(136, StepVisualFillAreaStyle, RwStepVisualRwFillAreaStyle),
    write_row!(
        137,
        StepVisualFillAreaStyleColour,
        RwStepVisualRwFillAreaStyleColour
    ),
    write_row!(
        141,
        StepReprFunctionallyDefinedTransformation,
        RwStepReprRwFunctionallyDefinedTransformation
    ),
    write_row!(
        142,
        StepShapeGeometricCurveSet,
        RwStepShapeRwGeometricCurveSet
    ),
    write_row!(
        143,
        StepGeomGeometricRepresentationContext,
        RwStepGeomRwGeometricRepresentationContext
    ),
    write_row!(
        144,
        StepGeomGeometricRepresentationItem,
        RwStepGeomRwGeometricRepresentationItem
    ),
    write_row!(145, StepShapeGeometricSet, RwStepShapeRwGeometricSet),
    write_row!(
        146,
        StepShapeGeometricallyBoundedSurfaceShapeRepresentation,
        RwStepShapeRwGeometricallyBoundedSurfaceShapeRepresentation
    ),
    write_row!(
        147,
        StepShapeGeometricallyBoundedWireframeShapeRepresentation,
        RwStepShapeRwGeometricallyBoundedWireframeShapeRepresentation
    ),
    write_row!(
        148,
        StepReprGlobalUncertaintyAssignedContext,
        RwStepReprRwGlobalUncertaintyAssignedContext
    ),
    write_row!(
        149,
        StepReprGlobalUnitAssignedContext,
        RwStepReprRwGlobalUnitAssignedContext
    ),
    write_row!(150, StepBasicGroup, RwStepBasicRwGroup),
    write_row!(
        152,
        StepBasicGroupRelationship,
        RwStepBasicRwGroupRelationship
    ),
    write_row!(153, StepShapeHalfSpaceSolid, RwStepShapeRwHalfSpaceSolid),
    write_row!(154, StepGeomHyperbola, RwStepGeomRwHyperbola),
    write_row!(
        155,
        StepGeomIntersectionCurve,
        RwStepGeomRwIntersectionCurve
    ),
    write_row!(156, StepVisualInvisibility, RwStepVisualRwInvisibility),
    write_row!(
        157,
        StepBasicLengthMeasureWithUnit,
        RwStepBasicRwLengthMeasureWithUnit
    ),
    write_row!(158, StepBasicLengthUnit, RwStepBasicRwLengthUnit),
    write_row!(159, StepGeomLine, RwStepGeomRwLine),
    write_row!(160, StepBasicLocalTime, RwStepBasicRwLocalTime),
    write_row!(161, StepShapeLoop, RwStepShapeRwLoop),
    write_row!(
        162,
        StepShapeManifoldSolidBrep,
        RwStepShapeRwManifoldSolidBrep
    ),
    write_row!(
        163,
        StepShapeManifoldSurfaceShapeRepresentation,
        RwStepShapeRwManifoldSurfaceShapeRepresentation
    ),
    write_row!(164, StepReprMappedItem, RwStepReprRwMappedItem),
    write_row!(165, StepBasicMeasureWithUnit, RwStepBasicRwMeasureWithUnit),
    write_row!(
        166,
        StepVisualMechanicalDesignGeometricPresentationArea,
        RwStepVisualRwMechanicalDesignGeometricPresentationArea
    ),
    write_row!(
        167,
        StepVisualMechanicalDesignGeometricPresentationRepresentation,
        RwStepVisualRwMechanicalDesignGeometricPresentationRepresentation
    ),
    write_row!(169, StepBasicNamedUnit, RwStepBasicRwNamedUnit),
    write_row!(171, StepGeomOffsetCurve3d, RwStepGeomRwOffsetCurve3d),
    write_row!(172, StepGeomOffsetSurface, RwStepGeomRwOffsetSurface),
    write_row!(174, StepShapeOpenShell, RwStepShapeRwOpenShell),
    write_row!(175, StepBasicOrdinalDate, RwStepBasicRwOrdinalDate),
    write_row!(176, StepBasicOrganization, RwStepBasicRwOrganization),
    write_row!(
        178,
        StepBasicOrganizationRole,
        RwStepBasicRwOrganizationRole
    ),
    write_row!(
        179,
        StepBasicOrganizationalAddress,
        RwStepBasicRwOrganizationalAddress
    ),
    write_row!(
        180,
        StepShapeOrientedClosedShell,
        RwStepShapeRwOrientedClosedShell
    ),
    write_row!(181, StepShapeOrientedEdge, RwStepShapeRwOrientedEdge),
    write_row!(182, StepShapeOrientedFace, RwStepShapeRwOrientedFace),
    write_row!(
        183,
        StepShapeOrientedOpenShell,
        RwStepShapeRwOrientedOpenShell
    ),
    write_row!(184, StepShapeOrientedPath, RwStepShapeRwOrientedPath),
    write_row!(
        185,
        StepGeomOuterBoundaryCurve,
        RwStepGeomRwOuterBoundaryCurve
    ),
    write_row!(
        186,
        StepVisualOverRidingStyledItem,
        RwStepVisualRwOverRidingStyledItem
    ),
    write_row!(187, StepGeomParabola, RwStepGeomRwParabola),
    write_row!(
        188,
        StepReprParametricRepresentationContext,
        RwStepReprRwParametricRepresentationContext
    ),
    write_row!(189, StepShapePath, RwStepShapeRwPath),
    write_row!(190, StepGeomPcurve, RwStepGeomRwPcurve),
    write_row!(191, StepBasicPerson, RwStepBasicRwPerson),
    write_row!(
        192,
        StepBasicPersonAndOrganization,
        RwStepBasicRwPersonAndOrganization
    ),
    write_row!(
        194,
        StepBasicPersonAndOrganizationRole,
        RwStepBasicRwPersonAndOrganizationRole
    ),
    write_row!(195, StepBasicPersonalAddress, RwStepBasicRwPersonalAddress),
    write_row!(196, StepGeomPlacement, RwStepGeomRwPlacement),
    write_row!(197, StepVisualPlanarBox, RwStepVisualRwPlanarBox),
    write_row!(198, StepVisualPlanarExtent, RwStepVisualRwPlanarExtent),
    write_row!(199, StepGeomPlane, RwStepGeomRwPlane),
    write_row!(
        200,
        StepBasicPlaneAngleMeasureWithUnit,
        RwStepBasicRwPlaneAngleMeasureWithUnit
    ),
    write_row!(201, StepBasicPlaneAngleUnit, RwStepBasicRwPlaneAngleUnit),
    write_row!(202, StepGeomPoint, RwStepGeomRwPoint),
    write_row!(203, StepGeomPointOnCurve, RwStepGeomRwPointOnCurve),
    write_row!(204, StepGeomPointOnSurface, RwStepGeomRwPointOnSurface),
    write_row!(205, StepGeomPointReplica, RwStepGeomRwPointReplica),
    write_row!(206, StepVisualPointStyle, RwStepVisualRwPointStyle),
    write_row!(207, StepShapePolyLoop, RwStepShapeRwPolyLoop),
    write_row!(208, StepGeomPolyline, RwStepGeomRwPolyline),
    write_row!(
        209,
        StepVisualPreDefinedColour,
        RwStepVisualRwPreDefinedColour
    ),
    write_row!(
        210,
        StepVisualPreDefinedCurveFont,
        RwStepVisualRwPreDefinedCurveFont
    ),
    write_row!(211, StepVisualPreDefinedItem, RwStepVisualRwPreDefinedItem),
    write_row!(213, StepVisualPreDefinedItem, RwStepVisualRwPreDefinedItem),
    write_row!(
        214,
        StepVisualPresentationArea,
        RwStepVisualRwPresentationArea
    ),
    write_row!(
        215,
        StepVisualPresentationLayerAssignment,
        RwStepVisualRwPresentationLayerAssignment
    ),
    write_row!(
        216,
        StepVisualPresentationRepresentation,
        RwStepVisualRwPresentationRepresentation
    ),
    write_row!(
        217,
        StepVisualPresentationSet,
        RwStepVisualRwPresentationSet
    ),
    write_row!(
        218,
        StepVisualPresentationSize,
        RwStepVisualRwPresentationSize
    ),
    write_row!(
        219,
        StepVisualPresentationStyleAssignment,
        RwStepVisualRwPresentationStyleAssignment
    ),
    write_row!(
        220,
        StepVisualPresentationStyleByContext,
        RwStepVisualRwPresentationStyleByContext
    ),
    write_row!(
        221,
        StepVisualPresentationView,
        RwStepVisualRwPresentationView
    ),
    write_row!(223, StepBasicProduct, RwStepBasicRwProduct),
    write_row!(224, StepBasicProductCategory, RwStepBasicRwProductCategory),
    write_row!(225, StepBasicProductContext, RwStepBasicRwProductContext),
    write_row!(
        227,
        StepBasicProductDefinition,
        RwStepBasicRwProductDefinition
    ),
    write_row!(
        228,
        StepBasicProductDefinitionContext,
        RwStepBasicRwProductDefinitionContext
    ),
    write_row!(
        229,
        StepBasicProductDefinitionFormation,
        RwStepBasicRwProductDefinitionFormation
    ),
    write_row!(
        230,
        StepBasicProductDefinitionFormationWithSpecifiedSource,
        RwStepBasicRwProductDefinitionFormationWithSpecifiedSource
    ),
    write_row!(
        231,
        StepReprProductDefinitionShape,
        RwStepReprRwProductDefinitionShape
    ),
    write_row!(
        232,
        StepBasicProductRelatedProductCategory,
        RwStepBasicRwProductRelatedProductCategory
    ),
    write_row!(233, StepBasicProductType, RwStepBasicRwProductType),
    write_row!(
        234,
        StepReprPropertyDefinition,
        RwStepReprRwPropertyDefinition
    ),
    write_row!(
        235,
        StepReprPropertyDefinitionRepresentation,
        RwStepReprRwPropertyDefinitionRepresentation
    ),
    write_row!(
        236,
        StepGeomQuasiUniformCurve,
        RwStepGeomRwQuasiUniformCurve
    ),
    write_row!(
        237,
        StepGeomQuasiUniformSurface,
        RwStepGeomRwQuasiUniformSurface
    ),
    write_row!(
        238,
        StepBasicRatioMeasureWithUnit,
        RwStepBasicRwRatioMeasureWithUnit
    ),
    write_row!(
        239,
        StepGeomRationalBSplineCurve,
        RwStepGeomRwRationalBSplineCurve
    ),
    write_row!(
        240,
        StepGeomRationalBSplineSurface,
        RwStepGeomRwRationalBSplineSurface
    ),
    write_row!(
        241,
        StepGeomRectangularCompositeSurface,
        RwStepGeomRwRectangularCompositeSurface
    ),
    write_row!(
        242,
        StepGeomRectangularTrimmedSurface,
        RwStepGeomRwRectangularTrimmedSurface
    ),
    write_row!(243, StepAP214RepItemGroup, RwStepAP214RwRepItemGroup),
    write_row!(
        244,
        StepGeomReparametrisedCompositeCurveSegment,
        RwStepGeomRwReparametrisedCompositeCurveSegment
    ),
    write_row!(245, StepReprRepresentation, RwStepReprRwRepresentation),
    write_row!(
        246,
        StepReprRepresentationContext,
        RwStepReprRwRepresentationContext
    ),
    write_row!(
        247,
        StepReprRepresentationItem,
        RwStepReprRwRepresentationItem
    ),
    write_row!(
        248,
        StepReprRepresentationMap,
        RwStepReprRwRepresentationMap
    ),
    write_row!(
        249,
        StepReprRepresentationRelationship,
        RwStepReprRwRepresentationRelationship
    ),
    write_row!(
        250,
        StepShapeRevolvedAreaSolid,
        RwStepShapeRwRevolvedAreaSolid
    ),
    write_row!(
        251,
        StepShapeRightAngularWedge,
        RwStepShapeRwRightAngularWedge
    ),
    write_row!(
        252,
        StepShapeRightCircularCone,
        RwStepShapeRwRightCircularCone
    ),
    write_row!(
        253,
        StepShapeRightCircularCylinder,
        RwStepShapeRwRightCircularCylinder
    ),
    write_row!(254, StepGeomSeamCurve, RwStepGeomRwSeamCurve),
    write_row!(
        255,
        StepBasicSecurityClassification,
        RwStepBasicRwSecurityClassification
    ),
    write_row!(
        257,
        StepBasicSecurityClassificationLevel,
        RwStepBasicRwSecurityClassificationLevel
    ),
    write_row!(258, StepReprShapeAspect, RwStepReprRwShapeAspect),
    write_row!(
        259,
        StepReprShapeAspectRelationship,
        RwStepReprRwShapeAspectRelationship
    ),
    write_row!(
        260,
        StepReprShapeAspectTransition,
        RwStepReprRwShapeAspectTransition
    ),
    write_row!(
        261,
        StepShapeShapeDefinitionRepresentation,
        RwStepShapeRwShapeDefinitionRepresentation
    ),
    write_row!(
        262,
        StepShapeShapeRepresentation,
        RwStepShapeRwShapeRepresentation
    ),
    write_row!(
        263,
        StepShapeShellBasedSurfaceModel,
        RwStepShapeRwShellBasedSurfaceModel
    ),
    write_row!(264, StepBasicSiUnit, RwStepBasicRwSiUnit),
    write_row!(
        265,
        StepBasicSolidAngleMeasureWithUnit,
        RwStepBasicRwSolidAngleMeasureWithUnit
    ),
    write_row!(266, StepShapeSolidModel, RwStepShapeRwSolidModel),
    write_row!(267, StepShapeSolidReplica, RwStepShapeRwSolidReplica),
    write_row!(268, StepShapeSphere, RwStepShapeRwSphere),
    write_row!(269, StepGeomSphericalSurface, RwStepGeomRwSphericalSurface),
    write_row!(270, StepVisualStyledItem, RwStepVisualRwStyledItem),
    write_row!(271, StepGeomSurface, RwStepGeomRwSurface),
    write_row!(272, StepGeomSurfaceCurve, RwStepGeomRwSurfaceCurve),
    write_row!(
        273,
        StepGeomSurfaceOfLinearExtrusion,
        RwStepGeomRwSurfaceOfLinearExtrusion
    ),
    write_row!(
        274,
        StepGeomSurfaceOfRevolution,
        RwStepGeomRwSurfaceOfRevolution
    ),
    write_row!(275, StepGeomSurfacePatch, RwStepGeomRwSurfacePatch),
];
