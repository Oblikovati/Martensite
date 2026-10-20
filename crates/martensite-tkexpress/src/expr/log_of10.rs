//! `ExprLogOf10`: The decimal logarithm of an operand.
use super::general_expression::{ExprGeneralExpression, ExprValues, ExprVariables};
use super::handles::ExprHandle;
use super::named_unknown::ExprNamedUnknown;
use super::numeric_value::ExprNumericValue;
use super::operators::expr_multiply;
use super::package::copy_share;
use super::unary_expression::{declare_unary_class, unary_expression_members};
use super::unary_kit::quotient_rule;
use super::unary_kit::value_shallow_simplified;
use super::unary_kit::{evaluate_operand, function_members, shallow_of};
use martensite_tkernel::standard::{Failure, Handle};

declare_unary_class!(
    ExprLogOf10,
    "Expr_LogOf10",
    "The decimal logarithm of an operand; `ExprLogOf10::new(&x).string()? == \"log(x)\"`."
);

impl ExprGeneralExpression for ExprLogOf10 {
    unary_expression_members!();
    function_members!(ExprLogOf10, "log(");
    fn shallow_simplified(&self) -> Result<ExprHandle, Failure> {
        value_shallow_simplified(self, martensite_cmath::log10)
    }
    fn derivative(&self, x: &Handle<ExprNamedUnknown>) -> Result<ExprHandle, Failure> {
        quotient_rule(self, x, |operand| {
            let ln10: ExprHandle = ExprNumericValue::expression(martensite_cmath::log(10.0_f64));
            shallow_of(&expr_multiply(&copy_share(operand)?, &ln10))
        })
    }
    fn evaluate(&self, vars: &ExprVariables<'_>, vals: &ExprValues<'_>) -> Result<f64, Failure> {
        let value: f64 = evaluate_operand(self, vars, vals)?;
        Ok(martensite_cmath::log10(value))
    }
}
