//! `bugs_modalg_1`: each statement is one line of its script.
use crate::support::{
    beyond_script::printed_mass,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, make_box, pcylinder, ttranslate},
    construction::tcopy,
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_named},
    one_step::{bcommon, bcut, bfuse, bsection},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::mkface,
    topology_commands::{plane_along, plane_axes},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub use super::bugs_modalg_1_hand::*;
pub use super::bugs_modalg_1_hand_2::*;
pub use super::bugs_modalg_1_hand_3::*;
pub use super::bugs_modalg_1_hand_9::*;
pub use super::bugs_modalg_1_hand_10::*;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
mod part_03;
pub use part_03::*;
mod part_04;
pub use part_04::*;
