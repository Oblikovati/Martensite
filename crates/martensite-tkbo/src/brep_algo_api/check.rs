//! `BRepAlgoAPICheck`: the validity of one or two shapes as Boolean arguments, both for the
//! Boolean operation (types, small edges, self-interference) and topologically.
use crate::bop_algo::{
    BOPAlgoArgumentAnalyzer, CheckResult, CheckStatus, Operation, Options, ProgressCancellation,
    ProgressCompletion, ProgressOperation, classify_area_progress, observe_user_break,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

/// The checker of Boolean arguments.
///
/// ```
/// use martensite_tkbo::brep_algo_api::BRepAlgoAPICheck;
/// use martensite_tkbrep::topods::Shape;
/// let mut check = BRepAlgoAPICheck::new();
/// check.set_data(&Shape::default(), true, true);
/// check.perform()?;
/// // A null argument is a bad type.
/// assert!(!check.is_valid());
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub struct BRepAlgoAPICheck {
    options: Options,
    shape1: Shape,
    shape2: Shape,
    test_small_edges: bool,
    test_self_interference: bool,
    operation: Operation,
    faulty_shapes: List<CheckResult>,
}

impl Default for BRepAlgoAPICheck {
    fn default() -> Self {
        Self::new()
    }
}

impl BRepAlgoAPICheck {
    /// A checker of small edges and self-interference without shapes (`BRepAlgoAPICheck()`);
    /// `BRepAlgoAPICheck::new()`.
    pub fn new() -> Self {
        Self {
            options: Options::new(),
            shape1: Shape::default(),
            shape2: Shape::default(),
            test_small_edges: true,
            test_self_interference: true,
            operation: Operation::Unknown,
            faulty_shapes: List::default(),
        }
    }

    /// Checks one shape at once (`BRepAlgoAPICheck(S, bTestSE, bTestSI, Range)`);
    /// `BRepAlgoAPICheck::from_shape(&shape, true, true)?.is_valid()`.
    pub fn from_shape(
        shape: &Shape,
        test_small_edges: bool,
        test_self_interference: bool,
    ) -> Result<Self, Failure> {
        let mut check = Self::new();
        check.set_data(shape, test_small_edges, test_self_interference);
        check.perform()?;
        Ok(check)
    }

    /// Checks two shapes for `operation` at once
    /// (`BRepAlgoAPICheck(S1, S2, Op, bTestSE, bTestSI, Range)`);
    /// `BRepAlgoAPICheck::from_shapes(&s1, &s2, Operation::Fuse, true, true)?`.
    pub fn from_shapes(
        first: &Shape,
        second: &Shape,
        operation: Operation,
        test_small_edges: bool,
        test_self_interference: bool,
    ) -> Result<Self, Failure> {
        let mut check = Self::new();
        check.set_data_pair(
            first,
            second,
            operation,
            test_small_edges,
            test_self_interference,
        );
        check.perform()?;
        Ok(check)
    }

    /// Checks one shape at once in the caller's range (`BRepAlgoAPICheck(S, bTestSE, bTestSI, Range)`).
    /// Example: `let (check, status) = BRepAlgoAPICheck::from_shape_with_progress(&shape, true, true, &range);`.
    pub fn from_shape_with_progress(
        shape: &Shape,
        test_small_edges: bool,
        test_self_interference: bool,
        range: &ProgressRange<'_>,
    ) -> (Self, ProgressCompletion) {
        let mut check = Self::new();
        check.set_data(shape, test_small_edges, test_self_interference);
        let completion = check.perform_with_progress(range);
        (check, completion)
    }

    /// Checks two shapes for `operation` at once in the caller's range
    /// (`BRepAlgoAPICheck(S1, S2, Op, bTestSE, bTestSI, Range)`).
    /// Example: `BRepAlgoAPICheck::from_shapes_with_progress((&a, &b), Operation::Fuse, true, true, &range)`.
    pub fn from_shapes_with_progress(
        (first, second): (&Shape, &Shape),
        operation: Operation,
        test_small_edges: bool,
        test_self_interference: bool,
        range: &ProgressRange<'_>,
    ) -> (Self, ProgressCompletion) {
        let mut check = Self::new();
        check.set_data_pair(
            first,
            second,
            operation,
            test_small_edges,
            test_self_interference,
        );
        let completion = check.perform_with_progress(range);
        (check, completion)
    }

    /// The algorithm options (`BOPAlgoOptions` base); `check.options().fuzzy_value()`.
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Mutable algorithm options; `check.options_mut().set_fuzzy_value(1e-5)`.
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.options
    }

    /// Sets one shape to check and the tests, clearing the faults; the operation is kept
    /// (`SetData(S, bTestSE, bTestSI)`); `check.set_data(&shape, true, true)`.
    pub fn set_data(
        &mut self,
        shape: &Shape,
        test_small_edges: bool,
        test_self_interference: bool,
    ) {
        self.shape1 = shape.clone();
        self.shape2 = Shape::default();
        self.test_small_edges = test_small_edges;
        self.test_self_interference = test_self_interference;
        self.faulty_shapes.clear();
    }

    /// Sets two shapes, the operation and the tests, clearing the faults
    /// (`SetData(S1, S2, Op, bTestSE, bTestSI)`);
    /// `check.set_data_pair(&s1, &s2, Operation::Cut, true, true)`.
    pub fn set_data_pair(
        &mut self,
        first: &Shape,
        second: &Shape,
        operation: Operation,
        test_small_edges: bool,
        test_self_interference: bool,
    ) {
        self.set_data(first, test_small_edges, test_self_interference);
        self.shape2 = second.clone();
        self.operation = operation;
    }

    /// Analyzes the argument types, the small edges and the self-interference as configured,
    /// then appends a not-valid fault naming each topologically invalid shape (`Perform`);
    /// `check.perform()?`.
    pub fn perform(&mut self) -> Result<(), Failure> {
        let observed = ProgressCancellation::default();
        self.run_check(&ProgressRange::default(), &observed)
            .map(|_| ())
    }

    /// `perform` in the caller's range (`Perform(Range)`): the analysis takes the whole range. The
    /// completion is about execution; a cancelled check has no faults, so `is_valid` alone does
    /// not say the shapes were checked.
    /// Example: `let status = check.perform_with_progress(&indicator.start());`.
    pub fn perform_with_progress(&mut self, range: &ProgressRange<'_>) -> ProgressCompletion {
        let observed = ProgressCancellation::default();
        let (result, analysis) = match self.run_check(range, &observed) {
            Ok(analysis) => (Ok(()), analysis),
            Err(cause) => (Err(cause), ProgressCompletion::Completed),
        };
        let completion =
            classify_area_progress(ProgressOperation::Check, &self.options, result, &observed);
        match (&completion, &analysis) {
            (ProgressCompletion::Completed, ProgressCompletion::Incomplete { .. }) => analysis,
            _ => completion,
        }
    }

    /// The whole check; returns what the analysis completed with.
    fn run_check(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ProgressCompletion, Failure> {
        let mut analyzer = self.configured_analyzer();
        let scope = ProgressScope::with_literal(range, "Checking shapes", 1., false);
        let analysis = analyzer.perform_with_progress(&scope.next(1.));
        if matches!(analysis, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        if observe_user_break(&self.options, &scope, observed) {
            return Ok(analysis);
        }
        self.faulty_shapes = analyzer.check_result().clone();
        self.add_invalid_shapes()?;
        Ok(analysis)
    }

    fn configured_analyzer(&self) -> BOPAlgoArgumentAnalyzer {
        let mut analyzer = BOPAlgoArgumentAnalyzer::new();
        analyzer.set_shape1(&self.shape1);
        analyzer.set_shape2(&self.shape2);
        *analyzer.operation_type() = self.operation;
        *analyzer.argument_type_mode() = true;
        *analyzer.small_edge_mode() = self.test_small_edges;
        *analyzer.self_inter_mode() = self.test_self_interference;
        analyzer
            .options_mut()
            .set_run_parallel(self.options.run_parallel());
        analyzer
            .options_mut()
            .set_fuzzy_value(self.options.fuzzy_value());
        analyzer
    }

    /// Appends the not-valid fault naming each topologically invalid shape.
    fn add_invalid_shapes(&mut self) -> Result<(), Failure> {
        let valid1 = is_topologically_valid(&self.shape1)?;
        let valid2 = is_topologically_valid(&self.shape2)?;
        if !valid1 || !valid2 {
            let mut result = CheckResult::new();
            result.set_check_status(CheckStatus::NotValid);
            if !valid1 {
                result.set_shape1(&self.shape1);
            }
            if !valid2 {
                result.set_shape2(&self.shape2);
            }
            self.faulty_shapes.append(result);
        }
        Ok(())
    }

    /// Whether no fault was found (`IsValid`); `check.is_valid()`.
    pub fn is_valid(&self) -> bool {
        self.faulty_shapes.is_empty()
    }

    /// The faults found (`Result`); `check.result().extent()`.
    pub fn result(&self) -> &List<CheckResult> {
        &self.faulty_shapes
    }
}

/// `BRepCheckAnalyzer(S).IsValid()`, true for a null shape.
fn is_topologically_valid(shape: &Shape) -> Result<bool, Failure> {
    if shape.is_null() {
        return Ok(true);
    }
    BRepCheckAnalyzer::new(shape, true, false, false)?.is_valid()
}
