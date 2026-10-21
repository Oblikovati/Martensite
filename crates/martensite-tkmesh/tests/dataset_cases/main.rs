//! The TKMesh-owned the retained behavior scenario cases as native Rust tests: the same inputs, assertions and
//! tolerances, without Tcl, scenario or the retained behavior at run time. Each case body (`cases/`) follows its
//! script line by line on the Rust API and writes the test log the harness would see; the
//! log then gets the verdict of the grid, group and root `parse.rules`. A case of a group
//! whose `begin` script sets a `cpulimit` runs in a child process under that limit.
//!
//! bugs/modalg_2/bug369 and bugs/moddata_1/bug14376 restore inputs (OCC369.brep,
//! OCC14376.brep) that are in no public source; they are recorded in
//! implementation/scope-decisions/2026-10-20-unavailable-public-inputs.json.

mod cases;
mod support;

mod bugs_moddata_1;
mod demo_draw;
