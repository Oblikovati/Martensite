//! The case bodies, one module per test group. Each body follows its script line
//! by line: it prints what the script prints, runs the same commands on the Rust API and
//! reports every variable and view step to its [`crate::support::session::Session`].
pub mod bugs_filling;
pub mod bugs_modalg_2;
pub mod bugs_modalg_4;
pub mod bugs_modalg_5;
pub mod bugs_modalg_6;
pub mod bugs_modalg_7;
pub mod bugs_modalg_8;
pub mod bugs_moddata_1;
pub mod bugs_moddata_2;
pub mod bugs_moddata_3;
pub mod lowalgos_2dgcc;
pub mod lowalgos_2dinter;
pub mod lowalgos_extcc;
pub mod lowalgos_intss;
pub mod perf_modalg;
pub mod script;
pub mod script_words;
