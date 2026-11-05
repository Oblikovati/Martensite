//! `bugs_modalg_2`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::{checkshape_named, checkshape_top},
    checkview::{CheckView, checkview},
    commands::{pcylinder, trotate},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcommon, bcut, bfuse, bsection},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

pub use super::bugs_modalg_2_hand::*;
pub use super::bugs_modalg_2_hand_2::*;
pub use super::bugs_modalg_2_hand_9::*;
pub use super::bugs_modalg_2_hand_10::*;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
