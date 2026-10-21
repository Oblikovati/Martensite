//! tests/demo/draw (demo group, no `begin` script); bodies in `cases/demo_draw.rs`.
use crate::support::process::{Group, draw_cases, run_case};

draw_cases!(run_case, Group::Demo, demo_draw; bug31908);
