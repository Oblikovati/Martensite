//! `ReadGDTs` of `STEPCAFControlReader`: dimensions, tolerances and the presentations without
//! semantic data.
use super::reader::{ReaderState, WHERE};
use super::reader_dim_object::set_dim_object_to_xcaf;
use super::reader_gdt_common::{collect_shape_aspect, shape_label_of};
use super::reader_pmi::{factors_of_representation, read_annotation_plane, read_pmi_presentation};
use super::reader_support::{SessionGraph, first_of};
use super::reader_tol_object::set_geom_tol_object_to_xcaf;
use crate::step_ap242::{
    StepAP242DraughtingModelItemAssociation, StepAP242GeometricItemSpecificUsage,
};
use crate::step_data::StepDataFactors;
use crate::step_dim_tol::{
    StepDimTolDatum, StepDimTolDatumFeature, StepDimTolDatumTarget, StepDimTolGeometricTolerance,
};
use crate::step_entity::step_live;
use crate::step_repr::{
    StepReprCompShAspAndDatumFeatAndShAsp, StepReprShapeAspect, StepReprShapeAspectRelationship,
};
use crate::step_shape::{StepShapeDimensionalLocation, StepShapeDimensionalSize};
use crate::step_visual::{
    StepVisualAnnotationOccurrence, StepVisualAnnotationPlane, StepVisualDraughtingCallout,
    StepVisualTessellatedAnnotationOccurrence,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::Axis2;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDimensionObject, XCAFDimTolObjectsDimensionType,
};
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocDimTolTool, XCAFDocDimension, XCAFDocDocumentTool, XCAFDocShapeTool,
};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// A dimension or a tolerance.
fn is_gdt(entity: &Handle<StandardTransient>) -> bool {
    entity.cast::<StepShapeDimensionalSize>().is_some()
        || entity.cast::<StepShapeDimensionalLocation>().is_some()
        || entity.cast::<StepDimTolGeometricTolerance>().is_some()
}

/// A datum, a datum target or a datum feature.
fn is_datum_like(aspect: &Handle<StepReprShapeAspect>) -> bool {
    aspect.cast::<StepDimTolDatum>().is_some()
        || aspect.cast::<StepDimTolDatumTarget>().is_some()
        || aspect.cast::<StepDimTolDatumFeature>().is_some()
        || aspect
            .cast::<StepReprCompShAspAndDatumFeatAndShAsp>()
            .is_some()
}

/// Whether a definition is a datum or has a GDT applied.
fn is_connected_to_gdt(
    session: &SessionGraph,
    definition: &Handle<StepReprShapeAspect>,
) -> Result<bool, Failure> {
    // Skip if definition is a datum
    if is_datum_like(definition) {
        return Ok(true);
    }
    // Skip if any GDT is applied to definition
    for sharing in session.sharings(&definition.cast())? {
        if is_gdt(&sharing) {
            return Ok(true);
        }
        let Some(relationship) = sharing
            .cast::<StepReprShapeAspectRelationship>()
            .get()
            .map(|live| live.related_shape_aspect())
        else {
            continue;
        };
        if relationship.is_null() {
            continue;
        }
        if is_datum_like(&relationship) {
            return Ok(true);
        }
        if session.sharings(&relationship.cast())?.iter().any(is_gdt) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// The labels of the shapes a presentation is on; `None` when it is attached to a GDT, so is skipped.
fn shapes_of_association(
    session: &SessionGraph,
    association: &StepAP242DraughtingModelItemAssociation,
    shape_tool: &Handle<XCAFDocShapeTool>,
) -> Result<Option<Vec<TdfLabel>>, Failure> {
    // Check entity, skip all, attached to GDTs
    let definition = association.definition().shape_aspect();
    if definition.is_some() {
        if is_connected_to_gdt(session, &definition)? {
            return Ok(None);
        }
    } else if association.definition().property_definition().is_null() {
        return Ok(None);
    }
    // Get shapes
    let mut aspects = Vec::new();
    collect_shape_aspect(&definition, session, &mut aspects)?;
    let mut labels = Vec::new();
    for aspect in aspects {
        let usage: Handle<StepAP242GeometricItemSpecificUsage> =
            first_of(&session.sharings(&aspect.cast())?);
        let Some(usage) = usage.get() else {
            continue;
        };
        for rank in 1..=usage.nb_identified_item() {
            let label = shape_label_of(&usage.identified_item_value(rank)?, session, shape_tool)?;
            if !label.is_null() {
                labels.push(label);
            }
        }
    }
    Ok(Some(labels))
}

impl ReaderState {
    /// A presentation of PMI without semantic data, as a dimension on its shapes.
    fn read_presentation_only(
        &mut self,
        entity: &Handle<StandardTransient>,
        ws: &Handle<XSControlWorkSession>,
        session: &SessionGraph,
        shape_tool: &Handle<XCAFDocShapeTool>,
        dim_tol_tool: &XCAFDocDimTolTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let sharings = session.sharings(entity)?;
        // Protection against import presentation twice
        if first_of::<StepVisualDraughtingCallout>(&sharings).is_some() {
            return Ok(());
        }
        // Read presentations for PMIs without semantic data.
        let association: Handle<StepAP242DraughtingModelItemAssociation> = first_of(&sharings);
        let mut labels = Vec::new();
        if let Some(live) = association.get() {
            match shapes_of_association(session, live, shape_tool)? {
                Some(found) => labels = found,
                None => return Ok(()),
            }
        }
        let is_common_label = labels.is_empty();
        // Calculate unit
        let mut factor = 1.0;
        let mut local = *factors;
        if let Some(live) = association.get() {
            local = factors_of_representation(
                &live.used_representation(),
                &session.tp,
                &step_live(ws, WHERE)?.model(),
                factors,
            )?;
            factor = local.length_factor();
        }
        // Presentation
        let mut presentation = Shape::default();
        let mut name = Handle::default();
        let mut bounds = BoundingBox3::default();
        if !read_pmi_presentation(
            entity,
            session,
            factor,
            &mut presentation,
            &mut name,
            &mut bounds,
            &local,
        )? {
            return Ok(());
        }
        // Annotation plane
        let plane: Handle<StepVisualAnnotationPlane> = first_of(&sharings);
        // Set object to XCAF
        let label = dim_tol_tool.add_dimension()?;
        self.gdt_map.bind(entity.clone(), label.clone());
        dim_tol_tool.lock(&label)?;
        let object = Handle::new(XCAFDimTolObjectsDimensionObject::new());
        let live_object = step_live(&object, WHERE)?;
        let attribute = XCAFDocDimension::set(&label)?;
        let (text, kind) = if is_common_label {
            (
                "DGT:Common_label",
                XCAFDimTolObjectsDimensionType::CommonLabel,
            )
        } else {
            (
                "DGT:Dimension",
                XCAFDimTolObjectsDimensionType::DimensionPresentation,
            )
        };
        live_object.set_type(kind);
        TDataStdName::set(&label, &ExtendedString::from_ascii(&AsciiString::new(text)))?;
        dim_tol_tool.set_dimension_shapes(&labels, &[], &label)?;
        let mut plane_axes = Axis2::default();
        if plane.is_some() && read_annotation_plane(&plane, &mut plane_axes, &local)? {
            live_object.set_plane(&plane_axes);
        }
        live_object.set_presentation(&presentation, &name);
        step_live(&attribute, WHERE)?.set_object(&object)?;
        Ok(())
    }
    /// `ReadGDTs`.
    pub(super) fn read_gdts(
        &mut self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let model = step_live(ws, WHERE)?.model();
        let main = step_live(doc, WHERE)?.main()?;
        let shape_tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let dim_tol_tool = XCAFDocDocumentTool::dim_tol_tool(&main)?;
        let Some(live_tool) = dim_tol_tool.get() else {
            return Ok(false);
        };
        let from_session = SessionGraph::of_session(ws)?;
        let from_process = SessionGraph::of_process(ws)?;
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            step_live(&entity, WHERE)?;
            if is_gdt(&entity) {
                let label = self.create_gdt_object_in_xcaf(&entity, doc, ws, factors)?;
                if !label.is_null() {
                    if entity.cast::<StepDimTolGeometricTolerance>().is_some() {
                        set_geom_tol_object_to_xcaf(&entity, &label, &from_process, factors)?;
                    } else {
                        set_dim_object_to_xcaf(&entity, &label, &from_process, factors)?;
                    }
                }
            } else if entity.cast::<StepVisualDraughtingCallout>().is_some()
                || entity.cast::<StepVisualAnnotationOccurrence>().is_some()
                || entity
                    .cast::<StepVisualTessellatedAnnotationOccurrence>()
                    .is_some()
            {
                self.read_presentation_only(
                    &entity,
                    ws,
                    &from_session,
                    &shape_tool,
                    live_tool,
                    factors,
                )?;
            }
        }
        Ok(true)
    }
}
