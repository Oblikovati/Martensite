//! Concrete constructor ancestry and stack behavior follow the actual native exception classes.
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::{
    prs3d::{prs3d_invalid_angle, prs3d_invalid_angle_with_stack},
    v3d::{v3d_bad_value, v3d_bad_value_with_stack, v3d_un_mapped, v3d_un_mapped_with_stack},
};
use std::{fmt::Write, process::Command};

const STACK_CHILD: &str = "MARTENSITE_TKV3D_EXCEPTION_STACK_CHILD";

#[derive(Clone, Copy)]
enum VisualExceptionFamily {
    InvalidAngle,
    BadValue,
    UnMapped,
}

impl VisualExceptionFamily {
    fn name(self) -> &'static str {
        match self {
            Self::InvalidAngle => "Prs3dInvalidAngle",
            Self::BadValue => "V3dBadValue",
            Self::UnMapped => "V3dUnMapped",
        }
    }
    fn native_first_row(self) -> usize {
        match self {
            Self::InvalidAngle => 1,
            Self::BadValue => 3,
            Self::UnMapped => 5,
        }
    }
    fn message_only(self, message: &str) -> Failure {
        match self {
            Self::InvalidAngle => prs3d_invalid_angle(message),
            Self::BadValue => v3d_bad_value(message),
            Self::UnMapped => v3d_un_mapped(message),
        }
    }
    fn stacked(self, message: &str, stack: &str) -> Failure {
        match self {
            Self::InvalidAngle => prs3d_invalid_angle_with_stack(message, stack),
            Self::BadValue => v3d_bad_value_with_stack(message, stack),
            Self::UnMapped => v3d_un_mapped_with_stack(message, stack),
        }
    }
    fn construct(self, stacked: bool) -> Failure {
        if !stacked {
            return self.message_only("owned-message");
        }
        self.stacked("owned-message", "owned-stack")
    }
}

fn exception_families() -> [VisualExceptionFamily; 3] {
    [
        VisualExceptionFamily::InvalidAngle,
        VisualExceptionFamily::BadValue,
        VisualExceptionFamily::UnMapped,
    ]
}

fn bytes_hex(bytes: &[u8]) -> String {
    let mut encoded = String::new();
    for byte in bytes {
        write!(encoded, "{byte:02x}").unwrap();
    }
    encoded
}

fn stack_field(failure: &Failure, presence: bool) -> String {
    if presence {
        return u8::from(!failure.stack_bytes().is_empty()).to_string();
    }
    bytes_hex(failure.stack_bytes())
}

fn exception_record(failure: &Failure, stack_presence: bool) -> String {
    format!(
        "{}|{}|{}|{}{}{}",
        failure.exception_type(),
        bytes_hex(failure.message_bytes()),
        stack_field(failure, stack_presence),
        u8::from(failure.is_kind("Standard_RangeError")),
        u8::from(failure.is_kind("Standard_OutOfRange")),
        u8::from(failure.is_kind("Standard_DomainError")),
    )
}

fn assert_native_constructor(family: VisualExceptionFamily, stacked: bool, actual: Failure) {
    let row = family.native_first_row() + usize::from(stacked);
    let expected = include_str!("fixtures/migration-exceptions/native-default.txt")
        .lines()
        .nth(row)
        .unwrap();
    let measured = exception_record(&actual, false);
    println!("{measured}");
    assert_eq!(measured, expected);
}

macro_rules! constructor_case {
    ($test:ident, $family:expr, $stacked:expr) => {
        #[test]
        fn $test() {
            let family = $family;
            assert_native_constructor(family, $stacked, family.construct($stacked));
        }
    };
}
constructor_case!(
    invalid_angle_message,
    VisualExceptionFamily::InvalidAngle,
    false
);
constructor_case!(
    invalid_angle_stack,
    VisualExceptionFamily::InvalidAngle,
    true
);
constructor_case!(bad_value_message, VisualExceptionFamily::BadValue, false);
constructor_case!(bad_value_stack, VisualExceptionFamily::BadValue, true);
constructor_case!(unmapped_message, VisualExceptionFamily::UnMapped, false);
constructor_case!(unmapped_stack, VisualExceptionFamily::UnMapped, true);

fn captured_records() -> (String, Vec<(&'static str, bool)>) {
    let mut report = format!(
        "default-stack-length={}\n",
        Failure::default_stack_trace_length()
    );
    let mut captured = Vec::new();
    for family in exception_families() {
        let automatic = family.message_only("owned-message");
        captured.push((family.name(), !automatic.stack_bytes().is_empty()));
        for actual in [
            automatic,
            family.stacked("owned-message", "owned-stack"),
            family.stacked("owned-message", ""),
        ] {
            writeln!(report, "{}", exception_record(&actual, true)).unwrap();
        }
    }
    (report, captured)
}

fn configured_stack_child() {
    let previous = Failure::default_stack_trace_length();
    Failure::set_default_stack_trace_length(2);
    let (report, captured) = captured_records();
    Failure::set_default_stack_trace_length(previous);
    println!("{report}");
    for (class, observed) in captured {
        assert!(
            observed,
            "class={class}, captured={observed}; expected configured default stack capture"
        );
    }
    assert_eq!(
        report,
        include_str!("fixtures/migration-exceptions/native-capture.txt")
    );
}

#[test]
fn configured_default_capture_is_process_local() {
    if std::env::var_os(STACK_CHILD).is_some() {
        return configured_stack_child();
    }
    let previous = Failure::default_stack_trace_length();
    let output = configured_stack_output();
    println!("{}", String::from_utf8_lossy(&output.stdout));
    assert!(
        output.status.success(),
        "child={:?}, stderr={}; expected successful configured-stack constructor process",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(Failure::default_stack_trace_length(), previous);
}

fn configured_stack_output() -> std::process::Output {
    let executable = exception_test_executable();
    Command::new(&executable)
        .args([
            "--exact",
            "configured_default_capture_is_process_local",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(STACK_CHILD, "1")
        .output()
        .unwrap_or_else(|cause| {
            panic!("executable={executable:?}; expected isolated constructor process: {cause}")
        })
}

fn exception_test_executable() -> std::path::PathBuf {
    std::env::current_exe().unwrap_or_else(|cause| {
        panic!(
            "pid={}; expected readable current constructor-test executable: {cause}",
            std::process::id()
        )
    })
}

fn assert_detached_exception_buffers(family: VisualExceptionFamily) {
    let mut message = String::from("owned-message");
    let mut stack = String::from("owned-stack");
    let mut actual = family.stacked(&message, &stack);
    let copied = actual.clone();
    message.clear();
    stack.clear();
    actual.set_message("changed-message");
    actual.set_stack_string("changed-stack");
    assert_eq!(copied.message_bytes(), b"owned-message");
    assert_eq!(copied.stack_bytes(), b"owned-stack");
    assert_eq!(copied.exception_type(), family.name());
    let _: &(dyn Send + Sync) = &copied;
}

#[test]
fn exception_copies_own_their_case_message_and_stack_buffers() {
    for family in exception_families() {
        assert_detached_exception_buffers(family);
    }
}
