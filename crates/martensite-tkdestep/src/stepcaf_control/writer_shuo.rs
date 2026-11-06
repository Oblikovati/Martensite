//! `STEPCAFControlWriter::writeSHUOs`: the styled specified higher usage occurrences of assemblies.
use super::writer::{WHERE, WriterState};
use super::writer_colors::{bind_mdgpr, get_styled_item, set_default_instance_color};
use super::writer_support::{find_entities, finder_process_of, model_of, typed_result_of};
use crate::step_basic::StepBasicProductDefinition;
use crate::step_construct::{StepConstructRenderingProperties, StepConstructStyles};
use crate::step_data::{StepDataSelectType, StepDataStepModel};
use crate::step_entity::{
    step_array1_item, step_array1_len, step_array1_set, step_live, step_new_array1,
};
use crate::step_repr::{
    StepReprAssemblyComponentUsage, StepReprCharacterizedDefinition,
    StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape,
    StepReprRepresentationItem, StepReprSpecifiedHigherUsageOccurrence,
};
use crate::step_shape::StepShapeContextDependentShapeRepresentation;
use crate::step_visual::{
    StepVisualInvisibility, StepVisualInvisibleItem,
    StepVisualMechanicalDesignGeometricPresentationRepresentation,
    StepVisualPresentationRepresentation, StepVisualStyledItem,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::IndexedDataMap;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tklcaf::tdf::{TdfAttribute, TdfLabel};
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool, XCAFDocGraphNode, XCAFDocShapeTool,
    XCAFDocVisMaterialTool,
};
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashSet;

fn shape_of(label: &TdfLabel) -> Result<Shape, Failure> {
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(label, &mut shape)?;
    Ok(shape)
}

/// `getSHUOstyle`: false for a visible SHUO without any colour.
fn get_shuo_style(label: &TdfLabel, style: &mut XCAFPrsStyle) -> Result<bool, Failure> {
    if !XCAFDocColorTool::is_visible(label)? {
        style.set_visibility(false);
    } else {
        if let Some(color) = XCAFDocColorTool::get_label_color(label, XCAFDocColorType::ColorGen)? {
            style.set_color_curv(*color.rgb());
            style.set_color_surf(*color.rgb());
        }
        if let Some(color) = XCAFDocColorTool::get_label_color(label, XCAFDocColorType::ColorSurf)?
        {
            style.set_color_surf(*color.rgb());
        }
        if let Some(color) = XCAFDocColorTool::get_label_color(label, XCAFDocColorType::ColorCurv)?
        {
            style.set_color_curv(*color.rgb());
        }
        if !style.is_set_color_surf() {
            let material = XCAFDocVisMaterialTool::get_shape_material(label)?;
            if let Some(material) = material.get().filter(|m| !m.is_empty()) {
                // only color can be stored in STEP
                style.set_color_surf_rgba(material.base_color());
            }
        }
    }
    Ok(style.is_set_color_curv() || style.is_set_color_surf() || !style.is_visible())
}

/// `getProDefinitionOfNAUO`: the NAUO of a component shape and its relating or related product
/// definition; `None` when any of them is missing.
fn product_definition_of_nauo(
    ws: &Handle<XSControlWorkSession>,
    shape: &Shape,
    is_relating: bool,
) -> Result<
    Option<(
        Handle<StepBasicProductDefinition>,
        Handle<StepReprNextAssemblyUsageOccurrence>,
    )>,
    Failure,
> {
    if shape.is_null() {
        return Ok(None);
    }
    // get CDSR
    let fp = finder_process_of(ws)?;
    let cdsr = typed_result_of::<StepShapeContextDependentShapeRepresentation>(&fp, shape)?;
    let Some(cdsr) = cdsr.get() else {
        return Ok(None);
    };
    // get PDS of NAUO
    let pds = cdsr.represented_product_relation();
    if pds.is_null() {
        return Ok(None);
    }
    // get the NAUO entity
    let graph = step_live(ws, WHERE)?.graph()?;
    let shareds = step_live(&graph, WHERE)?
        .graph()
        .shareds(&pds.cast())?
        .content();
    let nauo = shareds
        .iter()
        .map(|shared| shared.cast::<StepReprNextAssemblyUsageOccurrence>())
        .find(Handle::is_some);
    let Some(nauo) = nauo else {
        return Ok(None);
    };
    // get Relatinf or Related product definition
    let live = step_live(&nauo, WHERE)?;
    let definition = if is_relating {
        live.relating_product_definition()
    } else {
        live.related_product_definition()
    };
    Ok(definition.is_some().then_some((definition, nauo)))
}

/// What the recursion of `writeSHUO` fills.
#[derive(Default)]
struct ShuoWalk {
    nauo_shape: Shape,
    relating_pd: Handle<StepBasicProductDefinition>,
    is_deepest: bool,
}

/// The arguments of an SHUO entity, with empty strings where the standard ones are.
fn init_shuo(
    entity: &StepReprSpecifiedHigherUsageOccurrence,
    relating: &Handle<StepBasicProductDefinition>,
    related: &Handle<StepBasicProductDefinition>,
    upper: &Handle<StepReprAssemblyComponentUsage>,
    next: &Handle<StepReprNextAssemblyUsageOccurrence>,
) -> Result<(), Failure> {
    // set the ProductDefinitionRelationship descriptin information as empty strings.
    let empty = Handle::new(HAsciiString::new(""));
    entity.init(
        &empty, &empty, false, &empty, relating, related, false, &empty, upper, next,
    )?;
    Ok(())
}

/// The first next usage of an SHUO, null without one.
fn next_usage_of(label: &TdfLabel) -> Result<(usize, Handle<XCAFDocGraphNode>), Failure> {
    let mut next_labels = Vec::new();
    XCAFDocShapeTool::get_shuo_next_usage(label, &mut next_labels)?;
    let mut next = Handle::default();
    if let Some(first) = next_labels.first() {
        XCAFDocShapeTool::get_shuo(first, &mut next)?;
    }
    Ok((next_labels.len(), next))
}

/// `writeSHUO`, for the top SHUO: `top` receives the entity.
fn write_top_shuo(
    shuo: &Handle<XCAFDocGraphNode>,
    ws: &Handle<XSControlWorkSession>,
    top: &mut Handle<StepReprSpecifiedHigherUsageOccurrence>,
    walk: &mut ShuoWalk,
) -> Result<bool, Failure> {
    let label = step_live(shuo, WHERE)?.attribute_base().label();
    let (count, next) = next_usage_of(&label)?;
    if count < 1 || next.is_null() {
        return Ok(false);
    }
    // get relating product definition
    let top_shape = shape_of(&label.father())?;
    let Some((relating, relating_nauo)) = product_definition_of_nauo(ws, &top_shape, true)? else {
        return Ok(false);
    };
    walk.relating_pd = relating;
    // get related product definition
    let next_shape = shape_of(&step_live(&next, WHERE)?.attribute_base().label().father())?;
    let Some((related, related_nauo)) = product_definition_of_nauo(ws, &next_shape, false)? else {
        return Ok(false);
    };
    let entity = Handle::new(StepReprSpecifiedHigherUsageOccurrence::new());
    // create deepest shuo EmptyString
    init_shuo(
        step_live(&entity, WHERE)?,
        &walk.relating_pd,
        &related,
        &relating_nauo.cast(),
        &related_nauo,
    )?;
    // write the other SHUO.
    if !write_inner_shuo(&next, ws, &entity, walk)? {
        return Ok(false);
    }
    *top = entity;
    Ok(true)
}

/// `writeSHUO`, below the top: the deepest one gives its shape, the others are stored on the way back.
fn write_inner_shuo(
    shuo: &Handle<XCAFDocGraphNode>,
    ws: &Handle<XSControlWorkSession>,
    top: &Handle<StepReprSpecifiedHigherUsageOccurrence>,
    walk: &mut ShuoWalk,
) -> Result<bool, Failure> {
    let label = step_live(shuo, WHERE)?.attribute_base().label();
    let (count, next) = next_usage_of(&label)?;
    if count == 0 {
        // get shape
        walk.nauo_shape = shape_of(&label.father())?;
        // return to the deepest level from SHUO shape level
        // it is because SHUO is attribute on deep level and shape level.
        walk.is_deepest = true;
        return Ok(true);
    }
    // store SHUO recursive (only one next_usage of current SHUO)
    let entity = Handle::new(StepReprSpecifiedHigherUsageOccurrence::new());
    if !write_inner_shuo(&next, ws, &entity, walk)? {
        return Ok(false);
    }
    // store the deepest SHUO to the dociment
    let next_shape = shape_of(&step_live(&next, WHERE)?.attribute_base().label().father())?;
    let upper_shape = shape_of(&label.father())?;
    // get relating PD with upper_usage and related PD with next_usage
    if product_definition_of_nauo(ws, &upper_shape, true)?.is_none() {
        return Ok(false);
    }
    let Some((related, next_nauo)) = product_definition_of_nauo(ws, &next_shape, false)? else {
        return Ok(false);
    };
    init_shuo(
        step_live(&entity, WHERE)?,
        &walk.relating_pd,
        &related,
        &top.cast(),
        &next_nauo,
    )?;
    walk.is_deepest = false;
    model_of(ws)?.add_with_refs(&entity.cast(), 0, false)?;
    Ok(true)
}

/// The arguments of `createSHUOStyledItem` that come from the caller.
struct ShuoStyleTarget<'a> {
    style: &'a XCAFPrsStyle,
    pds: &'a Handle<StepReprProductDefinitionShape>,
    shape: &'a Shape,
    tool: &'a Handle<XCAFDocShapeTool>,
}

/// The styled item of an SHUO added to the MDGPR of the top-level shape.
fn add_to_mdgpr(
    ws: &Handle<XSControlWorkSession>,
    styles: &StepConstructStyles,
    top_shape: &Shape,
    context: &Handle<crate::step_repr::StepReprRepresentationContext>,
    styled: &Handle<StepVisualStyledItem>,
    map_comp_mdgpr: &mut IndexedDataMap<Shape, Handle<StandardTransient>, ShapeMapHasher>,
) -> Result<(), Failure> {
    if top_shape.is_null() {
        // add as root to the model, but it is not good
        return model_of(ws)?.add_with_refs(&styled.cast(), 0, false);
    }
    if !map_comp_mdgpr.contains(top_shape) {
        // create MDGPR and record it in model
        let fp = finder_process_of(ws)?;
        let model: Handle<StepDataStepModel> = step_live(&fp, WHERE)?.model().cast();
        let mut mdgpr: Handle<StepVisualMechanicalDesignGeometricPresentationRepresentation> =
            Handle::default();
        styles.create_mdgpr(context, &mut mdgpr, &model)?;
        if mdgpr.is_some() {
            bind_mdgpr(map_comp_mdgpr, top_shape, mdgpr.cast())?;
        }
        return Ok(());
    }
    // get MDGPR of the top-level shape
    let mdgpr: Handle<StepVisualPresentationRepresentation> =
        map_comp_mdgpr.find_from_key(top_shape)?.cast();
    let live = step_live(&mdgpr, WHERE)?;
    // get old styled items to not lose it
    let old_items = live.items();
    let old_length = step_array1_len(&old_items);
    // create new array of styled items by an olds and new one
    let new_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, old_length + 1)?;
    for index in 1..=old_length {
        step_array1_set(
            &new_items,
            index,
            step_array1_item(&old_items, index, WHERE)?,
            WHERE,
        )?;
    }
    step_array1_set(&new_items, old_length + 1, styled.cast(), WHERE)?;
    // init MDGPR be new array of styled items
    live.set_items(&new_items);
    Ok(())
}

/// `createSHUOStyledItem`: the styled item of an SHUO, stored to the model.
fn create_shuo_styled_item(
    target: &ShuoStyleTarget<'_>,
    ws: &Handle<XSControlWorkSession>,
    map_comp_mdgpr: &mut IndexedDataMap<Shape, Handle<StandardTransient>, ShapeMapHasher>,
) -> Result<bool, Failure> {
    let ShuoStyleTarget {
        style,
        pds,
        shape,
        tool,
    } = *target;
    // create styled item for the indicated SHUO and store to the model
    let mut styles = StepConstructStyles::with_ws(ws)?;
    // translate colors to STEP
    let (mut surf_color, mut curv_color) = (Handle::default(), Handle::default());
    let mut render_transp = 0.0;
    if style.is_set_color_surf() {
        let color = style.get_color_surf_rgba();
        render_transp = 1.0 - f64::from(color.alpha());
        surf_color = StepConstructStyles::encode_color(color.rgb())?;
    }
    if style.is_set_color_curv() {
        curv_color = StepConstructStyles::encode_color(&style.get_color_curv())?;
    }
    // set default color for invisible SHUO.
    let mut is_set_default_color = false;
    if surf_color.is_null() && curv_color.is_null() && !style.is_visible() {
        surf_color = StepConstructStyles::encode_color(&Color::named(NamedColor::WHITE))?;
        is_set_default_color = true;
    }
    let mut render = StepConstructRenderingProperties::new();
    if render_transp > 0.0 {
        render.init_rgba(&style.get_color_surf_rgba());
    }
    // cause need to get PSBC
    let psa = styles.make_color_psa(&Handle::default(), &surf_color, &curv_color, &render, true)?;
    // find the repr item of the shape
    let fp = finder_process_of(ws)?;
    let cdsr = typed_result_of::<StepShapeContextDependentShapeRepresentation>(&fp, shape)?;
    if cdsr.is_null() {
        return Ok(false);
    }
    // find context
    let mut context = styles.find_context(shape)?;
    let mut top_shape = shape.clone();
    if context.is_null() {
        let mut top_label = TdfLabel::default();
        step_live(tool, WHERE)?.find_shape(shape, &mut top_label, false)?;
        if top_label.is_null() {
            return Ok(false);
        }
        top_shape = shape_of(&top_label)?;
        context = styles.find_context(&top_shape)?;
    }
    if context.is_null() {
        return Ok(false);
    }
    // get representation item of the shape
    let mut location = Location::default();
    let mut items = Vec::new();
    find_entities(&fp, shape, &mut location, &mut items)?;
    let item: Handle<StepReprRepresentationItem> = items
        .first()
        .ok_or_else(|| {
            martensite_tkernel::standard::exceptions::out_of_range(
                "STEPCAFControlWriter: no representation item of the shape of an SHUO; expected one",
            )
        })?
        .cast();
    // get overridden styled item
    let mut over = Handle::default();
    get_styled_item(shape, tool, &styles, &mut over, map_comp_mdgpr)?;
    // get STEP STYLED ITEM
    let styled = styles.add_style_for_item(&item, &psa, &over)?;
    // create SR, SDR and all necessary references between them and ST, PDS, PSBC, GRC
    styles.create_nauo_srd(&context, &cdsr, pds)?;
    // add step styled item of SHUO to the model
    // do it by additing styled item to the MDGPR
    add_to_mdgpr(ws, &styles, &top_shape, &context, &styled, map_comp_mdgpr)?;
    // create invisibility item for the styled item
    if !style.is_visible() {
        if is_set_default_color {
            // try to set default color from top-level shape
            set_default_instance_color(&over, &psa)?;
        }
        // create invisibility item and refer for styledItem
        let invisible_items = step_new_array1::<StepVisualInvisibleItem>(1, 1)?;
        let mut invisible_item = StepVisualInvisibleItem::default();
        invisible_item.set_value(&styled.cast())?;
        step_array1_set(&invisible_items, 1, invisible_item, WHERE)?;
        let invisibility = Handle::new(StepVisualInvisibility::new());
        step_live(&invisibility, WHERE)?.init(&invisible_items);
        model_of(ws)?.add_with_refs(&invisibility.cast(), 0, false)?;
    }
    Ok(true)
}

impl WriterState {
    /// `writeSHUOs`.
    pub(super) fn write_shuos(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        // map of transferred SHUO
        let mut main_shuos = HashSet::new();
        // Iterate on requested shapes
        for label in labels {
            if !self.labels.contains_key(label) || !XCAFDocShapeTool::is_assembly(label)? {
                continue; // not recorded as translated, skip
            }
            let mut components = Vec::new();
            XCAFDocShapeTool::get_components(label, &mut components, false)?;
            let tool = XCAFDocDocumentTool::shape_tool(label)?;
            // iterates on components of assembly
            for component in &components {
                let mut attributes = Vec::new();
                XCAFDocShapeTool::get_all_component_shuo(component, &mut attributes)?;
                // work with SHUO
                for attribute in &attributes {
                    self.write_one_shuo(ws, &attribute.cast(), &tool, &mut main_shuos)?;
                }
            }
        }
        Ok(true)
    }
    fn write_one_shuo(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        shuo: &Handle<XCAFDocGraphNode>,
        tool: &Handle<XCAFDocShapeTool>,
        main_shuos: &mut HashSet<usize>,
    ) -> Result<(), Failure> {
        // take label of SHUO
        let label = step_live(shuo, WHERE)?.attribute_base().label();
        let mut upper = Vec::new();
        // check is it SHUO of upper_usage
        XCAFDocShapeTool::get_shuo_upper_usage(&label, &mut upper)?;
        if !upper.is_empty() {
            return Ok(()); // transfer only main SHUO
        }
        if !main_shuos.insert(shuo.as_ptr() as usize) {
            return Ok(()); // do not try to transfer SHUO twice
        }
        // check if it is styled SHUO
        let mut style = XCAFPrsStyle::default();
        if !get_shuo_style(&label, &mut style)? {
            return Ok(());
        }
        // write SHUO to the model amd then add structure type.
        let mut walk = ShuoWalk::default();
        let mut entity = Handle::default();
        // create the top SHUO and all other.
        write_top_shuo(shuo, ws, &mut entity, &mut walk)?;
        if entity.is_null() || walk.nauo_shape.is_null() {
            return Ok(());
        }
        // create new Product Definition Shape for TOP SHUO
        let pds = Handle::new(StepReprProductDefinitionShape::new());
        let mut characterized = StepReprCharacterizedDefinition::default();
        characterized.set_value(&entity.cast())?;
        step_live(&pds, WHERE)?.init(
            &Handle::new(HAsciiString::new("SHUO")),
            false,
            &Handle::new(HAsciiString::new("")),
            &characterized,
        );
        // create styled item for SHUO and add to the model
        let target = ShuoStyleTarget {
            style: &style,
            pds: &pds,
            shape: &walk.nauo_shape,
            tool,
        };
        create_shuo_styled_item(&target, ws, &mut self.map_comp_mdgpr)?;
        Ok(())
    }
}
