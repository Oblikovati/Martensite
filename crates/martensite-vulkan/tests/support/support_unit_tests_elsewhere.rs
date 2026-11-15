// Shared support modules declare their unit tests through `support_unit_tests!`. Only the
// `support_units` test binary expands those declarations; every other test target that includes
// the modules defines the macro with this file, which expands them to nothing. Each support unit
// test then compiles and runs once instead of once per including binary (#471).
macro_rules! support_unit_tests {
    ($($tests:tt)*) => {};
}
