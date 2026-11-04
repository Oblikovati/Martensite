//! A legacy signed query must not wrap an invalid physical-device dimension.
use std::fmt;
/// Preserve the invalid physical dimension; for example checked_dimension(0).unwrap_err().
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct InvalidTextureDimension {
    received: u32,
}
impl fmt::Display for InvalidTextureDimension {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            out,
            "texture limit maxImageDimension2D={}; expected 1..={}",
            self.received,
            i32::MAX
        )
    }
}
impl std::error::Error for InvalidTextureDimension {}
/// Validate the existing trait's integer domain; for example checked_dimension(8192) == Ok(8192).
pub(crate) fn checked_dimension(maximum: u32) -> Result<i32, InvalidTextureDimension> {
    if maximum == 0 {
        return Err(InvalidTextureDimension { received: maximum });
    }
    i32::try_from(maximum).map_err(|_| InvalidTextureDimension { received: maximum })
}
#[cfg(test)]
mod tests {
    use super::*;
    struct FixedTextureDimension(u32);
    impl FixedTextureDimension {
        fn sample(&self) -> u32 {
            // Named fake models an already-retained physical-device property, without I/O.
            self.0
        }
    }
    #[test]
    fn representable_dimensions_preserve_the_exact_integer_value() {
        for value in [1, 8192, i32::MAX as u32] {
            let source = FixedTextureDimension(value);
            assert_eq!(checked_dimension(source.sample()), Ok(value as i32));
        }
    }
    #[test]
    fn malformed_dimensions_keep_the_offending_value_without_clamping() {
        for value in [0, i32::MAX as u32 + 1, u32::MAX] {
            let source = FixedTextureDimension(value);
            let failure = checked_dimension(source.sample()).unwrap_err();
            assert_eq!(failure.received, value);
            assert!(failure.to_string().contains(&value.to_string()));
            assert!(failure.to_string().contains("expected 1..=2147483647"));
        }
    }
}
