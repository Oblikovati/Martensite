//! Explicit per-request token storage for direct development replays.
use martensite_tkernel::standard::Failure;
use std::fmt::Write;

/// Owned protocol bytes; no global or thread-local output capture.
#[derive(Default)]
pub struct Sink {
    bytes: String,
}
impl Sink {
    /// Consume one request's bytes; `Sink::default().finish().is_empty()`.
    pub fn finish(self) -> String {
        self.bytes
    }
}

pub fn attempt(out: &mut Sink, block: impl FnOnce(&mut Sink) -> Result<(), Failure>) {
    if let Err(error) = block(out) {
        write!(out.bytes, "{} ", error.exception_type()).unwrap();
    }
}
pub fn real(out: &mut Sink, value: f64) {
    write!(out.bytes, "{} ", value.to_bits()).unwrap();
}
pub fn int(out: &mut Sink, value: i32) {
    write!(out.bytes, "{value} ").unwrap();
}
pub fn flag(out: &mut Sink, value: bool) {
    int(out, i32::from(value));
}
pub fn end(out: &mut Sink) {
    out.bytes.push('\n');
}

#[cfg(test)]
mod tests {
    use super::{Sink, attempt, end, flag, int, real};
    use martensite_tkernel::standard::Failure;

    #[test]
    fn local_tokens_keep_bits_flags_exceptions_spacing_and_line_end() {
        let mut out = Sink::default();
        real(&mut out, -0.);
        int(&mut out, -7);
        flag(&mut out, true);
        attempt(&mut out, |_| Ok(()));
        attempt(&mut out, |_| {
            Err(Failure::with_type("Standard_DomainError", "ignored", ""))
        });
        end(&mut out);
        assert_eq!(
            out.finish(),
            "9223372036854775808 -7 1 Standard_DomainError \n"
        );
        assert!(Sink::default().finish().is_empty());
    }
}

/// Encode text explicitly written by a member, followed by its exception type.
pub fn captured(out: &mut Sink, body: impl FnOnce(&mut String) -> Result<(), Failure>) {
    let mut console = String::new();
    let result = body(&mut console);
    out.bytes.push('t');
    for byte in console.bytes() {
        write!(out.bytes, "{byte:02x}").unwrap();
    }
    out.bytes.push(' ');
    attempt(out, |_| result);
}

#[cfg(test)]
mod captured_tests {
    use super::{Sink, captured};
    use martensite_tkernel::standard::Failure;

    #[test]
    fn captured_text_keeps_exact_bytes_and_exception_order() {
        let mut output = Sink::default();
        captured(&mut output, |console| {
            console.push_str("a\0\n");
            Err(Failure::with_type("Standard_DomainError", "ignored", ""))
        });
        assert_eq!(output.finish(), "t61000a Standard_DomainError ");
    }
}

/// Keep result formatting in the same request-local output buffer.
pub fn show<T>(out: &mut Sink, result: Result<T, Failure>, emit: impl FnOnce(&mut Sink, T)) {
    attempt(out, |out| {
        emit(out, result?);
        Ok(())
    });
}

#[cfg(test)]
mod result_tests {
    use super::{Sink, int, show};
    use martensite_tkernel::standard::Failure;

    #[test]
    fn result_tokens_emit_success_once_and_skip_failure_body() {
        let mut tokens = Sink::default();
        show(&mut tokens, Ok(7), int);
        show(
            &mut tokens,
            Err::<i32, _>(Failure::with_message("Standard_DomainError", "")),
            |_, _| panic!("failed value must not emit"),
        );
        assert_eq!(tokens.finish(), "7 Standard_DomainError ");
    }
}
