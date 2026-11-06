//! `bcut_simple`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass, view_mass_is_nonzero},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::{explode, make_box, pcone, pcylinder, prism, psphere, trotate, ttranslate},
    construction::{
        Command, copy, edge, mkplane, nurbsconvert, prism_semi_infinite, profile, vertex, wire,
    },
    one_step::{bcut, bfuse},
    properties::Prop,
    session::Session,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub use super::bcut_simple_hand::*;

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
mod part_08;
pub use part_08::*;
