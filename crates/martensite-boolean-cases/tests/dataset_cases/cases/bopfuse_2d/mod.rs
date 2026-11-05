//! `bopfuse_2d`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
