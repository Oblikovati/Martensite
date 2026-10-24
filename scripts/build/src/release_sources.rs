//! The source-artifact gate is injected so pipeline tests retain independent named fakes.
use crate::{
    archive_graph::{self, ArchiveGraph},
    context::BuildContext,
    error::BuildResult,
    publication_plan::PublicationPlan,
    release::ReleaseRequest,
    source_sdk::{self, SdkRecord},
    workspace::CargoWorkspace,
};

/// Verified SDK evidence paired with exact individual-archive requirements.
pub(crate) struct SourceArtifacts {
    pub sdk: SdkRecord,
    pub graph: ArchiveGraph,
}

/// Owns the source-artifact I/O boundary used by release packaging.
pub(crate) trait SourceRelease {
    /// Verifies source assets before a catalog can name the SDK as its supported route.
    /// Example: `let source = verifier.prepare(context, request, workspace, plan)?;`.
    fn prepare(
        &self,
        context: &BuildContext<'_>,
        request: &ReleaseRequest,
        workspace: &CargoWorkspace,
        plan: &PublicationPlan,
    ) -> BuildResult<SourceArtifacts>;
}

/// Production source verification; named fakes replace it only in pipeline tests.
pub(crate) struct VerifiedSourceRelease;

impl SourceRelease for VerifiedSourceRelease {
    fn prepare(
        &self,
        context: &BuildContext<'_>,
        request: &ReleaseRequest,
        workspace: &CargoWorkspace,
        plan: &PublicationPlan,
    ) -> BuildResult<SourceArtifacts> {
        let graph = archive_graph::inspect(request, workspace, plan)?;
        let sdk = source_sdk::create(context, request)?;
        Ok(SourceArtifacts { sdk, graph })
    }
}
