// Shared module tree of the grouped regression hosts; each host `include!`s this file.
// The grouped hosts run each regression in an isolated worker under its group's stream policy
// and drive the viewer through `case_view::viewer_session::ViewerSession`.
// Hosts leave the support modules' unit tests to the `support_units` binary (#471).
include!("../support/support_unit_tests_elsewhere.rs");
include!("session_module_tree.rs");
