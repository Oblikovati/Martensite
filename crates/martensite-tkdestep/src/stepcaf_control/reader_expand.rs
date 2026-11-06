//! `expand_sub_shapes` of `STEPCAFControlReader`: the names of sub-shapes as a CAF tree.
use super::reader::{ReaderState, ShapePdMap, WHERE};
use super::reader_support::{SessionGraph, first_of};
use crate::step_data::StepDataStepModel;
use crate::step_entity::{step_array1_item, step_live};
use crate::step_repr::{StepReprRepresentationItem, StepReprRepresentationRelationship};
use crate::step_shape::{
    StepShapeConnectedFaceSet, StepShapeEdgeLoop, StepShapeManifoldSolidBrep,
    StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation,
    StepShapeShellBasedSurfaceModel,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkxcaf::xcaf_doc::XCAFDocShapeTool;
use martensite_tkxsbase::transfer_brep::TransferBRep;

/// `settle_shape_data`: a named item as a sub-shape label with its name.
fn settle_shape_data(
    item: &Handle<StepReprRepresentationItem>,
    label: &TdfLabel,
    tool: &XCAFDocShapeTool,
    session: &SessionGraph,
) -> Result<TdfLabel, Failure> {
    let Some(live) = item.get() else {
        return Ok(label.clone());
    };
    let name = live.name();
    let Some(name) = name.get().filter(|text| !text.is_empty()) else {
        return Ok(label.clone());
    };
    let binder = session.process()?.find(&item.cast());
    if binder.is_null() {
        return Ok(label.clone());
    }
    let shape = TransferBRep::shape_result(&binder);
    if shape.is_null() {
        return Ok(label.clone());
    }
    // Allocate sub-Label
    let result = tool.add_sub_shape(label, &shape)?;
    if result.is_null() {
        return Ok(result);
    }
    TDataStdName::set(&result, &ExtendedString::from_ascii(&name.string()))?;
    tool.set_shape(&result, &shape)?;
    Ok(result)
}

/// `collectRepresentationItems`: the items of a representation and of those related to it.
fn collect_representation_items(
    session: &SessionGraph,
    representation: &Handle<StepShapeShapeRepresentation>,
    items: &mut Vec<Handle<StepReprRepresentationItem>>,
) -> Result<(), Failure> {
    let live = step_live(representation, WHERE)?;
    for rank in 1..=live.nb_items() {
        let item = live.items_value(rank)?;
        if item.is_some() {
            items.push(item);
        }
    }
    for sharing in
        session.typed_sharings::<StepReprRepresentationRelationship>(&representation.cast())?
    {
        let relationship = sharing.cast::<StepReprRepresentationRelationship>();
        let relationship = step_live(&relationship, WHERE)?;
        if relationship.rep1() == representation.cast::<crate::step_repr::StepReprRepresentation>()
        {
            let other = relationship.rep2().cast::<StepShapeShapeRepresentation>();
            if other.is_some() {
                collect_representation_items(session, &other, items)?;
            }
        }
    }
    Ok(())
}

/// `expand_shell`: the shell, its faces, loops, edges and vertices. Only edge loops have edges.
fn expand_shell(
    shell: &Handle<StepShapeConnectedFaceSet>,
    root: &TdfLabel,
    tool: &XCAFDocShapeTool,
    session: &SessionGraph,
) -> Result<(), Failure> {
    // Record CAF data
    settle_shape_data(&shell.cast(), root, tool, session)?;
    let live = step_live(shell, WHERE)?;
    // Access faces
    for face_rank in 1..=live.nb_cfs_faces()? {
        let face = live.cfs_faces_value(face_rank)?;
        let Some(live_face) = face.get() else {
            continue;
        };
        settle_shape_data(&face.cast(), root, tool, session)?;
        // Access face bounds
        let bounds = live_face.bounds()?;
        let Some(live_bounds) = bounds.get() else {
            continue;
        };
        for bound_rank in 1..=live_bounds.array1().length() as i32 {
            let bound = step_array1_item(&bounds, bound_rank, WHERE)?;
            let wire = step_live(&bound, WHERE)?.bound();
            settle_shape_data(&wire.cast(), root, tool, session)?;
            // Currently only EDGE LOOPs are considered (!), and exactly them: IsInstance
            step_live(&wire, WHERE)?;
            let is_instance = wire.dynamic_type().is_some_and(|kind| {
                std::ptr::eq(
                    kind,
                    <StepShapeEdgeLoop as martensite_tkernel::standard::RuntimeTyped>::static_type(
                    ),
                )
            });
            if !is_instance {
                continue;
            }
            expand_edge_loop(&wire.cast(), root, tool, session)?;
        }
    }
    Ok(())
}

/// The edges of an edge loop with their vertices.
fn expand_edge_loop(
    edge_loop: &Handle<StepShapeEdgeLoop>,
    root: &TdfLabel,
    tool: &XCAFDocShapeTool,
    session: &SessionGraph,
) -> Result<(), Failure> {
    let edges = step_live(edge_loop, WHERE)?.edge_list();
    let count = step_live(&edges, WHERE)?.array1().length() as i32;
    for rank in 1..=count {
        let oriented = step_array1_item(&edges, rank, WHERE)?;
        let Some(oriented) = oriented.get() else {
            continue;
        };
        let edge = oriented.edge_element();
        let Some(live_edge) = edge.get() else {
            continue;
        };
        // Record CAF data
        settle_shape_data(&edge.cast(), root, tool, session)?;
        // Access vertices
        settle_shape_data(&live_edge.edge_start().cast(), root, tool, session)?;
        settle_shape_data(&live_edge.edge_end().cast(), root, tool, session)?;
    }
    Ok(())
}

/// `expand_sbsm`: the boundary shells of a shell based surface model.
fn expand_sbsm(
    item: &Handle<StepReprRepresentationItem>,
    root: &TdfLabel,
    tool: &XCAFDocShapeTool,
    session: &SessionGraph,
) -> Result<(), Failure> {
    let model = item.cast::<StepShapeShellBasedSurfaceModel>();
    let shells = step_live(&model, WHERE)?.sbsm_boundary();
    let count = step_live(&shells, WHERE)?.array1().length() as i32;
    for rank in 1..=count {
        let shell = step_array1_item(&shells, rank, WHERE)?;
        let open = shell.open_shell();
        let face_set: Handle<StepShapeConnectedFaceSet> = if open.is_some() {
            open.cast()
        } else {
            shell.closed_shell().cast()
        };
        expand_shell(&face_set, root, tool, session)?;
    }
    Ok(())
}

impl ReaderState {
    /// One root of `expand_sub_shapes`.
    fn expand_root(
        &self,
        root_shape: &martensite_tkbrep::topods::Shape,
        product_definition: &Handle<crate::step_basic::StepBasicProductDefinition>,
        tool: &XCAFDocShapeTool,
        session: &SessionGraph,
    ) -> Result<(), Failure> {
        if product_definition.is_null() {
            return Ok(());
        }
        // Find SDR by Product
        let sharings = session
            .typed_sharings::<StepShapeShapeDefinitionRepresentation>(&product_definition.cast())?;
        let sdr: Handle<StepShapeShapeDefinitionRepresentation> = first_of(&sharings);
        let Some(sdr) = sdr.get() else {
            return Ok(());
        };
        // Access shape representation
        let representation = sdr
            .used_representation()
            .cast::<StepShapeShapeRepresentation>();
        if representation.is_null() {
            return Ok(());
        }
        // Access representation items
        let mut items = Vec::new();
        collect_representation_items(session, &representation, &mut items)?;
        if items.is_empty() {
            return Ok(());
        }
        let Ok(root) = self.map.find(root_shape) else {
            return Ok(());
        };
        // Do not add subshapes to assembly,
        // they will be processed with corresponding Shape_Product_Definition of necessary part.
        if XCAFDocShapeTool::is_assembly(root)? {
            return Ok(());
        }
        // Iterate over the top level representation items collecting the topological containers
        let solids: Vec<_> = items
            .iter()
            .filter(|item| item.cast::<StepShapeManifoldSolidBrep>().is_some())
            .collect();
        let models: Vec<_> = items
            .iter()
            .filter(|item| {
                item.cast::<StepShapeManifoldSolidBrep>().is_null()
                    && item.cast::<StepShapeShellBasedSurfaceModel>().is_some()
            })
            .collect();
        // Insert intermediate OCAF Labels for SOLIDs in case there are more
        // than one Manifold Solid BRep in the Shape Representation
        let insert_solid_labels = solids.len() > 1;
        for solid in solids {
            // Put additional Label for SOLID
            if insert_solid_labels {
                settle_shape_data(solid, root, tool, session)?;
            }
            // Access outer shell, and expand its contents to CAF tree: ExpandManifoldSolidBrep
            let brep = solid.cast::<StepShapeManifoldSolidBrep>();
            expand_shell(
                &step_live(&brep, WHERE)?.outer().cast(),
                root,
                tool,
                session,
            )?;
        }
        // Expand Shell-Based Surface Models
        for model in models {
            expand_sbsm(model, root, tool, session)?;
        }
        Ok(())
    }
    /// `expand_sub_shapes`: when the model reads sub-shape names.
    pub(super) fn expand_sub_shapes(
        &self,
        tool: &Handle<XCAFDocShapeTool>,
        shape_pd: &ShapePdMap,
    ) -> Result<(), Failure> {
        let session = SessionGraph::of_process(&self.main_ws)?;
        let model = step_live(&session.tp, WHERE)?
            .model()
            .cast::<StepDataStepModel>();
        if !step_live(&model, WHERE)?
            .internal_parameters()
            .read_subshape_names
        {
            return Ok(());
        }
        let session = SessionGraph::of_session(&self.main_ws)?;
        let live_tool = step_live(tool, WHERE)?;
        for (root_shape, product_definition) in shape_pd.items() {
            self.expand_root(root_shape, product_definition, live_tool, &session)?;
        }
        Ok(())
    }
}
