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
