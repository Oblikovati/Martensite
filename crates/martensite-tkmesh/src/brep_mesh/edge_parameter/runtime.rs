//! Edge parameter tools share the transient runtime descriptor.
use super::EdgeParameterProvider;
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped, StandardTransient, Transient};
impl RuntimeTyped for EdgeParameterProvider {
    const TYPE_NAME: &'static str = StandardTransient::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        StandardTransient::static_type()
    }
}
impl Transient for EdgeParameterProvider {
    fn dynamic_type(&self) -> &'static RuntimeType {
        StandardTransient::static_type()
    }
}
