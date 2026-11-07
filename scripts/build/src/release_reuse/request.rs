//! Trusted current-checkout inputs shared by discovery, sealing and restoration.
use super::{api, exact_head, receipt, repository, scope::ScopeBinding};
use crate::{context::BuildContext, error::BuildResult};
use std::collections::BTreeSet;

pub(super) struct ValidationRequest {
    pub repo: String,
    pub head: String,
    pub version: semver::Version,
    pub scope: ScopeBinding,
    pub jobs: BTreeSet<String>,
}

impl ValidationRequest {
    pub(super) fn tag(context: &BuildContext<'_>, version: semver::Version) -> BuildResult<Self> {
        let head = exact_head(context, &version)?;
        Self::load(context, version, head)
    }

    pub(super) fn rehearsal(
        context: &BuildContext<'_>,
        version: semver::Version,
    ) -> BuildResult<Self> {
        Self::load(context, version, api::git(context, "HEAD^{commit}")?)
    }

    fn load(
        context: &BuildContext<'_>,
        version: semver::Version,
        head: String,
    ) -> BuildResult<Self> {
        Ok(Self {
            repo: repository()?,
            head,
            scope: ScopeBinding::load(context, &version)?,
            jobs: receipt::required_jobs(context, &version)?,
            version,
        })
    }
}
