use super::{
    CaseFailure, CaseSpec,
    context::{CaseContext, hex},
    launch::{ChildObservation, ChildRequest},
    spec::HARNESS_PROFILE,
};
use std::{fs, io, path::Path};

pub(crate) trait ReceiptRecorder {
    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), io::Error>;
}

pub(crate) struct FilesystemReceipts;

impl ReceiptRecorder for FilesystemReceipts {
    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), io::Error> {
        fs::write(path, bytes)
    }
}

pub(crate) fn retain_logs(
    context: &CaseContext,
    output: Option<&ChildObservation>,
    recorder: &dyn ReceiptRecorder,
) -> Vec<io::Error> {
    let stdout = output.map_or(&[][..], |o| o.stdout.as_slice());
    let stderr = output.map_or(&[][..], |o| o.stderr.as_slice());
    [("stdout.log", stdout), ("stderr.log", stderr)]
        .into_iter()
        .filter_map(|(name, bytes)| recorder.write(&context.directory.join(name), bytes).err())
        .collect()
}

pub(crate) fn retain_result(
    request: &ChildRequest<'_>,
    output: Option<&ChildObservation>,
    failure: Option<&CaseFailure>,
    recorder: &dyn ReceiptRecorder,
) -> Result<(), io::Error> {
    let text = result_text(request, output, failure);
    recorder.write(
        &request.context.directory.join("receipt.json"),
        text.as_bytes(),
    )
}

fn result_text(
    request: &ChildRequest<'_>,
    output: Option<&ChildObservation>,
    failure: Option<&CaseFailure>,
) -> String {
    let mut fields = identity_fields(request.spec);
    fields.extend(executable_fields(request.context));
    fields.extend(log_path_fields(request.context));
    fields.push(arguments_field(request));
    fields.push(format!(
        "\"declared_child_lifecycle\":{}",
        json_string(request.context.lifecycle.description())
    ));
    fields.extend(outcome_fields(output, failure));
    format!("{{{}}}\n", fields.join(","))
}

fn outcome_fields(output: Option<&ChildObservation>, failure: Option<&CaseFailure>) -> Vec<String> {
    let mut fields = Vec::new();
    fields.push(format!(
        "\"exit_code\":{}",
        optional_number(output.and_then(|o| o.code))
    ));
    fields.push(format!(
        "\"signal\":{}",
        optional_number(output.and_then(|o| o.signal))
    ));
    fields.push(format!("\"accepted\":{}", failure.is_none()));
    fields.push(format!(
        "\"failure\":{}",
        failure.map_or_else(|| "null".into(), |e| json_string(&e.to_string()))
    ));
    fields
}

fn identity_fields(spec: CaseSpec<'_>) -> Vec<String> {
    vec![
        "\"schema_version\":1".into(),
        format!("\"case\":{}", json_string(spec.identity)),
        format!("\"budget_seconds\":{}", spec.seconds),
        format!(
            "\"declared_harness_profile\":{}",
            json_string(HARNESS_PROFILE)
        ),
        "\"required_factor\":\"unset\"".into(),
        "\"child_environment_removed\":[\"RUST_TEST_NOCAPTURE\"]".into(),
    ]
}

fn executable_fields(context: &CaseContext) -> Vec<String> {
    vec![
        format!(
            "\"executable_path\":{}",
            json_string(&context.executable.to_string_lossy())
        ),
        format!(
            "\"executable_os_bytes_hex\":{}",
            json_string(&hex(context.executable.as_os_str().as_encoded_bytes()))
        ),
        format!(
            "\"executable_sha256\":{}",
            json_string(&context.fingerprint)
        ),
    ]
}

fn log_path_fields(context: &CaseContext) -> Vec<String> {
    ["stdout", "stderr"]
        .into_iter()
        .map(|name| {
            let path = context.directory.join(format!("{name}.log"));
            format!("\"{name}_path\":{}", json_string(&path.to_string_lossy()))
        })
        .collect()
}

fn arguments_field(request: &ChildRequest<'_>) -> String {
    let arguments = request
        .arguments()
        .iter()
        .map(|a| json_string(a))
        .collect::<Vec<_>>()
        .join(",");
    format!("\"arguments\":[{arguments}]")
}

fn optional_number(value: Option<i32>) -> String {
    value.map_or_else(|| "null".into(), |number| number.to_string())
}

pub(crate) fn json_string(text: &str) -> String {
    let mut quoted = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            c if c <= '\u{1f}' => quoted.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted
}
