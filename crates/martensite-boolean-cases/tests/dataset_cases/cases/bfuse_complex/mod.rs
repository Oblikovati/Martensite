//! `bfuse_complex`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass, view_mass_is_nonzero},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, pcylinder, prism, trotate, ttranslate},
    construction::{copy, fsameparameter, mkplane, revol, tcopy, tscale, updatetolerance},
    hooks,
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcut, bfuse},
    properties::Prop,
    restore::restore,
    session::Session,
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
