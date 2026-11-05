//! `BRepAlgoAPIBooleanOperation`: the API of the Boolean operations and the section.
use super::builder_algo::BRepAlgoAPIBuilderAlgo;
use crate::bop_algo::alerts::alert;
use crate::bop_algo::{
    AlertBOPNotSet, AlertTooFewArguments, BOPAlgoBOP, BOPAlgoPaveFiller, BOPAlgoSection,
    GeneralFuseAlgorithm, Operation, ProgressCancellation, ProgressCompletion, ProgressOperation,
    classify_area_progress, progress_result::retain_intersection_completion,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{AlertHandle, ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use std::ops::{Deref, DerefMut};

/// A Boolean operation (or section) of the arguments (objects) and the tools.
///
/// ```
/// use martensite_tkbo::bop_algo::Operation;
/// use martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation;
/// let mut bop = BRepAlgoAPIBooleanOperation::new();
/// bop.set_operation(Operation::Fuse);
/// assert_eq!(bop.operation(), Operation::Fuse);
/// ```
pub struct BRepAlgoAPIBooleanOperation {
    builder_algo: BRepAlgoAPIBuilderAlgo,
    tools: List<Shape>,
    operation: Operation,
}

impl Default for BRepAlgoAPIBooleanOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for BRepAlgoAPIBooleanOperation {
    type Target = BRepAlgoAPIBuilderAlgo;
    fn deref(&self) -> &BRepAlgoAPIBuilderAlgo {
        &self.builder_algo
    }
}

impl DerefMut for BRepAlgoAPIBooleanOperation {
    fn deref_mut(&mut self) -> &mut BRepAlgoAPIBuilderAlgo {
        &mut self.builder_algo
    }
}

impl BRepAlgoAPIBooleanOperation {
    /// An operation of unknown type (`BRepAlgoAPIBooleanOperation()`).
    /// `BRepAlgoAPIBooleanOperation::new()`.
    pub fn new() -> Self {
        Self::with_builder(BRepAlgoAPIBuilderAlgo::new(), Operation::Unknown)
    }

    /// An operation on the results of a performed `filler`
    /// (`BRepAlgoAPIBooleanOperation(thePF)`); `BRepAlgoAPIBooleanOperation::with_filler(pf)`.
    pub fn with_filler(filler: BOPAlgoPaveFiller) -> Self {
        Self::with_builder(
            BRepAlgoAPIBuilderAlgo::with_filler(filler),
            Operation::Unknown,
        )
    }

    /// The operation of `object` and `tool`, intersected by an own filler or by `filler`
    /// (the protected constructors).
    pub(crate) fn with_shapes(
        object: &Shape,
        tool: &Shape,
        filler: Option<BOPAlgoPaveFiller>,
        operation: Operation,
    ) -> Self {
        let builder_algo = match filler {
            Some(filler) => BRepAlgoAPIBuilderAlgo::with_filler(filler),
            None => BRepAlgoAPIBuilderAlgo::new(),
        };
        let mut boolean = Self::with_builder(builder_algo, operation);
        boolean.builder_algo.arguments.append(object.clone());
        boolean.tools.append(tool.clone());
        boolean
    }

    fn with_builder(builder_algo: BRepAlgoAPIBuilderAlgo, operation: Operation) -> Self {
        Self {
            builder_algo,
            tools: List::default(),
            operation,
        }
    }

    /// The first argument (`Shape1`); `bop.shape1()`.
    pub fn shape1(&self) -> Option<&Shape> {
        self.builder_algo.arguments.iter().next()
    }

    /// The first tool (`Shape2`); `bop.shape2()`.
    pub fn shape2(&self) -> Option<&Shape> {
        self.tools.iter().next()
    }

    /// Replaces the tools by a copy of `tools` (`SetTools`); `bop.set_tools(&tools)`.
    pub fn set_tools(&mut self, tools: &List<Shape>) {
        self.tools = tools.clone();
    }

    /// The tools (`Tools`); `bop.tools().len()`.
    pub fn tools(&self) -> &List<Shape> {
        &self.tools
    }

    /// Sets the operation (`SetOperation`); `bop.set_operation(Operation::Cut)`.
    pub fn set_operation(&mut self, operation: Operation) {
        self.operation = operation;
    }

    /// The operation (`Operation`); `bop.operation()`.
    pub fn operation(&self) -> Operation {
        self.operation
    }

    /// Performs the operation (`Build`): the arguments and tools intersected together, then
    /// the Boolean operation or the section on the results; `bop.build()?`.
    pub fn build(&mut self) -> Result<(), Failure> {
        self.builder_algo.base.set_not_done();
        self.builder_algo.clear();
        if self.builder_algo.arguments.is_empty() || self.tools.is_empty() {
            self.builder_algo
                .options
                .add_error(&alert(AlertTooFewArguments));
            return Ok(());
        }
        if self.operation == Operation::Unknown {
            self.builder_algo.options.add_error(&alert(AlertBOPNotSet));
            return Ok(());
        }
        if self.builder_algo.is_intersection_needed {
            let mut shapes = self.builder_algo.arguments.clone();
            for tool in self.tools.iter() {
                shapes.append(tool.clone());
            }
            self.builder_algo.intersect_shapes(&shapes)?;
            if self.has_errors() {
                return Ok(());
            }
        }
        self.builder_algo.builder = Some(self.operation_builder());
        self.builder_algo.build_result()
    }

    /// `build` in the caller's range (`Build(Range)`): with an own intersection 70 of its 100 units
    /// go to it, and 30 go to the result. The completion is about execution, not validity.
    /// Example: `let status = bop.build_with_progress(&indicator.start());`.
    pub fn build_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        self.builder_algo.base.set_not_done();
        self.builder_algo.clear();
        if self.builder_algo.arguments.is_empty() || self.tools.is_empty() {
            return self.refused(alert(AlertTooFewArguments));
        }
        let Some(name) = operation_scope_name(self.operation) else {
            return self.refused(alert(AlertBOPNotSet));
        };
        let units = self.builder_algo.operation_units();
        let scope = ProgressScope::with_literal(range, name, units, false);
        let intersection = self
            .builder_algo
            .intersect_with_tools_progress(&self.tools, &scope);
        if self.has_errors() {
            return intersection;
        }
        self.builder_algo.builder = Some(self.operation_builder());
        let built = self.builder_algo.build_result_progress(&scope.next(30.));
        retain_intersection_completion(intersection, built)
    }

    /// Records a refusal before any work starts and classifies it.
    fn refused(&mut self, failure: AlertHandle) -> ProgressCompletion {
        self.builder_algo.options.add_error(&failure);
        classify_area_progress(
            ProgressOperation::BooleanApi,
            &self.builder_algo.options,
            Ok(()),
            &ProgressCancellation::default(),
        )
    }

    /// The section on the filler arguments, or the Boolean operation of the arguments and
    /// the tools.
    fn operation_builder(&self) -> Box<dyn GeneralFuseAlgorithm> {
        if self.operation == Operation::Section {
            let mut section = BOPAlgoSection::new();
            if let Some(filler) = self.builder_algo.ds_filler() {
                section.set_arguments(filler.arguments());
            }
            return Box::new(section);
        }
        let mut bop = BOPAlgoBOP::new();
        bop.set_arguments(&self.builder_algo.arguments);
        bop.set_tools(&self.tools);
        bop.set_operation(self.operation);
        Box::new(bop)
    }
}

/// The native name of the scope of one operation, none for an unset operation.
fn operation_scope_name(operation: Operation) -> Option<&'static str> {
    match operation {
        Operation::Common => Some("Performing COMMON operation"),
        Operation::Fuse => Some("Performing FUSE operation"),
        Operation::Cut | Operation::Cut21 => Some("Performing CUT operation"),
        Operation::Section => Some("Performing SECTION operation"),
        Operation::Unknown => None,
    }
}
