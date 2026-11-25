// SPDX-License-Identifier: GPL-2.0-only
//! Source-bound census certificates do not certify the mapped candidate search.
mod candidate_coverage;
mod capture;
mod census;
mod coverage;
mod outcome;
mod pair_root_failure;
mod work_receipt;
pub(crate) use candidate_coverage::inspect_candidate_coverage;
pub(crate) use capture::capture_boundary_inputs;
pub(crate) use census::{enumerate_boundary_census, enumerate_source_bound_line_diagnostic};
pub(crate) use pair_root_failure::pair_root_failure;
pub use work_receipt::CensusWorkReceipt;
mod bernstein_binding;
mod bernstein_execution;
mod certified_contact;
mod conditioned_stage;
mod domain_certificate;
mod graph_policy;
mod paired_admission;
mod paired_binding;
mod paired_execution;
mod paired_types;
pub(crate) use certified_contact::{CertifiedDomainContact, ContactProofKey};
pub(crate) use coverage::inspect_contact_coverage;
pub(crate) use domain_certificate::CertifiedPairDomain;
pub(crate) use graph_policy::{GraphPolicyMismatch, bind_graph_policy};
pub(crate) use outcome::*;
pub(crate) use paired_admission::{
    enumerate_carrier_census, enumerate_exclusion_census, enumerate_filtered_exclusion_census,
    enumerate_source_bound_census,
};
#[cfg(test)]
mod tests;

mod coverage_status;
mod incidence_binding;
mod parameter_binding;
mod source_status;

#[cfg(test)]
mod binding_mutation_tests;
#[cfg(test)]
mod source_binding_tests;

#[cfg(test)]
mod coverage_binding_tests;

#[cfg(test)]
mod live_geometry_tests;

#[cfg(test)]
mod paired_tests;

#[cfg(test)]
mod paired_retention_tests;

#[cfg(test)]
mod paired_native_tests;

#[cfg(test)]
mod paired_domain_tests;

#[cfg(test)]
mod paired_constant_tests;

pub(crate) use paired_binding::{DomainCertificates, domain_certificate};

#[cfg(test)]
pub(crate) use paired_tests::{
    bezier_shape as census_bezier_fixture, curved_pair as census_curved_pair_fixture,
};

#[cfg(test)]
mod paired_validation_tests;

#[cfg(test)]
mod bernstein_tests;

#[cfg(test)]
mod filtered_tests;
mod source_access;
pub(crate) use source_access::{SourceAccess, certificate_in, unified_certificate_in};

pub(crate) use paired_admission::{
    enumerate_carrier_census_with_access, enumerate_exclusion_census_with_access,
    enumerate_filtered_exclusion_census_with_access, enumerate_geometric_census_with_access,
};

#[cfg(test)]
mod geometric_tests;
