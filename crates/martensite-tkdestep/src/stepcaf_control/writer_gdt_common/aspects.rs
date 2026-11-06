//! The shape aspects and presentations of GD\&T for `STEPCAFControlWriter`.
use super::*;

/// A measure of a name, a value and a unit.
pub(in crate::stepcaf_control) fn measure_with_unit(
    value: f64,
    unit: &StepBasicUnit,
    measure_name: &str,
) -> Result<Handle<StepBasicMeasureWithUnit>, Failure> {
    let member = Handle::new(StepBasicMeasureValueMember::new());
    member.set_name(measure_name);
    member.set_real(value);
    let measure = Handle::new(StepBasicMeasureWithUnit::new());
    measure.init(&member, unit);
    Ok(measure)
}

/// `create_dim_value`: a value, without qualifiers.
pub(in crate::stepcaf_control) fn create_dim_value(
    value: f64,
    unit: &StepBasicUnit,
    name: &Handle<HAsciiString>,
    measure_name: &str,
    is_angle: bool,
) -> Result<Handle<StepReprReprItemAndMeasureWithUnit>, Failure> {
    let item = Handle::new(StepReprRepresentationItem::new());
    item.init(&Handle::new(HAsciiString::copied(step_live(name, WHERE)?)));
    let measure = measure_with_unit(value, unit, measure_name)?;
    Ok(if is_angle {
        // Angle & without qualifiers
        let result = Handle::new(StepReprReprItemAndPlaneAngleMeasureWithUnit::new());
        result.init(&measure, &item)?;
        result.cast()
    } else {
        // Length & without qualifiers
        let result = Handle::new(StepReprReprItemAndLengthMeasureWithUnit::new());
        result.init(&measure, &item)?;
        result.cast()
    })
}

/// `create_dim_value`: a value with qualifiers.
pub(in crate::stepcaf_control) fn create_qualified_dim_value(
    value: f64,
    unit: &StepBasicUnit,
    name: &Handle<HAsciiString>,
    measure_name: &str,
    is_angle: bool,
    qualified: &Handle<StepShapeQualifiedRepresentationItem>,
) -> Result<Handle<StepReprReprItemAndMeasureWithUnit>, Failure> {
    let item = Handle::new(StepReprRepresentationItem::new());
    item.init(&Handle::new(HAsciiString::copied(step_live(name, WHERE)?)));
    let measure = measure_with_unit(value, unit, measure_name)?;
    Ok(if is_angle {
        // Angle & with qualifiers
        let result = Handle::new(StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI::new());
        result.init(&measure, &item, qualified)?;
        result.cast()
    } else {
        // Length & with qualifiers
        let result = Handle::new(StepReprReprItemAndLengthMeasureWithUnitAndQRI::new());
        result.init(&measure, &item, qualified)?;
        result.cast()
    })
}

/// The name of a shape aspect: what follows the first space of the name of the label.
fn shape_aspect_name(label: &TdfLabel) -> Result<Handle<HAsciiString>, Failure> {
    let mut attribute: Handle<TDataStdName> = Handle::default();
    if !label.find_attribute_as(&TDataStdName::get_id(), &mut attribute)? {
        return Ok(empty());
    }
    let name = AsciiString::from_extended(&step_live(&attribute, WHERE)?.get(), None);
    let bytes = name.as_bytes();
    Ok(match bytes.iter().position(|byte| *byte == b' ') {
        Some(space) => Handle::new(HAsciiString::new(&bytes[space + 1..])),
        None => empty(),
    })
}

/// The array holding one representation item.
pub(in crate::stepcaf_control) fn one_item(
    item: &Handle<StepReprRepresentationItem>,
) -> Result<Handle<HArray1<Handle<StepReprRepresentationItem>>>, Failure> {
    let items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, 1)?;
    step_array1_set(&items, 1, item.clone(), WHERE)?;
    Ok(items)
}

/// The arguments of `writePresentation` that describe it.
pub(in crate::stepcaf_control) struct GdtPresentation {
    pub(in crate::stepcaf_control) shape: Shape,
    pub(in crate::stepcaf_control) name: Handle<HAsciiString>,
    pub(in crate::stepcaf_control) has_semantic: bool,
    pub(in crate::stepcaf_control) has_plane: bool,
    pub(in crate::stepcaf_control) plane: Axis2,
    pub(in crate::stepcaf_control) text_position: Point3,
}

impl WriterState {
    /// `writeShapeAspect`: the shape aspect of a shape, its usage in `gisu`, its context in `context`.
    pub(in crate::stepcaf_control) fn write_shape_aspect(
        &self,
        ws: &Handle<XSControlWorkSession>,
        label: &TdfLabel,
        shape: &Shape,
        context: &mut Handle<StepReprRepresentationContext>,
        gisu: &mut Handle<StepAP242GeometricItemSpecificUsage>,
    ) -> Result<Handle<StepReprShapeAspect>, Failure> {
        // Get working data
        let model = model_of(ws)?;
        let fp = finder_process_of(ws)?;
        let graph = graph_of(ws)?;
        if graph.is_null() {
            return Ok(Handle::default());
        }
        let mut location = Location::default();
        let mut items = Vec::new();
        find_entities(&fp, shape, &mut location, &mut items)?;
        let Some(entity) = items.first() else {
            return Ok(Handle::default());
        };
        let mut found_context = Handle::default();
        let pds = find_pds(&graph, entity, &mut found_context)?;
        if pds.is_null() {
            return Ok(Handle::default());
        }
        *context = found_context;
        // Shape_Aspect
        let name = shape_aspect_name(label)?;
        let description = empty();
        let aspect = Handle::new(StepReprShapeAspect::new());
        aspect.init(&name, &description, &pds, StepDataLogical::LTrue);
        // Geometric_Item_Specific_Usage
        let usage = Handle::new(StepAP242GeometricItemSpecificUsage::new());
        let mut definition = StepAP242ItemIdentifiedRepresentationUsageDefinition::default();
        definition.set_value(&aspect.cast())?;
        let identified = one_item(&entity.cast())?;
        let sdr = sharing_of_type::<_, StepShapeShapeDefinitionRepresentation>(&graph, &pds)?;
        let Some(sdr) = sdr.first() else {
            return Ok(Handle::default());
        };
        // Set entities to model
        let used = step_live(sdr, WHERE)?.used_representation();
        usage.init(&name, &description, &definition, &used, &identified);
        model.add_with_refs(&aspect.cast(), 0, false)?;
        model.add_with_refs(&usage.cast(), 0, false)?;
        *gisu = usage;
        Ok(aspect)
    }
    /// `writePresentation`: the tessellated presentation of a GD&T object and its annotation plane.
    pub(in crate::stepcaf_control) fn write_presentation(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        presentation: &GdtPresentation,
        dimension: &Handle<StandardTransient>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        if presentation.shape.is_null() {
            return Ok(());
        }
        // Get working data
        let model = model_of(ws)?;
        // Presentation
        let geom_set = STEPCAFControlGDTProperty::get_tessellation(&presentation.shape)?;
        if geom_set.is_null() {
            return Ok(());
        }
        let occurrence = Handle::new(StepVisualTessellatedAnnotationOccurrence::new());
        occurrence.init(&empty(), &self.gdt_prs_curve_style, &geom_set.cast());
        let mut element = StepVisualDraughtingCalloutElement::default();
        element.set_value(&occurrence.cast())?;
        let occurrences = step_new_array1::<StepVisualDraughtingCalloutElement>(1, 1)?;
        step_array1_set(&occurrences, 1, element, WHERE)?;
        let callout = Handle::new(StepVisualDraughtingCallout::new());
        let name = if presentation.name.is_null() {
            empty()
        } else {
            presentation.name.clone()
        };
        callout.init(&name, &occurrences);
        let callouts = one_item(&callout.cast())?;
        self.gdt_annotations.push(callout.cast());
        let mut definition = StepAP242ItemIdentifiedRepresentationUsageDefinition::default();
        definition.set_value(dimension)?;
        let link_name = if presentation.has_semantic {
            text("PMI representation to presentation link")
        } else {
            empty()
        };
        let association = Handle::new(StepAP242DraughtingModelItemAssociation::new());
        association.init(
            &link_name,
            &empty(),
            &definition,
            &self.gdt_presentation_dm.cast(),
            &callouts,
        );
        model.add_with_refs(&association.cast(), 0, false)?;
        if presentation.has_plane {
            self.write_annotation_plane(ws, presentation, &callout, factors)?;
        }
        Ok(())
    }
    /// The annotation plane of a presentation: at the position of the text.
    fn write_annotation_plane(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        presentation: &GdtPresentation,
        callout: &Handle<StepVisualDraughtingCallout>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        // Presentation Style
        let null_style = Handle::new(StepVisualNullStyleMember::new());
        null_style.set_enum_text(0, b".NULL.");
        let mut style_item = StepVisualPresentationStyleSelect::default();
        style_item.set_value(&null_style.cast())?;
        let styles = step_new_array1::<StepVisualPresentationStyleSelect>(1, 1)?;
        step_array1_set(&styles, 1, style_item, WHERE)?;
        let style = Handle::new(StepVisualPresentationStyleAssignment::new());
        style.init(&styles);
        let plane_styles = step_new_array1::<Handle<StepVisualPresentationStyleAssignment>>(1, 1)?;
        step_array1_set(&plane_styles, 1, style, WHERE)?;
        // Plane
        let plane = Handle::new(StepGeomPlane::new());
        let axis =
            GeomToStepMakeAxis2Placement3d::from_ax2(&presentation.plane, factors)?.value()?;
        // Set text position to plane origin
        let text_position = Handle::new(StepGeomCartesianPoint::new());
        let coordinates = step_new_array1::<f64>(1, 3)?;
        for index in 1..=3 {
            let value = presentation.text_position.xyz().coord(index)?;
            step_array1_set(&coordinates, index, value, WHERE)?;
        }
        text_position.init(&empty(), &coordinates)?;
        step_live(&axis, WHERE)?.set_location(&text_position);
        plane.init(&empty(), &axis);
        // Annotation plane element
        let mut plane_element = StepVisualAnnotationPlaneElement::default();
        plane_element.set_value(&callout.cast())?;
        let elements = step_new_array1::<StepVisualAnnotationPlaneElement>(1, 1)?;
        step_array1_set(&elements, 1, plane_element, WHERE)?;
        // Init AnnotationPlane entity
        let annotation_plane = Handle::new(StepVisualAnnotationPlane::new());
        annotation_plane.init(&empty(), &plane_styles, &plane.cast(), &elements);
        self.gdt_annotations.push(annotation_plane.cast());
        model_of(ws)?.add_with_refs(&annotation_plane.cast(), 0, false)
    }
}
