//! Owned query results retain actual API metadata rather than inferred shader order.
use ash::vk;

/// Bind an actual stage and module handle to its immutable creation bytes; `input.2`.
pub(crate) type ExecutableModuleInput = (u32, u64, &'static [u8]);

#[derive(Clone, Debug, PartialEq, Eq)]
/// Keep every reported stage/name; `property.index` is the API lookup index.
pub(crate) struct ExecutableProperty {
    pub index: u32,
    pub stages: u32,
    pub name: String,
    pub description: String,
    pub subgroup_size: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Retain driver labels and size separately; `metadata.bytes` bounds owned storage.
pub(crate) struct RepresentationMetadata {
    pub name: String,
    pub description: String,
    pub text: bool,
    pub bytes: usize,
}
/// Keep opaque driver bytes unchanged; `representation.bytes` can be hashed independently.
pub(crate) struct OwnedRepresentation {
    pub metadata: RepresentationMetadata,
    pub bytes: Vec<u8>,
}
#[derive(Debug)]
/// Bind complete returned IR to the queried pipeline; `witness.selected` records the reported executable.
pub(crate) struct ExecutableWitness {
    pub pipeline: u64,
    pub properties: Vec<ExecutableProperty>,
    pub selected: u32,
    pub representations: Vec<OwnedRepresentation>,
    pub raw: super::raw::RawQueryEvidence,
}
#[derive(Debug)]
/// Keep refusal causes and incomplete witnesses typed; `ExecutableGap::MissingAssembly(witness)` is not completion.
pub(crate) enum ExecutableGap {
    Capability {
        physical: u64,
        extension: bool,
        feature: u32,
    },
    Backend(crate::RenderError),
    Api {
        operation: &'static str,
        pipeline: u64,
        result: vk::Result,
    },
    Contract {
        field: &'static str,
        actual: String,
        expected: &'static str,
    },
    AmbiguousStage(Vec<ExecutableProperty>),
    MissingAssembly(Box<ExecutableWitness>),
    ModuleInputs {
        evidence: Box<super::raw::RawModuleInputs>,
        cause: Box<ExecutableGap>,
    },
    PartialQuery {
        evidence: Box<super::raw::RawQueryEvidence>,
        cause: Box<ExecutableGap>,
    },
    CapturedFrame {
        frame: Box<super::CapturedExecutableFrame>,
        cause: Box<ExecutableGap>,
    },
    BoundPipeline {
        pipeline: u64,
        cause: Box<ExecutableGap>,
    },
}
impl From<crate::RenderError> for ExecutableGap {
    fn from(error: crate::RenderError) -> Self {
        // Device-loss causes stay owned; inspection must not erase the renderer failure.
        Self::Backend(error)
    }
}
/// Name the offending value and expected shape; `contract("count", 0, "positive")`.
pub(super) fn contract(
    field: &'static str,
    actual: impl std::fmt::Debug,
    expected: &'static str,
) -> ExecutableGap {
    ExecutableGap::Contract {
        field,
        actual: format!("{actual:?}"),
        expected,
    }
}

impl std::fmt::Debug for OwnedRepresentation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // A missing-ISA failure must not expand an8MiB payload into a larger panic log.
        formatter
            .debug_struct("OwnedRepresentation")
            .field("metadata", &self.metadata)
            .field("retained_bytes", &self.bytes.len())
            .finish()
    }
}
