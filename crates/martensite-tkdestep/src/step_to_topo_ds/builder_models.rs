//! `StepToTopoDSBuilder`: the wireframe and face based model overloads of `init`.
use super::StepToTopoDSBuilder;
use super::builder::new_tool;
use super::{StepToTopoDSNMTool, StepToTopoDSTranslateEdge, StepToTopoDSTranslateFace};
use crate::step_data::StepDataFactors;
use crate::step_entity::{step_array1_item, step_array1_len, step_live};
use crate::step_shape::{
    StepShapeConnectedFaceSet, StepShapeEdgeBasedWireframeModel, StepShapeFaceBasedSurfaceModel,
    StepShapeFaceSurface,
};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Compound, Shell, Wire};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::transfer::TransferTransientProcess;

impl StepToTopoDSBuilder {
    /// `init(StepShapeEdgeBasedWireframeModel, TP, factors)`.
    pub fn init_edge_based_wireframe_model(
        &mut self,
        model: &Handle<StepShapeEdgeBasedWireframeModel>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        self.my_result.nullify();
        let process = step_live(tp, "StepToTopoDSBuilder")?;
        let live = step_live(model, "StepToTopoDSBuilder")?;
        let boundary = live.ebwm_boundary();
        let count = step_array1_len(&boundary);
        if count < 1 {
            process.add_warning_text(&model.cast(), "List of boundaries is empty", "");
            return Ok(());
        }
        let mut tool = new_tool(tp);
        let mut translator = StepToTopoDSTranslateEdge::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let mut nm_tool = StepToTopoDSNMTool::new();
        for rank in 1..=count {
            let edge_set = step_array1_item(&boundary, rank, "StepToTopoDSBuilder")?;
            let Some(live_set) = edge_set.get() else {
                continue;
            };
            let edges = live_set.ces_edges();
            let nb_edges = step_array1_len(&edges);
            if nb_edges < 1 {
                process.add_warning_text(&edge_set.cast(), "No edges in connected_edge_set", "");
                continue;
            }
            let mut wire = Wire::default();
            for edge_rank in 1..=nb_edges {
                let edge = step_array1_item(&edges, edge_rank, "StepToTopoDSBuilder")?;
                translator.init(&edge, &mut tool, &mut nm_tool, factors)?;
                if !translator.is_done() {
                    continue;
                }
                let edge = topods::edge(translator.value()?)?;
                if edge.is_null() {
                    continue; // NULL, on saute
                }
                if wire.is_null() {
                    builder.make_wire(&mut wire);
                }
                builder.add(&mut wire, &edge)?;
            }
            if wire.is_null() {
                continue;
            }
            wire.set_closed(BRepTool::is_closed(&wire)?)?;
            builder.add(&mut compound, &wire)?;
            self.my_result = if self.my_result.is_null() {
                wire.into_shape()
            } else {
                compound.clone().into_shape()
            };
        }
        self.finish_model(process)
    }
    /// `init(StepShapeFaceBasedSurfaceModel, TP, factors)`.
    pub fn init_face_based_surface_model(
        &mut self,
        model: &Handle<StepShapeFaceBasedSurfaceModel>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        self.my_result.nullify();
        let process = step_live(tp, "StepToTopoDSBuilder")?;
        let live = step_live(model, "StepToTopoDSBuilder")?;
        let boundary = live.fbsm_faces();
        let count = step_array1_len(&boundary);
        if count < 1 {
            process.add_warning_text(&model.cast(), "List of faces is empty", "");
            return Ok(());
        }
        let mut tool = new_tool(tp);
        let mut translator = StepToTopoDSTranslateFace::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        // Non-manifold topology is not mapped via FaceBasedSurfaceModel (ssv; 14.11.2010)
        let mut nm_tool = StepToTopoDSNMTool::new();
        for rank in 1..=count {
            let face_set: Handle<StepShapeConnectedFaceSet> =
                step_array1_item(&boundary, rank, "StepToTopoDSBuilder")?;
            let Some(live_set) = face_set.get() else {
                continue;
            };
            let nb_faces = live_set.nb_cfs_faces()?;
            if nb_faces < 1 {
                process.add_warning_text(&face_set.cast(), "No faces in connected_face_set", "");
                continue;
            }
            let mut shell = Shell::default();
            for face_rank in 1..=nb_faces {
                let face = live_set
                    .cfs_faces_value(face_rank)?
                    .cast::<StepShapeFaceSurface>();
                translator.init(&face, &mut tool, &mut nm_tool, factors)?;
                if !translator.is_done() {
                    continue;
                }
                let made = topods::face(translator.value()?)?;
                if made.is_null() {
                    continue; // NULL, on saute
                }
                if shell.is_null() {
                    builder.make_shell(&mut shell);
                }
                builder.add(&mut shell, &made)?;
            }
            if shell.is_null() {
                continue;
            }
            shell.set_closed(BRepTool::is_closed(&shell)?)?;
            builder.add(&mut compound, &shell)?;
            self.my_result = if self.my_result.is_null() {
                shell.into_shape()
            } else {
                compound.clone().into_shape()
            };
        }
        self.finish_model(process)
    }
}
