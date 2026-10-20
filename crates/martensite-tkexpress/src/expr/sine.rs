//! `ExprSine`: The sine of an operand.
use super::arc_sine::ExprArcSine;
use super::cosine::ExprCosine;
use super::general_expression::{ExprGeneralExpression, ExprValues, ExprVariables};
use super::handles::ExprHandle;
use super::named_unknown::ExprNamedUnknown;
use super::package::copy_share;
use super::unary_expression::{declare_unary_class, unary_expression_members};
use super::unary_kit::chain;
use super::unary_kit::function_shallow_simplified;
use super::unary_kit::{evaluate_operand, function_members, shallow_of};
use martensite_tkernel::standard::{Failure, Handle};

declare_unary_class!(
    ExprSine,
    "Expr_Sine",
    "The sine of an operand; `ExprSine::new(&x).string()? == \"Sin(x)\"`."
);

impl ExprGeneralExpression for ExprSine {
    unary_expression_members!();
    function_members!(ExprSine, "Sin(");
    fn shallow_simplified(&self) -> Result<ExprHandle, Failure> {
        function_shallow_simplified::<ExprArcSine, _>(self, martensite_cmath::sin)
    }
    fn derivative(&self, x: &Handle<ExprNamedUnknown>) -> Result<ExprHandle, Failure> {
        chain(self, x, |operand| {
            shallow_of(&ExprCosine::new(&copy_share(operand)?))
        })
    }
    fn evaluate(&self, vars: &ExprVariables<'_>, vals: &ExprValues<'_>) -> Result<f64, Failure> {
        let value: f64 = evaluate_operand(self, vars, vals)?;
        Ok(martensite_cmath::sin(value))
    }
}
