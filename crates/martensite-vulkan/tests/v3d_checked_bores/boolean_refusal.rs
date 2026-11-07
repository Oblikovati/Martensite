//! Private typed refusals and report ownership, independent of geometry admission.
use super::super::{boolean_coverage, boolean_history};
use martensite_tkbo::bop_algo::{Operation, ProgressCompletion};
use martensite_tkernel::standard::Failure;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub(in super::super) enum OperandAuthority {
    ProceduralSource,
    FontProviderUnobserved,
}

#[derive(Clone, Copy, Debug)]
pub(in super::super) struct BooleanRequest {
    pub case: &'static str,
    pub ordinal: usize,
    pub operation: Operation,
    pub operands: [&'static str; 2],
    pub authority: OperandAuthority,
}

#[derive(Debug)]
pub(in super::super) enum BooleanCause {
    Kernel(Failure),
    Execution(ProgressCompletion),
    Validation {
        entity: String,
        expected: &'static str,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) enum RefusalClass {
    Failed,
    Cancelled,
    Incomplete,
    CompletedEmpty,
}

#[derive(Debug)]
pub(in super::super) struct BooleanIssue {
    pub request: Box<BooleanRequest>,
    pub stage: &'static str,
    pub class: RefusalClass,
    pub cause: Box<BooleanCause>,
    history_observation: Option<Box<boolean_history::HistoryObservation>>,
    diagnostics: Box<KernelDiagnostics>,
    coverage: Box<boolean_coverage::OperationCoverage>,
    lineage: Box<LineageRetention>,
}

/// Prior and immediate successor observations are not claims of composed final ancestry.
#[derive(Clone, Debug, Default)]
pub(in super::super) struct LineageRetention {
    pub prior: Option<boolean_history::HistoryObservation>,
    pub prior_coverage: Option<boolean_coverage::OperationCoverage>,
    pub successor: Option<boolean_history::HistoryObservation>,
    pub successor_state: &'static str,
    pub composition: &'static str,
}

impl fmt::Display for BooleanCause {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kernel(cause) => write!(
                output,
                "{}: {}",
                cause.exception_type(),
                String::from_utf8_lossy(cause.what())
            ),
            Self::Execution(status) => write!(output, "{status:?}"),
            Self::Validation { entity, expected } => {
                write!(output, "entity={entity}; expected {expected}")
            }
        }
    }
}

impl fmt::Display for BooleanIssue {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "Error: case={} operands={:?} operation={:?} invocation={} stage={} class={:?} cause={}; expected completed, checked nonempty body",
            self.request.case,
            self.request.operands,
            self.request.operation,
            self.request.ordinal,
            self.stage,
            self.class,
            self.cause
        )
    }
}
impl std::error::Error for BooleanIssue {}

#[derive(Clone, Debug, Default)]
pub(in super::super) struct KernelDiagnostics {
    pub warnings: String,
    pub errors: String,
}

impl KernelDiagnostics {
    pub(in super::super) fn followed_by(mut self, next: Self) -> Self {
        self.warnings.push_str(&next.warnings);
        self.errors.push_str(&next.errors);
        self
    }
}

impl BooleanIssue {
    pub(in super::super) fn lineage(&self) -> &LineageRetention {
        &self.lineage
    }
    pub(super) fn with_lineage(mut self, lineage: LineageRetention) -> Self {
        self.lineage = Box::new(lineage);
        self
    }

    pub(in super::super) fn coverage(&self) -> &boolean_coverage::OperationCoverage {
        &self.coverage
    }
    pub(in super::super) fn with_coverage(
        mut self,
        coverage: boolean_coverage::OperationCoverage,
    ) -> Self {
        self.coverage = Box::new(coverage);
        self
    }

    pub(in super::super) fn diagnostics(&self) -> &KernelDiagnostics {
        // Native report text remains available even when no shape can be returned.
        &self.diagnostics
    }
    pub(super) fn prepend_diagnostics(mut self, previous: KernelDiagnostics) -> Self {
        self.diagnostics = Box::new(previous.followed_by(*self.diagnostics));
        self
    }
    pub(super) fn with_diagnostics(mut self, diagnostics: KernelDiagnostics) -> Self {
        self.diagnostics = Box::new(diagnostics);
        self
    }
    pub(in super::super) fn history_observation(
        &self,
    ) -> Option<&boolean_history::HistoryObservation> {
        // Refusals retain diagnostic values without any reachable geometry handles.
        self.history_observation.as_deref()
    }
    pub(in super::super) fn with_history(
        mut self,
        snapshot: Option<boolean_history::HistoryObservation>,
    ) -> Self {
        self.history_observation = snapshot.map(Box::new);
        self
    }
}

pub(in super::super) fn kernel_issue(
    request: BooleanRequest,
    stage: &'static str,
    cause: Failure,
) -> BooleanIssue {
    BooleanIssue {
        request: Box::new(request),
        stage,
        class: if cause.exception_type() == "BOP4_IncompleteCoverage" {
            RefusalClass::Incomplete
        } else {
            RefusalClass::Failed
        },
        cause: Box::new(BooleanCause::Kernel(cause)),
        history_observation: None,
        diagnostics: Box::default(),
        coverage: Box::default(),
        lineage: Box::default(),
    }
}

pub(in super::super) fn refused(
    request: BooleanRequest,
    stage: &'static str,
    entity: String,
    expected: &'static str,
    class: RefusalClass,
) -> BooleanIssue {
    BooleanIssue {
        request: Box::new(request),
        stage,
        class,
        cause: Box::new(BooleanCause::Validation { entity, expected }),
        history_observation: None,
        diagnostics: Box::default(),
        coverage: Box::default(),
        lineage: Box::default(),
    }
}

pub(in super::super) fn execution_status(
    request: BooleanRequest,
    stage: &'static str,
    status: ProgressCompletion,
) -> Result<(), BooleanIssue> {
    let class = match &status {
        ProgressCompletion::Completed => return Ok(()),
        ProgressCompletion::Cancelled { .. } => RefusalClass::Cancelled,
        ProgressCompletion::Failed { .. } => RefusalClass::Failed,
        ProgressCompletion::Incomplete { .. } => RefusalClass::Incomplete,
    };
    Err(BooleanIssue {
        request: Box::new(request),
        stage,
        class,
        cause: Box::new(BooleanCause::Execution(status)),
        history_observation: None,
        diagnostics: Box::default(),
        coverage: Box::default(),
        lineage: Box::default(),
    })
}
