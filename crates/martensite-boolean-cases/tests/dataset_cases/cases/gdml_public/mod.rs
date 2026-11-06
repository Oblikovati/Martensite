//! `gdml_public`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, printed_mass},
    checkprops::{Expected, checkprops},
    commands::{add, make_box, trotate, ttranslate},
    construction::{copy, mkplane, revol, wire},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcommon, bcut, bfuse},
    properties::Prop,
    session::Session,
    surface_commands::{circle_along, mkedge},
    topology_commands::plane_axes,
    volume_commands::{compound, pcone_on, pcylinder_on, psphere_turned, ptorus_on, settolerance},
};
use martensite_tkg3d::top_abs::ShapeEnum;

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
mod part_06;
pub use part_06::*;
mod part_07;
pub use part_07::*;
mod part_08;
pub use part_08::*;
mod part_09;
pub use part_09::*;
mod part_10;
pub use part_10::*;
mod part_11;
pub use part_11::*;
