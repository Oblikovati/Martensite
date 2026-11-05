//! `cells_test`: each statement is one line of its script.
use crate::support::{
    bop_session::BopSession,
    cells_session::CellsSession,
    checkprops::{Expected, checkprops},
    commands::{explode, make_box, pcone, pcylinder, psphere, ttranslate},
    construction::{copy, mkplane, wire},
    hooks::require_bopcheck_ok,
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_named},
    properties::Prop,
    session::{Session, ViewStep},
    surface_commands::{
        circle, circle_along, line, mkedge, mkedge_bounded, mkface, mkface_bounded, uiso, viso,
    },
    topology_commands::plane_along,
    volume_commands::{compound, cylinder},
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub use super::cells_test_hand::*;

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
