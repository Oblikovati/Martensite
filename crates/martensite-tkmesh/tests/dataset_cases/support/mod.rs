//! Fixture logs, verdict rules, group CPU budgets and geometric quality checks.
#![allow(dead_code)] // Each fixture uses only its required command subset.

pub mod draw;
#[path = "geomalgo/log.rs"]
pub mod log;
pub mod memory;
pub mod mesh_export;
#[path = "geomalgo/pattern.rs"]
pub mod pattern;
pub mod process;
pub mod qa;
