//! Test-only attribution of changed results to individual libm calls (#201).
//!
//! Compiled only with the `libm-attribution` feature, which no release build or CI
//! gate enables. Every adapter call evaluates both the portable function and the
//! platform C math library entry point that the kernel called before #201, records
//! the call when their result words differ, and returns the selected result:
//!
//! - `MARTENSITE_CMATH_SELECT=platform` returns the platform libm result (glibc on
//!   Linux), reproducing the pre-#201 kernel; `portable` (the default) returns the
//!   portable result.
//! - `MARTENSITE_CMATH_ATTRIBUTION_LOG=<dir>` appends one JSON line per distinct
//!   differing call and test to `<dir>/<executable>-<pid>.jsonl`.
//!
//! The recorder is process-global because the adapter's signatures carry no context;
//! it only observes, so it cannot change the results of the selected mode.

use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{LineWriter, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

/// Exact bit words of an argument or result, for comparison and logging.
pub(crate) trait Words {
    fn words(&self) -> Vec<u64>;
}

impl Words for f64 {
    fn words(&self) -> Vec<u64> {
        vec![self.to_bits()]
    }
}

impl Words for f32 {
    fn words(&self) -> Vec<u64> {
        vec![u64::from(self.to_bits())]
    }
}

impl Words for i32 {
    fn words(&self) -> Vec<u64> {
        vec![u64::from(self.cast_unsigned())]
    }
}

impl Words for (f64, f64) {
    fn words(&self) -> Vec<u64> {
        vec![self.0.to_bits(), self.1.to_bits()]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Selection {
    Platform,
    Portable,
}

fn selection() -> Selection {
    static SELECTION: OnceLock<Selection> = OnceLock::new();
    *SELECTION.get_or_init(|| {
        match std::env::var("MARTENSITE_CMATH_SELECT").as_deref() {
            Ok("platform") => Selection::Platform,
            Ok("portable") | Err(std::env::VarError::NotPresent) => Selection::Portable,
            // A silent default would let a mistyped replay pass as the wrong mode.
            other => panic!("MARTENSITE_CMATH_SELECT={other:?}; expected `platform` or `portable`"),
        }
    })
}

/// Compares the two results of one call, records a difference and returns the
/// result of the selected mode.
pub(crate) fn select<T: Words>(
    function: &'static str,
    inputs: &[Vec<u64>],
    portable: T,
    platform: T,
) -> T {
    let portable_words = portable.words();
    let platform_words = platform.words();
    if portable_words != platform_words {
        let call = DifferingCall {
            function,
            inputs: inputs.concat(),
            portable_words,
            platform_words,
        };
        record(call);
    }
    match selection() {
        Selection::Platform => platform,
        Selection::Portable => portable,
    }
}

struct DifferingCall {
    function: &'static str,
    inputs: Vec<u64>,
    portable_words: Vec<u64>,
    platform_words: Vec<u64>,
}

struct Recorder {
    seen: HashSet<(String, &'static str, Vec<u64>)>,
    log: LineWriter<File>,
    binary: String,
}

fn record(call: DifferingCall) {
    static RECORDER: OnceLock<Option<Mutex<Recorder>>> = OnceLock::new();
    let Some(recorder) = RECORDER.get_or_init(open_recorder) else {
        return;
    };
    // A poisoned lock only means another test panicked mid-record; keep recording.
    let mut recorder = recorder
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    // libtest names each test's thread after the test.
    let test = std::thread::current()
        .name()
        .unwrap_or("<unnamed>")
        .to_owned();
    if !recorder
        .seen
        .insert((test.clone(), call.function, call.inputs.clone()))
    {
        return;
    }
    let line = json_line(&recorder.binary, &test, &call);
    if let Err(error) = writeln!(recorder.log, "{line}") {
        panic!("cannot append libm attribution record {line}: {error}");
    }
}

fn open_recorder() -> Option<Mutex<Recorder>> {
    let directory = PathBuf::from(std::env::var_os("MARTENSITE_CMATH_ATTRIBUTION_LOG")?);
    let binary = std::env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "unknown".to_owned());
    let path = directory.join(format!("{binary}-{}.jsonl", std::process::id()));
    let file = OpenOptions::new().create(true).append(true).open(&path);
    let file = file.unwrap_or_else(|error| panic!("cannot open attribution log {path:?}: {error}"));
    Some(Mutex::new(Recorder {
        seen: HashSet::new(),
        log: LineWriter::new(file),
        binary,
    }))
}

fn json_line(binary: &str, test: &str, call: &DifferingCall) -> String {
    format!(
        r#"{{"binary":"{}","test":"{}","function":"{}","inputs":{},"platform":{},"portable":{}}}"#,
        json_escape(binary),
        json_escape(test),
        call.function,
        hex_words(&call.inputs),
        hex_words(&call.platform_words),
        hex_words(&call.portable_words),
    )
}

fn hex_words(words: &[u64]) -> String {
    let hex: Vec<String> = words
        .iter()
        .map(|word| format!("\"{word:016x}\""))
        .collect();
    format!("[{}]", hex.join(","))
}

fn json_escape(text: &str) -> String {
    text.chars()
        .flat_map(|character| match character {
            '"' | '\\' => vec!['\\', character],
            control if control.is_control() => {
                format!("\\u{:04x}", u32::from(control)).chars().collect()
            }
            other => vec![other],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{DifferingCall, Words, json_line};
    use crate::host_platform as platform;

    #[test]
    fn words_keep_exact_bits_of_every_argument_kind() {
        assert_eq!((-0.0_f64).words(), vec![0x8000_0000_0000_0000]);
        assert_eq!(1.5_f32.words(), vec![0x3fc0_0000]);
        assert_eq!((-1_i32).words(), vec![0xffff_ffff]);
        assert_eq!(
            (1.0, -2.0).words(),
            vec![0x3ff0_0000_0000_0000, 0xc000_0000_0000_0000]
        );
    }

    #[test]
    fn json_line_records_inputs_and_both_results_as_hex_words() {
        let call = DifferingCall {
            function: "pow",
            inputs: vec![0x4000_0000_0000_0000, 1],
            portable_words: vec![2],
            platform_words: vec![3],
        };
        let line = json_line("toolkit", "a::\"b\"", &call);
        let expected = r#"{"binary":"toolkit","test":"a::\"b\"","function":"pow","inputs":["4000000000000000","0000000000000001"],"platform":["0000000000000003"],"portable":["0000000000000002"]}"#;
        assert_eq!(line, expected);
    }

    #[test]
    fn platform_entry_points_match_the_pre_201_methods_on_exact_cases() {
        // The glibc 2.39 kernel is one ulp off on these (the pre-#201 regression values).
        assert_eq!(platform::cbrt(2e100).to_bits(), 5107287141773884165);
        assert_eq!(platform::cbrt(27.0), 3.0000000000000004);
        assert_eq!(platform::cbrt(-8.0), -2.0);
        assert_eq!(platform::powi(2.0, -2), 0.25);
        assert_eq!(platform::powf(4.0, 0.5), 2.0);
        assert_eq!(platform::sincos(0.0), (0.0, 1.0));
        assert_eq!(platform::atan2(0.0, 1.0), 0.0);
    }
}
