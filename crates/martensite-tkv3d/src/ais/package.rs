//! The `AIS` package class: an empty namespace whose members are its own allocation operators.

/// `AIS`: the package class, with no member besides allocation; `let _package = Ais;`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ais;

#[cfg(test)]
mod tests {
    use super::Ais;

    #[test]
    fn package_class_is_a_unit_value() {
        assert_eq!(format!("{Ais:?}"), "Ais");
        assert_eq!(std::mem::size_of::<Ais>(), 0);
    }
}
