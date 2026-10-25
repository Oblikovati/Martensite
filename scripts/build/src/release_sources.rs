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

/// Owns the synchronous, driver-local source-artifact I/O boundary (#279).
///
/// Trait objects stay on the calling driver thread: `prepare` borrows a build
/// context whose injected runner can be a recording fake with interior mutability.
/// Requiring `Send` or `Sync` here would change that injection contract. Owned
/// output evidence and the production verifier have separate structural assertions.
/// Example: `let verifier: &dyn SourceRelease = &VerifiedSourceRelease;`.
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
