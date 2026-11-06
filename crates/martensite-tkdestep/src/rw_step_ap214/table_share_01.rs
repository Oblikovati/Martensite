use super::tables::*;
use crate::step_rw::*;

/// A part of the table.
pub(super) const TABLE_SHARE_01: &[ShareRow] = &[
    share_row!(
        2,
        StepShapeAdvancedBrepShapeRepresentation,
        RwStepShapeRwAdvancedBrepShapeRepresentation
    ),
    share_row!(3, StepShapeAdvancedFace, RwStepShapeRwAdvancedFace),
    share_row!(
        4,
        StepVisualAnnotationCurveOccurrence,
        RwStepVisualRwAnnotationCurveOccurrence
    ),
    share_row!(
        5,
        StepVisualAnnotationFillArea,
        RwStepVisualRwAnnotationFillArea
    ),
    share_row!(
        6,
        StepVisualAnnotationFillAreaOccurrence,
        RwStepVisualRwAnnotationFillAreaOccurrence
    ),
    share_row!(
        7,
        StepVisualAnnotationOccurrence,
        RwStepVisualRwAnnotationOccurrence
    ),
    share_row!(11, StepReprMappedItem, RwStepReprRwMappedItem),
    share_row!(12, StepVisualStyledItem, RwStepVisualRwStyledItem),
    share_row!(
        14,
        StepBasicApplicationContextElement,
        RwStepBasicRwApplicationContextElement
    ),
    share_row!(
        15,
        StepBasicApplicationProtocolDefinition,
        RwStepBasicRwApplicationProtocolDefinition
    ),
    share_row!(16, StepBasicApproval, RwStepBasicRwApproval),
    share_row!(
        18,
        StepBasicApprovalPersonOrganization,
        RwStepBasicRwApprovalPersonOrganization
    ),
    share_row!(
        19,
        StepBasicApprovalRelationship,
        RwStepBasicRwApprovalRelationship
    ),
    share_row!(22, StepVisualAreaInSet, RwStepVisualRwAreaInSet),
    share_row!(
        23,
        StepAP214AutoDesignActualDateAndTimeAssignment,
        RwStepAP214RwAutoDesignActualDateAndTimeAssignment
    ),
    share_row!(
        24,
        StepAP214AutoDesignActualDateAssignment,
        RwStepAP214RwAutoDesignActualDateAssignment
    ),
    share_row!(
        25,
        StepAP214AutoDesignApprovalAssignment,
        RwStepAP214RwAutoDesignApprovalAssignment
    ),
    share_row!(
        26,
        StepAP214AutoDesignDateAndPersonAssignment,
        RwStepAP214RwAutoDesignDateAndPersonAssignment
    ),
    share_row!(
        27,
        StepAP214AutoDesignGroupAssignment,
        RwStepAP214RwAutoDesignGroupAssignment
    ),
    share_row!(
        28,
        StepAP214AutoDesignNominalDateAndTimeAssignment,
        RwStepAP214RwAutoDesignNominalDateAndTimeAssignment
    ),
    share_row!(
        29,
        StepAP214AutoDesignNominalDateAssignment,
        RwStepAP214RwAutoDesignNominalDateAssignment
    ),
    share_row!(
        30,
        StepAP214AutoDesignOrganizationAssignment,
        RwStepAP214RwAutoDesignOrganizationAssignment
    ),
    share_row!(
        31,
        StepAP214AutoDesignPersonAndOrganizationAssignment,
        RwStepAP214RwAutoDesignPersonAndOrganizationAssignment
    ),
    share_row!(
        32,
        StepAP214AutoDesignPresentedItem,
        RwStepAP214RwAutoDesignPresentedItem
    ),
    share_row!(
        33,
        StepAP214AutoDesignSecurityClassificationAssignment,
        RwStepAP214RwAutoDesignSecurityClassificationAssignment
    ),
    share_row!(35, StepGeomAxis1Placement, RwStepGeomRwAxis1Placement),
    share_row!(36, StepGeomAxis2Placement2d, RwStepGeomRwAxis2Placement2d),
    share_row!(37, StepGeomAxis2Placement3d, RwStepGeomRwAxis2Placement3d),
    share_row!(38, StepGeomBSplineCurve, RwStepGeomRwBSplineCurve),
    share_row!(
        39,
        StepGeomBSplineCurveWithKnots,
        RwStepGeomRwBSplineCurveWithKnots
    ),
    share_row!(40, StepGeomBSplineSurface, RwStepGeomRwBSplineSurface),
    share_row!(
        41,
        StepGeomBSplineSurfaceWithKnots,
        RwStepGeomRwBSplineSurfaceWithKnots
    ),
    share_row!(
        42,
        StepVisualBackgroundColour,
        RwStepVisualRwBackgroundColour
    ),
    share_row!(43, StepGeomBezierCurve, RwStepGeomRwBezierCurve),
    share_row!(44, StepGeomBezierSurface, RwStepGeomRwBezierSurface),
    share_row!(45, StepShapeBlock, RwStepShapeRwBlock),
    share_row!(46, StepShapeBooleanResult, RwStepShapeRwBooleanResult),
    share_row!(47, StepGeomBoundaryCurve, RwStepGeomRwBoundaryCurve),
    share_row!(50, StepShapeBoxDomain, RwStepShapeRwBoxDomain),
    share_row!(51, StepShapeBoxedHalfSpace, RwStepShapeRwBoxedHalfSpace),
    share_row!(52, StepShapeBrepWithVoids, RwStepShapeRwBrepWithVoids),
    share_row!(54, StepVisualCameraImage, RwStepVisualRwCameraImage),
    share_row!(56, StepVisualCameraModelD2, RwStepVisualRwCameraModelD2),
    share_row!(57, StepVisualCameraModelD3, RwStepVisualRwCameraModelD3),
    share_row!(58, StepVisualCameraUsage, RwStepVisualRwCameraUsage),
    share_row!(
        60,
        StepGeomCartesianTransformationOperator,
        RwStepGeomRwCartesianTransformationOperator
    ),
    share_row!(
        61,
        StepGeomCartesianTransformationOperator3d,
        RwStepGeomRwCartesianTransformationOperator3d
    ),
    share_row!(62, StepGeomCircle, RwStepGeomRwCircle),
    share_row!(63, StepShapeClosedShell, RwStepShapeRwClosedShell),
    share_row!(67, StepGeomCompositeCurve, RwStepGeomRwCompositeCurve),
    share_row!(
        68,
        StepGeomCompositeCurveOnSurface,
        RwStepGeomRwCompositeCurveOnSurface
    ),
    share_row!(
        69,
        StepGeomCompositeCurveSegment,
        RwStepGeomRwCompositeCurveSegment
    ),
    share_row!(70, StepVisualCompositeText, RwStepVisualRwCompositeText),
    share_row!(
        73,
        StepVisualCompositeTextWithExtent,
        RwStepVisualRwCompositeTextWithExtent
    ),
    share_row!(74, StepGeomConic, RwStepGeomRwConic),
    share_row!(75, StepGeomConicalSurface, RwStepGeomRwConicalSurface),
    share_row!(76, StepShapeConnectedFaceSet, RwStepShapeRwConnectedFaceSet),
    share_row!(
        77,
        StepVisualContextDependentInvisibility,
        RwStepVisualRwContextDependentInvisibility
    ),
    share_row!(
        78,
        StepVisualContextDependentOverRidingStyledItem,
        RwStepVisualRwContextDependentOverRidingStyledItem
    ),
    share_row!(
        79,
        StepBasicConversionBasedUnit,
        RwStepBasicRwConversionBasedUnit
    ),
    share_row!(
        82,
        StepShapeCsgShapeRepresentation,
        RwStepShapeRwCsgShapeRepresentation
    ),
    share_row!(83, StepShapeCsgSolid, RwStepShapeRwCsgSolid),
    share_row!(
        85,
        StepGeomCurveBoundedSurface,
        RwStepGeomRwCurveBoundedSurface
    ),
    share_row!(86, StepGeomCurveReplica, RwStepGeomRwCurveReplica),
    share_row!(87, StepVisualCurveStyle, RwStepVisualRwCurveStyle),
    share_row!(88, StepVisualCurveStyleFont, RwStepVisualRwCurveStyleFont),
    share_row!(
        90,
        StepGeomCylindricalSurface,
        RwStepGeomRwCylindricalSurface
    ),
    share_row!(92, StepBasicDateAndTime, RwStepBasicRwDateAndTime),
    share_row!(
        98,
        StepReprDefinitionalRepresentation,
        RwStepReprRwDefinitionalRepresentation
    ),
    share_row!(99, StepGeomDegeneratePcurve, RwStepGeomRwDegeneratePcurve),
    share_row!(
        100,
        StepGeomDegenerateToroidalSurface,
        RwStepGeomRwDegenerateToroidalSurface
    ),
    share_row!(106, StepVisualStyledItem, RwStepVisualRwStyledItem),
    share_row!(
        107,
        StepVisualDraughtingCallout,
        RwStepVisualRwDraughtingCallout
    ),
    share_row!(116, StepShapeEdgeCurve, RwStepShapeRwEdgeCurve),
    share_row!(117, StepShapeEdgeLoop, RwStepShapeRwEdgeLoop),
    share_row!(
        118,
        StepGeomElementarySurface,
        RwStepGeomRwElementarySurface
    ),
    share_row!(119, StepGeomEllipse, RwStepGeomRwEllipse),
    share_row!(
        120,
        StepGeomEvaluatedDegeneratePcurve,
        RwStepGeomRwEvaluatedDegeneratePcurve
    ),
    share_row!(
        122,
        StepVisualExternallyDefinedCurveFont,
        RwStepVisualRwExternallyDefinedCurveFont
    ),
    share_row!(
        124,
        StepBasicExternallyDefinedItem,
        RwStepBasicRwExternallyDefinedItem
    ),
    share_row!(
        126,
        StepBasicExternallyDefinedItem,
        RwStepBasicRwExternallyDefinedItem
    ),
    share_row!(
        128,
        StepShapeExtrudedAreaSolid,
        RwStepShapeRwExtrudedAreaSolid
    ),
    share_row!(129, StepShapeFace, RwStepShapeRwFace),
    share_row!(131, StepShapeFaceBound, RwStepShapeRwFaceBound),
    share_row!(132, StepShapeFaceOuterBound, RwStepShapeRwFaceOuterBound),
    share_row!(133, StepShapeFaceSurface, RwStepShapeRwFaceSurface),
    share_row!(134, StepShapeFacetedBrep, RwStepShapeRwFacetedBrep),
    share_row!(
        135,
        StepShapeFacetedBrepShapeRepresentation,
        RwStepShapeRwFacetedBrepShapeRepresentation
    ),
    share_row!(136, StepVisualFillAreaStyle, RwStepVisualRwFillAreaStyle),
    share_row!(
        137,
        StepVisualFillAreaStyleColour,
        RwStepVisualRwFillAreaStyleColour
    ),
    share_row!(
        142,
        StepShapeGeometricCurveSet,
        RwStepShapeRwGeometricCurveSet
    ),
    share_row!(145, StepShapeGeometricSet, RwStepShapeRwGeometricSet),
    share_row!(
        146,
        StepShapeGeometricallyBoundedSurfaceShapeRepresentation,
        RwStepShapeRwGeometricallyBoundedSurfaceShapeRepresentation
    ),
    share_row!(
        147,
        StepShapeGeometricallyBoundedWireframeShapeRepresentation,
        RwStepShapeRwGeometricallyBoundedWireframeShapeRepresentation
    ),
    share_row!(
        148,
        StepReprGlobalUncertaintyAssignedContext,
        RwStepReprRwGlobalUncertaintyAssignedContext
    ),
    share_row!(
        149,
        StepReprGlobalUnitAssignedContext,
        RwStepReprRwGlobalUnitAssignedContext
    ),
    share_row!(
        152,
        StepBasicGroupRelationship,
        RwStepBasicRwGroupRelationship
    ),
    share_row!(153, StepShapeHalfSpaceSolid, RwStepShapeRwHalfSpaceSolid),
    share_row!(154, StepGeomHyperbola, RwStepGeomRwHyperbola),
    share_row!(
        155,
        StepGeomIntersectionCurve,
        RwStepGeomRwIntersectionCurve
    ),
    share_row!(156, StepVisualInvisibility, RwStepVisualRwInvisibility),
    share_row!(
        157,
        StepBasicLengthMeasureWithUnit,
        RwStepBasicRwLengthMeasureWithUnit
    ),
    share_row!(158, StepBasicLengthUnit, RwStepBasicRwLengthUnit),
    share_row!(159, StepGeomLine, RwStepGeomRwLine),
    share_row!(160, StepBasicLocalTime, RwStepBasicRwLocalTime),
    share_row!(
        162,
        StepShapeManifoldSolidBrep,
        RwStepShapeRwManifoldSolidBrep
    ),
    share_row!(
        163,
        StepShapeManifoldSurfaceShapeRepresentation,
        RwStepShapeRwManifoldSurfaceShapeRepresentation
    ),
    share_row!(164, StepReprMappedItem, RwStepReprRwMappedItem),
    share_row!(165, StepBasicMeasureWithUnit, RwStepBasicRwMeasureWithUnit),
    share_row!(
        166,
        StepVisualMechanicalDesignGeometricPresentationArea,
        RwStepVisualRwMechanicalDesignGeometricPresentationArea
    ),
    share_row!(
        167,
        StepVisualMechanicalDesignGeometricPresentationRepresentation,
        RwStepVisualRwMechanicalDesignGeometricPresentationRepresentation
    ),
    share_row!(169, StepBasicNamedUnit, RwStepBasicRwNamedUnit),
    share_row!(171, StepGeomOffsetCurve3d, RwStepGeomRwOffsetCurve3d),
    share_row!(172, StepGeomOffsetSurface, RwStepGeomRwOffsetSurface),
    share_row!(174, StepShapeOpenShell, RwStepShapeRwOpenShell),
    share_row!(
        179,
        StepBasicOrganizationalAddress,
        RwStepBasicRwOrganizationalAddress
    ),
    share_row!(
        180,
        StepShapeOrientedClosedShell,
        RwStepShapeRwOrientedClosedShell
    ),
    share_row!(181, StepShapeOrientedEdge, RwStepShapeRwOrientedEdge),
    share_row!(182, StepShapeOrientedFace, RwStepShapeRwOrientedFace),
    share_row!(
        183,
        StepShapeOrientedOpenShell,
        RwStepShapeRwOrientedOpenShell
    ),
    share_row!(184, StepShapeOrientedPath, RwStepShapeRwOrientedPath),
    share_row!(
        185,
        StepGeomOuterBoundaryCurve,
        RwStepGeomRwOuterBoundaryCurve
    ),
    share_row!(
        186,
        StepVisualOverRidingStyledItem,
        RwStepVisualRwOverRidingStyledItem
    ),
    share_row!(187, StepGeomParabola, RwStepGeomRwParabola),
    share_row!(189, StepShapePath, RwStepShapeRwPath),
    share_row!(190, StepGeomPcurve, RwStepGeomRwPcurve),
    share_row!(
        192,
        StepBasicPersonAndOrganization,
        RwStepBasicRwPersonAndOrganization
    ),
    share_row!(195, StepBasicPersonalAddress, RwStepBasicRwPersonalAddress),
    share_row!(196, StepGeomPlacement, RwStepGeomRwPlacement),
    share_row!(197, StepVisualPlanarBox, RwStepVisualRwPlanarBox),
    share_row!(199, StepGeomPlane, RwStepGeomRwPlane),
    share_row!(
        200,
        StepBasicPlaneAngleMeasureWithUnit,
        RwStepBasicRwPlaneAngleMeasureWithUnit
    ),
    share_row!(201, StepBasicPlaneAngleUnit, RwStepBasicRwPlaneAngleUnit),
    share_row!(203, StepGeomPointOnCurve, RwStepGeomRwPointOnCurve),
    share_row!(204, StepGeomPointOnSurface, RwStepGeomRwPointOnSurface),
    share_row!(205, StepGeomPointReplica, RwStepGeomRwPointReplica),
    share_row!(206, StepVisualPointStyle, RwStepVisualRwPointStyle),
    share_row!(207, StepShapePolyLoop, RwStepShapeRwPolyLoop),
    share_row!(208, StepGeomPolyline, RwStepGeomRwPolyline),
    share_row!(
        214,
        StepVisualPresentationArea,
        RwStepVisualRwPresentationArea
    ),
    share_row!(
        215,
        StepVisualPresentationLayerAssignment,
        RwStepVisualRwPresentationLayerAssignment
    ),
    share_row!(
        216,
        StepVisualPresentationRepresentation,
        RwStepVisualRwPresentationRepresentation
    ),
    share_row!(
        218,
        StepVisualPresentationSize,
        RwStepVisualRwPresentationSize
    ),
    share_row!(
        219,
        StepVisualPresentationStyleAssignment,
        RwStepVisualRwPresentationStyleAssignment
    ),
    share_row!(
        220,
        StepVisualPresentationStyleByContext,
        RwStepVisualRwPresentationStyleByContext
    ),
    share_row!(
        221,
        StepVisualPresentationView,
        RwStepVisualRwPresentationView
    ),
    share_row!(223, StepBasicProduct, RwStepBasicRwProduct),
    share_row!(225, StepBasicProductContext, RwStepBasicRwProductContext),
    share_row!(
        227,
        StepBasicProductDefinition,
        RwStepBasicRwProductDefinition
    ),
    share_row!(
        228,
        StepBasicProductDefinitionContext,
        RwStepBasicRwProductDefinitionContext
    ),
    share_row!(
        229,
        StepBasicProductDefinitionFormation,
        RwStepBasicRwProductDefinitionFormation
    ),
    share_row!(
        230,
        StepBasicProductDefinitionFormationWithSpecifiedSource,
        RwStepBasicRwProductDefinitionFormationWithSpecifiedSource
    ),
    share_row!(
        231,
        StepReprProductDefinitionShape,
        RwStepReprRwProductDefinitionShape
    ),
    share_row!(
        232,
        StepBasicProductRelatedProductCategory,
        RwStepBasicRwProductRelatedProductCategory
    ),
    share_row!(233, StepBasicProductType, RwStepBasicRwProductType),
    share_row!(
        234,
        StepReprPropertyDefinition,
        RwStepReprRwPropertyDefinition
    ),
    share_row!(
        235,
        StepReprPropertyDefinitionRepresentation,
        RwStepReprRwPropertyDefinitionRepresentation
    ),
    share_row!(
        236,
        StepGeomQuasiUniformCurve,
        RwStepGeomRwQuasiUniformCurve
    ),
    share_row!(
        237,
        StepGeomQuasiUniformSurface,
        RwStepGeomRwQuasiUniformSurface
    ),
    share_row!(
        238,
        StepBasicRatioMeasureWithUnit,
        RwStepBasicRwRatioMeasureWithUnit
    ),
    share_row!(
        239,
        StepGeomRationalBSplineCurve,
        RwStepGeomRwRationalBSplineCurve
    ),
    share_row!(
        240,
        StepGeomRationalBSplineSurface,
        RwStepGeomRwRationalBSplineSurface
    ),
    share_row!(
        241,
        StepGeomRectangularCompositeSurface,
        RwStepGeomRwRectangularCompositeSurface
    ),
    share_row!(
        242,
        StepGeomRectangularTrimmedSurface,
        RwStepGeomRwRectangularTrimmedSurface
    ),
    share_row!(
        244,
        StepGeomReparametrisedCompositeCurveSegment,
        RwStepGeomRwReparametrisedCompositeCurveSegment
    ),
    share_row!(245, StepReprRepresentation, RwStepReprRwRepresentation),
    share_row!(
        248,
        StepReprRepresentationMap,
        RwStepReprRwRepresentationMap
    ),
    share_row!(
        249,
        StepReprRepresentationRelationship,
        RwStepReprRwRepresentationRelationship
    ),
    share_row!(
        250,
        StepShapeRevolvedAreaSolid,
        RwStepShapeRwRevolvedAreaSolid
    ),
    share_row!(
        251,
        StepShapeRightAngularWedge,
        RwStepShapeRwRightAngularWedge
    ),
    share_row!(
        252,
        StepShapeRightCircularCone,
        RwStepShapeRwRightCircularCone
    ),
    share_row!(
        253,
        StepShapeRightCircularCylinder,
        RwStepShapeRwRightCircularCylinder
    ),
    share_row!(254, StepGeomSeamCurve, RwStepGeomRwSeamCurve),
    share_row!(
        255,
        StepBasicSecurityClassification,
        RwStepBasicRwSecurityClassification
    ),
    share_row!(258, StepReprShapeAspect, RwStepReprRwShapeAspect),
    share_row!(
        259,
        StepReprShapeAspectRelationship,
        RwStepReprRwShapeAspectRelationship
    ),
    share_row!(
        260,
        StepReprShapeAspectTransition,
        RwStepReprRwShapeAspectTransition
    ),
    share_row!(
        261,
        StepShapeShapeDefinitionRepresentation,
        RwStepShapeRwShapeDefinitionRepresentation
    ),
    share_row!(
        262,
        StepShapeShapeRepresentation,
        RwStepShapeRwShapeRepresentation
    ),
    share_row!(
        263,
        StepShapeShellBasedSurfaceModel,
        RwStepShapeRwShellBasedSurfaceModel
    ),
    share_row!(
        265,
        StepBasicSolidAngleMeasureWithUnit,
        RwStepBasicRwSolidAngleMeasureWithUnit
    ),
    share_row!(267, StepShapeSolidReplica, RwStepShapeRwSolidReplica),
    share_row!(268, StepShapeSphere, RwStepShapeRwSphere),
    share_row!(269, StepGeomSphericalSurface, RwStepGeomRwSphericalSurface),
    share_row!(270, StepVisualStyledItem, RwStepVisualRwStyledItem),
    share_row!(272, StepGeomSurfaceCurve, RwStepGeomRwSurfaceCurve),
    share_row!(
        273,
        StepGeomSurfaceOfLinearExtrusion,
        RwStepGeomRwSurfaceOfLinearExtrusion
    ),
    share_row!(
        274,
        StepGeomSurfaceOfRevolution,
        RwStepGeomRwSurfaceOfRevolution
    ),
    share_row!(275, StepGeomSurfacePatch, RwStepGeomRwSurfacePatch),
    share_row!(276, StepGeomSurfaceReplica, RwStepGeomRwSurfaceReplica),
    share_row!(
        277,
        StepVisualSurfaceSideStyle,
        RwStepVisualRwSurfaceSideStyle
    ),
    share_row!(
        278,
        StepVisualSurfaceStyleBoundary,
        RwStepVisualRwSurfaceStyleBoundary
    ),
    share_row!(
        279,
        StepVisualSurfaceStyleControlGrid,
        RwStepVisualRwSurfaceStyleControlGrid
    ),
    share_row!(
        280,
        StepVisualSurfaceStyleFillArea,
        RwStepVisualRwSurfaceStyleFillArea
    ),
    share_row!(
        281,
        StepVisualSurfaceStyleParameterLine,
        RwStepVisualRwSurfaceStyleParameterLine
    ),
    share_row!(
        282,
        StepVisualSurfaceStyleSegmentationCurve,
        RwStepVisualRwSurfaceStyleSegmentationCurve
    ),
    share_row!(
        283,
        StepVisualSurfaceStyleSilhouette,
        RwStepVisualRwSurfaceStyleSilhouette
    ),
    share_row!(
        284,
        StepVisualSurfaceStyleUsage,
        RwStepVisualRwSurfaceStyleUsage
    ),
    share_row!(285, StepShapeSweptAreaSolid, RwStepShapeRwSweptAreaSolid),
    share_row!(286, StepGeomSweptSurface, RwStepGeomRwSweptSurface),
    share_row!(292, StepVisualTemplate, RwStepVisualRwTemplate),
    share_row!(
        293,
        StepVisualTemplateInstance,
        RwStepVisualRwTemplateInstance
    ),
    share_row!(295, StepVisualTextLiteral, RwStepVisualRwTextLiteral),
    share_row!(300, StepVisualTextStyle, RwStepVisualRwTextStyle),
    share_row!(
        301,
        StepVisualTextStyleForDefinedFont,
        RwStepVisualRwTextStyleForDefinedFont
    ),
    share_row!(
        302,
        StepVisualTextStyleWithBoxCharacteristics,
        RwStepVisualRwTextStyleWithBoxCharacteristics
    ),
    share_row!(305, StepGeomToroidalSurface, RwStepGeomRwToroidalSurface),
    share_row!(306, StepShapeTorus, RwStepShapeRwTorus),
    share_row!(
        307,
        StepShapeTransitionalShapeRepresentation,
        RwStepShapeRwTransitionalShapeRepresentation
    ),
    share_row!(308, StepGeomTrimmedCurve, RwStepGeomRwTrimmedCurve),
    share_row!(
        310,
        StepBasicUncertaintyMeasureWithUnit,
        RwStepBasicRwUncertaintyMeasureWithUnit
    ),
    share_row!(311, StepGeomUniformCurve, RwStepGeomRwUniformCurve),
    share_row!(312, StepGeomUniformSurface, RwStepGeomRwUniformSurface),
    share_row!(313, StepGeomVector, RwStepGeomRwVector),
    share_row!(315, StepShapeVertexLoop, RwStepShapeRwVertexLoop),
    share_row!(316, StepShapeVertexPoint, RwStepShapeRwVertexPoint),
    share_row!(317, StepVisualViewVolume, RwStepVisualRwViewVolume),
    share_row!(
        319,
        StepGeomUniformCurveAndRationalBSplineCurve,
        RwStepGeomRwUniformCurveAndRationalBSplineCurve
    ),
    share_row!(
        320,
        StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve,
        RwStepGeomRwBSplineCurveWithKnotsAndRationalBSplineCurve
    ),
    share_row!(
        321,
        StepGeomQuasiUniformCurveAndRationalBSplineCurve,
        RwStepGeomRwQuasiUniformCurveAndRationalBSplineCurve
    ),
    share_row!(
        322,
        StepGeomBezierCurveAndRationalBSplineCurve,
        RwStepGeomRwBezierCurveAndRationalBSplineCurve
    ),
    share_row!(
        323,
        StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface,
        RwStepGeomRwBSplineSurfaceWithKnotsAndRationalBSplineSurface
    ),
    share_row!(
        324,
        StepGeomUniformSurfaceAndRationalBSplineSurface,
        RwStepGeomRwUniformSurfaceAndRationalBSplineSurface
    ),
    share_row!(
        325,
        StepGeomQuasiUniformSurfaceAndRationalBSplineSurface,
        RwStepGeomRwQuasiUniformSurfaceAndRationalBSplineSurface
    ),
    share_row!(
        326,
        StepGeomBezierSurfaceAndRationalBSplineSurface,
        RwStepGeomRwBezierSurfaceAndRationalBSplineSurface
    ),
    share_row!(
        329,
        StepBasicConversionBasedUnitAndLengthUnit,
        RwStepBasicRwConversionBasedUnitAndLengthUnit
    ),
    share_row!(
        330,
        StepBasicConversionBasedUnitAndPlaneAngleUnit,
        RwStepBasicRwConversionBasedUnitAndPlaneAngleUnit
    ),
    share_row!(
        331,
        StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext,
        RwStepGeomRwGeometricRepresentationContextAndGlobalUnitAssignedContext
    ),
    share_row!(332, StepShapeLoopAndPath, RwStepShapeRwLoopAndPath),
    share_row!(
        333,
        StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx,
        RwStepGeomRwGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx
    ),
    share_row!(
        334,
        StepBasicConversionBasedUnitAndSolidAngleUnit,
        RwStepBasicRwConversionBasedUnitAndSolidAngleUnit
    ),
    share_row!(336, StepBasicSolidAngleUnit, RwStepBasicRwSolidAngleUnit),
    share_row!(
        337,
        StepShapeFacetedBrepAndBrepWithVoids,
        RwStepShapeRwFacetedBrepAndBrepWithVoids
    ),
    share_row!(
        338,
        StepGeomGeometricRepresentationContextAndParametricRepresentationContext,
        RwStepGeomRwGeometricRepresentationContextAndParametricRepresentationContext
    ),
    share_row!(
        339,
        StepBasicMechanicalContext,
        RwStepBasicRwMechanicalContext
    ),
    share_row!(
        340,
        StepBasicProductDefinitionContext,
        RwStepBasicRwProductDefinitionContext
    ),
    share_row!(341, StepBasicMeasureWithUnit, RwStepBasicRwMeasureWithUnit),
    share_row!(342, StepBasicNamedUnit, RwStepBasicRwNamedUnit),
    share_row!(343, StepBasicNamedUnit, RwStepBasicRwNamedUnit),
    share_row!(
        348,
        StepBasicApprovalDateTime,
        RwStepBasicRwApprovalDateTime
    ),
];
