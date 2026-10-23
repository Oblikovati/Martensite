//! Edge parameter tools share the transient runtime descriptor.
use super::EdgeParameterProvider;
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped, StandardTransient, Transient};
impl RuntimeTyped for EdgeParameterProvider {
    const TYPE_NAME: &'static str = StandardTransient::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        StandardTransient::static_type()
    }
    fn project_from(object: &dyn Transient) -> Option<&Self> {
        martensite_tkernel::standard::project_concrete::<Self>(object)
    }
}
impl Transient for EdgeParameterProvider {
    fn dynamic_type(&self) -> &'static RuntimeType {
        StandardTransient::static_type()
    }
    fn project(&self, requested: std::any::TypeId) -> Option<&dyn std::any::Any> {
        if requested == std::any::TypeId::of::<Self>() {
            return Some(self);
        }
        self.transient.project(requested)
    }
    fn transient_base(&self) -> &StandardTransient {
        &self.transient
    }
}
