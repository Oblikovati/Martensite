//! `bopcommon_complex`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::{explode, make_box, prism, ttranslate},
    one_step::bfuse,
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
