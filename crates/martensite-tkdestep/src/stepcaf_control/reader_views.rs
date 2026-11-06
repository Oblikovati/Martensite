//! `read_views` of `STEPCAFControlReader`: saved views with their shapes, GD&T and clipping planes.
use super::reader::{ReaderState, WHERE};
use super::reader_pmi::{factors_of_representation, model_of};
use super::reader_support::SessionGraph;
use crate::step_ap242::StepAP242DraughtingModelItemAssociation;
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_geom::{StepGeomGeometricRepresentationItem, StepGeomPlane};
use crate::step_repr::{
    StepReprMappedItem, StepReprRepresentation, StepReprRepresentationRelationship,
};
use crate::step_to_geom::StepToGeom;
use crate::step_visual::{
    StepVisualAnnotationOccurrence, StepVisualAnnotationPlane, StepVisualCameraModelD3,
    StepVisualCameraModelD3MultiClipping, StepVisualCameraModelD3MultiClippingIntersection,
    StepVisualCameraModelD3MultiClippingUnion, StepVisualCentralOrParallel,
    StepVisualDraughtingCallout, StepVisualDraughtingModel,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
use martensite_tkg3d::geom::GeomPoint;
use martensite_tklcaf::tdf::{TdfLabel, TdfTool};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocClippingPlaneTool, XCAFDocDocumentTool, XCAFDocShapeTool, XCAFDocView,
};
use martensite_tkxcaf::xcaf_view::{XCAFViewObject, XCAFViewProjectionType};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// `collectViewShapes`: the labels of a representation and of the representations it relates to.
fn collect_view_shapes(
    session: &SessionGraph,
    tool: &XCAFDocShapeTool,
    representation: &Handle<StepReprRepresentation>,
    shapes: &mut Vec<TdfLabel>,
) -> Result<(), Failure> {
    let shape = session.shape_at(session.process()?.map_index(&representation.cast()))?;
    if !shape.is_null() {
        let mut label = TdfLabel::default();
        tool.find_shape(&shape, &mut label, false)?;
        if !label.is_null() {
            shapes.push(label);
        }
    }
    for sharing in session.sharings(&representation.cast())? {
        let relationship = sharing.cast::<StepReprRepresentationRelationship>();
        let Some(relationship) = relationship.get() else {
            continue;
        };
        let first = relationship.rep1();
        if first.is_some() && first != *representation {
            collect_view_shapes(session, tool, &first, shapes)?;
        }
    }
    Ok(())
}

/// The operands of a clipping model; for a root with one union, that union.
enum ClippingOperands {
    Root(Handle<StepVisualCameraModelD3MultiClippingUnion>),
    Planes(
        Vec<Handle<StepGeomGeometricRepresentationItem>>,
        &'static str,
    ),
}

/// The values of a list of selects.
macro_rules! planes_of_list {
    ($list:expr) => {{
        let list = $list;
        let mut planes = Vec::new();
        for rank in 1..=step_live(&list, WHERE)?.array1().length() as i32 {
            planes.push(
                step_array1_item(&list, rank, WHERE)?
                    .value()
                    .cast::<StepGeomGeometricRepresentationItem>(),
            );
        }
        planes
    }};
}

/// The operands of a model, `None` when it has no clipping.
fn clipping_operands(
    model: &Handle<StepGeomGeometricRepresentationItem>,
) -> Result<Option<ClippingOperands>, Failure> {
    if let Some(camera) = model.cast::<StepVisualCameraModelD3MultiClipping>().get() {
        let list = camera.shape_clipping();
        let Some(live) = list.get() else {
            return Ok(None);
        };
        // Root of clipping planes tree
        if live.array1().length() == 1 {
            let union = step_array1_item(&list, 1, WHERE)?.camera_model_d3_multi_clipping_union();
            if union.is_some() {
                return Ok(Some(ClippingOperands::Root(union)));
            }
        }
        return Ok(Some(ClippingOperands::Planes(
            planes_of_list!(list.clone()),
            "*",
        )));
    }
    if let Some(camera) = model
        .cast::<StepVisualCameraModelD3MultiClippingUnion>()
        .get()
    {
        return Ok(Some(ClippingOperands::Planes(
            planes_of_list!(camera.shape_clipping()),
            "+",
        )));
    }
    if let Some(camera) = model
        .cast::<StepVisualCameraModelD3MultiClippingIntersection>()
        .get()
    {
        return Ok(Some(ClippingOperands::Planes(
            planes_of_list!(camera.shape_clipping()),
            "*",
        )));
    }
    Ok(Some(ClippingOperands::Planes(Vec::new(), "*")))
}

/// `buildClippingPlanes`: the expression of the planes of a clipping camera model; the planes go to
/// the document.
fn build_clipping_planes(
    model: &Handle<StepGeomGeometricRepresentationItem>,
    planes: &mut Vec<TdfLabel>,
    tool: &XCAFDocClippingPlaneTool,
    factors: &StepDataFactors,
) -> Result<String, Failure> {
    let (operands, operation) = match clipping_operands(model)? {
        None => return Ok(String::new()),
        Some(ClippingOperands::Root(union)) => {
            return build_clipping_planes(&union.cast(), planes, tool, factors);
        }
        Some(ClippingOperands::Planes(operands, operation)) => (operands, operation),
    };
    // Build expression
    let mut expression = String::from("(");
    for operand in &operands {
        let plane_entity = operand.cast::<StepGeomPlane>();
        if let Some(live) = plane_entity.get() {
            let plane = StepToGeom::make_plane(&plane_entity, factors)?;
            if let Some(plane) = plane.get() {
                let label = tool.add_clipping_plane_ascii(&plane.pln(), &live.name())?;
                let mut entry = AsciiString::default();
                TdfTool::entry(&label, &mut entry);
                planes.push(label);
                expression.push_str(&String::from_utf8_lossy(entry.as_bytes()));
            }
        } else {
            expression.push_str(&build_clipping_planes(operand, planes, tool, factors)?);
        }
        expression.push_str(operation);
    }
    // Insert brace instead of operation after last operand.
    expression.pop();
    expression.push(')');
    Ok(expression)
}

/// The attributes of a view from its camera.
fn set_camera_attributes(
    object: &XCAFViewObject,
    camera: &StepVisualCameraModelD3,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    object.set_name(&camera.name());
    let axis = StepToGeom::make_axis2_placement(&camera.view_reference_system(), factors)?;
    let axis = step_live(&axis, WHERE)?.ax2()?;
    object.set_view_direction(&axis.direction());
    object.set_up_direction(&axis.direction().crossed(axis.x_direction())?);
    let volume = camera.perspective_of_volume();
    let volume = step_live(&volume, WHERE)?;
    object.set_type(match volume.projection_type() {
        StepVisualCentralOrParallel::CopCentral => XCAFViewProjectionType::Central,
        StepVisualCentralOrParallel::CopParallel => XCAFViewProjectionType::Parallel,
    });
    let point = StepToGeom::make_cartesian_point(&volume.projection_point(), factors)?;
    object.set_projection_point(&step_live(&point, WHERE)?.pnt());
    object.set_zoom_factor(volume.view_plane_distance());
    let window = volume.view_window();
    let window = step_live(&window, WHERE)?;
    object.set_window_horizontal_size(window.size_in_x());
    object.set_window_vertical_size(window.size_in_y());
    if volume.front_plane_clipping() {
        object.set_front_plane_distance(volume.front_plane_distance());
    }
    if volume.back_plane_clipping() {
        object.set_back_plane_distance(volume.back_plane_distance());
    }
    object.set_view_volume_sides_clipping(volume.view_volume_sides_clipping());
    Ok(())
}

impl ReaderState {
    /// The labels of GD&T an annotation of a view has: `fallback_on_item` also tries the annotation.
    fn gdts_of_annotation(
        &self,
        session: &SessionGraph,
        annotation: &Handle<StandardTransient>,
        fallback_on_item: bool,
        gdts: &mut Vec<TdfLabel>,
    ) -> Result<(), Failure> {
        for sharing in session.sharings(annotation)? {
            let association = sharing.cast::<StepAP242DraughtingModelItemAssociation>();
            let Some(association) = association.get() else {
                continue;
            };
            let mut found = self.gdt_map.find(&association.definition().value()).ok();
            if found.is_none() && fallback_on_item {
                found = self.gdt_map.find(annotation).ok();
            }
            if let Some(label) = found {
                gdts.push(label.clone());
            }
        }
        Ok(())
    }
    /// One camera of `read_views`; false when it has no draughting model.
    fn read_view(
        &self,
        entity: &Handle<StandardTransient>,
        session: &SessionGraph,
        doc: &Handle<TDocStdDocument>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let main = step_live(doc, WHERE)?.main()?;
        let shape_tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let view_tool = XCAFDocDocumentTool::view_tool(&main)?;
        let object = Handle::new(XCAFViewObject::new());
        let live_object = step_live(&object, WHERE)?;
        // Import attributes of view
        let camera = entity.cast::<StepVisualCameraModelD3>();
        let live_camera = step_live(&camera, WHERE)?;
        // find the proper DraughtingModel
        let model = super::reader_support::first_of::<StepVisualDraughtingModel>(
            &session.sharings(entity)?,
        );
        let mut local = *factors;
        if model.is_some() {
            local = factors_of_representation(
                &model.cast(),
                &session.tp,
                &model_of(&session.tp)?,
                factors,
            )?;
        }
        set_camera_attributes(live_object, live_camera, &local)?;
        // Clipping plane
        let mut clipping_planes = Vec::new();
        if entity
            .cast::<StepVisualCameraModelD3MultiClipping>()
            .is_some()
        {
            let clipping_tool = XCAFDocDocumentTool::clipping_plane_tool(&main)?;
            let expression = build_clipping_planes(
                &entity.cast(),
                &mut clipping_planes,
                step_live(&clipping_tool, WHERE)?,
                &local,
            )?;
            live_object.set_clipping_expression(&Handle::new(HAsciiString::new(&expression)));
        }
        // Collect shapes and GDTs
        if model.is_null() {
            return Ok(false);
        }
        let (mut shapes, mut gdts) = (Vec::new(), Vec::new());
        for shared in session.shareds(&model.cast())? {
            if let Some(mapped) = shared.cast::<StepReprMappedItem>().get() {
                let source = mapped.mapping_source();
                let representation = step_live(&source, WHERE)?.mapped_representation();
                if representation.is_some() {
                    collect_view_shapes(
                        session,
                        step_live(&shape_tool, WHERE)?,
                        &representation,
                        &mut shapes,
                    )?;
                }
            } else if shared.cast::<StepVisualAnnotationOccurrence>().is_some()
                || shared.cast::<StepVisualDraughtingCallout>().is_some()
            {
                self.gdts_of_annotation(session, &shared, true, &mut gdts)?;
            } else if let Some(plane) = shared.cast::<StepVisualAnnotationPlane>().get() {
                for _ in 1..=plane.nb_elements() {
                    self.gdts_of_annotation(session, &shared, false, &mut gdts)?;
                }
            }
        }
        let live_view_tool = step_live(&view_tool, WHERE)?;
        let view = live_view_tool.add_view()?;
        let attribute = XCAFDocView::set(&view)?;
        step_live(&attribute, WHERE)?.set_object(&object)?;
        live_view_tool.set_view_with_planes(&shapes, &gdts, &clipping_planes, &view)?;
        live_view_tool.lock(&view)?;
        Ok(true)
    }
    /// `read_views`: false at a camera without a draughting model.
    pub(super) fn read_views(
        &self,
        ws: &Handle<XSControlWorkSession>,
        doc: &Handle<TDocStdDocument>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        let main = step_live(doc, WHERE)?.main()?;
        if XCAFDocDocumentTool::dim_tol_tool(&main)?.is_null() {
            return Ok(false);
        }
        let model = step_live(ws, WHERE)?.model();
        let session = SessionGraph::of_process(ws)?;
        for i in 1..=model.nb_entities() {
            let entity = model.value(i)?;
            if entity.cast::<StepVisualCameraModelD3>().is_some()
                && !self.read_view(&entity, &session, doc, factors)?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
