//! Product and instance label selection settings.
use super::mesh_integer_setting;

mesh_integer_setting!(
    RwMeshNameFormat,
    "Preference for product and instance labels; `RwMeshNameFormat::INSTANCE_OR_PRODUCT`."
);

impl RwMeshNameFormat {
    /// Omit the name.
    pub const EMPTY: Self = Self(0);
    /// Select the product name.
    pub const PRODUCT: Self = Self(1);
    /// Select the instance name.
    pub const INSTANCE: Self = Self(2);
    /// Select the instance name, falling back to the product name.
    pub const INSTANCE_OR_PRODUCT: Self = Self(3);
    /// Select the product name, falling back to the instance name.
    pub const PRODUCT_OR_INSTANCE: Self = Self(4);
    /// Combine product and instance names.
    pub const PRODUCT_AND_INSTANCE: Self = Self(5);
    /// Combine product, instance and document label names.
    pub const PRODUCT_AND_INSTANCE_AND_OCAF: Self = Self(6);
}

#[cfg(test)]
mod tests {
    use super::RwMeshNameFormat as Format;

    #[test]
    fn all_declared_name_preferences_have_their_exact_integer_values() {
        let cases = [
            Format::EMPTY,
            Format::PRODUCT,
            Format::INSTANCE,
            Format::INSTANCE_OR_PRODUCT,
            Format::PRODUCT_OR_INSTANCE,
            Format::PRODUCT_AND_INSTANCE,
            Format::PRODUCT_AND_INSTANCE_AND_OCAF,
        ];
        for (value, format) in cases.into_iter().enumerate() {
            assert_eq!(format.as_raw(), value as i32);
        }
    }
}
