//! `bcommon_simple`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass, rotation_is, view_mass_is_nonzero},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::{explode, make_box, pcylinder, prism, psphere, trotate, ttranslate},
    construction::{copy, edge, mkplane, nurbsconvert, revol, treverse, vertex, wire},
    one_step::{bcommon, bcut},
    properties::Prop,
    session::Session,
    surface_commands::{circle, circle_along, mkedge},
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
