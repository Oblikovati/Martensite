//! One executable per toolkit keeps repeated linkage out of Martensite tests.
#![allow(clippy::module_inception, clippy::duplicate_mod)] // Preserve case identity and fixture isolation.
include!("suites/cases_00.rs");

#[path = "ais_light_selection_reference.rs"]
mod ais_light_selection_reference;

#[path = "animation_getter_retention.rs"]
mod animation_getter_retention;

#[path = "support/font_availability_disposition.rs"]
mod font_availability_disposition;
