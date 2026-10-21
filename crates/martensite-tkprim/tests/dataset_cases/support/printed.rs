//! How the case harness prints a real into the script result: appending a real uses
//! `Sprintf(s, "%.17g", r)`; the cases then cut that text with script string commands.

/// C `printf("%.17g", value)`; `g17(0.5) == "0.5"`.
pub fn g17(value: f64) -> String {
    martensite_tkernel::storage::format_general(value, 17)
}
