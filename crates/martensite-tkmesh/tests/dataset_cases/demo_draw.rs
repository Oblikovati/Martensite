//! Demonstration fixtures without group setup or teardown.
use crate::support::process::{Group, draw_cases, run_case};

draw_cases!(run_case, Group::Demo, demo_draw; bug31908);
