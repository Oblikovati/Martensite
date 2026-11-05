//! `bopcut_complex`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, printed_mass, view_mass_is_nonzero},
    bop_session::BopSession,
    cells_session::CellsSession,
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::{explode, make_box, prism, ttranslate},
    construction::{bopaddpcs, btolx, copy},
    nbshapes::{Counting, NbCount, checknbshapes},
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
mod part_03;
pub use part_03::*;
