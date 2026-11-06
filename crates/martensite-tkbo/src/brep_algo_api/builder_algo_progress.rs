//! `BRepAlgoAPIBuilderAlgo` in the caller's range: the same steps, with raw completion returned.
use super::BRepAlgoAPIBuilderAlgo;
use crate::bop_algo::{
    BOPAlgoBuilder, ProgressCompletion, ProgressFailureClass, ProgressOperation, ProgressStage,
    progress_result::retain_intersection_completion,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::List;

impl BRepAlgoAPIBuilderAlgo {
    /// Splits the arguments in the caller's range (`Build(Range)`): 70 of its 100 units go to the
    /// intersection and 30 to the result. The completion is about execution, not validity.
    /// Example: `let status = builder.build_with_progress(&indicator.start());`.
    pub fn build_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        self.base.set_not_done();
        self.clear();
        let scope =
            ProgressScope::with_literal(range, "Performing General Fuse operation", 100., false);
        let arguments = self.arguments.clone();
        let intersection = self.intersect_shapes_progress(&arguments, &scope.next(70.));
        if self.has_errors() {
            return intersection;
        }
        let mut general_fuse = BOPAlgoBuilder::new();
        general_fuse.set_arguments(&arguments);
        self.builder = Some(Box::new(general_fuse));
        let built = self.build_result_progress(&scope.next(30.));
        retain_intersection_completion(intersection, built)
    }

    /// `intersect_shapes` in the caller's range; `builder.intersect_shapes_progress(&args, range)`.
    pub(crate) fn intersect_shapes_progress(
        &mut self,
        arguments: &List<Shape>,
        range: &ProgressRange<'_>,
    ) -> ProgressCompletion {
        if !self.is_intersection_needed {
            return ProgressCompletion::Completed;
        }
        let mut filler = self.configured_filler(arguments);
        let completion = filler.perform_with_progress(range);
        self.adopt_intersection(filler);
        completion
    }

    /// The units of a Boolean or split scope: 100 with an own intersection, 30 without.
    pub(crate) fn operation_units(&self) -> f64 {
        if self.is_intersection_needed {
            100.
        } else {
            30.
        }
    }

    /// Intersects the arguments and `tools` together on 70 units of `scope`, when needed.
    pub(crate) fn intersect_with_tools_progress(
        &mut self,
        tools: &List<Shape>,
        scope: &ProgressScope<'_>,
    ) -> ProgressCompletion {
        if !self.is_intersection_needed {
            return ProgressCompletion::Completed;
        }
        let mut shapes = self.arguments.clone();
        for tool in tools.iter() {
            shapes.append(tool.clone());
        }
        self.intersect_shapes_progress(&shapes, &scope.next(70.))
    }

    /// `build_result` in the caller's range; `builder.build_result_progress(range)`.
    pub(crate) fn build_result_progress(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> ProgressCompletion {
        self.configure_general_fuse();
        let (Some(builder), Some(filler)) = (self.builder.as_mut(), self.ds_filler.as_mut()) else {
            return ProgressCompletion::Completed;
        };
        let built = builder.perform_with_filler_on_progress(filler, range);
        let retained = self.retain_general_fuse_result();
        match built.and_then(|completion| retained.map(|()| completion)) {
            Ok(completion) => completion,
            Err(cause) => ProgressCompletion::Failed {
                operation: ProgressOperation::BooleanApi,
                stage: ProgressStage::Completion,
                class: ProgressFailureClass::KernelFailure,
                cause: Some(cause),
                context: None,
            },
        }
    }
}
