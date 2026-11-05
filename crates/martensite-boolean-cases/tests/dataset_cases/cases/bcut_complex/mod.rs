//! `bcut_complex`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass, view_mass_is_nonzero},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_top,
    checkview::{CheckView, checkview},
    commands::{explode, make_box, prism},
    construction::revol,
    one_step::{bcut, bfuse, bsection},
    properties::Prop,
    restore::restore,
    session::Session,
    topology_commands::halfspace,
    volume_commands::compound,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub use super::bcut_complex_hand::*;
pub use super::bcut_complex_hand_2::*;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
mod part_03;
pub use part_03::*;
mod part_04;
pub use part_04::*;
