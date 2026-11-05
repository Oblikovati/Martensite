//! `bcut_2d`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcut,
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
};

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
