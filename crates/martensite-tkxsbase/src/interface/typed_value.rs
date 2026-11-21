//! `InterfaceTypedValue`: a `MoniToolTypedValue` whose type is an
//! `InterfaceParamType` (the two enumerations have the same values).
use super::InterfaceParamType;
use crate::moni_tool::{MoniToolTypedValueOps, MoniToolValue, MoniToolValueType};
use martensite_tkernel::{
    impl_transient, impl_transient_interface_target,
    standard::{Failure, HandleUpcast, RuntimeType, RuntimeTyped},
};
use std::ops::Deref;
use std::sync::Arc;

/// A typed value classified by its exchange parameter type.///
/// ```
/// use martensite_tkxsbase::interface::{InterfaceParamType, InterfaceTypedValue};
/// let value = InterfaceTypedValue::new("count", InterfaceParamType::ParamInteger, "3").unwrap();
/// assert_eq!(value.param_type(), InterfaceParamType::ParamInteger);
/// assert_eq!(value.integer_value(), 3);
/// ```
pub struct InterfaceTypedValue {
    value: MoniToolValue,
}
impl_transient!(
    InterfaceTypedValue,
    "InterfaceTypedValue",
    dyn MoniToolTypedValueOps,
    interfaces: [dyn MoniToolTypedValueOps, dyn InterfaceTypedValueOps]
);

impl Deref for InterfaceTypedValue {
    type Target = MoniToolValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MoniToolTypedValueOps for InterfaceTypedValue {}
impl InterfaceTypedValueOps for InterfaceTypedValue {}

/// A typed value with a parameter type, including a recorded static parameter.
pub trait InterfaceTypedValueOps: MoniToolTypedValueOps {
    /// Get the parameter type, e.g. `value.param_type() == InterfaceParamType::ParamText`.
    fn param_type(&self) -> InterfaceParamType {
        InterfaceTypedValue::value_type_to_param_type(self.value_type())
    }
}

impl RuntimeTyped for dyn InterfaceTypedValueOps {
    const TYPE_NAME: &'static str = InterfaceTypedValue::TYPE_NAME;

    fn static_type() -> &'static RuntimeType {
        InterfaceTypedValue::static_type()
    }
}
impl_transient_interface_target!(dyn InterfaceTypedValueOps);
impl HandleUpcast<dyn MoniToolTypedValueOps> for dyn InterfaceTypedValueOps {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn MoniToolTypedValueOps> {
        owner
    }
}

impl InterfaceTypedValue {
    /// `InterfaceTypedValue(name, type = ParamText, init = "")`.
    pub fn new(name: &str, kind: InterfaceParamType, init: &str) -> Result<Self, Failure> {
        Ok(Self {
            value: MoniToolValue::new(name, Self::param_type_to_value_type(kind), init)?,
        })
    }
    /// `Type`: the value type as a parameter type.
    pub fn param_type(&self) -> InterfaceParamType {
        InterfaceTypedValueOps::param_type(self)
    }
    /// `ParamTypeToValueType(typ)`: the same value.
    pub fn param_type_to_value_type(typ: InterfaceParamType) -> MoniToolValueType {
        use InterfaceParamType as P;
        use MoniToolValueType as V;
        match typ {
            P::ParamMisc => V::ValueMisc,
            P::ParamInteger => V::ValueInteger,
            P::ParamReal => V::ValueReal,
            P::ParamIdent => V::ValueIdent,
            P::ParamVoid => V::ValueVoid,
            P::ParamText => V::ValueText,
            P::ParamEnum => V::ValueEnum,
            P::ParamLogical => V::ValueLogical,
            P::ParamSub => V::ValueSub,
            P::ParamHexa => V::ValueHexa,
            P::ParamBinary => V::ValueBinary,
        }
    }
    /// `ValueTypeToParamType(typ)`: the same value.
    pub fn value_type_to_param_type(typ: MoniToolValueType) -> InterfaceParamType {
        use InterfaceParamType as P;
        use MoniToolValueType as V;
        match typ {
            V::ValueMisc => P::ParamMisc,
            V::ValueInteger => P::ParamInteger,
            V::ValueReal => P::ParamReal,
            V::ValueIdent => P::ParamIdent,
            V::ValueVoid => P::ParamVoid,
            V::ValueText => P::ParamText,
            V::ValueEnum => P::ParamEnum,
            V::ValueLogical => P::ParamLogical,
            V::ValueSub => P::ParamSub,
            V::ValueHexa => P::ParamHexa,
            V::ValueBinary => P::ParamBinary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::standard::Handle;

    #[test]
    fn conversions_keep_the_value() {
        for value in 0..=10 {
            let typ: InterfaceParamType = param_of(value);
            let back = InterfaceTypedValue::param_type_to_value_type(typ);
            assert_eq!(back as i32, value);
            assert_eq!(InterfaceTypedValue::value_type_to_param_type(back), typ);
        }
        let bad = InterfaceTypedValue::new("x", InterfaceParamType::ParamHexa, "")
            .err()
            .unwrap();
        assert_eq!(bad.exception_type(), "Standard_ConstructionError");
        let value =
            Handle::new(InterfaceTypedValue::new("t", InterfaceParamType::ParamText, "").unwrap());
        let view = value.view::<dyn MoniToolTypedValueOps>();
        assert_eq!(view.object_address(), value.object_address());
        assert!(value.is_kind(<dyn MoniToolTypedValueOps>::static_type()));
    }

    fn param_of(value: i32) -> InterfaceParamType {
        use InterfaceParamType as P;
        [
            P::ParamMisc,
            P::ParamInteger,
            P::ParamReal,
            P::ParamIdent,
            P::ParamVoid,
            P::ParamText,
            P::ParamEnum,
            P::ParamLogical,
            P::ParamSub,
            P::ParamHexa,
            P::ParamBinary,
        ][value as usize]
    }
}
