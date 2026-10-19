//! `ExprNamedExpression`: an expression known by its name (unknowns and named
//! constants).
use super::general_expression::ExprGeneralExpression;
use super::handles::{ExprHandle, deref};
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkernel::tcollection::AsciiString;

/// A named expression. The name (`myName`, a private field internally) is stored in each
/// concrete class and reached through [`ExprNamedExpression::name_state`]; the
/// provided members hold the shared logic once.
///
/// `ExprNamedUnknown::new(&AsciiString::new("x")).get_name() == "x"`
pub trait ExprNamedExpression: ExprGeneralExpression {
    /// The locked name of the concrete object; `named.name_state().read().len()`.
    fn name_state(&self) -> &SharedState<AsciiString>;
    /// The name (`GetName`); a copy, because the name lives behind a lock;
    /// `named.get_name()`.
    fn get_name(&self) -> AsciiString {
        self.name_state().read().clone()
    }
    /// Replace the name (`SetName`); `named.set_name(&AsciiString::new("y"))`.
    fn set_name(&self, name: &AsciiString) {
        *self.name_state().write() = name.clone();
    }
}
impl_transient_interface!(
    dyn ExprNamedExpression,
    "Expr_NamedExpression",
    dyn ExprGeneralExpression
);

/// `ExprNamedExpression::IsIdentical`: `other` is a named expression that is this
/// very object or has the same name; a null `other` is the null-handle failure;
/// `named_is_identical(&x, &other)?`.
pub fn named_is_identical<T: ExprNamedExpression + ?Sized>(
    me: &T,
    other: &ExprHandle,
) -> Result<bool, Failure> {
    deref(other, "Expr_NamedExpression::IsIdentical")?;
    let named: Handle<dyn ExprNamedExpression> = other.cast();
    let Some(named) = named.get() else {
        return Ok(false);
    };
    let same_object: bool = std::ptr::addr_eq(std::ptr::from_ref(named), std::ptr::from_ref(me));
    Ok(same_object || named.get_name() == me.get_name())
}

/// `ExprNamedExpression::IsShareable`: a named expression is shared, never copied, by
/// `Expr::CopyShare`; `named_is_shareable()`.
pub fn named_is_shareable() -> bool {
    true
}

/// `ExprNamedExpression::String`: the name; `named_string(&x)`.
pub fn named_string<T: ExprNamedExpression + ?Sized>(me: &T) -> Result<AsciiString, Failure> {
    Ok(me.get_name())
}
