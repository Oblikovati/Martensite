//! The foundation of the data exchange formats: interface models of entities with their checks,
//! generic libraries keyed by protocol, transfer processes with their binders and actors, graphs
//! of entities, selections and work sessions.
mod attr_core;
mod c_text;
pub mod if_graph;
pub mod if_select;
pub mod interface;
pub mod lib_ctl;
pub mod moni_tool;
mod ostream_text;
pub mod transfer;
pub mod transfer_brep;
pub mod xs_algo;
pub mod xs_control;
pub mod xs_message;

#[cfg(test)]
mod declared_base_tests;
