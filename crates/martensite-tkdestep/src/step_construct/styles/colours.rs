//! The colours and PSAs of `StepConstructStyles`.
use super::*;

impl StepConstructStyles {
    /// `CreateNAUOSRD`: a shape representation and its SDR for the styles of a NAUO, added to the
    /// model. `init_pds` must be null when used for NAUO colors, and is given only for SHUO.
    pub fn create_nauo_srd(
        &self,
        context: &Handle<StepReprRepresentationContext>,
        cdsr: &Handle<StepShapeContextDependentShapeRepresentation>,
        init_pds: &Handle<StepReprProductDefinitionShape>,
    ) -> Result<bool, Failure> {
        let live_cdsr = step_live(cdsr, WHERE)?;
        // get PDS
        let pds = if init_pds.is_null() {
            live_cdsr.represented_product_relation()
        } else {
            init_pds.clone()
        };
        let relation = live_cdsr
            .representation_relation()
            .cast::<StepReprRepresentationRelationshipWithTransformation>();
        let Some(relation) = relation.get() else {
            return Ok(false);
        };
        // take Item defined transformation
        let defined = relation
            .transformation_operator()
            .item_defined_transformation();
        // element for axis 2 placement
        let elems = single::<Handle<StepReprRepresentationItem>>(
            step_live(&defined, WHERE)?.transform_item2(),
        )?;
        // init Shape representation.
        let sr = Handle::new(StepShapeShapeRepresentation::new());
        sr.init(&text(""), &elems, context);
        // register reference between PresentationStyleByContext and ShapeRepresentation
        for psa in &self.my_psa {
            if let Some(by_context) = psa.cast::<StepVisualPresentationStyleByContext>().get() {
                let mut style_context = StepVisualStyleContextSelect::default();
                style_context.set_value(&sr.cast())?;
                by_context.set_style_context(&style_context);
            }
        }
        let mut represented = StepReprRepresentedDefinition::default();
        represented.set_value(&pds.cast())?;
        let sdr = Handle::new(StepShapeShapeDefinitionRepresentation::new());
        sdr.init(&represented, &sr.cast());
        self.tool.model()?.add_with_refs(&sdr.cast(), 0, false)?;
        Ok(true)
    }
    /// `find_context`: the representation context a shape is defined in, null when not found. It can
    /// be used in a call to `create_mdgpr`.
    pub fn find_context(
        &self,
        shape: &Shape,
    ) -> Result<Handle<StepReprRepresentationContext>, Failure> {
        // find context of items
        let mapper = TransferBRep::shape_mapper(self.finder_process(), shape)?.cast();
        let process = step_live(self.finder_process(), WHERE)?;
        let mut sr = Handle::<StepShapeShapeRepresentation>::default();
        if process.find_typed_transient_as(
            &mapper,
            StepShapeShapeRepresentation::static_type(),
            &mut sr,
        ) {
            return Ok(step_live(&sr, WHERE)?.context_of_items());
        }
        let mut item = Handle::<StepGeomGeometricRepresentationItem>::default();
        if !process.find_typed_transient_as(
            &mapper,
            StepGeomGeometricRepresentationItem::static_type(),
            &mut item,
        ) {
            return Ok(Handle::default());
        }
        let hgraph = self.tool.graph(false)?;
        let sharings = step_live(&hgraph, WHERE)?.graph().sharings(&item.cast())?;
        for sharing in sharings.content() {
            if let Some(sr) = sharing.cast::<StepShapeShapeRepresentation>().get() {
                let context = sr.context_of_items();
                if context.is_some() {
                    return Ok(context);
                }
            }
        }
        Ok(Handle::default())
    }
    /// Whether a style is on a whole shape representation, which makes it a root style.
    fn styles_representation(style: &StepVisualStyledItem) -> Result<bool, Failure> {
        let item = style.item_ap242()?.value();
        Ok(item.get().is_some() && item.cast::<StepShapeShapeRepresentation>().is_some())
    }
    /// The styled items of one MDGPR or DM.
    fn load_container(
        &mut self,
        container: &Handle<StepReprRepresentation>,
    ) -> Result<(), Failure> {
        let container = step_live(container, WHERE)?;
        for j in 1..=container.nb_items() {
            let style = container.items_value(j)?.cast::<StepVisualStyledItem>();
            let Some(live) = style.get() else {
                continue;
            };
            if Self::styles_representation(live)? {
                self.my_root_styles.add(style.cast());
            } else {
                self.my_styles.add(style.cast());
            }
        }
        Ok(())
    }
    /// `load_styles`: fills the styles from the MDGPR and DM entities of the model and its styled
    /// items; true when any style was found.
    pub fn load_styles(&mut self) -> Result<bool, Failure> {
        self.clear_styles();
        // find all MDGPRs and DMs and collect all defined styles in myStyles
        let model = self.tool.model()?;
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            if is_exactly::<StepVisualMechanicalDesignGeometricPresentationRepresentation>(&entity)
                || is_exactly::<StepVisualDraughtingModel>(&entity)
            {
                self.load_container(&entity.cast())?;
            } else if let Some(style) = entity.cast::<StepVisualStyledItem>().get() {
                if Self::styles_representation(style)? && !self.my_root_styles.contains(&entity) {
                    self.my_root_styles.add(entity);
                } else if !self.my_styles.contains(&entity) {
                    self.my_styles.add(entity);
                }
            }
        }
        Ok(!self.my_styles.is_empty())
    }
    /// `load_invis_styles`: fills a sequence, created when null, with the styled items of the INVISIBILITY
    /// entities; true when it holds any.
    pub fn load_invis_styles(
        &self,
        inv_styles: &mut Handle<HSequence<Handle<StandardTransient>>>,
    ) -> Result<bool, Failure> {
        let model = self.tool.model()?;
        // search for invisibility
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            if !is_exactly::<StepVisualInvisibility>(&entity) {
                continue;
            }
            // search for styled items
            let container = entity.cast::<StepVisualInvisibility>();
            let container = step_live(&container, WHERE)?;
            for j in 1..=container.nb_invisible_items()? {
                let style = container.invisible_items_value(j)?.styled_item();
                if style.is_null() {
                    continue;
                }
                // collect the invisible styled items
                if inv_styles.is_null() {
                    *inv_styles = Handle::new(HSequence::default());
                }
                step_live(inv_styles, WHERE)?.append(style.cast());
            }
        }
        Ok(inv_styles
            .get()
            .is_some_and(|sequence| sequence.sequence(|items| items.length() > 0)))
    }
    /// The surface part of `make_color_psa`.
    fn surface_style_usage(
        surf_col: &Handle<StepVisualColour>,
        rendering_props: &StepConstructRenderingProperties,
    ) -> Result<Handle<StepVisualSurfaceStyleUsage>, Failure> {
        let colour = Handle::new(StepVisualFillAreaStyleColour::new());
        colour.init(&text(""), surf_col);
        let mut fill_select = StepVisualFillStyleSelect::default();
        fill_select.set_value(&colour.cast())?;
        let fill_area_style = Handle::new(StepVisualFillAreaStyle::new());
        fill_area_style.init(&text(""), &single(fill_select)?);
        let fill_area = Handle::new(StepVisualSurfaceStyleFillArea::new());
        fill_area.init(&fill_area_style);
        let mut element = StepVisualSurfaceStyleElementSelect::default();
        element.set_value(&fill_area.cast())?;
        let rendering = rendering_props.create_rendering_properties(surf_col)?;
        let elements = step_new_array1::<StepVisualSurfaceStyleElementSelect>(
            1,
            1 + i32::from(rendering.is_some()),
        )?;
        if rendering.is_some() {
            let mut second = StepVisualSurfaceStyleElementSelect::default();
            second.set_value(&rendering.cast())?;
            step_array1_set(&elements, 2, second, WHERE)?;
        }
        step_array1_set(&elements, 1, element, WHERE)?;
        let side_style = Handle::new(StepVisualSurfaceSideStyle::new());
        side_style.init(&text(""), &elements);
        let usage = Handle::new(StepVisualSurfaceStyleUsage::new());
        usage.init(StepVisualSurfaceSide::SsBoth, &side_style);
        Ok(usage)
    }
    /// The curve part of `make_color_psa`.
    fn curve_style(
        curve_col: &Handle<StepVisualColour>,
    ) -> Result<Handle<StepVisualCurveStyle>, Failure> {
        let font = Handle::new(StepVisualDraughtingPreDefinedCurveFont::new());
        font.init(&text("continuous"));
        let val = Handle::new(StepBasicMeasureValueMember::new());
        val.set_real(0.1);
        val.set_name("POSITIVE_LENGTH_MEASURE");
        let mut font_select = StepVisualCurveStyleFontSelect::default();
        font_select.set_value(&font.cast())?;
        let mut size = StepBasicSizeSelect::default();
        size.set_value(&val.cast())?;
        let style = Handle::new(StepVisualCurveStyle::new());
        style.init(&text(""), &font_select, &size, curve_col);
        Ok(style)
    }
    /// `make_color_psa`: a PSA defining two colors, for filling surfaces and curves; a
    /// PresentationStyleByContext when `is_for_nauo`. Null without any colour.
    pub fn make_color_psa(
        &self,
        _item: &Handle<StepReprRepresentationItem>,
        surf_col: &Handle<StepVisualColour>,
        curve_col: &Handle<StepVisualColour>,
        rendering_props: &StepConstructRenderingProperties,
        is_for_nauo: bool,
    ) -> Result<Handle<StepVisualPresentationStyleAssignment>, Failure> {
        let mut items: Vec<Handle<StandardTransient>> = Vec::new();
        if surf_col.is_some() {
            items.push(Self::surface_style_usage(surf_col, rendering_props)?.cast());
        }
        if curve_col.is_some() {
            items.push(Self::curve_style(curve_col)?.cast());
        }
        if items.is_empty() {
            return Ok(Handle::default());
        }
        // general part
        let selects = step_new_array1::<StepVisualPresentationStyleSelect>(1, items.len() as i32)?;
        for (index, item) in items.iter().enumerate() {
            let mut select = StepVisualPresentationStyleSelect::default();
            select.set_value(item)?;
            step_array1_set(&selects, index as i32 + 1, select, WHERE)?;
        }
        let psa: Handle<StepVisualPresentationStyleAssignment> = if is_for_nauo {
            Handle::new(StepVisualPresentationStyleByContext::new()).cast()
        } else {
            Handle::new(StepVisualPresentationStyleAssignment::new())
        };
        step_live(&psa, WHERE)?.init(&selects);
        Ok(psa)
    }
    /// `get_color_psa`: the PSA defining surface and curve colors as `col`: created, or taken from the
    /// map where all PSAs created here are remembered.
    pub fn get_color_psa(
        &mut self,
        item: &Handle<StepReprRepresentationItem>,
        col: &Handle<StepVisualColour>,
    ) -> Result<Handle<StepVisualPresentationStyleAssignment>, Failure> {
        // if this color already was processed, just use the same PSA, else create new and add it to map
        let key: Handle<StandardTransient> = col.cast();
        if let Some((_, psa)) = self
            .my_map_of_styles
            .iter()
            .find(|(known, _)| *known == key)
        {
            return Ok(psa.clone());
        }
        let psa = self.make_color_psa(
            item,
            col,
            col,
            &StepConstructRenderingProperties::new(),
            false,
        )?;
        self.my_map_of_styles.push((key, psa.clone()));
        Ok(psa)
    }
    /// `get_colors`: the color definitions of a style. Each is null when the style does not define it,
    /// or the last definition.
    pub fn get_colors(
        &self,
        style: &Handle<StepVisualStyledItem>,
        surface_colour: &mut Handle<StepVisualColour>,
        boundary_colour: &mut Handle<StepVisualColour>,
        curve_colour: &mut Handle<StepVisualColour>,
        rendering_props: &mut StepConstructRenderingProperties,
        is_component: &mut bool,
    ) -> Result<bool, Failure> {
        *surface_colour = Handle::default();
        *boundary_colour = Handle::default();
        *curve_colour = Handle::default();
        *rendering_props = StepConstructRenderingProperties::new();
        // parse on styles
        let style = step_live(style, WHERE)?;
        for psa_index in 1..=style.nb_styles() {
            let psa = style.styles_value(psa_index)?;
            let Some(psa) = psa.get().filter(|live| live.styles().is_some()) else {
                continue;
            };
            *is_component = true;
            for pss_index in 1..=psa.nb_styles()? {
                let select = psa.styles_value(pss_index)?;
                // A select is of only one of these types, so stop as soon as its type is processed.
                let _processed = process_as_surface_style_usage(
                    &select,
                    surface_colour,
                    boundary_colour,
                    rendering_props,
                )? || process_as_curve_style(&select, curve_colour);
            }
        }
        Ok(surface_colour.is_some()
            || boundary_colour.is_some()
            || curve_colour.is_some()
            || rendering_props.is_defined())
    }
}
