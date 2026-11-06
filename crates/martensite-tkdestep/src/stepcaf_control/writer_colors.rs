//! `STEPCAFControlWriter::writeColors`: the colours and visibility of the shapes, as STEP styles.
use super::writer::{WHERE, WriterState};
use super::writer_support::{find_entities, finder_process_of, typed_result_of};
use crate::step_construct::{StepConstructRenderingProperties, StepConstructStyles};
use crate::step_data::{StepDataSelectType, StepDataStepModel};
use crate::step_entity::{
    step_array1_item, step_array1_len, step_array1_set, step_live, step_new_array1,
};
use crate::step_repr::{
    StepReprProductDefinitionShape, StepReprRepresentationContext, StepReprRepresentationItem,
};
use crate::step_shape::StepShapeContextDependentShapeRepresentation;
use crate::step_visual::{
    StepVisualColour, StepVisualInvisibility, StepVisualInvisibleItem,
    StepVisualMechanicalDesignGeometricPresentationRepresentation,
    StepVisualPresentationRepresentation, StepVisualPresentationStyleAssignment,
    StepVisualPresentationStyleByContext, StepVisualPresentationStyleSelect, StepVisualStyledItem,
};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkbrep::toptools::{MapOfShape, ShapeMapHasher};
use martensite_tkernel::ncollection::IndexedDataMap;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool, XCAFDocShapeTool,
    XCAFDocVisMaterialTool,
};
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashMap;

mod labels;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use labels::*;

type MdgprMap = IndexedDataMap<Shape, Handle<StandardTransient>, ShapeMapHasher>;
type Settings = IndexedDataMap<Shape, XCAFPrsStyle, ShapeMapHasher>;

/// `bind`: replaces the value of a shape already there.
pub(super) fn bind_mdgpr(
    map: &mut MdgprMap,
    shape: &Shape,
    mdgpr: Handle<StandardTransient>,
) -> Result<(), Failure> {
    if map.contains(shape) {
        *map.change_from_key(shape)? = mdgpr;
    } else {
        map.add(shape.clone(), mdgpr);
    }
    Ok(())
}

/// The colours created so far, one entity per colour.
#[derive(Default)]
struct SharedColours {
    dpdcs: HashMap<String, Handle<StandardTransient>>,
    col_rgbs: HashMap<[u64; 3], Handle<StandardTransient>>,
}

impl SharedColours {
    fn encode(&mut self, color: &Color) -> Result<Handle<StepVisualColour>, Failure> {
        StepConstructStyles::encode_color_shared(color, &mut self.dpdcs, &mut self.col_rgbs)
    }
}

fn is_by_context(psa: &Handle<StepVisualPresentationStyleAssignment>) -> bool {
    psa.get().is_none() || psa.cast::<StepVisualPresentationStyleByContext>().is_some()
}

/// The first PSA of a styled item for the top level (not a style by context for a NAUO).
fn has_top_level_psa(item: &StepVisualStyledItem) -> Result<bool, Failure> {
    let styles = item.styles();
    if styles.is_null() {
        return Ok(false);
    }
    for index in 1..=step_array1_len(&styles) {
        if !is_by_context(&step_array1_item(&styles, index, WHERE)?) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// `getStyledItem`: the styled item of the top-level shape of `shape`, for overriding it.
pub(super) fn get_styled_item(
    shape: &Shape,
    shape_tool: &Handle<XCAFDocShapeTool>,
    styles: &StepConstructStyles,
    result: &mut Handle<StepVisualStyledItem>,
    map_comp_mdgpr: &MdgprMap,
) -> Result<bool, Failure> {
    let mut top_label = TdfLabel::default();
    step_live(shape_tool, WHERE)?.find_shape(shape, &mut top_label, false)?;
    let mut top_shape = Shape::default();
    if !top_label.is_null() {
        XCAFDocShapeTool::get_shape(&top_label, &mut top_shape)?;
    }
    if top_shape.is_null() || !map_comp_mdgpr.contains(&top_shape) {
        return Ok(false);
    }
    let mdgpr: Handle<StepVisualPresentationRepresentation> =
        map_comp_mdgpr.find_from_key(&top_shape)?.cast();
    let items = step_live(&mdgpr, WHERE)?.items();
    if items.is_null() {
        return Ok(false);
    }
    // Search for PSA of Manifold solid
    let mut found_items = Vec::new();
    let mut is_filled = false;
    for index in 1..=step_array1_len(&items) {
        let selected: Handle<StepVisualStyledItem> = step_array1_item(&items, index, WHERE)?.cast();
        let Some(live) = selected.get() else {
            continue;
        };
        // Check that it is a styled item for manifold solid brep
        if !is_filled {
            let mut location = Location::default();
            find_entities(
                styles.finder_process(),
                &top_shape,
                &mut location,
                &mut found_items,
            )?;
            is_filled = true;
        }
        if !found_items.is_empty() {
            let item = live.item();
            let same: Handle<StandardTransient> = item.cast();
            if same.is_null() || !found_items.contains(&same) {
                continue;
            }
        }
        if has_top_level_psa(live)? {
            *result = selected.clone();
            return Ok(true);
        }
    }
    Ok(false)
}

/// `setDefaultInstanceColor`: the styles of the top-level PSA of a styled item into `psa`.
pub(super) fn set_default_instance_color(
    style_item: &Handle<StepVisualStyledItem>,
    psa: &Handle<StepVisualPresentationStyleAssignment>,
) -> Result<bool, Failure> {
    let styles = step_live(style_item, WHERE)?.styles();
    let mut is_found = false;
    for index in 1..=step_array1_len(&styles) {
        let father = step_array1_item(&styles, index, WHERE)?;
        // check for PSA for top-level (not Presentation style by context for NAUO)
        if is_by_context(&father) {
            return Ok(false);
        }
        // get style select from father PSA
        let live_father = step_live(&father, WHERE)?;
        let count = live_father.nb_styles()?;
        if count <= 0 {
            continue;
        }
        let father_styles = live_father.styles();
        let copied = step_new_array1::<StepVisualPresentationStyleSelect>(1, count)?;
        let mut setting = 1;
        for select in 1..=step_array1_len(&father_styles) {
            let old = step_array1_item(&father_styles, select, WHERE)?;
            if old.point_style().is_null()
                && old.curve_style().is_null()
                && old.surface_style_usage().is_null()
            {
                is_found = false;
                break;
            }
            step_array1_set(&copied, setting, old, WHERE)?;
            setting += 1;
            is_found = true;
        }
        // init PSA of NAUO
        if is_found {
            step_live(psa, WHERE)?.init(&copied);
        }
    }
    Ok(is_found)
}

/// What stays the same in the recursion of `MakeSTEPStyles`.
struct StyleWalk<'a> {
    styles: &'a mut StepConstructStyles,
    settings: &'a Settings,
    map: MapOfShape,
    map_comp_mdgpr: &'a MdgprMap,
    colours: &'a mut SharedColours,
    shape_tool: &'a Handle<XCAFDocShapeTool>,
    vis_material_mode: bool,
}

impl StyleWalk<'_> {
    /// The style of a shape: its own over what it inherits.
    fn style_of(
        &self,
        shape: &Shape,
        inherit: Option<&XCAFPrsStyle>,
    ) -> Result<XCAFPrsStyle, Failure> {
        let mut style = inherit.cloned().unwrap_or_default();
        if self.settings.contains(shape) {
            let own = self.settings.find_from_key(shape)?;
            if !own.is_visible() {
                style.set_visibility(false);
            }
            if own.is_set_color_curv() {
                style.set_color_curv(own.get_color_curv());
            }
            if own.is_set_color_surf() {
                style.set_color_surf_rgba(own.get_color_surf_rgba());
            }
            if own.material().is_some() {
                style.set_material(own.material());
            }
        }
        Ok(style)
    }
    fn has_material(&self, style: &XCAFPrsStyle) -> bool {
        self.vis_material_mode && style.material().get().is_some_and(|m| !m.is_empty())
    }
    /// The PSA and style of one item.
    #[allow(clippy::too_many_arguments)]
    fn style_item(
        &mut self,
        item: &Handle<StepReprRepresentationItem>,
        style: &XCAFPrsStyle,
        surf_color: &mut Handle<StepVisualColour>,
        curv_color: &Handle<StepVisualColour>,
        render_transp: f64,
        over: &Handle<StepVisualStyledItem>,
        is_component: bool,
    ) -> Result<Handle<StepVisualStyledItem>, Failure> {
        let psa = if style.is_visible()
            || surf_color.is_some()
            || curv_color.is_some()
            || self.has_material(style)
        {
            let mut render = StepConstructRenderingProperties::new();
            if self.has_material(style) {
                render.init_material(style.material())?;
            } else if render_transp > 0.0 {
                render.init_rgba(&style.get_color_surf_rgba());
            }
            self.styles
                .make_color_psa(item, surf_color, curv_color, &render, is_component)?
        } else {
            // default white color
            *surf_color = self.colours.encode(&Color::named(NamedColor::WHITE))?;
            let psa = self.styles.make_color_psa(
                item,
                surf_color,
                curv_color,
                &StepConstructRenderingProperties::new(),
                is_component,
            )?;
            if is_component {
                set_default_instance_color(over, &psa)?;
            }
            psa
        };
        self.styles.add_style_for_item(item, &psa, over)
    }
}
