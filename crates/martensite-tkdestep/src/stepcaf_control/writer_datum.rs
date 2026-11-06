//! `STEPCAFControlWriter::writeDatumAP242`: a datum, with the shapes of its features or its targets.
use super::STEPCAFControlGDTProperty;
use super::writer::{WHERE, WriterState};
use super::writer_gdt_common::{
    GdtPresentation, create_dim_value, empty, find_pds, get_unit, graph_of, sharings_of, text,
};
use super::writer_support::{find_entities, finder_process_of, model_of};
use crate::geom_to_step::GeomToStepMakeAxis2Placement3d;
use crate::step_ap242::{
    StepAP242GeometricItemSpecificUsage, StepAP242ItemIdentifiedRepresentationUsageDefinition,
};
use crate::step_data::{StepDataFactors, StepDataLogical, StepDataSelectType};
use crate::step_dim_tol::{
    StepDimTolDatum, StepDimTolDatumFeature, StepDimTolDatumTarget,
    StepDimTolPlacedDatumTargetFeature,
};
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_repr::{
    StepReprCharacterizedDefinition, StepReprCompShAspAndDatumFeatAndShAsp,
    StepReprCompositeShapeAspect, StepReprFeatureForDatumTargetRelationship,
    StepReprProductDefinitionShape, StepReprPropertyDefinition, StepReprRepresentationContext,
    StepReprRepresentationItem, StepReprRepresentedDefinition, StepReprShapeAspect,
    StepReprShapeAspectRelationship,
};
use crate::step_shape::{
    StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentationWithParameters,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDatumObject, XCAFDimTolObjectsDatumTargetType,
};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDatum, XCAFDocShapeTool};
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceModelEntities};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// A relationship without description between two shape aspects; `no_name` keeps the null.
pub(super) fn relate_aspects(
    model: &Handle<dyn InterfaceInterfaceModel>,
    relating: &Handle<StepReprShapeAspect>,
    related: &Handle<StepReprShapeAspect>,
    description: Handle<HAsciiString>,
) -> Result<(), Failure> {
    let relationship = Handle::new(StepReprShapeAspectRelationship::new());
    relationship.init(&empty(), false, &description, relating, related);
    model.add_with_refs(&relationship.cast(), 0, false)
}

/// What `writeDatumAP242` finds of the shapes of a datum.
#[derive(Default)]
struct DatumFeatures {
    aspects: Vec<Handle<StepReprShapeAspect>>,
    pds: Handle<StepReprProductDefinitionShape>,
    context: Handle<StepReprRepresentationContext>,
    usage: Handle<StepAP242GeometricItemSpecificUsage>,
    aspect_number: i32,
    usage_number: i32,
}

impl WriterState {
    /// Link with datum feature: a shape aspect for each shape of the datum.
    fn datum_features(
        &self,
        ws: &Handle<XSControlWorkSession>,
        shape_labels: &[TdfLabel],
        datum_label: &TdfLabel,
    ) -> Result<DatumFeatures, Failure> {
        let model = model_of(ws)?;
        let fp = finder_process_of(ws)?;
        let graph = graph_of(ws)?;
        let mut features = DatumFeatures::default();
        for shape_label in shape_labels {
            let mut shape = Shape::default();
            XCAFDocShapeTool::get_shape(shape_label, &mut shape)?;
            let mut location = Location::default();
            let mut items = Vec::new();
            find_entities(&fp, &shape, &mut location, &mut items)?;
            let Some(entity) = items.first() else {
                continue;
            };
            features.pds = find_pds(&graph, entity, &mut features.context)?;
            if features.pds.is_null() {
                continue;
            }
            let aspect = self.write_shape_aspect(
                ws,
                datum_label,
                &shape,
                &mut features.context,
                &mut features.usage,
            )?;
            if aspect.is_null() {
                continue;
            }
            features.aspect_number = model.number(&aspect.cast());
            features.usage_number = model.number(&features.usage.cast());
            features.aspects.push(aspect);
        }
        if features.pds.is_null() {
            // Workaround for datums without shape
            features.pds = self.gdt_common_pds.clone();
            for sharing in sharings_of(&graph, &features.pds)? {
                let sdr: Handle<StepShapeShapeDefinitionRepresentation> = sharing.cast();
                let Some(sdr) = sdr.get() else {
                    continue;
                };
                if let Some(used) = sdr.used_representation().get() {
                    features.context = used.context_of_items();
                }
                break;
            }
        }
        Ok(features)
    }
    /// `writeDatumAP242`: the datum, or the given datum for a target after the first of a name.
    pub(super) fn write_datum_ap242(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        shape_labels: &[TdfLabel],
        datum_label: &TdfLabel,
        is_first_target: bool,
        written_datum: &Handle<StepDimTolDatum>,
        factors: &StepDataFactors,
    ) -> Result<Handle<StepDimTolDatum>, Failure> {
        let model = model_of(ws)?;
        if graph_of(ws)?.is_null() {
            return Ok(Handle::default());
        }
        let mut features = self.datum_features(ws, shape_labels, datum_label)?;
        // Find if datum has datum targets and get common datum attributes
        let mut attribute: Handle<XCAFDocDatum> = Handle::default();
        if !datum_label.find_attribute_as(&XCAFDocDatum::get_id(), &mut attribute)? {
            return Ok(Handle::default());
        }
        let object = step_live(&attribute, WHERE)?.get_object()?;
        let Some(live) = object.get() else {
            return Ok(Handle::default());
        };
        // If datum type is area, but there is no area in object, write as simple datum
        let is_simple = !live.is_datum_target()
            || (live.get_datum_target_type() == XCAFDimTolObjectsDatumTargetType::Area
                && live.get_datum_target().is_null());
        let aspect = if is_simple {
            write_simple_datum(&model, &mut features)?
        } else {
            self.write_datum_targets(ws, datum_label, live, &mut features, factors)?
        };
        // Datum
        let mut datum = written_datum.clone();
        if is_first_target {
            datum = Handle::new(StepDimTolDatum::new());
            step_live(&datum, WHERE)?.init(
                &empty(),
                &empty(),
                &features.pds,
                StepDataLogical::LFalse,
                &live.get_name(),
            );
            model.add_with_refs(&datum.cast(), 0, false)?;
        }
        // Shape_Aspect_Relationship
        if aspect.is_some() {
            relate_aspects(&model, &aspect, &datum.cast(), Handle::default())?;
        }
        // Annotation plane and Presentation
        let presentation = GdtPresentation {
            shape: live.get_presentation(),
            name: live.get_presentation_name(),
            has_semantic: true,
            has_plane: live.has_plane(),
            plane: live.get_plane(),
            text_position: live.get_point_text_attach(),
        };
        self.write_presentation(ws, &presentation, &aspect.cast(), factors)?;
        Ok(datum)
    }
    /// Datum with datum targets.
    fn write_datum_targets(
        &self,
        ws: &Handle<XSControlWorkSession>,
        datum_label: &TdfLabel,
        object: &XCAFDimTolObjectsDatumObject,
        features: &mut DatumFeatures,
        factors: &StepDataFactors,
    ) -> Result<Handle<StepReprShapeAspect>, Failure> {
        let model = model_of(ws)?;
        let target_id = match object.get_datum_target_number() {
            0 => empty(),
            number => Handle::new(HAsciiString::from_integer(number)),
        };
        let kind = object.get_datum_target_type();
        let mut target: Handle<StepDimTolDatumTarget> = Handle::default();
        // Note: the given way to write such datum type may be incorrect (too little information)
        if kind == XCAFDimTolObjectsDatumTargetType::Area {
            self.write_area_target(ws, datum_label, object, &target_id, features)?;
        } else {
            let placed = Handle::new(StepDimTolPlacedDatumTargetFeature::new());
            placed.init(
                &empty(),
                &STEPCAFControlGDTProperty::get_datum_target_name(kind),
                &features.pds,
                StepDataLogical::LTrue,
                &target_id,
            );
            model.add_with_refs(&placed.cast(), 0, false)?;
            target = placed.cast();
            if object.has_datum_target_params() {
                write_target_parameters(ws, object, &placed, features, factors)?;
            }
        }
        // Link datum target to datum feature
        // if aSASeq.Length() == 0 nothing to do
        link_target_to_features(&model, features, &target)?;
        Ok(target.cast())
    }
    /// The target of an area: it replaces the shape aspect of the area, and it is not the
    /// target returned.
    fn write_area_target(
        &self,
        ws: &Handle<XSControlWorkSession>,
        datum_label: &TdfLabel,
        object: &XCAFDimTolObjectsDatumObject,
        target_id: &Handle<HAsciiString>,
        features: &mut DatumFeatures,
    ) -> Result<(), Failure> {
        let model = model_of(ws)?;
        let mut usage = Handle::default();
        let area = self.write_shape_aspect(
            ws,
            datum_label,
            &object.get_datum_target(),
            &mut features.context,
            &mut usage,
        )?;
        let aspect_number = model.number(&area.cast());
        let usage_number = model.number(&usage.cast());
        let target = Handle::new(StepDimTolDatumTarget::new());
        target.init(
            &empty(),
            &text("area"),
            &step_live(&area, WHERE)?.of_shape(),
            StepDataLogical::LTrue,
            target_id,
        );
        model.replace_entity(aspect_number, &target.cast())?;
        let mut definition = StepAP242ItemIdentifiedRepresentationUsageDefinition::default();
        definition.set_value(&target.cast())?;
        step_live(&usage, WHERE)?.set_definition(&definition);
        model.replace_entity(usage_number, &usage.cast())
    }
}

/// Simple datum: the feature of no shape, of one shape, or a composite of several.
fn write_simple_datum(
    model: &Handle<dyn InterfaceInterfaceModel>,
    features: &mut DatumFeatures,
) -> Result<Handle<StepReprShapeAspect>, Failure> {
    match features.aspects.len() {
        0 => {
            // Create empty datum with name and presentation only
            let feature = Handle::new(StepDimTolDatumFeature::new());
            feature.init(&empty(), &empty(), &features.pds, StepDataLogical::LTrue);
            model.add_with_refs(&feature.cast(), 0, false)?;
            Ok(feature.cast())
        }
        1 => {
            let feature = Handle::new(StepDimTolDatumFeature::new());
            feature.init(&empty(), &empty(), &features.pds, StepDataLogical::LTrue);
            model.replace_entity(features.aspect_number, &feature.cast())?;
            let mut definition = StepAP242ItemIdentifiedRepresentationUsageDefinition::default();
            definition.set_value(&feature.cast())?;
            step_live(&features.usage, WHERE)?.set_definition(&definition);
            model.replace_entity(features.usage_number, &features.usage.cast())?;
            Ok(feature.cast())
        }
        _ => {
            let feature = Handle::new(StepReprCompShAspAndDatumFeatAndShAsp::new());
            feature.init(&empty(), &empty(), &features.pds, StepDataLogical::LTrue);
            for aspect in &features.aspects {
                relate_aspects(model, &feature.cast(), aspect, empty())?;
            }
            model.add_with_refs(&feature.cast(), 0, false)?;
            Ok(feature.cast())
        }
    }
}

/// All parameters of a datum target.
fn write_target_parameters(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsDatumObject,
    placed: &Handle<StepDimTolPlacedDatumTargetFeature>,
    features: &DatumFeatures,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let model = model_of(ws)?;
    let kind = object.get_datum_target_type();
    // Datum targets
    let property = Handle::new(StepReprPropertyDefinition::new());
    let mut characterized = StepReprCharacterizedDefinition::default();
    characterized.set_value(&placed.cast())?;
    property.init(&empty(), false, &Handle::default(), &characterized);
    // write all parameters of datum target
    let representation = Handle::new(StepShapeShapeRepresentationWithParameters::new());
    // Common for all datum targets
    let unit = get_unit(&features.context, false)?;
    let placement =
        GeomToStepMakeAxis2Placement3d::from_ax2(&object.get_datum_target_axis(), factors)?
            .value()?;
    step_live(&placement, WHERE)?.set_name(&text("orientation"));
    // Process each datum target type
    let count = match kind {
        XCAFDimTolObjectsDatumTargetType::Point => 1,
        XCAFDimTolObjectsDatumTargetType::Rectangle => 3,
        _ => 2,
    };
    let items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, count)?;
    if kind != XCAFDimTolObjectsDatumTargetType::Point {
        if kind == XCAFDimTolObjectsDatumTargetType::Rectangle {
            // Additional value
            let width = create_dim_value(
                object.get_datum_target_width(),
                &unit,
                &text("target width"),
                "POSITIVE_LENGTH_MEASURE",
                false,
            )?;
            step_array1_set(&items, 2, width.cast(), WHERE)?;
            model.add_with_refs(&width.cast(), 0, false)?;
        }
        let name = if kind == XCAFDimTolObjectsDatumTargetType::Circle {
            "target diameter"
        } else {
            "target length"
        };
        // Value
        let value = create_dim_value(
            object.get_datum_target_length(),
            &unit,
            &text(name),
            "POSITIVE_LENGTH_MEASURE",
            false,
        )?;
        step_array1_set(&items, 1, value.cast(), WHERE)?;
        model.add_with_refs(&value.cast(), 0, false)?;
    }
    step_array1_set(&items, count, placement.cast(), WHERE)?;
    representation.init(&empty(), &items, &features.context);
    // Create and write auxiliary entities
    let sdr = Handle::new(StepShapeShapeDefinitionRepresentation::new());
    let mut represented = StepReprRepresentedDefinition::default();
    represented.set_value(&property.cast())?;
    sdr.init(&represented, &representation.cast());
    model.add_with_refs(&property.cast(), 0, false)?;
    model.add_with_refs(&representation.cast(), 0, false)?;
    model.add_with_refs(&sdr.cast(), 0, false)
}

/// The relationship of the features of a datum to its target.
fn link_target_to_features(
    model: &Handle<dyn InterfaceInterfaceModel>,
    features: &DatumFeatures,
    target: &Handle<StepDimTolDatumTarget>,
) -> Result<(), Failure> {
    let relating: Handle<StepReprShapeAspect> = match features.aspects.as_slice() {
        [] => return Ok(()),
        [single] => single.clone(),
        several => {
            let composite = Handle::new(StepReprCompositeShapeAspect::new());
            composite.init(
                &empty(),
                &empty(),
                &features.pds,
                step_live(&several[0], WHERE)?.product_definitional(),
            );
            for aspect in several {
                relate_aspects(model, &composite.cast(), aspect, empty())?;
            }
            composite.cast()
        }
    };
    let relationship = Handle::new(StepReprFeatureForDatumTargetRelationship::new());
    relationship.init(
        &empty(),
        false,
        &Handle::default(),
        &relating,
        &target.cast(),
    );
    model.add_with_refs(&relationship.cast(), 0, false)
}
