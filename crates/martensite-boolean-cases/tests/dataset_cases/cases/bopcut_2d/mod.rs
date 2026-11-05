//! `bopcut_2d`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, printed_mass},
    checkprops::{Expected, checkprops},
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub use super::bopcut_2d_hand::*;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
