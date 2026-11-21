//! Primitive array subclasses lend their original state through real Rust interfaces (#592).
use super::Graphic3dArrayOfPrimitives;
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped, Transient};

/// A primitive array object; `array.primitive_array().vertex_number()`.
pub trait Graphic3dArraySource: Transient {
    /// Borrow the original primitive storage; `array.primitive_array().is_valid()`.
    fn primitive_array(&self) -> &Graphic3dArrayOfPrimitives;
}

impl RuntimeTyped for dyn Graphic3dArraySource {
    const TYPE_NAME: &'static str = <Graphic3dArrayOfPrimitives as RuntimeTyped>::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        Graphic3dArrayOfPrimitives::static_type()
    }
}
martensite_tkernel::impl_transient_interface_target!(dyn Graphic3dArraySource);

impl std::ops::Deref for dyn Graphic3dArraySource {
    type Target = Graphic3dArrayOfPrimitives;
    fn deref(&self) -> &Self::Target {
        self.primitive_array()
    }
}

impl Graphic3dArraySource for Graphic3dArrayOfPrimitives {
    fn primitive_array(&self) -> &Graphic3dArrayOfPrimitives {
        self
    }
}
