#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_geom::*;
#[allow(unused_imports)]
use crate::step_repr::*;
#[allow(unused_imports)]
use crate::step_shape::*;

/// Central or parallel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepVisualCentralOrParallel {
    /// Central.
    #[default]
    CopCentral,
    /// Parallel.
    CopParallel,
}

/// Marker type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepVisualMarkerType {
    /// Dot.
    #[default]
    MtDot,
    /// X.
    MtX,
    /// Plus.
    MtPlus,
    /// Asterisk.
    MtAsterisk,
    /// Ring.
    MtRing,
    /// Square.
    MtSquare,
    /// Triangle.
    MtTriangle,
}

/// Null style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepVisualNullStyle {
    /// Null.
    #[default]
    Null,
}

/// Shading surface method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepVisualShadingSurfaceMethod {
    /// Constant shading.
    #[default]
    SsmConstantShading,
    /// Colour shading.
    SsmColourShading,
    /// Dot shading.
    SsmDotShading,
    /// Normal shading.
    SsmNormalShading,
}

/// Surface side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepVisualSurfaceSide {
    /// Negative.
    #[default]
    SsNegative,
    /// Positive.
    SsPositive,
    /// Both.
    SsBoth,
}

/// Text path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepVisualTextPath {
    /// Up.
    #[default]
    TpUp,
    /// Right.
    TpRight,
    /// Down.
    TpDown,
    /// Left.
    TpLeft,
}

step_select! {
    /// Annotation plane element.
    StepVisualAnnotationPlaneElement, [StepVisualDraughtingCallout, StepVisualStyledItem], {
        draughting_callout: StepVisualDraughtingCallout,
        styled_item: StepVisualStyledItem,
    }
}

step_select! {
    /// Area or view.
    StepVisualAreaOrView, [StepVisualPresentationArea, StepVisualPresentationView], {
        presentation_area: StepVisualPresentationArea,
        presentation_view: StepVisualPresentationView,
    }
}

step_select! {
    /// Curve style font select.
    StepVisualCurveStyleFontSelect, [StepVisualCurveStyleFont, StepVisualPreDefinedCurveFont, StepVisualExternallyDefinedCurveFont], {
        curve_style_font: StepVisualCurveStyleFont,
        pre_defined_curve_font: StepVisualPreDefinedCurveFont,
        externally_defined_curve_font: StepVisualExternallyDefinedCurveFont,
    }
}

step_select! {
    /// Draughting callout element.
    StepVisualDraughtingCalloutElement, [StepVisualAnnotationCurveOccurrence, StepVisualAnnotationTextOccurrence, StepVisualTessellatedAnnotationOccurrence, StepVisualAnnotationFillAreaOccurrence], {
        annotation_curve_occurrence: StepVisualAnnotationCurveOccurrence,
        annotation_text_occurrence: StepVisualAnnotationTextOccurrence,
        tessellated_annotation_occurrence: StepVisualTessellatedAnnotationOccurrence,
        annotation_fill_area_occurrence: StepVisualAnnotationFillAreaOccurrence,
    }
}

step_select! {
    /// Representation of STEP SELECT type EdgeOrCurve
    StepVisualEdgeOrCurve, [StepGeomCurve, StepShapeEdge], {
        curve: StepGeomCurve,
        edge: StepShapeEdge,
    }
}

step_select! {
    /// Representation of STEP SELECT type FaceOrSurface
    StepVisualFaceOrSurface, [StepShapeFace, StepGeomSurface], {
        face: StepShapeFace,
        surface: StepGeomSurface,
    }
}

step_select! {
    /// Fill style select.
    StepVisualFillStyleSelect, [StepVisualFillAreaStyleColour], {
        fill_area_style_colour: StepVisualFillAreaStyleColour,
    }
}

step_select! {
    /// Font select.
    StepVisualFontSelect, [StepVisualPreDefinedTextFont, StepVisualExternallyDefinedTextFont], {
        pre_defined_text_font: StepVisualPreDefinedTextFont,
        externally_defined_text_font: StepVisualExternallyDefinedTextFont,
    }
}

step_select! {
    /// Invisibility context.
    StepVisualInvisibilityContext, [StepVisualPresentationRepresentation, StepVisualPresentationSet, StepVisualDraughtingModel], {
        presentation_representation: StepVisualPresentationRepresentation,
        presentation_set: StepVisualPresentationSet,
        draughting_model: StepVisualDraughtingModel,
    }
}

step_select! {
    /// Invisible item.
    StepVisualInvisibleItem, [StepVisualStyledItem, StepVisualPresentationLayerAssignment, StepVisualPresentationRepresentation], {
        styled_item: StepVisualStyledItem,
        presentation_layer_assignment: StepVisualPresentationLayerAssignment,
        presentation_representation: StepVisualPresentationRepresentation,
    }
}

step_select! {
    /// Layered item.
    StepVisualLayeredItem, [StepVisualPresentationRepresentation, StepReprRepresentationItem], {
        presentation_representation: StepVisualPresentationRepresentation,
        representation_item: StepReprRepresentationItem,
    }
}

step_select! {
    /// Representation of STEP SELECT type PathOrCompositeCurve
    StepVisualPathOrCompositeCurve, [StepGeomCompositeCurve, StepShapePath], {
        composite_curve: StepGeomCompositeCurve,
        path: StepShapePath,
    }
}

step_select! {
    /// Presentation representation select.
    StepVisualPresentationRepresentationSelect, [StepVisualPresentationRepresentation, StepVisualPresentationSet], {
        presentation_representation: StepVisualPresentationRepresentation,
        presentation_set: StepVisualPresentationSet,
    }
}

step_select! {
    /// Presentation size assignment select.
    StepVisualPresentationSizeAssignmentSelect, [StepVisualPresentationView, StepVisualPresentationArea, StepVisualAreaInSet], {
        presentation_view: StepVisualPresentationView,
        presentation_area: StepVisualPresentationArea,
        area_in_set: StepVisualAreaInSet,
    }
}

step_select! {
    /// Presentation style select.
    StepVisualPresentationStyleSelect, numbered [StepVisualPointStyle = 1, StepVisualCurveStyle = 2, StepVisualSurfaceStyleUsage = 3, StepVisualNullStyleMember = 7], {
        point_style: StepVisualPointStyle,
        curve_style: StepVisualCurveStyle,
        null_style: StepVisualNullStyleMember,
        surface_style_usage: StepVisualSurfaceStyleUsage,
    }
}

step_select! {
    /// Representation of STEP SELECT type RenderingPropertiesSelect
    StepVisualRenderingPropertiesSelect, [StepVisualSurfaceStyleReflectanceAmbient, StepVisualSurfaceStyleTransparent], {
        surface_style_reflectance_ambient: StepVisualSurfaceStyleReflectanceAmbient,
        surface_style_transparent: StepVisualSurfaceStyleTransparent,
    }
}

step_select! {
    /// Style context select.
    StepVisualStyleContextSelect, [StepReprRepresentation, StepReprRepresentationItem, StepVisualPresentationSet], {
        representation: StepReprRepresentation,
        representation_item: StepReprRepresentationItem,
        presentation_set: StepVisualPresentationSet,
    }
}

step_select! {
    /// Styled item target.
    StepVisualStyledItemTarget, [StepGeomGeometricRepresentationItem, StepReprMappedItem, StepReprRepresentation, StepShapeTopologicalRepresentationItem], {
        geometric_representation_item: StepGeomGeometricRepresentationItem,
        mapped_item: StepReprMappedItem,
        representation: StepReprRepresentation,
        topological_representation_item: StepShapeTopologicalRepresentationItem,
    }
}

step_select! {
    /// Surface style element select.
    StepVisualSurfaceStyleElementSelect, numbered [StepVisualSurfaceStyleFillArea = 1, StepVisualSurfaceStyleBoundary = 2, StepVisualSurfaceStyleParameterLine = 3, StepVisualSurfaceStyleRendering = 7], {
        surface_style_fill_area: StepVisualSurfaceStyleFillArea,
        surface_style_boundary: StepVisualSurfaceStyleBoundary,
        surface_style_parameter_line: StepVisualSurfaceStyleParameterLine,
        surface_style_rendering: StepVisualSurfaceStyleRendering,
    }
}

step_select! {
    /// Representation of STEP SELECT type TessellatedEdgeOrVertex
    StepVisualTessellatedEdgeOrVertex, [StepVisualTessellatedEdge, StepVisualTessellatedVertex], {
        tessellated_edge: StepVisualTessellatedEdge,
        tessellated_vertex: StepVisualTessellatedVertex,
    }
}

step_select! {
    /// Text or character.
    StepVisualTextOrCharacter, [StepVisualAnnotationText, StepVisualCompositeText, StepVisualTextLiteral], {
        annotation_text: StepVisualAnnotationText,
        composite_text: StepVisualCompositeText,
        text_literal: StepVisualTextLiteral,
    }
}

step_entity! {
    /// Annotation curve occurrence.
    StepVisualAnnotationCurveOccurrence, StepVisualAnnotationCurveOccurrenceState, "StepVisualAnnotationCurveOccurrence", StepVisualAnnotationOccurrence, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    /// Complex STEP entity AnnotationCurveOccurrence & AnnotationOccurrence &
    /// GeometricRepresentationItem & RepresentationItem & StyledItem
    StepVisualAnnotationCurveOccurrenceAndGeomReprItem, StepVisualAnnotationCurveOccurrenceAndGeomReprItemState, "StepVisualAnnotationCurveOccurrenceAndGeomReprItem", StepVisualAnnotationCurveOccurrence, {
    }
}

step_entity! {
    /// Annotation fill area.
    StepVisualAnnotationFillArea, StepVisualAnnotationFillAreaState, "StepVisualAnnotationFillArea", StepShapeGeometricCurveSet, {
    }
}
