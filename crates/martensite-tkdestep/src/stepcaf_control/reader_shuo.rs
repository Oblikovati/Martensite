//! The SHUOs of `STEPCAFControlReader`: colours on specified higher usage occurrences.
use super::reader::{
    PdFileMap, ReaderState, STEPCAFControlShapeLabelMap, WHERE, find_instance_label,
};
use super::reader_colors::{decode_style_colors, find_styled_sr, set_label_colors};
use crate::step_construct::{StepConstructRenderingProperties, StepConstructStyles};
use crate::step_entity::{HSequence, step_live};
use crate::step_repr::{
    StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape,
    StepReprSpecifiedHigherUsageOccurrence,
};
use crate::step_shape::StepShapeShapeDefinitionRepresentation;
use crate::step_visual::StepVisualStyledItem;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tklcaf::tdf::{TdfAttribute, TdfLabel};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocGraphNode, XCAFDocShapeTool};
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// `findNextSHUOlevel`: the labels of the next usages, recursively.
fn find_next_shuo_level(
    ws: &Handle<XSControlWorkSession>,
    shuo: &Handle<StepReprSpecifiedHigherUsageOccurrence>,
    tool: &Handle<XCAFDocShapeTool>,
    tp: &Handle<TransferTransientProcess>,
    map: &STEPCAFControlShapeLabelMap,
    labels: &mut Vec<TdfLabel>,
) -> Result<bool, Failure> {
    let hgraph = step_live(ws, WHERE)?.h_graph()?;
    let sharings = step_live(&hgraph, WHERE)?
        .graph()
        .sharings(&shuo.cast())?
        .content();
    let sub = sharings
        .iter()
        .map(|sharing| sharing.cast::<StepReprSpecifiedHigherUsageOccurrence>())
        .find(Handle::is_some);
    let Some(sub) = sub else {
        return Ok(false);
    };
    let next_usage = step_live(&sub, WHERE)?.next_usage();
    if next_usage.is_null() {
        return Ok(false);
    }
    // get label of NAUO next level
    let label = find_instance_label(&next_usage, tool, tp, map)?;
    if label.is_null() {
        return Ok(false);
    }
    labels.push(label);
    // and check by recurse.
    find_next_shuo_level(ws, &sub, tool, tp, map, labels)?;
    Ok(true)
}

/// `setSHUOintoDoc`: the SHUO structure in the document; null when it cannot be made.
fn set_shuo_into_doc(
    ws: &Handle<XSControlWorkSession>,
    shuo: &Handle<StepReprSpecifiedHigherUsageOccurrence>,
    tool: &Handle<XCAFDocShapeTool>,
    tp: &Handle<TransferTransientProcess>,
    map: &STEPCAFControlShapeLabelMap,
) -> Result<TdfLabel, Failure> {
    // get upper usage NAUO from SHUO.
    let live = step_live(shuo, WHERE)?;
    let upper = live
        .upper_usage()
        .cast::<StepReprNextAssemblyUsageOccurrence>();
    let next = live.next_usage();
    if upper.is_null() || next.is_null() {
        return Ok(TdfLabel::default());
    }
    // get first labels for first SHUO attribute
    let upper_label = find_instance_label(&upper, tool, tp, map)?;
    let next_label = find_instance_label(&next, tool, tp, map)?;
    if upper_label.is_null() || next_label.is_null() {
        return Ok(TdfLabel::default());
    }
    // create sequence fo labels to set SHUO structure into the document
    let mut labels = vec![upper_label, next_label];
    // add all other labels of sub SHUO entities
    find_next_shuo_level(ws, shuo, tool, tp, map, &mut labels)?;
    // last accord for SHUO
    let mut node = Handle::<XCAFDocGraphNode>::default();
    if step_live(tool, WHERE)?.set_shuo(&labels, &mut node)? {
        return Ok(step_live(&node, WHERE)?.attribute_base().label());
    }
    Ok(TdfLabel::default())
}

impl ReaderState {
    /// One styled item of `ReadSHUOs`.
    fn read_shuo_style(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc_tools: (
            &Handle<martensite_tkxcaf::xcaf_doc::XCAFDocColorTool>,
            &Handle<XCAFDocShapeTool>,
        ),
        styles: &StepConstructStyles,
        invisible: &Handle<HSequence<Handle<StandardTransient>>>,
        style: &Handle<StepVisualStyledItem>,
    ) -> Result<(), Failure> {
        let (color_tool, shape_tool) = doc_tools;
        let Some(live_style) = style.get() else {
            return Ok(());
        };
        // check the visibility of styled item.
        let key: Handle<StandardTransient> = style.cast();
        let visible =
            !step_live(invisible, WHERE)?.sequence(|items| items.iter().any(|item| *item == key));
        let (mut surface, mut boundary, mut curve) =
            (Handle::default(), Handle::default(), Handle::default());
        // check if it is component style
        let mut is_component = false;
        let mut rendering = StepConstructRenderingProperties::new();
        if !styles.get_colors(
            style,
            &mut surface,
            &mut boundary,
            &mut curve,
            &mut rendering,
            &mut is_component,
        )? && visible
        {
            return Ok(());
        }
        if !is_component {
            return Ok(());
        }
        let sr = find_styled_sr(live_style)?;
        // search for SR along model
        if sr.is_null() {
            return Ok(());
        }
        let live_ws = step_live(ws, WHERE)?;
        let hgraph = live_ws.h_graph()?;
        let sharings = step_live(&hgraph, WHERE)?
            .graph()
            .sharings(&sr.cast())?
            .content();
        let tr = live_ws.transfer_reader();
        let tp = step_live(&tr, WHERE)?.transient_process();
        let colors = decode_style_colors(&surface, &boundary, &curve, &rendering)?;
        for sharing in sharings {
            let sdr = sharing.cast::<StepShapeShapeDefinitionRepresentation>();
            let Some(sdr) = sdr.get() else {
                continue;
            };
            let pds = sdr
                .definition()
                .property_definition()
                .cast::<StepReprProductDefinitionShape>();
            let Some(pds) = pds.get() else {
                continue;
            };
            let shuo = pds
                .definition()
                .product_definition_relationship()
                .cast::<StepReprSpecifiedHigherUsageOccurrence>();
            if shuo.is_null() {
                continue;
            }
            // set the SHUO structure to the document
            let label = set_shuo_into_doc(ws, &shuo, shape_tool, &tp, &self.map)?;
            if label.is_null() {
                continue;
            }
            // now set the style to the SHUO main label.
            let live_color_tool = step_live(color_tool, WHERE)?;
            set_label_colors(live_color_tool, &label, &colors)?;
            if !visible {
                // sets the invisibility for shape.
                live_color_tool.set_visibility(&label, false)?;
            }
        }
        Ok(())
    }
    /// `ReadSHUOs`: the colours of SHUOs in the model; the big part is as in `read_colors`.
    pub(super) fn read_shuos(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        _pd_file: &PdFileMap,
    ) -> Result<bool, Failure> {
        let main = step_live(doc, WHERE)?.main()?;
        let color_tool = XCAFDocDocumentTool::color_tool(&main)?;
        let shape_tool = step_live(&color_tool, WHERE)?.shape_tool()?;
        let mut styles = StepConstructStyles::with_ws(ws)?;
        if !styles.load_styles()? {
            return Ok(false);
        }
        // searching for invisible items in the model
        let mut invisible = Handle::new(HSequence::<Handle<StandardTransient>>::default());
        styles.load_invis_styles(&mut invisible)?;
        // parse and search for color attributes
        for i in 1..=styles.nb_styles() {
            self.read_shuo_style(
                ws,
                (&color_tool, &shape_tool),
                &styles,
                &invisible,
                &styles.style(i)?,
            )?;
        }
        Ok(true)
    }
}
