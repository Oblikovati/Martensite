//! `BOPAlgoBuilder::BuildBOP` in the caller's range: the same steps, with raw completion returned.
use super::super::operation_progress::combine;
use super::{BOPAlgoBuilder, BopRequest, operation_states};
use crate::bop_algo::alerts::alert;
use crate::bop_algo::{
    AlertBuilderFailed, BOPAlgoPaveFiller, Operation, ProgressCancellation, ProgressCompletion,
    ProgressOperation, classify_area_progress,
};
use crate::bop_ds::DataStructure;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{Gravity, ProgressRange, ProgressScope, Report};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;
use std::sync::Arc;

impl BOPAlgoBuilder {
    /// `BuildBOP` in the caller's range on the data structure of the General Fuse run: the solid
    /// building and the history take one of its two units each.
    fn build_bop_progress(
        &mut self,
        structure: &DataStructure,
        request: &BopRequest<'_>,
        range: &ProgressRange<'_>,
    ) -> ProgressCompletion {
        let observed = ProgressCancellation::default();
        let mut solid_completion = ProgressCompletion::Completed;
        let result =
            self.run_bop_progress(structure, request, range, &observed, &mut solid_completion);
        let completion = classify_area_progress(
            ProgressOperation::Builder,
            self.options(),
            result,
            &observed,
        );
        combine(solid_completion, completion)
    }

    fn run_bop_progress(
        &mut self,
        structure: &DataStructure,
        request: &BopRequest<'_>,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
        solid_completion: &mut ProgressCompletion,
    ) -> Result<(), Failure> {
        let Some(inputs) = self.bop_result_faces(structure, request)? else {
            return Ok(());
        };
        let scope = ProgressScope::unnamed(range, 2., false);
        let mut solid_builder = self.bop_solid_builder(&inputs.faces);
        *solid_completion = solid_builder.perform_with_progress(&scope.next(1.));
        if solid_builder.options().has_errors() {
            return Ok(());
        }
        self.keep_bop_solids(&solid_builder, &inputs, request)?;
        self.prepare_history_progress_raw(structure, &scope.next(1.), observed)
    }

    /// `BuildBOP(theObjects, theObjState, theTools, theToolsState, theRange, theReport)` in the
    /// caller's range, on the General Fuse result of `perform`; returns raw execution completion.
    /// Example: `builder.build_bop_with_progress(&objects, State::Out, &tools, State::In, &range, None)`.
    pub fn build_bop_with_progress(
        &mut self,
        objects: &List<Shape>,
        object_state: State,
        tools: &List<Shape>,
        tools_state: State,
        range: &ProgressRange<'_>,
        report: Option<&Arc<Report>>,
    ) -> ProgressCompletion {
        let request = BopRequest {
            objects,
            object_state,
            tools,
            tools_state,
            report,
        };
        let Some(filler) = self.filler.take() else {
            // Only the filler of `perform` is kept by the builder.
            let report = report
                .cloned()
                .unwrap_or_else(|| self.options().report().clone());
            report.add_alert(Gravity::Fail, &alert(AlertBuilderFailed));
            return classify_area_progress(
                ProgressOperation::Builder,
                self.options(),
                Ok(()),
                &ProgressCancellation::default(),
            );
        };
        let completion = match filler.data_structure() {
            Some(structure) => self.build_bop_progress(structure, &request, range),
            None => classify_area_progress(
                ProgressOperation::Builder,
                self.options(),
                Err(super::super::perform::no_structure()),
                &ProgressCancellation::default(),
            ),
        };
        self.filler = Some(filler);
        completion
    }

    /// `BuildBOP(theObjects, theTools, theOperation, theRange, theReport)`: the states of the
    /// operation, in the caller's range.
    /// Example: `builder.build_bop_operation_with_progress(&objects, &tools, Operation::Cut, &range, None)`.
    pub fn build_bop_operation_with_progress(
        &mut self,
        objects: &List<Shape>,
        tools: &List<Shape>,
        operation: Operation,
        range: &ProgressRange<'_>,
        report: Option<&Arc<Report>>,
    ) -> ProgressCompletion {
        let (object_state, tools_state) = operation_states(operation);
        self.build_bop_with_progress(objects, object_state, tools, tools_state, range, report)
    }

    /// `build_bop_operation_with_progress` after `perform_with_filler(filler)`: the Rust builder
    /// is given that filler again instead of keeping a pointer to its data structure.
    /// Example: `builder.build_bop_operation_with_filler_progress(&filler, &objects, &tools, Operation::Cut, &range, None)`.
    pub fn build_bop_operation_with_filler_progress(
        &mut self,
        filler: &BOPAlgoPaveFiller,
        request: (&List<Shape>, &List<Shape>, Operation),
        range: &ProgressRange<'_>,
        report: Option<&Arc<Report>>,
    ) -> ProgressCompletion {
        let (objects, tools, operation) = request;
        let (object_state, tools_state) = operation_states(operation);
        let request = BopRequest {
            objects,
            object_state,
            tools,
            tools_state,
            report,
        };
        match filler.data_structure() {
            Some(structure) => self.build_bop_progress(structure, &request, range),
            None => classify_area_progress(
                ProgressOperation::Builder,
                self.options(),
                Err(super::super::perform::no_structure()),
                &ProgressCancellation::default(),
            ),
        }
    }
}
