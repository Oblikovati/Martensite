//! Hidden-line regression cases: each case body (`cases/`) follows its script line by
//! line on the Rust API and writes the test log the harness would see; the log then gets
//! the verdict of the bugs and root `parse.rules`. Each case runs alone in a child process
//! under the bugs group's 1000 s budget.
//!
//! These tests check the kernel results with [`support::session::NoView`], which skips
//! the view steps. The inputs come from the `external/martensite-test-data` submodule
//! (see `support/dataset.rs`); without it the cases fail and name
//! `git submodule update --init`.

mod cases;
mod support;

mod bugs_modalg_6;
