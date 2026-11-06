//! The zone and entity of a geometric tolerance for `STEPCAFControlWriter`.
use super::*;

/// A length with its unit and the given value.
fn length_measure(
    value: f64,
    unit: &StepBasicUnit,
) -> Result<Handle<StepBasicLengthMeasureWithUnit>, Failure> {
    let member = Handle::new(StepBasicMeasureValueMember::new());
    member.set_name("LENGTH_MEASURE");
    member.set_real(value);
    let measure = Handle::new(StepBasicLengthMeasureWithUnit::new());
    measure.init(&member, unit);
    Ok(measure)
}

/// `writeToleranceZone`.
fn write_tolerance_zone(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsGeomToleranceObject,
    entity: &Handle<StepDimTolGeometricTolerance>,
    context: &Handle<StepReprRepresentationContext>,
) -> Result<(), Failure> {
    // Get working data
    let model = model_of(ws)?;
    let Some(live) = entity.get() else {
        return Ok(());
    };
    // Return if there is no tolerance zones
    if object.get_type_of_value() == XCAFDimTolObjectsGeomToleranceTypeValue::None
        && object.get_zone_modifier() != XCAFDimTolObjectsGeomToleranceZoneModif::Runout
    {
        return Ok(());
    }
    // Create Tolerance_Zone
    let form = Handle::new(StepDimTolToleranceZoneForm::new());
    model.add_with_refs(&form.cast(), 0, false)?;
    form.init(&STEPCAFControlGDTProperty::get_tol_value_type_name(
        object.get_type_of_value(),
    ));
    let targets = step_new_array1::<StepDimTolToleranceZoneTarget>(1, 1)?;
    let mut target = StepDimTolToleranceZoneTarget::default();
    target.set_value(&entity.cast())?;
    step_array1_set(&targets, 1, target, WHERE)?;
    let zone = Handle::new(StepDimTolToleranceZone::new());
    let aspect = live.toleranced_shape_aspect().shape_aspect();
    zone.init(
        &empty(),
        &empty(),
        &step_live(&aspect, WHERE)?.of_shape(),
        StepDataLogical::LFalse,
        &targets,
        &form,
    );
    model.add_with_refs(&zone.cast(), 0, false)?;
    // Runout_Tolerance_Zone
    let angle = Handle::new(StepBasicPlaneAngleMeasureWithUnit::new());
    let member = Handle::new(StepBasicMeasureValueMember::new());
    member.set_name("PLANE_ANGLE_MEASURE");
    member.set_real(object.get_value_of_zone_modifier());
    angle.init(&member, &get_unit(context, true)?);
    let orientation = Handle::new(StepDimTolRunoutZoneOrientation::new());
    orientation.init(&angle);
    let definition = Handle::new(StepDimTolRunoutZoneDefinition::new());
    definition.init(&zone, &Handle::default(), &orientation);
    model.add_with_refs(&definition.cast(), 0, false)?;
    model.add_with_refs(&orientation.cast(), 0, false)?;
    model.add_with_refs(&angle.cast(), 0, false)
}

/// The modifiers of a tolerance, without all-around and all-over; null without any.
fn tolerance_modifiers(
    object: &XCAFDimTolObjectsGeomToleranceObject,
) -> Result<Handle<HArray1<StepDimTolGeometricToleranceModifier>>, Failure> {
    let simple: Vec<_> = object
        .get_modifiers()
        .into_iter()
        .filter(|modifier| {
            !matches!(
                modifier,
                XCAFDimTolObjectsGeomToleranceModif::AllAround
                    | XCAFDimTolObjectsGeomToleranceModif::AllOver
            )
        })
        .collect();
    let material = object.get_material_requirement_modifier();
    let has_material = material != XCAFDimTolObjectsGeomToleranceMatReqModif::None;
    let count = simple.len() as i32 + i32::from(has_material);
    if count <= 0 {
        return Ok(Handle::default());
    }
    let array = step_new_array1::<StepDimTolGeometricToleranceModifier>(1, count)?;
    for (index, modifier) in simple.iter().enumerate() {
        let value = STEPCAFControlGDTProperty::get_geom_tolerance_modifier(*modifier);
        step_array1_set(&array, index as i32 + 1, value, WHERE)?;
    }
    if material == XCAFDimTolObjectsGeomToleranceMatReqModif::L {
        let value = StepDimTolGeometricToleranceModifier::GTMLeastMaterialRequirement;
        step_array1_set(&array, count, value, WHERE)?;
    } else if material == XCAFDimTolObjectsGeomToleranceMatReqModif::M {
        let value = StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement;
        step_array1_set(&array, count, value, WHERE)?;
    }
    Ok(array)
}

/// The subtype of Geometric_Tolerance that the modifiers and the datums ask for.
fn make_geometric_tolerance(
    object: &XCAFDimTolObjectsGeomToleranceObject,
    measure: &Handle<StepBasicLengthMeasureWithUnit>,
    target: &StepDimTolGeometricToleranceTarget,
    datum_system: &Handle<HArray1<StepDimTolDatumSystemOrReference>>,
    modifiers: &Handle<HArray1<StepDimTolGeometricToleranceModifier>>,
    max_measure: &Handle<StepBasicLengthMeasureWithUnit>,
) -> Result<Handle<StepDimTolGeometricTolerance>, Failure> {
    let (name, description) = (empty(), empty());
    let with_datums = Handle::new(StepDimTolGeometricToleranceWithDatumReference::new());
    with_datums.set_datum_system_fields(datum_system)?;
    let with_modifiers = Handle::new(StepDimTolGeometricToleranceWithModifiers::new());
    with_modifiers.set_modifiers(modifiers);
    let kind = STEPCAFControlGDTProperty::get_step_tolerance_type(object.get_type());
    let measure_shared: Handle<StandardTransient> = measure.cast();
    Ok(
        match (
            modifiers.is_some(),
            max_measure.is_some(),
            datum_system.is_some(),
        ) {
            (true, true, true) => {
                // Geometric_Tolerance & Geometric_Tolerance_With_Datum_Reference &
                // Geometric_Tolerance_With_Maximum_Tolerance & Geometric_Tolerance_With_Modifiers
                let result =
                    Handle::new(StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol::new());
                result.init_geometric_tolerance_target(
                    &name,
                    &description,
                    &measure_shared,
                    target,
                    &with_datums,
                    &with_modifiers,
                    max_measure,
                    kind,
                );
                result.cast()
            }
            (true, true, false) => {
                // Geometric_Tolerance & Geometric_Tolerance_With_Maximum_Tolerance &
                // Geometric_Tolerance_With_Modifiers
                let result = Handle::new(StepDimTolGeoTolAndGeoTolWthMaxTol::new());
                result.init_geometric_tolerance_target(
                    &name,
                    &description,
                    &measure_shared,
                    target,
                    &with_modifiers,
                    max_measure,
                    kind,
                );
                result.cast()
            }
            (true, false, true) => {
                // Geometric_Tolerance & Geometric_Tolerance_With_Datum_Reference &
                // Geometric_Tolerance_With_Modifiers
                let result = Handle::new(StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod::new());
                result.init_geometric_tolerance_target(
                    &name,
                    &description,
                    &measure_shared,
                    target,
                    &with_datums,
                    &with_modifiers,
                    kind,
                );
                result.cast()
            }
            (true, false, false) => {
                // Geometric_Tolerance & Geometric_Tolerance_With_Modifiers
                let result = Handle::new(StepDimTolGeoTolAndGeoTolWthMod::new());
                result.init_geometric_tolerance_target(
                    &name,
                    &description,
                    &measure_shared,
                    target,
                    &with_modifiers,
                    kind,
                );
                result.cast()
            }
            (false, _, true) => {
                // Geometric_Tolerance & Geometric_Tolerance_With_Datum_Reference
                let result = Handle::new(StepDimTolGeoTolAndGeoTolWthDatRef::new());
                result.init_geometric_tolerance_target(
                    &name,
                    &description,
                    &measure_shared,
                    target,
                    &with_datums,
                    kind,
                );
                result.cast()
            }
            (false, _, false) => {
                // Geometric_Tolerance
                let result = STEPCAFControlGDTProperty::get_geom_tolerance(object.get_type());
                if let Some(live) = result.get() {
                    live.init_geometric_tolerance_target(
                        &name,
                        &description,
                        &measure_shared,
                        target,
                    );
                }
                result
            }
        },
    )
}

impl WriterState {
    /// The target of a tolerance: the shape aspect of one shape, or a composite of those of several.
    /// With several shapes and none of them written, null.
    fn tolerance_target_aspect(
        &self,
        ws: &Handle<XSControlWorkSession>,
        shape_labels: &[TdfLabel],
        tolerance_label: &TdfLabel,
    ) -> Result<Handle<StepReprShapeAspect>, Failure> {
        let model = model_of(ws)?;
        let (mut context, mut usage) = (Handle::default(), Handle::default());
        let shape_of = |label: &TdfLabel| -> Result<Shape, Failure> {
            let mut shape = Shape::default();
            XCAFDocShapeTool::get_shape(label, &mut shape)?;
            Ok(shape)
        };
        if let [single] = shape_labels {
            let aspect = self.write_shape_aspect(
                ws,
                tolerance_label,
                &shape_of(single)?,
                &mut context,
                &mut usage,
            )?;
            model.add_with_refs(&aspect.cast(), 0, false)?;
            return Ok(aspect);
        }
        let mut composite: Handle<StepReprCompositeShapeAspect> = Handle::default();
        for label in shape_labels {
            let aspect = self.write_shape_aspect(
                ws,
                tolerance_label,
                &shape_of(label)?,
                &mut context,
                &mut usage,
            )?;
            let Some(live) = aspect.get() else {
                continue;
            };
            if composite.is_null() {
                composite = Handle::new(StepReprCompositeShapeAspect::new());
                step_live(&composite, WHERE)?.init(
                    &live.name(),
                    &live.description(),
                    &live.of_shape(),
                    live.product_definitional(),
                );
                model.add_with_refs(&composite.cast(), 0, false)?;
            }
            relate_aspects(&model, &composite.cast(), &aspect, Handle::default())?;
        }
        Ok(composite.cast())
    }
    /// `writeGeomTolerance`.
    pub(in crate::stepcaf_control) fn write_geom_tolerance(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        shape_labels: &[TdfLabel],
        tolerance_label: &TdfLabel,
        datum_system: &Handle<HArray1<StepDimTolDatumSystemOrReference>>,
        context: &Handle<StepReprRepresentationContext>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        // Get working data
        let model = model_of(ws)?;
        let mut attribute: Handle<XCAFDocGeomTolerance> = Handle::default();
        if !tolerance_label.find_attribute_as(&XCAFDocGeomTolerance::get_id(), &mut attribute)? {
            return Ok(());
        }
        let object = step_live(&attribute, WHERE)?.get_object()?;
        let Some(live) = object.get() else {
            return Ok(());
        };
        // Value
        let unit = get_unit(context, false)?;
        let measure = length_measure(live.get_value(), &unit)?;
        model.add_with_refs(&measure.cast(), 0, false)?;
        // Geometric_Tolerance target
        let main_aspect = self.tolerance_target_aspect(ws, shape_labels, tolerance_label)?;
        let mut target = StepDimTolGeometricToleranceTarget::default();
        target.set_value(&main_aspect.cast())?;
        // Modifiers
        let modifiers = tolerance_modifiers(live)?;
        // Modifier with value
        let mut max_measure = Handle::default();
        if modifiers.is_some() && live.get_max_value_modifier() != 0.0 {
            max_measure = length_measure(live.get_max_value_modifier(), &unit)?;
            model.add_with_refs(&max_measure.cast(), 0, false)?;
        }
        // Init and write necessary subtype of Geometric_Tolerance entity
        let tolerance = make_geometric_tolerance(
            live,
            &measure,
            &target,
            datum_system,
            &modifiers,
            &max_measure,
        )?;
        model.add_with_refs(&tolerance.cast(), 0, false)?;
        write_tolerance_zone(ws, live, &tolerance, context)?;
        // Annotation plane and Presentation
        let presentation = GdtPresentation {
            shape: live.get_presentation(),
            name: live.get_presentation_name(),
            has_semantic: true,
            has_plane: live.has_plane(),
            plane: live.get_plane(),
            text_position: live.get_point_text_attach(),
        };
        self.write_presentation(ws, &presentation, &tolerance.cast(), factors)
    }
}
