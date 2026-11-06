//! `bsection`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, printed_mass},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::{explode, pcylinder, ttranslate},
    construction::copy,
    one_step::{bsection, bsection_no_pcurves},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::mkface_bounded,
    topology_commands::plane_along,
    volume_commands::compound,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub use super::bsection_hand::*;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
mod part_03;
pub use part_03::*;
mod part_04;
pub use part_04::*;
