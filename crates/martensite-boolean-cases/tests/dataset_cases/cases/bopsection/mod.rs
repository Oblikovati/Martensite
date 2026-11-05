//! `bopsection`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, printed_mass},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::{explode, pcylinder, prism},
    construction::{bopaddpcs, btolx, copy, edge, mkplane, tcopy, vertex, wire},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkg3d::top_abs::ShapeEnum;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
