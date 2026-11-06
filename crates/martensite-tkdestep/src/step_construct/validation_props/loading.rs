//! Loading and reading the validation properties of `StepConstructValidationProps`.
use super::*;

impl StepConstructValidationProps {
    /// `find_target`: the STEP entity validation props of a shape are assigned to, and its context.
    /// With `instance`, nothing is found: that search is disabled.
    pub fn find_target(
        &self,
        shape: &Shape,
        target: &mut StepReprCharacterizedDefinition,
        context: &mut Handle<StepReprRepresentationContext>,
        instance: bool,
    ) -> Result<bool, Failure> {
        // find the target STEP entity corresponding to a shape
        let mapper: Handle<dyn martensite_tkxsbase::transfer::TransferFinder> =
            TransferBRep::shape_mapper(self.finder_process(), shape)?.cast();
        let binder = step_live(self.finder_process(), WHERE)?.find(&mapper);
        if !instance {
            if shape.shape_type()? == ShapeEnum::Compound {
                self.target_of_compound(&mapper, target, context)?;
            } else {
                self.target_of_subshape(&mapper, &binder, target, context)?;
            }
        }
        // if target not found and shape has location, try the same shape without location
        Ok(!target.is_null())
    }
    /// `add_prop(target, Context, Prop, Descr)`: the structure of 5 entities linking a shape and its
    /// property; true on success.
    pub fn add_prop_to_target(
        &self,
        target: &StepReprCharacterizedDefinition,
        context: &Handle<StepReprRepresentationContext>,
        prop: &Handle<StepReprRepresentationItem>,
        descr: &str,
    ) -> Result<bool, Failure> {
        let propdef = Handle::new(StepReprPropertyDefinition::new());
        propdef.init(
            &text("geometric validation property"),
            true,
            &text(descr),
            target,
        );
        let items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, 1)?;
        step_array1_set(&items, 1, prop.clone(), WHERE)?;
        let rep = Handle::new(StepReprRepresentation::new());
        rep.init(&text(descr), &items, context);
        let pdr = Handle::new(StepReprPropertyDefinitionRepresentation::new());
        let mut represented = StepReprRepresentedDefinition::default();
        represented.set_value(&propdef.cast())?;
        pdr.init(&represented, &rep);
        // record SDR in order to have it written to the file
        let model = self.model()?;
        model.add_with_refs(&pdr.cast(), 0, false)?;
        let step_model = model.cast::<StepDataStepModel>();
        // for AP203, add subschema name
        let schema = step_live(&step_model, WHERE)?
            .internal_parameters()
            .write_schema
            .as_raw();
        if schema == 3 {
            let mut header =
                APIHeaderSectionMakeHeader::from_model(step_live(&step_model, WHERE)?)?;
            header.add_schema_identifier(&text("GEOMETRIC_VALIDATION_PROPERTIES_MIM"))?;
        }
        Ok(true)
    }
    /// `add_prop(Shape, Prop, Descr, instance)`: the same on the target of a shape already mapped.
    pub fn add_prop(
        &self,
        shape: &Shape,
        prop: &Handle<StepReprRepresentationItem>,
        descr: &str,
        instance: bool,
    ) -> Result<bool, Failure> {
        let mut target = StepReprCharacterizedDefinition::default();
        let mut context = Handle::default();
        if !self.find_target(shape, &mut target, &mut context, instance)? {
            return Ok(false);
        }
        self.add_prop_to_target(&target, &context, prop, descr)
    }
    /// A measure item of `value` with the given member name, unit and item name.
    fn measure_item(
        value: f64,
        member_name: &str,
        unit: &StepBasicUnit,
        item_name: &str,
    ) -> Handle<StepReprRepresentationItem> {
        let val = Handle::new(StepBasicMeasureValueMember::new());
        val.set_real(value);
        val.set_name(member_name);
        let mri = Handle::new(StepReprMeasureRepresentationItem::new());
        mri.init(&text(item_name), &val, unit);
        mri.cast()
    }
    /// `add_volume`: true on success.
    pub fn add_volume(&mut self, shape: &Shape, vol: f64) -> Result<bool, Failure> {
        // for volume unit, either take existing or create a new
        if self.vol_unit.derived_unit().is_null() {
            self.vol_unit
                .set_value(&millimetre_power_unit(3.)?.cast())?;
        }
        let mri = Self::measure_item(vol, "VOLUME_MEASURE", &self.vol_unit, "volume measure");
        self.add_prop(shape, &mri, "volume", false)
    }
    /// `add_area`: true on success.
    pub fn add_area(&mut self, shape: &Shape, area: f64) -> Result<bool, Failure> {
        // for area unit, either take existing or create a new
        if self.area_unit.derived_unit().is_null() {
            self.area_unit
                .set_value(&millimetre_power_unit(2.)?.cast())?;
        }
        let mri = Self::measure_item(
            area,
            "AREA_MEASURE",
            &self.area_unit,
            "surface area measure",
        );
        self.add_prop(shape, &mri, "surface area", false)
    }
    /// `add_centroid`: with `instance`, the centroid goes to an instance of a component in assembly.
    pub fn add_centroid(
        &self,
        shape: &Shape,
        pnt: &Point3,
        instance: bool,
    ) -> Result<bool, Failure> {
        let point = Handle::new(StepGeomCartesianPoint::new());
        point.init_3d(&text("centre point"), pnt.x(), pnt.y(), pnt.z());
        self.add_prop(shape, &point.cast(), "centroid", instance)
    }
    /// `load_props`: fills `seq` with the validation PropertyDefinitionRepresentations of the model.
    pub fn load_props(&self, seq: &mut Vec<Handle<StandardTransient>>) -> Result<bool, Failure> {
        let model = self.model()?;
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            let Some(pdr) = entity
                .cast::<StepReprPropertyDefinitionRepresentation>()
                .get()
                .map(|pdr| pdr.definition().property_definition())
            else {
                continue;
            };
            // Check that PDR is for validation props.
            if pdr.get().is_some_and(|pd| !is_validation_name(&pd.name())) {
                continue;
            }
            seq.push(entity);
        }
        Ok(!seq.is_empty())
    }
    /// `get_prop_pd`: the product definition of a property definition, null if not found.
    pub fn get_prop_pd(
        &self,
        pd: &Handle<StepReprPropertyDefinition>,
    ) -> Result<Handle<StepBasicProductDefinition>, Failure> {
        let cd = step_live(pd, WHERE)?.definition();
        let hgraph = self.hgraph()?;
        let hgraph = step_live(&hgraph, WHERE)?;
        let graph = hgraph.graph();
        // detect target entity of valprop
        let pds = cd.product_definition_shape();
        if pds.is_some() {
            return Ok(first_of(&graph.shareds(&pds.cast())?.content()));
        }
        let shape_aspect = cd.shape_aspect();
        if shape_aspect.is_null() {
            return Ok(Handle::default());
        }
        let mut prod_def = Handle::default();
        for sharing in graph.sharings(&shape_aspect.cast())?.content() {
            let property = sharing.cast::<StepReprPropertyDefinition>();
            if property.is_null() {
                return Ok(prod_def);
            }
            if prod_def.is_null() {
                prod_def = first_of(&graph.shareds(&property.cast())?.content());
            }
        }
        Ok(prod_def)
    }
    /// `get_prop_nauo`: the NAUO of a property definition, null if not found.
    pub fn get_prop_nauo(
        &self,
        pd: &Handle<StepReprPropertyDefinition>,
    ) -> Result<Handle<StepReprNextAssemblyUsageOccurrence>, Failure> {
        let pds = step_live(pd, WHERE)?
            .definition()
            .product_definition_shape();
        if pds.is_null() {
            return Ok(Handle::default()); // not found
        }
        let hgraph = self.hgraph()?;
        let hgraph = step_live(&hgraph, WHERE)?;
        let shareds = hgraph.graph().shareds(&pds.cast())?;
        Ok(first_of(&shareds.content()))
    }
    /// `get_prop_shape(ProdDef)`: the shape of a product definition, null if not found.
    pub fn get_prop_shape(
        &self,
        prod_def: &Handle<StepBasicProductDefinition>,
    ) -> Result<Shape, Failure> {
        // find target shape
        let binder = step_live(self.transient_process(), WHERE)?.find(&prod_def.cast());
        match binder.get() {
            Some(live) if live.has_result() => Ok(TransferBRep::shape_result(&binder)),
            _ => Ok(Shape::default()),
        }
    }
    /// `get_prop_shape(PD)`: the shape of a property definition, null if not found.
    pub fn get_prop_shape_of_definition(
        &self,
        pd: &Handle<StepReprPropertyDefinition>,
    ) -> Result<Shape, Failure> {
        let prod_def = self.get_prop_pd(pd)?;
        if prod_def.is_null() {
            return Ok(Shape::default());
        }
        self.get_prop_shape(&prod_def)
    }
    /// The scale of a measure unit to the units of the session.
    fn unit_scale(unit: &StepBasicUnit, local_factors: &StepDataFactors) -> Result<f64, Failure> {
        let mut scale = 1.;
        if let Some(derived) = unit.derived_unit().get() {
            for ind in 1..=derived.nb_elements()? {
                let element = derived.elements_value(ind)?;
                let element = step_live(&element, WHERE)?;
                let mut context = StepConstructUnitContext::new();
                context.compute_unit_factors(&element.unit(), local_factors)?;
                if context.length_done() {
                    scale *= context.length_factor().powf(element.exponent());
                }
            }
            return Ok(scale);
        }
        let named = unit.named_unit();
        if named.is_some() {
            let mut context = StepConstructUnitContext::new();
            context.compute_unit_factors(&named, local_factors)?;
            if context.area_done() {
                scale = context.area_factor();
            }
            if context.volume_done() {
                scale = context.volume_factor();
            }
        }
        Ok(scale)
    }
    /// `get_prop_real`: the value of an area or a volume; false for another property. `is_area` tells
    /// which it is. `val` is written before the kind is known.
    pub fn get_prop_real(
        &self,
        item: &Handle<StepReprRepresentationItem>,
        val: &mut f64,
        is_area: &mut bool,
        local_factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        // decode volume & area
        step_live(item, WHERE)?;
        let mri = item.cast::<StepReprMeasureRepresentationItem>();
        let Some(mri) = mri.get() else {
            return Ok(false);
        };
        let measure = mri.measure();
        let measure = step_live(&measure, WHERE)?;
        let member = measure.value_component_member();
        let name = step_live(&member, WHERE)?.name();
        let scale = Self::unit_scale(&measure.unit_component(), local_factors)?;
        *val = measure.value_component() * scale;
        match name.as_str() {
            "AREA_MEASURE" => *is_area = true,
            "VOLUME_MEASURE" => *is_area = false,
            _ => return Ok(false),
        }
        Ok(true)
    }
    /// The unit assignment of a context, null when it has none.
    fn global_unit_context(
        context: &Handle<StepReprRepresentationContext>,
    ) -> Handle<StepReprGlobalUnitAssignedContext> {
        if let Some(both) = context
            .cast::<StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext>()
            .get()
        {
            return both.global_unit_assigned_context();
        }
        context
            .cast::<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx>()
            .get()
            .map(StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::global_unit_assigned_context)
            .unwrap_or_default()
    }
    /// `get_prop_pnt`: the value of a centroid; false for another property.
    pub fn get_prop_pnt(
        &self,
        item: &Handle<StepReprRepresentationItem>,
        context: &Handle<StepReprRepresentationContext>,
        pnt: &mut Point3,
        local_factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        // centroid
        step_live(item, WHERE)?;
        let point = item.cast::<StepGeomCartesianPoint>();
        let Some(point) = point.get() else {
            return Ok(false);
        };
        if point.nb_coordinates() != 3 {
            return Ok(false);
        }
        let mut pos = Point3::new(
            point.coordinates_value(1)?,
            point.coordinates_value(2)?,
            point.coordinates_value(3)?,
        );
        // scale according to units
        let units = Self::global_unit_context(context);
        if units.is_some() {
            let mut unit_tool = StepConstructUnitContext::new();
            unit_tool.compute_factors(&units, local_factors)?;
            pos = pos.scaled(Point3::new(0., 0., 0.), unit_tool.length_factor());
        }
        *pnt = pos;
        Ok(true)
    }
    /// `set_assembly_shape`: the current assembly shape (for FindCDSR calls).
    pub fn set_assembly_shape(&mut self, shape: &Shape) -> Result<(), Failure> {
        let mapper = TransferBRep::shape_mapper(self.finder_process(), shape)?;
        let process = step_live(self.tool.finder_process(), WHERE)?;
        process.find_typed_transient_as(
            &mapper.cast(),
            StepBasicProductDefinition::static_type(),
            &mut self.my_assembly_pd,
        );
        Ok(())
    }
}

/// The name of a validation property definition, tolerating underscores: per "Recommended Practices
/// for Geometric and Assembly Validation Properties" Release 4.4 as of August 17, 2016, item 4.6, the
/// name has words separated by spaces, but older versions of the same document used underscores.
/// A null name counts as valid.
fn is_validation_name(name: &Handle<HAsciiString>) -> bool {
    let Some(name) = name.get() else {
        return true;
    };
    let lowered: Vec<u8> = name
        .string()
        .as_bytes()
        .iter()
        .map(|byte| match byte {
            b'_' => b' ',
            other => other.to_ascii_lowercase(),
        })
        .collect();
    lowered == b"geometric validation property"
}
