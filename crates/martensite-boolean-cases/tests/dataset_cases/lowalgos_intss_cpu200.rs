//! The `lowalgos_intss` case whose script sets its own budget. Its intersection takes 52 s in
//! release against native's 12 s (recorded reference run), about 80 s
//! of the 200 at opt-level 1; an unoptimized build runs out of the budget.
use crate::support::{groups, hooks, process::optimized_dataset_cases};

optimized_dataset_cases!(groups::LOWALGOS_CPU200, lowalgos_intss_cpu200, hooks::no_grid_end,
    "needs an optimized Linux build: elsewhere it exceeds the script's cpulimit 200"; bug25950);
