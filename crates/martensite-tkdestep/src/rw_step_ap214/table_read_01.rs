use super::tables::*;
use crate::step_rw::*;

/// A part of the table.
pub(super) const TABLE_READ_01: &[ReadRow] = &[
    read_row!(1, StepBasicAddress, RwStepBasicRwAddress),
    read_row!(
        2,
        StepShapeAdvancedBrepShapeRepresentation,
        RwStepShapeRwAdvancedBrepShapeRepresentation
    ),
    read_row!(3, StepShapeAdvancedFace, RwStepShapeRwAdvancedFace),
    read_row!(
        4,
        StepVisualAnnotationCurveOccurrence,
        RwStepVisualRwAnnotationCurveOccurrence
    ),
    read_row!(
        5,
        StepVisualAnnotationFillArea,
        RwStepVisualRwAnnotationFillArea
    ),
    read_row!(
        6,
        StepVisualAnnotationFillAreaOccurrence,
        RwStepVisualRwAnnotationFillAreaOccurrence
    ),
    read_row!(
        7,
        StepVisualAnnotationOccurrence,
        RwStepVisualRwAnnotationOccurrence
    ),
    read_row!(11, StepReprMappedItem, RwStepReprRwMappedItem),
    read_row!(12, StepVisualStyledItem, RwStepVisualRwStyledItem),
    read_row!(
        13,
        StepBasicApplicationContext,
        RwStepBasicRwApplicationContext
    ),
    read_row!(
        14,
        StepBasicApplicationContextElement,
        RwStepBasicRwApplicationContextElement
    ),
    read_row!(
        15,
        StepBasicApplicationProtocolDefinition,
        RwStepBasicRwApplicationProtocolDefinition
    ),
    read_row!(16, StepBasicApproval, RwStepBasicRwApproval),
    read_row!(
        18,
        StepBasicApprovalPersonOrganization,
        RwStepBasicRwApprovalPersonOrganization
    ),
    read_row!(
        19,
        StepBasicApprovalRelationship,
        RwStepBasicRwApprovalRelationship
    ),
    read_row!(20, StepBasicApprovalRole, RwStepBasicRwApprovalRole),
    read_row!(21, StepBasicApprovalStatus, RwStepBasicRwApprovalStatus),
    read_row!(22, StepVisualAreaInSet, RwStepVisualRwAreaInSet),
    read_row!(
        23,
        StepAP214AutoDesignActualDateAndTimeAssignment,
        RwStepAP214RwAutoDesignActualDateAndTimeAssignment
    ),
    read_row!(
        24,
        StepAP214AutoDesignActualDateAssignment,
        RwStepAP214RwAutoDesignActualDateAssignment
    ),
    read_row!(
        25,
        StepAP214AutoDesignApprovalAssignment,
        RwStepAP214RwAutoDesignApprovalAssignment
    ),
    read_row!(
        26,
        StepAP214AutoDesignDateAndPersonAssignment,
        RwStepAP214RwAutoDesignDateAndPersonAssignment
    ),
    read_row!(
        27,
        StepAP214AutoDesignGroupAssignment,
        RwStepAP214RwAutoDesignGroupAssignment
    ),
    read_row!(
        28,
        StepAP214AutoDesignNominalDateAndTimeAssignment,
        RwStepAP214RwAutoDesignNominalDateAndTimeAssignment
    ),
    read_row!(
        29,
        StepAP214AutoDesignNominalDateAssignment,
        RwStepAP214RwAutoDesignNominalDateAssignment
    ),
    read_row!(
        30,
        StepAP214AutoDesignOrganizationAssignment,
        RwStepAP214RwAutoDesignOrganizationAssignment
    ),
    read_row!(
        31,
        StepAP214AutoDesignPersonAndOrganizationAssignment,
        RwStepAP214RwAutoDesignPersonAndOrganizationAssignment
    ),
    read_row!(
        32,
        StepAP214AutoDesignPresentedItem,
        RwStepAP214RwAutoDesignPresentedItem
    ),
    read_row!(
        33,
        StepAP214AutoDesignSecurityClassificationAssignment,
        RwStepAP214RwAutoDesignSecurityClassificationAssignment
    ),
    read_row!(35, StepGeomAxis1Placement, RwStepGeomRwAxis1Placement),
    read_row!(36, StepGeomAxis2Placement2d, RwStepGeomRwAxis2Placement2d),
    read_row!(37, StepGeomAxis2Placement3d, RwStepGeomRwAxis2Placement3d),
    read_row!(38, StepGeomBSplineCurve, RwStepGeomRwBSplineCurve),
    read_row!(
        39,
        StepGeomBSplineCurveWithKnots,
        RwStepGeomRwBSplineCurveWithKnots
    ),
    read_row!(40, StepGeomBSplineSurface, RwStepGeomRwBSplineSurface),
    read_row!(
        41,
        StepGeomBSplineSurfaceWithKnots,
        RwStepGeomRwBSplineSurfaceWithKnots
    ),
    read_row!(
        42,
        StepVisualBackgroundColour,
        RwStepVisualRwBackgroundColour
    ),
    read_row!(43, StepGeomBezierCurve, RwStepGeomRwBezierCurve),
    read_row!(44, StepGeomBezierSurface, RwStepGeomRwBezierSurface),
    read_row!(45, StepShapeBlock, RwStepShapeRwBlock),
    read_row!(46, StepShapeBooleanResult, RwStepShapeRwBooleanResult),
    read_row!(47, StepGeomBoundaryCurve, RwStepGeomRwBoundaryCurve),
    read_row!(48, StepGeomBoundedCurve, RwStepGeomRwBoundedCurve),
    read_row!(49, StepGeomBoundedSurface, RwStepGeomRwBoundedSurface),
    read_row!(50, StepShapeBoxDomain, RwStepShapeRwBoxDomain),
    read_row!(51, StepShapeBoxedHalfSpace, RwStepShapeRwBoxedHalfSpace),
    read_row!(52, StepShapeBrepWithVoids, RwStepShapeRwBrepWithVoids),
    read_row!(53, StepBasicCalendarDate, RwStepBasicRwCalendarDate),
    read_row!(54, StepVisualCameraImage, RwStepVisualRwCameraImage),
    read_row!(55, StepVisualCameraModel, RwStepVisualRwCameraModel),
    read_row!(56, StepVisualCameraModelD2, RwStepVisualRwCameraModelD2),
    read_row!(57, StepVisualCameraModelD3, RwStepVisualRwCameraModelD3),
    read_row!(58, StepVisualCameraUsage, RwStepVisualRwCameraUsage),
    read_row!(59, StepGeomCartesianPoint, RwStepGeomRwCartesianPoint),
    read_row!(
        60,
        StepGeomCartesianTransformationOperator,
        RwStepGeomRwCartesianTransformationOperator
    ),
    read_row!(
        61,
        StepGeomCartesianTransformationOperator3d,
        RwStepGeomRwCartesianTransformationOperator3d
    ),
    read_row!(62, StepGeomCircle, RwStepGeomRwCircle),
    read_row!(63, StepShapeClosedShell, RwStepShapeRwClosedShell),
    read_row!(64, StepVisualColour, RwStepVisualRwColour),
    read_row!(65, StepVisualColourRgb, RwStepVisualRwColourRgb),
    read_row!(
        66,
        StepVisualColourSpecification,
        RwStepVisualRwColourSpecification
    ),
    read_row!(67, StepGeomCompositeCurve, RwStepGeomRwCompositeCurve),
    read_row!(
        68,
        StepGeomCompositeCurveOnSurface,
        RwStepGeomRwCompositeCurveOnSurface
    ),
    read_row!(
        69,
        StepGeomCompositeCurveSegment,
        RwStepGeomRwCompositeCurveSegment
    ),
    read_row!(70, StepVisualCompositeText, RwStepVisualRwCompositeText),
    read_row!(
        73,
        StepVisualCompositeTextWithExtent,
        RwStepVisualRwCompositeTextWithExtent
    ),
    read_row!(74, StepGeomConic, RwStepGeomRwConic),
    read_row!(75, StepGeomConicalSurface, RwStepGeomRwConicalSurface),
    read_row!(76, StepShapeConnectedFaceSet, RwStepShapeRwConnectedFaceSet),
    read_row!(
        77,
        StepVisualContextDependentInvisibility,
        RwStepVisualRwContextDependentInvisibility
    ),
    read_row!(
        78,
        StepVisualContextDependentOverRidingStyledItem,
        RwStepVisualRwContextDependentOverRidingStyledItem
    ),
    read_row!(
        79,
        StepBasicConversionBasedUnit,
        RwStepBasicRwConversionBasedUnit
    ),
    read_row!(
        80,
        StepBasicCoordinatedUniversalTimeOffset,
        RwStepBasicRwCoordinatedUniversalTimeOffset
    ),
    read_row!(
        82,
        StepShapeCsgShapeRepresentation,
        RwStepShapeRwCsgShapeRepresentation
    ),
    read_row!(83, StepShapeCsgSolid, RwStepShapeRwCsgSolid),
    read_row!(84, StepGeomCurve, RwStepGeomRwCurve),
    read_row!(
        85,
        StepGeomCurveBoundedSurface,
        RwStepGeomRwCurveBoundedSurface
    ),
    read_row!(86, StepGeomCurveReplica, RwStepGeomRwCurveReplica),
    read_row!(87, StepVisualCurveStyle, RwStepVisualRwCurveStyle),
    read_row!(88, StepVisualCurveStyleFont, RwStepVisualRwCurveStyleFont),
    read_row!(
        89,
        StepVisualCurveStyleFontPattern,
        RwStepVisualRwCurveStyleFontPattern
    ),
    read_row!(
        90,
        StepGeomCylindricalSurface,
        RwStepGeomRwCylindricalSurface
    ),
    read_row!(91, StepBasicDate, RwStepBasicRwDate),
    read_row!(92, StepBasicDateAndTime, RwStepBasicRwDateAndTime),
    read_row!(95, StepBasicDateRole, RwStepBasicRwDateRole),
    read_row!(96, StepBasicDateTimeRole, RwStepBasicRwDateTimeRole),
    read_row!(
        98,
        StepReprDefinitionalRepresentation,
        RwStepReprRwDefinitionalRepresentation
    ),
    read_row!(99, StepGeomDegeneratePcurve, RwStepGeomRwDegeneratePcurve),
    read_row!(
        100,
        StepGeomDegenerateToroidalSurface,
        RwStepGeomRwDegenerateToroidalSurface
    ),
    read_row!(
        101,
        StepReprDescriptiveRepresentationItem,
        RwStepReprRwDescriptiveRepresentationItem
    ),
    read_row!(
        104,
        StepBasicDimensionalExponents,
        RwStepBasicRwDimensionalExponents
    ),
    read_row!(105, StepGeomDirection, RwStepGeomRwDirection),
    read_row!(106, StepVisualStyledItem, RwStepVisualRwStyledItem),
    read_row!(
        107,
        StepVisualDraughtingCallout,
        RwStepVisualRwDraughtingCallout
    ),
    read_row!(
        108,
        StepVisualDraughtingPreDefinedColour,
        RwStepVisualRwDraughtingPreDefinedColour
    ),
    read_row!(
        109,
        StepVisualDraughtingPreDefinedCurveFont,
        RwStepVisualRwDraughtingPreDefinedCurveFont
    ),
    read_row!(115, StepShapeEdge, RwStepShapeRwEdge),
    read_row!(116, StepShapeEdgeCurve, RwStepShapeRwEdgeCurve),
    read_row!(117, StepShapeEdgeLoop, RwStepShapeRwEdgeLoop),
    read_row!(
        118,
        StepGeomElementarySurface,
        RwStepGeomRwElementarySurface
    ),
    read_row!(119, StepGeomEllipse, RwStepGeomRwEllipse),
    read_row!(
        120,
        StepGeomEvaluatedDegeneratePcurve,
        RwStepGeomRwEvaluatedDegeneratePcurve
    ),
    read_row!(121, StepBasicExternalSource, RwStepBasicRwExternalSource),
    read_row!(
        122,
        StepVisualExternallyDefinedCurveFont,
        RwStepVisualRwExternallyDefinedCurveFont
    ),
    read_row!(
        124,
        StepBasicExternallyDefinedItem,
        RwStepBasicRwExternallyDefinedItem
    ),
    read_row!(
        126,
        StepBasicExternallyDefinedItem,
        RwStepBasicRwExternallyDefinedItem
    ),
    read_row!(
        128,
        StepShapeExtrudedAreaSolid,
        RwStepShapeRwExtrudedAreaSolid
    ),
    read_row!(129, StepShapeFace, RwStepShapeRwFace),
    read_row!(131, StepShapeFaceBound, RwStepShapeRwFaceBound),
    read_row!(132, StepShapeFaceOuterBound, RwStepShapeRwFaceOuterBound),
    read_row!(133, StepShapeFaceSurface, RwStepShapeRwFaceSurface),
    read_row!(134, StepShapeFacetedBrep, RwStepShapeRwFacetedBrep),
    read_row!(
        135,
        StepShapeFacetedBrepShapeRepresentation,
        RwStepShapeRwFacetedBrepShapeRepresentation
    ),
    read_row!(136, StepVisualFillAreaStyle, RwStepVisualRwFillAreaStyle),
    read_row!(
        137,
        StepVisualFillAreaStyleColour,
        RwStepVisualRwFillAreaStyleColour
    ),
    read_row!(
        141,
        StepReprFunctionallyDefinedTransformation,
        RwStepReprRwFunctionallyDefinedTransformation
    ),
    read_row!(
        142,
        StepShapeGeometricCurveSet,
        RwStepShapeRwGeometricCurveSet
    ),
    read_row!(
        143,
        StepGeomGeometricRepresentationContext,
        RwStepGeomRwGeometricRepresentationContext
    ),
    read_row!(
        144,
        StepGeomGeometricRepresentationItem,
        RwStepGeomRwGeometricRepresentationItem
    ),
    read_row!(145, StepShapeGeometricSet, RwStepShapeRwGeometricSet),
    read_row!(
        146,
        StepShapeGeometricallyBoundedSurfaceShapeRepresentation,
        RwStepShapeRwGeometricallyBoundedSurfaceShapeRepresentation
    ),
    read_row!(
        147,
        StepShapeGeometricallyBoundedWireframeShapeRepresentation,
        RwStepShapeRwGeometricallyBoundedWireframeShapeRepresentation
    ),
    read_row!(
        148,
        StepReprGlobalUncertaintyAssignedContext,
        RwStepReprRwGlobalUncertaintyAssignedContext
    ),
    read_row!(
        149,
        StepReprGlobalUnitAssignedContext,
        RwStepReprRwGlobalUnitAssignedContext
    ),
    read_row!(150, StepBasicGroup, RwStepBasicRwGroup),
    read_row!(
        152,
        StepBasicGroupRelationship,
        RwStepBasicRwGroupRelationship
    ),
    read_row!(153, StepShapeHalfSpaceSolid, RwStepShapeRwHalfSpaceSolid),
    read_row!(154, StepGeomHyperbola, RwStepGeomRwHyperbola),
    read_row!(
        155,
        StepGeomIntersectionCurve,
        RwStepGeomRwIntersectionCurve
    ),
    read_row!(156, StepVisualInvisibility, RwStepVisualRwInvisibility),
    read_row!(
        157,
        StepBasicLengthMeasureWithUnit,
        RwStepBasicRwLengthMeasureWithUnit
    ),
    read_row!(158, StepBasicLengthUnit, RwStepBasicRwLengthUnit),
    read_row!(159, StepGeomLine, RwStepGeomRwLine),
    read_row!(160, StepBasicLocalTime, RwStepBasicRwLocalTime),
    read_row!(161, StepShapeLoop, RwStepShapeRwLoop),
    read_row!(
        162,
        StepShapeManifoldSolidBrep,
        RwStepShapeRwManifoldSolidBrep
    ),
    read_row!(
        163,
        StepShapeManifoldSurfaceShapeRepresentation,
        RwStepShapeRwManifoldSurfaceShapeRepresentation
    ),
    read_row!(164, StepReprMappedItem, RwStepReprRwMappedItem),
    read_row!(165, StepBasicMeasureWithUnit, RwStepBasicRwMeasureWithUnit),
    read_row!(
        166,
        StepVisualMechanicalDesignGeometricPresentationArea,
        RwStepVisualRwMechanicalDesignGeometricPresentationArea
    ),
    read_row!(
        167,
        StepVisualMechanicalDesignGeometricPresentationRepresentation,
        RwStepVisualRwMechanicalDesignGeometricPresentationRepresentation
    ),
    read_row!(169, StepBasicNamedUnit, RwStepBasicRwNamedUnit),
    read_row!(171, StepGeomOffsetCurve3d, RwStepGeomRwOffsetCurve3d),
    read_row!(172, StepGeomOffsetSurface, RwStepGeomRwOffsetSurface),
    read_row!(174, StepShapeOpenShell, RwStepShapeRwOpenShell),
    read_row!(175, StepBasicOrdinalDate, RwStepBasicRwOrdinalDate),
    read_row!(176, StepBasicOrganization, RwStepBasicRwOrganization),
    read_row!(
        178,
        StepBasicOrganizationRole,
        RwStepBasicRwOrganizationRole
    ),
    read_row!(
        179,
        StepBasicOrganizationalAddress,
        RwStepBasicRwOrganizationalAddress
    ),
    read_row!(
        180,
        StepShapeOrientedClosedShell,
        RwStepShapeRwOrientedClosedShell
    ),
    read_row!(181, StepShapeOrientedEdge, RwStepShapeRwOrientedEdge),
    read_row!(182, StepShapeOrientedFace, RwStepShapeRwOrientedFace),
    read_row!(
        183,
        StepShapeOrientedOpenShell,
        RwStepShapeRwOrientedOpenShell
    ),
    read_row!(184, StepShapeOrientedPath, RwStepShapeRwOrientedPath),
    read_row!(
        185,
        StepGeomOuterBoundaryCurve,
        RwStepGeomRwOuterBoundaryCurve
    ),
    read_row!(
        186,
        StepVisualOverRidingStyledItem,
        RwStepVisualRwOverRidingStyledItem
    ),
    read_row!(187, StepGeomParabola, RwStepGeomRwParabola),
    read_row!(
        188,
        StepReprParametricRepresentationContext,
        RwStepReprRwParametricRepresentationContext
    ),
    read_row!(189, StepShapePath, RwStepShapeRwPath),
    read_row!(190, StepGeomPcurve, RwStepGeomRwPcurve),
    read_row!(191, StepBasicPerson, RwStepBasicRwPerson),
    read_row!(
        192,
        StepBasicPersonAndOrganization,
        RwStepBasicRwPersonAndOrganization
    ),
    read_row!(
        194,
        StepBasicPersonAndOrganizationRole,
        RwStepBasicRwPersonAndOrganizationRole
    ),
    read_row!(195, StepBasicPersonalAddress, RwStepBasicRwPersonalAddress),
    read_row!(196, StepGeomPlacement, RwStepGeomRwPlacement),
    read_row!(197, StepVisualPlanarBox, RwStepVisualRwPlanarBox),
    read_row!(198, StepVisualPlanarExtent, RwStepVisualRwPlanarExtent),
    read_row!(199, StepGeomPlane, RwStepGeomRwPlane),
    read_row!(
        200,
        StepBasicPlaneAngleMeasureWithUnit,
        RwStepBasicRwPlaneAngleMeasureWithUnit
    ),
    read_row!(201, StepBasicPlaneAngleUnit, RwStepBasicRwPlaneAngleUnit),
    read_row!(202, StepGeomPoint, RwStepGeomRwPoint),
    read_row!(203, StepGeomPointOnCurve, RwStepGeomRwPointOnCurve),
    read_row!(204, StepGeomPointOnSurface, RwStepGeomRwPointOnSurface),
    read_row!(205, StepGeomPointReplica, RwStepGeomRwPointReplica),
    read_row!(206, StepVisualPointStyle, RwStepVisualRwPointStyle),
    read_row!(207, StepShapePolyLoop, RwStepShapeRwPolyLoop),
    read_row!(208, StepGeomPolyline, RwStepGeomRwPolyline),
    read_row!(
        209,
        StepVisualPreDefinedColour,
        RwStepVisualRwPreDefinedColour
    ),
    read_row!(
        210,
        StepVisualPreDefinedCurveFont,
        RwStepVisualRwPreDefinedCurveFont
    ),
    read_row!(211, StepVisualPreDefinedItem, RwStepVisualRwPreDefinedItem),
    read_row!(213, StepVisualPreDefinedItem, RwStepVisualRwPreDefinedItem),
    read_row!(
        214,
        StepVisualPresentationArea,
        RwStepVisualRwPresentationArea
    ),
    read_row!(
        215,
        StepVisualPresentationLayerAssignment,
        RwStepVisualRwPresentationLayerAssignment
    ),
    read_row!(
        216,
        StepVisualPresentationRepresentation,
        RwStepVisualRwPresentationRepresentation
    ),
    read_row!(
        217,
        StepVisualPresentationSet,
        RwStepVisualRwPresentationSet
    ),
    read_row!(
        218,
        StepVisualPresentationSize,
        RwStepVisualRwPresentationSize
    ),
    read_row!(
        219,
        StepVisualPresentationStyleAssignment,
        RwStepVisualRwPresentationStyleAssignment
    ),
    read_row!(
        220,
        StepVisualPresentationStyleByContext,
        RwStepVisualRwPresentationStyleByContext
    ),
    read_row!(
        221,
        StepVisualPresentationView,
        RwStepVisualRwPresentationView
    ),
    read_row!(223, StepBasicProduct, RwStepBasicRwProduct),
    read_row!(224, StepBasicProductCategory, RwStepBasicRwProductCategory),
    read_row!(225, StepBasicProductContext, RwStepBasicRwProductContext),
    read_row!(
        227,
        StepBasicProductDefinition,
        RwStepBasicRwProductDefinition
    ),
    read_row!(
        228,
        StepBasicProductDefinitionContext,
        RwStepBasicRwProductDefinitionContext
    ),
    read_row!(
        229,
        StepBasicProductDefinitionFormation,
        RwStepBasicRwProductDefinitionFormation
    ),
    read_row!(
        230,
        StepBasicProductDefinitionFormationWithSpecifiedSource,
        RwStepBasicRwProductDefinitionFormationWithSpecifiedSource
    ),
    read_row!(
        231,
        StepReprProductDefinitionShape,
        RwStepReprRwProductDefinitionShape
    ),
    read_row!(
        232,
        StepBasicProductRelatedProductCategory,
        RwStepBasicRwProductRelatedProductCategory
    ),
    read_row!(233, StepBasicProductType, RwStepBasicRwProductType),
    read_row!(
        234,
        StepReprPropertyDefinition,
        RwStepReprRwPropertyDefinition
    ),
    read_row!(
        235,
        StepReprPropertyDefinitionRepresentation,
        RwStepReprRwPropertyDefinitionRepresentation
    ),
    read_row!(
        236,
        StepGeomQuasiUniformCurve,
        RwStepGeomRwQuasiUniformCurve
    ),
    read_row!(
        237,
        StepGeomQuasiUniformSurface,
        RwStepGeomRwQuasiUniformSurface
    ),
    read_row!(
        238,
        StepBasicRatioMeasureWithUnit,
        RwStepBasicRwRatioMeasureWithUnit
    ),
    read_row!(
        239,
        StepGeomRationalBSplineCurve,
        RwStepGeomRwRationalBSplineCurve
    ),
    read_row!(
        240,
        StepGeomRationalBSplineSurface,
        RwStepGeomRwRationalBSplineSurface
    ),
    read_row!(
        241,
        StepGeomRectangularCompositeSurface,
        RwStepGeomRwRectangularCompositeSurface
    ),
    read_row!(
        242,
        StepGeomRectangularTrimmedSurface,
        RwStepGeomRwRectangularTrimmedSurface
    ),
    read_row!(243, StepAP214RepItemGroup, RwStepAP214RwRepItemGroup),
    read_row!(
        244,
        StepGeomReparametrisedCompositeCurveSegment,
        RwStepGeomRwReparametrisedCompositeCurveSegment
    ),
    read_row!(245, StepReprRepresentation, RwStepReprRwRepresentation),
    read_row!(
        246,
        StepReprRepresentationContext,
        RwStepReprRwRepresentationContext
    ),
    read_row!(
        247,
        StepReprRepresentationItem,
        RwStepReprRwRepresentationItem
    ),
    read_row!(
        248,
        StepReprRepresentationMap,
        RwStepReprRwRepresentationMap
    ),
    read_row!(
        249,
        StepReprRepresentationRelationship,
        RwStepReprRwRepresentationRelationship
    ),
    read_row!(
        250,
        StepShapeRevolvedAreaSolid,
        RwStepShapeRwRevolvedAreaSolid
    ),
    read_row!(
        251,
        StepShapeRightAngularWedge,
        RwStepShapeRwRightAngularWedge
    ),
    read_row!(
        252,
        StepShapeRightCircularCone,
        RwStepShapeRwRightCircularCone
    ),
    read_row!(
        253,
        StepShapeRightCircularCylinder,
        RwStepShapeRwRightCircularCylinder
    ),
    read_row!(254, StepGeomSeamCurve, RwStepGeomRwSeamCurve),
    read_row!(
        255,
        StepBasicSecurityClassification,
        RwStepBasicRwSecurityClassification
    ),
    read_row!(
        257,
        StepBasicSecurityClassificationLevel,
        RwStepBasicRwSecurityClassificationLevel
    ),
    read_row!(258, StepReprShapeAspect, RwStepReprRwShapeAspect),
    read_row!(
        259,
        StepReprShapeAspectRelationship,
        RwStepReprRwShapeAspectRelationship
    ),
    read_row!(
        260,
        StepReprShapeAspectTransition,
        RwStepReprRwShapeAspectTransition
    ),
    read_row!(
        261,
        StepShapeShapeDefinitionRepresentation,
        RwStepShapeRwShapeDefinitionRepresentation
    ),
    read_row!(
        262,
        StepShapeShapeRepresentation,
        RwStepShapeRwShapeRepresentation
    ),
    read_row!(
        263,
        StepShapeShellBasedSurfaceModel,
        RwStepShapeRwShellBasedSurfaceModel
    ),
    read_row!(264, StepBasicSiUnit, RwStepBasicRwSiUnit),
    read_row!(
        265,
        StepBasicSolidAngleMeasureWithUnit,
        RwStepBasicRwSolidAngleMeasureWithUnit
    ),
    read_row!(266, StepShapeSolidModel, RwStepShapeRwSolidModel),
    read_row!(267, StepShapeSolidReplica, RwStepShapeRwSolidReplica),
    read_row!(268, StepShapeSphere, RwStepShapeRwSphere),
    read_row!(269, StepGeomSphericalSurface, RwStepGeomRwSphericalSurface),
    read_row!(270, StepVisualStyledItem, RwStepVisualRwStyledItem),
    read_row!(271, StepGeomSurface, RwStepGeomRwSurface),
    read_row!(272, StepGeomSurfaceCurve, RwStepGeomRwSurfaceCurve),
    read_row!(
        273,
        StepGeomSurfaceOfLinearExtrusion,
        RwStepGeomRwSurfaceOfLinearExtrusion
    ),
    read_row!(
        274,
        StepGeomSurfaceOfRevolution,
        RwStepGeomRwSurfaceOfRevolution
    ),
    read_row!(275, StepGeomSurfacePatch, RwStepGeomRwSurfacePatch),
];
