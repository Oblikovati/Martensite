//! The case bodies, one module per test group. Each body follows its script line
//! by line, logs what the script and its commands print and reports every variable
//! and view step to its [`crate::support::session::Session`].
pub mod bugs_fclasses;
pub mod bugs_modalg_1;
pub mod bugs_modalg_2;
pub mod bugs_modalg_4;
pub mod bugs_modalg_5;
pub mod bugs_modalg_6;
pub mod bugs_modalg_7;
pub mod bugs_modalg_8;
pub mod bugs_moddata_1;
pub mod bugs_moddata_2;
pub mod bugs_moddata_3;
mod heal;
mod other_grids;

pub use heal::{
    checkshape as heal_checkshape, same_parameter as heal_same_parameter,
    same_parameter_locked as heal_same_parameter_locked,
    update_tolerance_locked as heal_update_tolerance_locked,
};
pub use other_grids::{
    classifier as lowalgos_classifier, intss as lowalgos_intss, perf as perf_modalg,
    revsurf as geometry_revsurf,
};
