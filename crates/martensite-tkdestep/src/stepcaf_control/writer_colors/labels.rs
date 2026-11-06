//! The colours of a shape label for `STEPCAFControlWriter`.
use super::*;

impl StyleWalk<'_> {
    /// `MakeSTEPStyles`: the styles of a shape and its subshapes.
    fn make_styles(
        &mut self,
        shape: &Shape,
        over: &mut Handle<StepVisualStyledItem>,
        inherit: Option<&XCAFPrsStyle>,
        is_component: bool,
    ) -> Result<(), Failure> {
        // skip already processed shapes
        if !self.map.add(shape.clone()) {
            return Ok(());
        }
        // check if shape has its own style (r inherits from ancestor)
        let style = self.style_of(shape, inherit)?;
        // translate colors to STEP
        let (mut surf_color, mut curv_color) = (Handle::default(), Handle::default());
        let mut render_transp = 0.0;
        if style.is_set_color_surf() {
            let color = style.get_color_surf_rgba();
            render_transp = 1.0 - f64::from(color.alpha());
            surf_color = self.colours.encode(color.rgb())?;
        }
        if style.is_set_color_curv() {
            curv_color = self.colours.encode(&style.get_color_curv())?;
        }
        let mut has_own = surf_color.is_some() || curv_color.is_some() || !style.is_visible();
        // find target item and assign style to it
        let mut step_style = over.clone();
        // skip compounds, let subshapes inherit its colors
        if has_own && (shape.shape_type()? != ShapeEnum::Compound || is_component) {
            let mut location = Location::default();
            let mut items = Vec::new();
            let count = find_entities(
                self.styles.finder_process(),
                shape,
                &mut location,
                &mut items,
            )?;
            // Get overridden style gka 10.06.03
            if is_component && count > 0 {
                get_styled_item(
                    shape,
                    self.shape_tool,
                    self.styles,
                    over,
                    self.map_comp_mdgpr,
                )?;
            }
            for item in &items {
                step_style = self.style_item(
                    &item.cast(),
                    &style,
                    &mut surf_color,
                    &curv_color,
                    render_transp,
                    over,
                    is_component,
                )?;
                has_own = false;
            }
        }
        // iterate on subshapes (except vertices :)
        if shape.shape_type()? == ShapeEnum::Edge || is_component {
            return Ok(());
        }
        let mut children = ShapeIterator::new(shape, true, true);
        while children.more() {
            let child = children.value()?.clone();
            children.next();
            self.make_styles(&child, &mut step_style, has_own.then_some(&style), false)?;
        }
        Ok(())
    }
}

/// The settings on a label and its subshapes; the flag is false when the object on the label is
/// invisible.
fn collect_settings(label: &TdfLabel) -> Result<(Settings, bool), Failure> {
    let mut settings = Settings::default();
    let mut sequence = vec![label.clone()];
    XCAFDocShapeTool::get_sub_shapes(label, &mut sequence)?;
    let mut is_visible = true;
    for value in &sequence {
        let mut style = XCAFPrsStyle::default();
        // check for invisible status of object on label
        if value == label && !XCAFDocColorTool::is_visible(value)? {
            is_visible = false;
            style.set_visibility(false);
        }
        if let Some(color) = XCAFDocColorTool::get_label_color(value, XCAFDocColorType::ColorGen)? {
            style.set_color_curv(*color.rgb());
            style.set_color_surf_rgba(color);
        }
        if let Some(color) = XCAFDocColorTool::get_label_color(value, XCAFDocColorType::ColorSurf)?
        {
            style.set_color_surf_rgba(color);
        }
        if let Some(color) = XCAFDocColorTool::get_label_color(value, XCAFDocColorType::ColorCurv)?
        {
            style.set_color_curv(*color.rgb());
        }
        if !style.is_set_color_surf() {
            let material = XCAFDocVisMaterialTool::get_shape_material(value)?;
            if let Some(material) = material.get().filter(|m| !m.is_empty()) {
                // only color can be stored in STEP
                style.set_color_surf_rgba(material.base_color());
            }
        }
        if !style.is_set_color_curv() && !style.is_set_color_surf() && is_visible {
            continue;
        }
        let mut sub = Shape::default();
        XCAFDocShapeTool::get_shape(value, &mut sub)?;
        if settings.contains(&sub) {
            *settings.change_from_key(&sub)? = style;
        } else {
            settings.add(sub, style);
        }
    }
    Ok((settings, is_visible))
}

/// The items of an MDGPR with the styles appended.
fn append_styles(
    mdgpr: &Handle<StepVisualMechanicalDesignGeometricPresentationRepresentation>,
    styles: &StepConstructStyles,
) -> Result<(), Failure> {
    let live = step_live(mdgpr, WHERE)?;
    let old_items = live.items();
    let old_length = if old_items.is_null() {
        0
    } else {
        step_array1_len(&old_items)
    };
    let count = old_length + styles.nb_styles();
    if count == 0 {
        return Ok(());
    }
    let new_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, count)?;
    for index in 1..=old_length {
        step_array1_set(
            &new_items,
            index,
            step_array1_item(&old_items, index, WHERE)?,
            WHERE,
        )?;
    }
    for index in 1..=styles.nb_styles() {
        step_array1_set(
            &new_items,
            old_length + index,
            styles.style(index)?.cast(),
            WHERE,
        )?;
    }
    live.set_items(&new_items);
    Ok(())
}

impl WriterState {
    /// `writeColors`.
    pub(in crate::stepcaf_control) fn write_colors(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        let mut styles = StepConstructStyles::with_ws(ws)?;
        let mut colours = SharedColours::default();
        self.write_colors_of(ws, labels, &mut styles, &mut colours)?;
        Ok(true)
    }
    /// The `Styles`, DPDCs and ColRGBs are local to each call: the recursion for the components
    /// of an assembly gets its own.
    fn write_colors_of(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
        styles: &mut StepConstructStyles,
        colours: &mut SharedColours,
    ) -> Result<(), Failure> {
        // Iterate on requested shapes
        for label in labels {
            // Iterate on shapes in the document
            let tool = XCAFDocDocumentTool::shape_tool(label)?;
            // Skip assemblies: colors assigned to assemblies and their instances
            // are not supported (it is not clear how to encode that in STEP)
            if XCAFDocShapeTool::is_assembly(label)? {
                // PTV 22.01.2003 Write color for instances.
                let mut components = Vec::new();
                if XCAFDocShapeTool::get_components(label, &mut components, false)? {
                    self.write_colors(ws, &components)?;
                }
                continue;
            }
            styles.clear_styles();
            self.write_colors_of_label(ws, label, &tool, styles, colours)?;
        }
        Ok(())
    }
    /// The colours of one shape label.
    fn write_colors_of_label(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        label: &TdfLabel,
        tool: &Handle<XCAFDocShapeTool>,
        styles: &mut StepConstructStyles,
        colours: &mut SharedColours,
    ) -> Result<(), Failure> {
        // get a target shape and try to find corresponding context
        // (all the colors set under that label will be put into that context)
        let mut shape = Shape::default();
        if !XCAFDocShapeTool::get_shape(label, &mut shape)? {
            return Ok(());
        }
        let is_component =
            XCAFDocShapeTool::is_component(label)? || self.pure_ref_labels.contains_key(label);
        let mut top_shape = shape.clone();
        let mut context = styles.find_context(&shape)?;
        if is_component {
            let mut top_label = TdfLabel::default();
            step_live(tool, WHERE)?.find_shape(&shape, &mut top_label, false)?;
            if top_label.is_null() {
                return Ok(());
            }
            XCAFDocShapeTool::get_shape(&top_label, &mut top_shape)?;
            context = styles.find_context(&top_shape)?;
        }
        if context.is_null() {
            return Ok(());
        }
        // collect settings set on that label
        let (settings, is_visible) = collect_settings(label)?;
        if settings.extent() == 0 {
            return Ok(());
        }
        // iterate on subshapes and create STEP styles
        let mut over = Handle::default();
        StyleWalk {
            styles: &mut *styles,
            settings: &settings,
            map: MapOfShape::default(),
            map_comp_mdgpr: &self.map_comp_mdgpr,
            colours,
            shape_tool: tool,
            vis_material_mode: self.vis_mat_mode,
        }
        .make_styles(&shape, &mut over, None, is_component)?;
        if is_component {
            if !self.write_component_styles(ws, &shape, &top_shape, &context, styles)? {
                return Ok(());
            }
        } else {
            self.write_top_level_styles(ws, &top_shape, &context, styles)?;
        }
        if !is_visible {
            write_invisibility(ws, styles)?;
        }
        Ok(())
    }
    /// create MDGPR and record it in model
    fn write_top_level_styles(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        top_shape: &Shape,
        context: &Handle<StepReprRepresentationContext>,
        styles: &StepConstructStyles,
    ) -> Result<(), Failure> {
        let fp = finder_process_of(ws)?;
        let model: Handle<StepDataStepModel> = step_live(&fp, WHERE)?.model().cast();
        let mut mdgpr = Handle::default();
        styles.create_mdgpr(context, &mut mdgpr, &model)?;
        if mdgpr.is_some() {
            bind_mdgpr(&mut self.map_comp_mdgpr, top_shape, mdgpr.cast())?;
        }
        Ok(())
    }
    /// create SDR for the NAUO, and add the styles to the MDGPR of the component top-level shape.
    /// False when there is nothing to add: the caller then ends this label, as the `continue`.
    fn write_component_styles(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        shape: &Shape,
        top_shape: &Shape,
        context: &Handle<StepReprRepresentationContext>,
        styles: &StepConstructStyles,
    ) -> Result<bool, Failure> {
        let fp = finder_process_of(ws)?;
        let cdsr = typed_result_of::<StepShapeContextDependentShapeRepresentation>(&fp, shape)?;
        if cdsr.is_null() {
            return Ok(true);
        }
        let null_pds: Handle<StepReprProductDefinitionShape> = Handle::default(); // important to be NULL
        styles.create_nauo_srd(context, &cdsr, &null_pds)?;
        // search for MDGPR of the component top-level shape
        let mdgpr: Handle<StepVisualMechanicalDesignGeometricPresentationRepresentation> =
            if self.map_comp_mdgpr.contains(top_shape) {
                self.map_comp_mdgpr.find_from_key(top_shape)?.cast()
            } else {
                let mdgpr = Handle::new(
                    StepVisualMechanicalDesignGeometricPresentationRepresentation::new(),
                );
                let live = step_live(&mdgpr, WHERE)?;
                live.set_name(&Handle::new(HAsciiString::new("")));
                live.set_context_of_items(context);
                bind_mdgpr(&mut self.map_comp_mdgpr, top_shape, mdgpr.cast())?;
                mdgpr
            };
        let items = step_live(&mdgpr, WHERE)?.items();
        let old = if items.is_null() {
            0
        } else {
            step_array1_len(&items)
        };
        if old + styles.nb_styles() == 0 {
            return Ok(false);
        }
        append_styles(&mdgpr, styles)?;
        Ok(true)
    }
}

/// An invisibility item referring to each styled item.
fn write_invisibility(
    ws: &Handle<XSControlWorkSession>,
    styles: &StepConstructStyles,
) -> Result<(), Failure> {
    let invisibility = Handle::new(StepVisualInvisibility::new());
    let items = step_new_array1::<StepVisualInvisibleItem>(1, styles.nb_styles())?;
    // put all style item into the harray
    for index in 1..=styles.nb_styles() {
        let mut item = StepVisualInvisibleItem::default();
        item.set_value(&styles.style(index)?.cast())?;
        step_array1_set(&items, index, item, WHERE)?;
    }
    // set the invisibility of items
    step_live(&invisibility, WHERE)?.init(&items);
    step_live(ws, WHERE)?
        .model()
        .add_with_refs(&invisibility.cast(), 0, false)
}
