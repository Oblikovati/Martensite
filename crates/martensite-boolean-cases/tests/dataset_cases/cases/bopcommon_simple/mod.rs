//! `bopcommon_simple`: the common of the operands, in two steps. Each script builds its operands, runs
//! `bop` and the result command, checks the area and views the result or the operands.
#[allow(unused_imports)] // Each part uses the commands of its scripts.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::*,
    named_shapes::tcopy,
    properties::Prop,
    session::Session,
    topology_commands::*,
    two_step::{TwoStepBoolean, operands_checked},
};
#[allow(unused_imports)]
use martensite_tkbo::bop_algo::Operation;
#[allow(unused_imports)]
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
#[allow(unused_imports)]
use std::f64::consts::PI;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
mod part_03;
pub use part_03::*;
mod part_04;
pub use part_04::*;
mod part_05;
pub use part_05::*;
mod part_06;
pub use part_06::*;
mod part_07;
pub use part_07::*;
