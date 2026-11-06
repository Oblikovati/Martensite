//! `StepTidyDuplicateCleaner`: merges the duplicates of a work session's model and rebuilds the
//! model without them.
use super::reducers::{
    StepTidyAxis2Placement3dReducer, StepTidyCartesianPointReducer, StepTidyCircleReducer,
    StepTidyDirectionReducer, StepTidyLineReducer, StepTidyPlaneReducer, StepTidyVectorReducer,
};
use crate::step_data::StepDataStepModel;
use crate::step_entity::step_live;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceModelEntities};
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashSet;

const WHERE: &str = "StepTidyDuplicateCleaner";

/// Duplicate cleaner.
pub struct StepTidyDuplicateCleaner {
    my_ws: Handle<XSControlWorkSession>,
}

impl StepTidyDuplicateCleaner {
    /// `StepTidyDuplicateCleaner(theWS)`.
    pub fn new(ws: &Handle<XSControlWorkSession>) -> Self {
        Self { my_ws: ws.clone() }
    }
    /// `perform`: nothing without a STEP model.
    pub fn perform(&self) -> Result<(), Failure> {
        let session = step_live(&self.my_ws, WHERE)?;
        let model: Handle<StepDataStepModel> = session.model().cast();
        let Some(live) = model.get() else {
            return Ok(());
        };
        let mut points = StepTidyCartesianPointReducer::new(&self.my_ws);
        let mut directions = StepTidyDirectionReducer::new(&self.my_ws);
        let mut placements = StepTidyAxis2Placement3dReducer::new(&self.my_ws);
        let mut vectors = StepTidyVectorReducer::new(&self.my_ws);
        let mut lines = StepTidyLineReducer::new(&self.my_ws);
        let mut planes = StepTidyPlaneReducer::new(&self.my_ws);
        let mut circles = StepTidyCircleReducer::new(&self.my_ws);
        for rank in 1..=live.nb_entities() {
            let entity = live.value(rank)?;
            points.process_entity(&entity)?;
            directions.process_entity(&entity)?;
            placements.process_entity(&entity)?;
            vectors.process_entity(&entity)?;
            lines.process_entity(&entity)?;
            planes.process_entity(&entity)?;
            circles.process_entity(&entity)?;
        }
        let mut replaced = Vec::new();
        let mut seen = HashSet::new();
        points.perform(&mut replaced, &mut seen)?;
        directions.perform(&mut replaced, &mut seen)?;
        placements.perform(&mut replaced, &mut seen)?;
        vectors.perform(&mut replaced, &mut seen)?;
        lines.perform(&mut replaced, &mut seen)?;
        planes.perform(&mut replaced, &mut seen)?;
        circles.perform(&mut replaced, &mut seen)?;
        self.remove_entities(&seen)
    }
    /// A new model with the protocol and the general tool of `model`.
    fn sibling_model(model: &StepDataStepModel) -> Result<Handle<StepDataStepModel>, Failure> {
        let made = StepDataStepModel::new();
        let live = step_live(&made, WHERE)?;
        live.set_protocol(&model.protocol());
        live.set_gtool(&model.gtool());
        Ok(made)
    }
    /// `removeEntities(theToRemove)`: a first model without the replaced entities, then a second
    /// without the entities nothing shares any more and that share nothing.
    fn remove_entities(&self, to_remove: &HashSet<usize>) -> Result<(), Failure> {
        if to_remove.is_empty() {
            return Ok(());
        }
        let session = step_live(&self.my_ws, WHERE)?;
        let read: Handle<StepDataStepModel> = session.model().cast();
        let live_read = step_live(&read, WHERE)?;
        let intermediate = Self::sibling_model(live_read)?;
        let live_intermediate = step_live(&intermediate, WHERE)?;
        for rank in 1..=live_read.nb_entities() {
            let entity: Handle<StandardTransient> = live_read.value(rank)?;
            if !to_remove.contains(&(entity.object_address() as usize)) {
                live_intermediate.add_with_refs(&entity, 0, false)?;
            }
        }
        session.set_model(&intermediate.cast(), true)?;
        session.compute_graph(true)?;
        let final_model = Self::sibling_model(live_intermediate)?;
        let live_final = step_live(&final_model, WHERE)?;
        let hgraph = session.graph()?;
        let holder = step_live(&hgraph, WHERE)?.graph();
        for rank in 1..=live_intermediate.nb_entities() {
            let entity = live_intermediate.value(rank)?;
            if holder.shareds(&entity)?.nb_entities() > 0
                || holder.sharings(&entity)?.nb_entities() > 0
            {
                live_final.add_with_refs(&entity, 0, false)?;
            }
        }
        drop(holder);
        session.set_model(&final_model.cast(), true)?;
        session.compute_graph(true)?;
        Ok(())
    }
}
