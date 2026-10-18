//! `ExprGeneralExpression`: the root of the expression hierarchy.
use super::errors::{not_evaluable, out_of_range};
use super::handles::{ExprHandle, deref, one_based};
use super::named_unknown::ExprNamedUnknown;
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, Transient};
use martensite_tkernel::tcollection::AsciiString;

/// The variables of an evaluation (`Array1<Handle<ExprNamedUnknown>>`).
pub type ExprVariables<'a> = Array1<'a, Handle<ExprNamedUnknown>>;
/// The values of an evaluation (`Array1<double>`).
pub type ExprValues<'a> = Array1<'a, f64>;

/// Common behaviour of every expression: structure, simplification, derivation,
/// evaluation and text. Every member takes `&self`; a member that changes the
/// expression (`replace`) changes it for every handle that shares it.
///
/// `let text = ExprSum::new(&x, &y).string()?;`
pub trait ExprGeneralExpression: Transient {
    /// Number of sub-expressions (`NbSubExpressions`); `sum.nb_sub_expressions()`.
    fn nb_sub_expressions(&self) -> i32;
    /// The `index`-th sub-expression, 1-based (`SubExpression`); an out-of-range failure
    /// outside `1..=nb_sub_expressions()`; `sum.sub_expression(1)?`.
    fn sub_expression(&self, index: i32) -> Result<ExprHandle, Failure>;
    /// Copy with assigned unknowns replaced and values computed (`Simplified`);
    /// `expression.simplified()?`.
    fn simplified(&self) -> Result<ExprHandle, Failure>;
    /// Simplification of this level only, the operands are kept (`ShallowSimplified`);
    /// it can return the expression itself; `expression.shallow_simplified()?`.
    fn shallow_simplified(&self) -> Result<ExprHandle, Failure>;
    /// A copy sharing the shareable sub-expressions (`Copy`); `expression.copy()?`.
    fn copy(&self) -> Result<ExprHandle, Failure>;
    /// True when a named unknown occurs (`ContainsUnknowns`); `expression.contains_unknowns()?`.
    fn contains_unknowns(&self) -> Result<bool, Failure>;
    /// True when `exp` occurs in this expression (`Contains`); `expression.contains(&x)?`.
    fn contains(&self, exp: &ExprHandle) -> Result<bool, Failure>;
    /// True when linear on every unknown (`IsLinear`); `expression.is_linear()?`.
    fn is_linear(&self) -> Result<bool, Failure>;
    /// True when the object may be shared instead of copied (`IsShareable`, false by
    /// default, true for named expressions); `expression.is_shareable()`.
    fn is_shareable(&self) -> bool {
        false
    }
    /// True when `other` is the same expression, without simplification
    /// (`IsIdentical`); `expression.is_identical(&other)?`.
    fn is_identical(&self, other: &ExprHandle) -> Result<bool, Failure>;
    /// The derivative on `x` (`Derivative`); `expression.derivative(&x)?`.
    fn derivative(&self, x: &Handle<ExprNamedUnknown>) -> Result<ExprHandle, Failure>;
    /// The `n`-th derivative on `x` (`NDerivative`); an out-of-range failure for `n <= 0`;
    /// `expression.n_derivative(&x, 2)?`.
    fn n_derivative(&self, x: &Handle<ExprNamedUnknown>, n: i32) -> Result<ExprHandle, Failure> {
        general_n_derivative(self, x, n)
    }
    /// Replace every occurrence of `var` with `with` (`Replace`); an invalid-operand failure
    /// when `with` contains the expression; `expression.replace(&x, &value)?`.
    fn replace(&self, var: &Handle<ExprNamedUnknown>, with: &ExprHandle) -> Result<(), Failure>;
    /// The value with `vars` replaced by `vals` (`Evaluate`); an unresolved-value failure for an
    /// unknown outside `vars`; `expression.evaluate(&vars, &vals)?`.
    fn evaluate(&self, vars: &ExprVariables<'_>, vals: &ExprValues<'_>) -> Result<f64, Failure>;
    /// The value of an expression without unknowns (`EvaluateNumeric`);
    /// an unresolved-value failure when it contains unknowns; `expression.evaluate_numeric()?`.
    fn evaluate_numeric(&self) -> Result<f64, Failure> {
        general_evaluate_numeric(self)
    }
    /// The readable text of the expression (`String`); `expression.string()?`.
    fn string(&self) -> Result<AsciiString, Failure>;
}
impl_transient_interface!(
    dyn ExprGeneralExpression,
    "Expr_GeneralExpression",
    StandardTransient
);

/// `ExprGeneralExpression::NDerivative`: the first derivative, derived again `n - 1`
/// times through each result's own `NDerivative`; `general_n_derivative(&sum, &x, 2)?`.
pub fn general_n_derivative<T: ExprGeneralExpression + ?Sized>(
    me: &T,
    x: &Handle<ExprNamedUnknown>,
    n: i32,
) -> Result<ExprHandle, Failure> {
    if n <= 0 {
        return Err(out_of_range(&format!(
            "Expr_GeneralExpression::NDerivative: N = {n}; expected >= 1"
        )));
    }
    let first: ExprHandle = me.derivative(x)?;
    if n > 1 {
        return deref(&first, "NDerivative")?.n_derivative(x, n - 1);
    }
    Ok(first)
}

/// `ExprGeneralExpression::EvaluateNumeric`: evaluation with one null variable and one
/// zero value, after refusing unknowns; `general_evaluate_numeric(&sum)?`.
pub fn general_evaluate_numeric<T: ExprGeneralExpression + ?Sized>(me: &T) -> Result<f64, Failure> {
    if me.contains_unknowns()? {
        return Err(not_evaluable(
            "Expr_GeneralExpression::EvaluateNumeric: the expression contains unknowns; expected none",
        ));
    }
    let vars: ExprVariables<'static> = one_based(vec![Handle::default()]);
    let vals: ExprValues<'static> = one_based(vec![0.0]);
    me.evaluate(&vars, &vals)
}
