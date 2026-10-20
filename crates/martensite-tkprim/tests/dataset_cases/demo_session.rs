//! The tests of the `demo` group's session cases (no `cpulimit` in the `demo` group).
use super::*;

dataset_cases!(run_unlimited, NoView, demo_session, NO_CPULIMIT; binpersist_2, binpersist_3);
