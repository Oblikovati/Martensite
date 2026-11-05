//! Boolean operations: the general fuse and its builders, the intersection tools of [`int_tools`],
//! splitting, defeaturing and the checks of their results.
pub mod body_check;
pub mod bop_algo;
pub mod bop_ds;
pub mod bop_tools;
pub mod brep_algo_api;
pub mod int_tools;
pub mod measured;
pub mod profile_extrusion;

#[allow(dead_code)]
// Owned snapshots serve private stages until operation acceptance.
mod sketch_region_geometry_capture;
#[allow(dead_code, unused_imports)] // Proof stage cannot expose an uncertified healthy region.
mod sketch_region_source_conformance;
#[allow(dead_code)] // Private partial stage, not a successful region API.
mod sketch_region_validation;

// Keeps this prerequisite internal until the complete owned-region contract passes.
#[allow(dead_code, unused_imports)]
pub(crate) mod sketch_region;

// Candidate diagnostics cannot expose an uncertified healthy body.
#[allow(dead_code, unused_imports)]
pub(crate) mod sketch_region_candidates;

#[cfg(test)]
#[path = "../tests/support/changing_child_count.rs"]
mod changing_child_count;

#[cfg(test)]
#[path = "../tests/support/closed_analytic_cubic.rs"]
pub(crate) mod closed_analytic_cubic;

#[allow(dead_code, unused_imports)]
mod sketch_region_certification;

// Owned binding evidence cannot promote incomplete contact/search coverage.
#[allow(dead_code, unused_imports)]
mod sketch_region_partition_bindings;
// Retained diagnostic prism history cannot publish an uncertified healthy body.
#[allow(dead_code, unused_imports)]
mod sketch_region_prism;

// Retains contact construction gaps until independent whole-operation acceptance.
#[allow(dead_code)]
mod sketch_region_contact_transport;

mod sketch_region_vertex_observation;
