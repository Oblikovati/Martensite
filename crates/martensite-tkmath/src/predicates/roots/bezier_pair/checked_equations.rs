// SPDX-License-Identifier: GPL-2.0-only
//! Establish both regular source domains before deriving cleared pair equations.
use super::super::RootIsolationOptions;
use super::{
    equations::PairedEquations,
    operation_failure::{PairOperationFailure, source_domain_failure},
    source::{ObservedHomogeneous, OwnedBezierSource},
    source_certificate::{self, SourceCertificate},
    work_budget::PairWorkBudget,
};

pub(super) type CheckedPairEquations = (PairedEquations, [SourceCertificate; 2]);

pub(super) fn checked_equations(
    sources: &[OwnedBezierSource; 2],
    self_domain: bool,
    options: RootIsolationOptions,
    budget: &PairWorkBudget,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<CheckedPairEquations, PairOperationFailure> {
    checked(
        sources.each_ref().map(DomainInput::Literal),
        self_domain,
        options,
        budget,
        cancelled,
    )
}

pub(super) fn checked_observed_equations(
    sources: [ObservedHomogeneous<'_>; 2],
    options: RootIsolationOptions,
    budget: &PairWorkBudget,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<CheckedPairEquations, PairOperationFailure> {
    checked(
        sources.map(DomainInput::Observed),
        false,
        options,
        budget,
        cancelled,
    )
}

enum DomainInput<'a> {
    Literal(&'a OwnedBezierSource),
    Observed(ObservedHomogeneous<'a>),
}

impl DomainInput<'_> {
    fn certify(
        self,
        index: usize,
        options: RootIsolationOptions,
        budget: &PairWorkBudget,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<SourceCertificate, PairOperationFailure> {
        match self {
            Self::Literal(source) => {
                source_certificate::certify(source, options, budget, cancelled)
            }
            Self::Observed(source) => {
                source_certificate::certify_observed(source, options, budget, cancelled)
            }
        }
        .map_err(|cause| source_domain_failure(index, cause))
    }
}

fn checked(
    sources: [DomainInput<'_>; 2],
    self_domain: bool,
    options: RootIsolationOptions,
    budget: &PairWorkBudget,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<CheckedPairEquations, PairOperationFailure> {
    let [left, right] = sources;
    let left = left.certify(0, options, budget, cancelled)?;
    let right = right.certify(1, options, budget, cancelled)?;
    let equations = PairedEquations::derive(&left.homogeneous, &right.homogeneous, cancelled)
        .map_err(PairOperationFailure::Root)?;
    let equations = if self_domain {
        equations
            .exclude_diagonal(cancelled)
            .map_err(PairOperationFailure::Diagonal)?
    } else {
        equations
    };
    Ok((equations, [left, right]))
}
