//! `removefeatures`: each statement is one line of its script.
use crate::support::{
    beyond_script::{kind_is, part_count, printed_mass},
    checkprops::{CheckTolerances, Expected, checkprops, checkprops_equal, checkprops_within},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    hooks::require_bopcheck_ok,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    removal_commands::{
        FeatureRemoval, generated, modified, require_not_generated, require_same_shapes,
    },
    restore::restore,
    session::Session,
    volume_commands::compound,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub use super::removefeatures_hand::*;

mod part_01;
pub use part_01::*;
mod part_02;
pub use part_02::*;
mod part_03;
pub use part_03::*;
mod part_04;
pub use part_04::*;
