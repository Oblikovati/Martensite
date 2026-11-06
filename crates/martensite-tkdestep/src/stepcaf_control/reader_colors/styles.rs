//! The styles of a shape read by `STEPCAFControlReader`.
use super::*;

/// What `set_style` shares down its recursion.
struct StyleContext<'a> {
    ws: &'a Handle<XSControlWorkSession>,
    map: &'a STEPCAFControlShapeLabelMap,
    color_tool: &'a Handle<XCAFDocColorTool>,
    shape_tool: &'a Handle<XCAFDocShapeTool>,
    styles: &'a StepConstructStyles,
    invisible: &'a Handle<HSequence<Handle<StandardTransient>>>,
    factors: &'a StepDataFactors,
}

/// The result of looking for the shape of a component style.
enum ComponentShape {
    /// A styled item that refers to a SHUO: PTV 10.02.2003 skips it.
    SkipShuo,
    /// The shape with its real location, if any.
    Found(Option<Shape>),
}

impl StyleContext<'_> {
    /// Whether a style is among the invisible ones.
    fn is_visible_style(&self, style: &Handle<StepVisualStyledItem>) -> Result<bool, Failure> {
        let key: Handle<StandardTransient> = style.cast();
        Ok(!step_live(self.invisible, WHERE)?
            .sequence(|items| items.iter().any(|item| *item == key)))
    }
    /// The instance shape of a NAUO referred to by a component style.
    fn component_shape(&self, style: &StepVisualStyledItem) -> Result<ComponentShape, Failure> {
        // take SR of NAUO
        let sr = find_styled_sr(style)?;
        // search for SR along model
        if sr.is_null() {
            return Ok(ComponentShape::Found(None));
        }
        let tool = step_live(self.color_tool, WHERE)?.shape_tool()?;
        let live_ws = step_live(self.ws, WHERE)?;
        let hgraph = live_ws.h_graph()?;
        let sharings = step_live(&hgraph, WHERE)?.graph().sharings(&sr.cast())?;
        let tr = live_ws.transfer_reader();
        let tp = step_live(&tr, WHERE)?.transient_process();
        for sharing in sharings.content() {
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
            let usage = pds
                .definition()
                .product_definition_relationship()
                .cast::<StepReprAssemblyComponentUsage>();
            if usage.is_null() {
                continue;
            }
            // PTV 10.02.2003 skip styled item that refer to SHUO
            if usage
                .cast::<StepReprSpecifiedHigherUsageOccurrence>()
                .is_some()
            {
                return Ok(ComponentShape::SkipShuo);
            }
            let nauo = usage.cast::<StepReprNextAssemblyUsageOccurrence>();
            if nauo.is_null() {
                continue;
            }
            // PTV 10.02.2003 to find component of assembly CORRECTLY
            let label = find_instance_label(&nauo, &tool, &tp, self.map)?;
            let shape = XCAFDocShapeTool::shape_of(&label)?;
            if !shape.is_null() {
                return Ok(ComponentShape::Found(Some(shape)));
            }
        }
        Ok(ComponentShape::Found(None))
    }
    /// The colours and visibility on the label of a shape, or on its children when it has none.
    fn apply_to_shape(
        &self,
        shape: &Shape,
        colors: &DecodedColors,
        has_color: bool,
        visible: bool,
    ) -> Result<(), Failure> {
        if !has_color && visible {
            return Ok(());
        }
        let shape_tool = step_live(self.shape_tool, WHERE)?;
        let color_tool = step_live(self.color_tool, WHERE)?;
        let mut label = TdfLabel::default();
        let found = shape_tool.search_using_map(shape, &mut label, false, true)?;
        if has_color {
            if found {
                set_label_colors(color_tool, &label, colors)?;
            } else {
                let mut children = ShapeIterator::new(shape, true, true);
                while children.more() {
                    let mut child = TdfLabel::default();
                    if shape_tool.search_using_map(children.value()?, &mut child, false, true)? {
                        set_label_colors(color_tool, &child, colors)?;
                    }
                    children.next();
                }
            }
        }
        // sets the invisibility for shape.
        if !visible && found {
            color_tool.set_visibility(&label, false)?;
        }
        Ok(())
    }
    /// `set_style`: the style of parts and instances.
    fn set_style(&self, style: &Handle<StepVisualStyledItem>) -> Result<(), Failure> {
        let Some(live) = style.get() else {
            return Ok(());
        };
        let tr = step_live(self.ws, WHERE)?.transfer_reader();
        let tp = step_live(&tr, WHERE)?.transient_process();
        if let Some(overriding) = style.cast::<StepVisualOverRidingStyledItem>().get() {
            self.set_style(&overriding.over_ridden_style())?;
            let component = style.cast::<StepVisualContextDependentOverRidingStyledItem>();
            if component.is_some() {
                let color_tool = step_live(self.color_tool, WHERE)?;
                return set_assembly_component_style(
                    &tp,
                    color_tool,
                    self.styles,
                    &component,
                    self.factors,
                );
            }
        }
        // check the visibility of styled item.
        let visible = self.is_visible_style(style)?;
        let (mut surface, mut boundary, mut curve) =
            (Handle::default(), Handle::default(), Handle::default());
        // check if it is component style
        let mut is_component = false;
        let mut rendering = StepConstructRenderingProperties::new();
        if !self.styles.get_colors(
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
        // collect styled items
        let target = live.item_ap242()?;
        if target.is_null() {
            return Ok(());
        }
        let live_tp = step_live(&tp, WHERE)?;
        let index = live_tp.map_index(&target.value());
        let mut shape = if index > 0 {
            TransferBRep::shape_result(&live_tp.map_item(index)?)
        } else {
            Shape::default()
        };
        // take shape with real location.
        if is_component {
            match self.component_shape(live)? {
                ComponentShape::SkipShuo => return Ok(()),
                ComponentShape::Found(Some(found)) => shape = found,
                ComponentShape::Found(None) => {}
            }
        }
        if shape.is_null() {
            return Ok(());
        }
        let has_color =
            surface.is_some() || boundary.is_some() || curve.is_some() || rendering.is_defined();
        let colors = if has_color {
            decode_style_colors(&surface, &boundary, &curve, &rendering)?
        } else {
            DecodedColors::default()
        };
        self.apply_to_shape(&shape, &colors, has_color, visible)
    }
}

/// `IsOverriden`: for a root style, true only if it is overridden by other root style.
fn is_overridden(
    graph: &InterfaceGraph,
    style: &Handle<StepVisualStyledItem>,
    is_root: bool,
) -> Result<bool, Failure> {
    for sharing in graph.sharings(&style.cast())?.content() {
        let overriding = sharing.cast::<StepVisualOverRidingStyledItem>();
        let Some(overriding) = overriding.get() else {
            continue;
        };
        if !is_root {
            return Ok(true);
        }
        let item = overriding.item_ap242()?.value();
        if item.cast::<StepShapeShapeRepresentation>().is_some() {
            return Ok(true);
        }
    }
    Ok(false)
}

impl ReaderState {
    /// `read_colors`: the styles of the model on the labels.
    pub(in crate::stepcaf_control) fn read_colors(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let mut styles = StepConstructStyles::with_ws(ws)?;
        if !styles.load_styles()? {
            return Ok(false);
        }
        // searching for invisible items in the model
        let mut invisible = Handle::new(HSequence::<Handle<StandardTransient>>::default());
        styles.load_invis_styles(&mut invisible)?;
        let main = step_live(doc, WHERE)?.main()?;
        let color_tool = XCAFDocDocumentTool::color_tool(&main)?;
        let shape_tool = XCAFDocDocumentTool::shape_tool(&main)?;
        if color_tool.is_null() || shape_tool.is_null() {
            return Ok(false);
        }
        let context = StyleContext {
            ws,
            map: &self.map,
            color_tool: &color_tool,
            shape_tool: &shape_tool,
            styles: &styles,
            invisible: &invisible,
            factors,
        };
        let hgraph = styles.graph(false)?;
        // apply root styles earlier, as they can be overridden: IsOverriden for a root style returns
        // true only if it is overridden by other root style
        for i in 1..=styles.nb_root_styles() {
            let style = styles.root_style(i)?;
            // The guard is released first: a transfer of a style may read the graph again.
            let overridden = is_overridden(&step_live(&hgraph, WHERE)?.graph(), &style, true)?;
            if !overridden {
                context.set_style(&style)?;
            }
        }
        // apply leaf styles, they can override root styles
        for i in 1..=styles.nb_styles() {
            let style = styles.style(i)?;
            let overridden = is_overridden(&step_live(&hgraph, WHERE)?.graph(), &style, false)?;
            if !overridden {
                context.set_style(&style)?;
            }
        }
        step_live(&color_tool, WHERE)?.reverse_chains_of_tree_nodes()?;
        // some colors can be attached to assemblies, propagate them to components
        propagate_color_to_parts(
            &shape_tool,
            &color_tool,
            &step_live(&shape_tool, WHERE)?.base_label(),
        )?;
        Ok(true)
    }
}
