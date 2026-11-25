// SPDX-License-Identifier: GPL-2.0-only
//! Area displays retain their full selected numerical receipt.
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedIntegralProgram, CertifiedRationalMeasure,
};
use std::sync::Arc;
#[derive(Clone, PartialEq)]
pub(crate) enum GraphArea {
    Legacy(CertifiedRationalMeasure),
    Program(Arc<CertifiedIntegralProgram>),
}
impl GraphArea {
    pub(crate) fn representative(&self) -> f64 {
        match self {
            Self::Legacy(proof) => proof.representative(),
            Self::Program(proof) => proof.representative(),
        }
    }
    pub(crate) fn absolute_error_bound(&self) -> f64 {
        match self {
            Self::Legacy(proof) => proof.absolute_error_bound(),
            Self::Program(proof) => proof.absolute_error_bound(),
        }
    }
}
impl std::fmt::Debug for GraphArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Legacy(proof) => proof.fmt(f),
            Self::Program(proof) => proof.fmt(f),
        }
    }
}
