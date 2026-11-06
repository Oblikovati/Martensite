//! `bugs_modalg_5`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{make_box, pcylinder, psphere, ptorus, trotate, ttranslate},
    construction::{edge, mkplane, revol, vertex, wire},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcut,
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::{Session, ViewStep},
    surface_commands::{circle, mkedge},
    two_step::TwoStepBoolean,
    volume_commands::compound,
};
use martensite_tkbo::bop_algo::Operation;

pub use super::bugs_modalg_5_hand_2::*;
pub use super::bugs_modalg_5_hand_3::*;
pub use super::bugs_modalg_5_hand_9::*;
pub use super::bugs_modalg_5_hand_10::*;
pub use super::bugs_modalg_5_hand_11::*;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
