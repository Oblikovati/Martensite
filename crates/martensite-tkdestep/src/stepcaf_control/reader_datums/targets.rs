//! The datum targets read by `STEPCAFControlReader`.
use super::*;

impl ReaderState {
    /// `setDatumToXCAF`: the objects of a datum and its targets.
    fn set_datum_to_xcaf(
        &mut self,
        datum: &Handle<StepDimTolDatum>,
        gdt_label: &TdfLabel,
        modifiers: &DatumModifiers,
        doc: &Handle<TDocStdDocument>,
        ws: &Handle<XSControlWorkSession>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let main = step_live(doc, WHERE)?.main()?;
        let session = SessionGraph::of_process(ws)?;
        let tools = DatumTools {
            session: &session,
            shape_tool: XCAFDocDocumentTool::shape_tool(&main)?,
            dim_tol_tool: XCAFDocDocumentTool::dim_tol_tool(&main)?,
            factors,
        };
        let live_datum = step_live(datum, WHERE)?;
        let object = Handle::new(XCAFDimTolObjectsDatumObject::new());
        let live_object = step_live(&object, WHERE)?;
        // Collect all links to shapes
        let mut aspects: Vec<Handle<StepReprShapeAspect>> = Vec::new();
        for sharing in session.sharings(&datum.cast())? {
            let relationship = sharing.cast::<StepReprShapeAspectRelationship>();
            let Some(relationship) = relationship
                .get()
                .filter(|live| live.relating_shape_aspect().is_some())
            else {
                continue;
            };
            let relating = relationship.relating_shape_aspect();
            collect_shape_aspect(&relating, &session, &mut aspects)?;
            if relating.cast::<StepDimTolDatumTarget>().is_null() {
                read_annotation(&session, &relating.cast(), &object.cast(), factors)?;
            }
        }
        // Collect shape labels
        let mut labels = Vec::new();
        for aspect in &aspects {
            // Skip datum targets
            if aspect.is_null() || aspect.cast::<StepDimTolDatumTarget>().is_some() {
                continue;
            }
            // Process all connected GISU
            tools.labels_of_usages(&session.sharings(&aspect.cast())?, &mut labels)?;
        }
        // Process datum targets and create objects for them
        let mut has_target = false;
        for aspect in &aspects {
            let target = aspect.cast::<StepDimTolPlacedDatumTargetFeature>();
            if target.is_some() {
                has_target |=
                    self.set_datum_target(&tools, &target, live_datum, gdt_label, modifiers)?;
            }
        }
        if labels.is_empty() && has_target {
            return Ok(true);
        }
        // Create object for datum
        let dim_tol_tool = step_live(&tools.dim_tol_tool, WHERE)?;
        let label = dim_tol_tool.add_datum_with(
            &live_datum.name(),
            &live_datum.description(),
            &live_datum.identification(),
        )?;
        self.gdt_map.bind(datum.cast(), label.clone());
        // bind datum label with all reference datum_feature entities
        for aspect in &aspects {
            if aspect.is_some() && aspect.cast::<StepDimTolDatumTarget>().is_null() {
                self.gdt_map.bind(aspect.cast(), label.clone());
            }
        }
        dim_tol_tool.lock(&label)?;
        let attribute = XCAFDocDatum::set(&label)?;
        dim_tol_tool.set_datum_shapes(&labels, &label)?;
        live_object.set_name(&live_datum.identification());
        live_object.set_position(modifiers.position);
        if !modifiers.single.is_empty() {
            live_object.set_modifiers(&modifiers.single);
        }
        if modifiers.with_value != XCAFDimTolObjectsDatumModifWithValue::None {
            live_object.set_modifier_with_value(modifiers.with_value, modifiers.value);
        }
        dim_tol_tool.set_datum_to_geom_tol(&label, gdt_label)?;
        if live_object.get_presentation().is_null() {
            // Try find annotation connected to datum entity (not right case, according recommended
            // practices)
            read_annotation(&session, &datum.cast(), &object.cast(), factors)?;
        }
        step_live(&attribute, WHERE)?.set_object(&object)?;
        Ok(true)
    }
    /// The axis of a datum system on the tolerance.
    fn read_system_axis(
        usage: &StepAP242GeometricItemSpecificUsage,
        gdt_label: &TdfLabel,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let item = usage.identified_item_value(1)?;
        step_live(&item, WHERE)?;
        let axis = item.cast::<StepGeomAxis2Placement3d>();
        if axis.is_null() {
            return Ok(());
        }
        let mut tolerance = Handle::<XCAFDocGeomTolerance>::default();
        if !gdt_label.find_attribute_as(&XCAFDocGeomTolerance::get_id(), &mut tolerance)? {
            return Ok(());
        }
        let live = step_live(&tolerance, WHERE)?;
        let object = live.get_object()?;
        let placement = StepToGeom::make_axis2_placement(&axis, factors)?;
        if let Some(placement) = placement.get() {
            step_live(&object, WHERE)?.set_axis(&placement.ax2()?);
            live.set_object(&object)?;
        }
        Ok(())
    }
    /// The modifiers of an array into `modifiers`; a modifier with a value not in a named unit keeps
    /// the last value.
    fn read_reference_modifiers(
        array: &Handle<HArray1<StepDimTolDatumReferenceModifier>>,
        modifiers: &mut DatumModifiers,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let Some(live) = array.get() else {
            return Ok(());
        };
        let (lower, upper) = {
            let items = live.array1();
            (items.lower(), items.upper())
        };
        for rank in lower..=upper {
            let modifier = step_array1_item(array, rank, WHERE)?;
            let case = modifier.case_number();
            if case == 2 {
                let member = modifier.simple_datum_reference_modifier_member();
                let value = step_live(&member, WHERE)?.value() as i32;
                if let Some(single) = XCAFDimTolObjectsDatumSingleModif::from_i32(value) {
                    modifiers.single.push(single);
                }
            } else if case == 1 {
                let with_value = modifier.datum_reference_modifier_with_value();
                let with_value = step_live(&with_value, WHERE)?;
                if let Some(kind) = XCAFDimTolObjectsDatumModifWithValue::from_i32(
                    with_value.modifier_type() as i32 + 1,
                ) {
                    modifiers.with_value = kind;
                }
                let measure = with_value.modifier_value();
                let measure = step_live(&measure, WHERE)?;
                if let Some(named) = named_unit_of(&measure.unit_component()) {
                    modifiers.value = measure.value_component()
                        * context_of_unit(&named, factors)?.length_factor();
                }
            }
        }
        Ok(())
    }
    /// One compartment of a datum system: its datums, with their modifiers.
    fn read_compartment(
        &mut self,
        compartment: &Handle<crate::step_dim_tol::StepDimTolDatumReferenceCompartment>,
        position: i32,
        gdt_label: &TdfLabel,
        doc: &Handle<TDocStdDocument>,
        ws: &Handle<XSControlWorkSession>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let session = SessionGraph::of_process(ws)?;
        let live = step_live(compartment, WHERE)?;
        // get modifiers
        let mut modifiers = DatumModifiers {
            position,
            ..DatumModifiers::default()
        };
        Self::read_reference_modifiers(&live.modifiers(), &mut modifiers, factors)?;
        for shared in session.shareds(&compartment.cast())? {
            let datum = shared.cast::<StepDimTolDatum>();
            if datum.is_some() {
                self.set_datum_to_xcaf(&datum, gdt_label, &modifiers, doc, ws, factors)?;
                continue;
            }
            let element = shared.cast::<StepDimTolDatumReferenceElement>();
            let Some(element) = element.get() else {
                continue;
            };
            // get modifiers from group of datums: they add up for the next elements
            Self::read_reference_modifiers(&element.modifiers(), &mut modifiers, factors)?;
            for second in session.shareds(&shared)? {
                let datum = second.cast::<StepDimTolDatum>();
                if datum.is_some() {
                    self.set_datum_to_xcaf(&datum, gdt_label, &modifiers, doc, ws, factors)?;
                }
            }
        }
        Ok(())
    }
    /// `readDatumsAP242`: the datums of a tolerance through its datum systems.
    pub(in crate::stepcaf_control) fn read_datums_ap242(
        &mut self,
        entity: &Handle<StandardTransient>,
        gdt_label: &TdfLabel,
        doc: &Handle<TDocStdDocument>,
        ws: &Handle<XSControlWorkSession>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let session = SessionGraph::of_process(ws)?;
        for shared in session.shareds(entity)? {
            step_live(&shared, WHERE)?;
            if !shared.is_kind(StepDimTolDatumSystem::static_type()) {
                continue;
            }
            let system = shared.cast::<StepDimTolDatumSystem>();
            let system = step_live(&system, WHERE)?;
            for sharing in session.sharings(&shared)? {
                // get axis
                if let Some(usage) = sharing.cast::<StepAP242GeometricItemSpecificUsage>().get() {
                    Self::read_system_axis(usage, gdt_label, factors)?;
                }
            }
            if system.nb_constituents() <= 0 {
                continue;
            }
            // get datum feature and datum target from datum system
            let compartments = system.constituents();
            let Some(live) = compartments.get() else {
                continue;
            };
            let (lower, upper) = {
                let items = live.array1();
                (items.lower(), items.upper())
            };
            // position on frame
            let mut position = 0;
            for rank in lower..=upper {
                position += 1;
                self.read_compartment(
                    &step_array1_item(&compartments, rank, WHERE)?,
                    position,
                    gdt_label,
                    doc,
                    ws,
                    factors,
                )?;
            }
        }
        Ok(true)
    }
}
