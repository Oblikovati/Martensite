//! The pool uses the shared manual-reset event of `standard::Condition`:
//! `set` wakes every waiter and stays set until `reset`.
//! A panicking job never holds its waiter lock, so a poisoned flag remains consistent.
pub(super) use crate::standard::StandardCondition as ManualResetEvent;
