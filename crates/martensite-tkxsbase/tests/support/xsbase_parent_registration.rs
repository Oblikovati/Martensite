//! Fixed registrations keep each genuine original behind the same parent and shared controls.
/// Register actual descriptor/end controls once per original host.
/// Example: `register_xsbase_parent_controls!(super::body, super::commands)`.
macro_rules! register_xsbase_parent_controls {
    ($body:path, $commands:path) => {
        mod actual_parent_controls {
            #[test]
            fn stdout_faulty_after_case_end_is_rejected() {
                let identity = "actual_parent_controls::stdout_faulty_after_case_end_is_rejected";
                let body: fn() -> Result<(), Box<dyn std::error::Error>> = $body;
                crate::parent_controls::reject_descriptor(identity, body, false);
            }
            #[test]
            fn stderr_faulty_after_case_end_is_rejected() {
                let identity = "actual_parent_controls::stderr_faulty_after_case_end_is_rejected";
                let body: fn() -> Result<(), Box<dyn std::error::Error>> = $body;
                crate::parent_controls::reject_descriptor(identity, body, true);
            }
            #[test]
            fn genuine_commands_without_end_are_rejected() {
                let identity = "actual_parent_controls::genuine_commands_without_end_are_rejected";
                let commands: fn() -> Result<(), Box<dyn std::error::Error>> = $commands;
                crate::parent_controls::reject_missing_end(identity, commands);
            }
            #[test]
            fn duplicate_end_preserves_source_presence_rule() {
                let identity =
                    "actual_parent_controls::duplicate_end_preserves_source_presence_rule";
                let body: fn() -> Result<(), Box<dyn std::error::Error>> = $body;
                crate::parent_controls::accept_duplicate(identity, body);
            }
            #[test]
            fn benign_actual_descriptors_remain_accepted() {
                let identity = "actual_parent_controls::benign_actual_descriptors_remain_accepted";
                let body: fn() -> Result<(), Box<dyn std::error::Error>> = $body;
                crate::parent_controls::accept_benign(identity, body);
            }
            #[test]
            fn genuine_child_failure_preserves_named_cause() {
                let identity =
                    "actual_parent_controls::genuine_child_failure_preserves_named_cause";
                let body: fn() -> Result<(), Box<dyn std::error::Error>> = $body;
                crate::parent_controls::reject_failed_child(identity, body);
            }
        }
    };
}
pub(crate) use register_xsbase_parent_controls;
