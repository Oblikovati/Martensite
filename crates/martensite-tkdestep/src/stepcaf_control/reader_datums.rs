//! The datums of AP242 in `STEPCAFControlReader`.
use super::reader::{ReaderState, WHERE};
use super::reader_gdt_common::{collect_shape_aspect, find_shape_index_for_dgt, shape_label_of};
use super::reader_pmi::{factors_of_representation, model_of, read_annotation};
use super::reader_support::{SessionGraph, context_of_unit, first_of, named_unit_of};
use crate::step_ap242::StepAP242GeometricItemSpecificUsage;
use crate::step_data::StepDataFactors;
use crate::step_data::StepDataSelectType;
use crate::step_dim_tol::{
    StepDimTolDatum, StepDimTolDatumReferenceElement, StepDimTolDatumReferenceModifier,
    StepDimTolDatumSystem, StepDimTolDatumTarget, StepDimTolPlacedDatumTargetFeature,
};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_geom::StepGeomAxis2Placement3d;
use crate::step_repr::{
    StepReprFeatureForDatumTargetRelationship, StepReprPropertyDefinition,
    StepReprReprItemAndLengthMeasureWithUnit, StepReprRepresentationItem, StepReprShapeAspect,
    StepReprShapeAspectRelationship,
};
use crate::step_shape::{
    StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentationWithParameters,
};
use crate::step_to_geom::StepToGeom;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::NativeEnum;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDatumModifWithValue, XCAFDimTolObjectsDatumObject,
    XCAFDimTolObjectsDatumSingleModif, XCAFDimTolObjectsDatumTargetType,
};
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocDatum, XCAFDocDimTolTool, XCAFDocDocumentTool, XCAFDocGeomTolerance, XCAFDocShapeTool,
};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

mod targets;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use targets::*;

/// What a datum takes from its compartment: position and modifiers.
#[derive(Clone, Default)]
struct DatumModifiers {
    position: i32,
    single: Vec<XCAFDimTolObjectsDatumSingleModif>,
    with_value: XCAFDimTolObjectsDatumModifWithValue,
    value: f64,
}

/// What the translations of a datum read from the document.
struct DatumTools<'a> {
    session: &'a SessionGraph,
    shape_tool: Handle<XCAFDocShapeTool>,
    dim_tol_tool: Handle<XCAFDocDimTolTool>,
    factors: &'a StepDataFactors,
}

impl DatumTools<'_> {
    /// The shapes of the GISU among `sharings`.
    fn labels_of_usages(
        &self,
        sharings: &[Handle<StandardTransient>],
        labels: &mut Vec<TdfLabel>,
    ) -> Result<bool, Failure> {
        let mut found = false;
        for sharing in sharings {
            if let Some(usage) = sharing.cast::<StepAP242GeometricItemSpecificUsage>().get() {
                for rank in 1..=usage.nb_identified_item() {
                    let label = shape_label_of(
                        &usage.identified_item_value(rank)?,
                        self.session,
                        &self.shape_tool,
                    )?;
                    if !label.is_null() {
                        labels.push(label);
                        found = true;
                    }
                }
            }
        }
        Ok(found)
    }
    /// One parameter item of a datum target: its axis, or its length or width.
    fn read_target_parameter(
        &self,
        item: &Handle<StepReprRepresentationItem>,
        parameters: &Handle<StepShapeShapeRepresentationWithParameters>,
        object: &XCAFDimTolObjectsDatumObject,
    ) -> Result<(), Failure> {
        if item.is_null() {
            return Ok(());
        }
        let axis = item.cast::<StepGeomAxis2Placement3d>();
        if axis.is_some() {
            let local = factors_of_representation(
                &parameters.cast(),
                &self.session.tp,
                &model_of(&self.session.tp)?,
                self.factors,
            )?;
            let placement = StepToGeom::make_axis2_placement(&axis, &local)?;
            object.set_datum_target_axis(&step_live(&placement, WHERE)?.ax2()?);
            return Ok(());
        }
        let length = item.cast::<StepReprReprItemAndLengthMeasureWithUnit>();
        let Some(length) = length.get() else {
            return Ok(());
        };
        let measure = length.get_measure_with_unit();
        let measure = step_live(&measure, WHERE)?;
        let unit = measure.unit_component();
        let named = if unit.is_null() {
            Handle::default()
        } else {
            unit.named_unit()
        };
        if named.is_null() {
            return Ok(());
        }
        let value =
            measure.value_component() * context_of_unit(&named, self.factors)?.length_factor();
        let name = length.name();
        let name = step_live(&name, WHERE)?.string();
        if name.as_bytes() == b"target length" || name.as_bytes() == b"target diameter" {
            object.set_datum_target_length(value);
        } else {
            object.set_datum_target_width(value);
        }
        Ok(())
    }
    /// The shape and parameters of a datum target; true when it is valid.
    fn read_target_definition(
        &self,
        target: &Handle<StepDimTolPlacedDatumTargetFeature>,
        kind: XCAFDimTolObjectsDatumTargetType,
        relationship: &Handle<StepReprFeatureForDatumTargetRelationship>,
        object: &XCAFDimTolObjectsDatumObject,
        mut valid: bool,
    ) -> Result<Option<bool>, Failure> {
        if kind == XCAFDimTolObjectsDatumTargetType::Area {
            // Area datum target
            let Some(relationship) = relationship.get() else {
                return Ok(None);
            };
            let sharings = self
                .session
                .sharings(&relationship.relating_shape_aspect().cast())?;
            let usage: Handle<StepAP242GeometricItemSpecificUsage> = first_of(&sharings);
            let item = usage
                .get()
                .filter(|live| live.nb_identified_item() > 0)
                .map(|live| live.identified_item_value(1))
                .transpose()?
                .unwrap_or_default();
            if item.is_null() {
                return Ok(None);
            }
            let index = find_shape_index_for_dgt(&item.cast(), self.session)?;
            if index > 0 {
                object.set_datum_target(&self.session.shape_at(index)?);
                valid = true;
            }
            return Ok(Some(valid));
        }
        // Point/line/rectangle/circle datum targets
        let definition: Handle<StepReprPropertyDefinition> =
            first_of(&self.session.sharings(&target.cast())?);
        if definition.is_null() {
            return Ok(Some(valid));
        }
        let sdr: Handle<StepShapeShapeDefinitionRepresentation> =
            first_of(&self.session.sharings(&definition.cast())?);
        let Some(sdr) = sdr.get() else {
            return Ok(Some(valid));
        };
        let parameters = sdr
            .used_representation()
            .cast::<StepShapeShapeRepresentationWithParameters>();
        let Some(live) = parameters.get() else {
            return Ok(Some(valid));
        };
        valid = true;
        // Collect parameters of datum target
        for rank in 1..=live.nb_items() {
            self.read_target_parameter(&live.items_value(rank)?, &parameters, object)?;
        }
        Ok(Some(valid))
    }
}

impl ReaderState {
    /// One datum target of `setDatumToXCAF`: true when it makes a datum.
    #[allow(clippy::too_many_arguments, reason = "the locals of the member")]
    fn set_datum_target(
        &mut self,
        tools: &DatumTools<'_>,
        target: &Handle<StepDimTolPlacedDatumTargetFeature>,
        datum: &StepDimTolDatum,
        gdt_label: &TdfLabel,
        modifiers: &DatumModifiers,
    ) -> Result<bool, Failure> {
        let session = tools.session;
        let live = step_live(target, WHERE)?;
        let object = Handle::new(XCAFDimTolObjectsDatumObject::new());
        let live_object = step_live(&object, WHERE)?;
        let mut kind = XCAFDimTolObjectsDatumTargetType::default();
        if !super::STEPCAFControlGDTProperty::get_datum_target_type(&live.description(), &mut kind)?
        {
            session
                .process()?
                .add_warning_text(&target.cast(), "Unknown datum target type", "");
            return Ok(false);
        }
        live_object.set_datum_target_type(kind);
        let mut valid = false;
        // Feature for datum target
        let mut labels = Vec::new();
        let target_sharings = session.sharings(&target.cast())?;
        let relationship: Handle<StepReprFeatureForDatumTargetRelationship> =
            first_of(&target_sharings);
        if let Some(relationship) = relationship.get() {
            let sharings = session.sharings(&relationship.relating_shape_aspect().cast())?;
            valid |= tools.labels_of_usages(&sharings, &mut labels)?;
        }
        if kind != XCAFDimTolObjectsDatumTargetType::Area && !valid {
            // Try another way of feature connection
            valid |= tools.labels_of_usages(&target_sharings, &mut labels)?;
        }
        let Some(valid) =
            tools.read_target_definition(target, kind, &relationship, live_object, valid)?
        else {
            return Ok(false);
        };
        // Create datum target object
        if !valid {
            return Ok(false);
        }
        let dim_tol_tool = step_live(&tools.dim_tol_tool, WHERE)?;
        let label = dim_tol_tool.add_datum_with(
            &datum.name(),
            &datum.description(),
            &datum.identification(),
        )?;
        self.gdt_map.bind(target.cast(), label.clone());
        dim_tol_tool.lock(&label)?;
        let attribute = XCAFDocDatum::set(&label)?;
        dim_tol_tool.set_datum_shapes(&labels, &label)?;
        live_object.set_name(&datum.identification());
        live_object.set_position(modifiers.position);
        if !modifiers.single.is_empty() {
            live_object.set_modifiers(&modifiers.single);
        }
        if modifiers.with_value != XCAFDimTolObjectsDatumModifWithValue::None {
            live_object.set_modifier_with_value(modifiers.with_value, modifiers.value);
        }
        dim_tol_tool.set_datum_to_geom_tol(&label, gdt_label)?;
        live_object.set_is_datum_target(true);
        let number = live.target_id();
        live_object.set_datum_target_number(step_live(&number, WHERE)?.integer_value()?);
        read_annotation(session, &target.cast(), &object.cast(), tools.factors)?;
        step_live(&attribute, WHERE)?.set_object(&object)?;
        Ok(true)
    }
}
