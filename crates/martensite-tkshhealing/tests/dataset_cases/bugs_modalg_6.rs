//! The TKShHealing (no view) tests of modalg 6.
use super::*;

dataset_cases!(run_case, NoView, bugs_modalg_6, BUGS_SECONDS;
    bug26241, bug26647, bug27015, bug27552_2, bug28601);

// bug26627 sets `cpulimit 10` for its fixshape; it runs alone in the draw_dataset_timing target.
