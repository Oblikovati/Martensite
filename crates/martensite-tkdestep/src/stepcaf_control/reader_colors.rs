//! The colours of `STEPCAFControlReader`: styles to the colours and visibility of labels.
use super::reader::{ReaderState, STEPCAFControlShapeLabelMap, WHERE, find_instance_label};
use crate::step_construct::{StepConstructRenderingProperties, StepConstructStyles};
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::{HSequence, step_live};
use crate::step_geom::StepGeomAxis2Placement3d;
use crate::step_repr::{
    StepReprAssemblyComponentUsage, StepReprNextAssemblyUsageOccurrence,
    StepReprProductDefinitionShape, StepReprShapeRepresentationRelationship,
    StepReprShapeRepresentationRelationshipWithTransformation,
    StepReprSpecifiedHigherUsageOccurrence,
};
use crate::step_shape::{StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation};
use crate::step_to_geom::StepToGeom;
use crate::step_visual::{
    StepVisualColour, StepVisualContextDependentOverRidingStyledItem,
    StepVisualOverRidingStyledItem, StepVisualPresentationStyleByContext, StepVisualStyledItem,
};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::quantity::{Color, ColorRgba};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::{Axis3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool, XCAFDocShapeTool,
};
use martensite_tkxsbase::interface::InterfaceGraph;
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

mod styles;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use styles::*;

/// `findStyledSR`: the shape representation of a component styled item, null without one.
pub(super) fn find_styled_sr(
    style: &StepVisualStyledItem,
) -> Result<Handle<StepShapeShapeRepresentation>, Failure> {
    for j in 1..=style.nb_styles() {
        let by_context = style
            .styles_value(j)?
            .cast::<StepVisualPresentationStyleByContext>();
        let Some(by_context) = by_context.get() else {
            continue;
        };
        let current = by_context
            .style_context()
            .representation()
            .cast::<StepShapeShapeRepresentation>();
        if current.is_some() {
            return Ok(current);
        }
    }
    Ok(Handle::default())
}

/// `propagateColorToParts`: propagates the color styles from assemblies to parts.
fn propagate_color_to_parts(
    shape_tool: &Handle<XCAFDocShapeTool>,
    color_tool: &Handle<XCAFDocColorTool>,
    root: &TdfLabel,
) -> Result<(), Failure> {
    let live_shape_tool = step_live(shape_tool, WHERE)?;
    let live_color_tool = step_live(color_tool, WHERE)?;
    // collect components to propagate
    let mut components = Vec::new();
    if root.is_equal(&live_shape_tool.base_label()) {
        components = live_shape_tool.get_free_shapes()?;
    } else {
        XCAFDocShapeTool::get_components(root, &mut components, false)?;
    }
    // iterate each component
    for component in components {
        // get original label
        let mut original = component.clone();
        let mut referred = TdfLabel::default();
        if XCAFDocShapeTool::get_referred_shape(&component, &mut referred)? {
            original = referred;
        }
        // propagate to components without own colors. The loop casts 1, 2 and 3 to the kind of
        // color: 3 is out of the enumeration, and its GUID is the one of the generic color.
        for kind in [
            XCAFDocColorType::ColorSurf,
            XCAFDocColorType::ColorCurv,
            XCAFDocColorType::ColorGen,
        ] {
            let (mut color, mut dummy) = (TdfLabel::default(), TdfLabel::default());
            if XCAFDocColorTool::get_color_label(root, kind, &mut color)?
                && !XCAFDocColorTool::get_color_label(&original, kind, &mut dummy)?
            {
                live_color_tool.set_color_label(&original, &color, kind)?;
            }
        }
        if !XCAFDocColorTool::is_visible(root)? {
            live_color_tool.set_visibility(&original, false)?;
        }
        // propagate to next level children
        if XCAFDocShapeTool::is_assembly(&original)? {
            propagate_color_to_parts(shape_tool, color_tool, &original)?;
        }
    }
    Ok(())
}

/// The decoded colours of a style.
#[derive(Default)]
pub(super) struct DecodedColors {
    pub(super) surface: Option<ColorRgba>,
    pub(super) boundary: Option<Color>,
    pub(super) curve: Option<Color>,
}

/// The colours of a style as they go to the document. A surface colour takes the rendering
/// properties when they are defined.
pub(super) fn decode_style_colors(
    surface: &Handle<StepVisualColour>,
    boundary: &Handle<StepVisualColour>,
    curve: &Handle<StepVisualColour>,
    rendering: &StepConstructRenderingProperties,
) -> Result<DecodedColors, Failure> {
    let mut decoded = DecodedColors::default();
    if surface.is_some() {
        let mut color = Color::default();
        StepConstructStyles::decode_color(surface, &mut color)?;
        decoded.surface = Some(ColorRgba::from_color(color));
    }
    if boundary.is_some() {
        let mut color = Color::default();
        StepConstructStyles::decode_color(boundary, &mut color)?;
        decoded.boundary = Some(color);
    }
    if curve.is_some() {
        let mut color = Color::default();
        StepConstructStyles::decode_color(curve, &mut color)?;
        decoded.curve = Some(color);
    }
    if rendering.is_defined() {
        decoded.surface = Some(rendering.get_rgba_color());
    }
    Ok(decoded)
}

/// The colours on a label.
pub(super) fn set_label_colors(
    tool: &XCAFDocColorTool,
    label: &TdfLabel,
    colors: &DecodedColors,
) -> Result<(), Failure> {
    if let Some(surface) = &colors.surface {
        tool.set_color(label, surface, XCAFDocColorType::ColorSurf)?;
    }
    if let Some(boundary) = &colors.boundary {
        tool.set_color_rgb(label, boundary, XCAFDocColorType::ColorCurv)?;
    }
    if let Some(curve) = &colors.curve {
        tool.set_color_rgb(label, curve, XCAFDocColorType::ColorCurv)?;
    }
    Ok(())
}

/// The placement a style on a relationship gives to its shape.
fn location_of_style(
    graph: &InterfaceGraph,
    style: &Handle<StepVisualContextDependentOverRidingStyledItem>,
    factors: &StepDataFactors,
) -> Result<Location, Failure> {
    let mut location = Location::default();
    // get transformation
    for shared in graph.shareds(&style.cast())?.content() {
        let relation = shared.cast::<StepReprShapeRepresentationRelationshipWithTransformation>();
        let Some(relation) = relation.get() else {
            continue;
        };
        let item = relation
            .transformation_operator()
            .item_defined_transformation();
        let Some(item) = item.get() else {
            continue;
        };
        let first = item.transform_item1().cast::<StepGeomAxis2Placement3d>();
        let second = item.transform_item2().cast::<StepGeomAxis2Placement3d>();
        if first.is_null() || second.is_null() {
            continue;
        }
        let origin = StepToGeom::make_axis2_placement(&first, factors)?;
        let target = StepToGeom::make_axis2_placement(&second, factors)?;
        let origin = Axis3::from_axis2(step_live(&origin, WHERE)?.ax2()?);
        let target = Axis3::from_axis2(step_live(&target, WHERE)?.ax2()?);
        let mut transformation = Transform3::default();
        transformation.set_transformation_between(target, origin);
        location = location.multiplied(&Location::new(transformation))?;
    }
    Ok(location)
}

/// `set_assembly_component_style`: the override style of an assembly component.
fn set_assembly_component_style(
    tp: &Handle<TransferTransientProcess>,
    color_tool: &XCAFDocColorTool,
    styles: &StepConstructStyles,
    style: &Handle<StepVisualContextDependentOverRidingStyledItem>,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let Some(live_style) = style.get() else {
        return Ok(());
    };
    let (mut surface, mut boundary, mut curve) =
        (Handle::default(), Handle::default(), Handle::default());
    let mut rendering = StepConstructRenderingProperties::new();
    // check if it is component style
    let mut is_component = false;
    if !styles.get_colors(
        &style.cast(),
        &mut surface,
        &mut boundary,
        &mut curve,
        &mut rendering,
        &mut is_component,
    )? {
        return Ok(());
    }
    let live_tp = step_live(tp, WHERE)?;
    let guard = live_tp.graph()?;
    let graph = guard.get();
    // find shape
    let representation = live_style
        .item_ap242()?
        .value()
        .cast::<StepShapeShapeRepresentation>();
    if representation.is_null() {
        return Ok(());
    }
    let mut relationship = Handle::<StepReprShapeRepresentationRelationship>::default();
    for sharing in graph.sharings(&representation.cast())?.content() {
        let found = sharing.cast::<StepReprShapeRepresentationRelationship>();
        if found.is_some() {
            relationship = found;
        }
    }
    let binder = live_tp.find(&relationship.cast());
    let mut shape = if binder.is_some() {
        TransferBRep::shape_result(&binder)
    } else {
        Shape::default()
    };
    if shape.is_null() {
        return Ok(());
    }
    let location = location_of_style(&graph, style, factors)?;
    drop(graph);
    shape.set_location(&location, false)?;
    let colors = decode_style_colors(&surface, &boundary, &curve, &rendering)?;
    if let Some(color) = &colors.surface {
        color_tool.set_instance_color(&shape, XCAFDocColorType::ColorSurf, color, true)?;
    }
    if let Some(color) = &colors.boundary {
        color_tool.set_instance_color_rgb(&shape, XCAFDocColorType::ColorCurv, color, true)?;
    }
    if let Some(color) = &colors.curve {
        color_tool.set_instance_color_rgb(&shape, XCAFDocColorType::ColorCurv, color, true)?;
    }
    Ok(())
}
