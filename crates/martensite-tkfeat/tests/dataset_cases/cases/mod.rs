//! One module per grid; each case body is one statement per script line.
#![allow(non_snake_case, unused_imports)]
pub mod bugs_modalg_1;
pub mod bugs_modalg_1_hand;
pub mod bugs_modalg_2;
pub mod bugs_modalg_2_hand;
#[cfg(target_os = "linux")] // The renderer suite runs on Linux.
pub mod bugs_modalg_5_hand;
pub mod bugs_modalg_6_hand;
#[cfg(target_os = "linux")] // The renderer suite runs on Linux.
pub mod bugs_modalg_6_view_hand;
pub mod bugs_modalg_7;
pub mod bugs_modalg_7_hand;
pub mod bugs_moddata_1_hand;
pub mod feat_featdprism;
pub mod feat_featlf;
pub mod feat_featprism;
pub mod feat_featprism_cpu700;
pub mod feat_featrevol;
pub mod feat_featrf;
pub mod perf_modalg_hand;
