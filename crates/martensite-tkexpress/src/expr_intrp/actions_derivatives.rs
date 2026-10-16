//! The semantic actions of derivations: `Deriv[e, x, n]`, `f'(u)` and `@f/@X1(u, v)`.
use super::actions_functions::standard_function;
use super::context::{ExprIntrpParseContext, atoi_after_first};
use super::errors::syntax_error;
use crate::expr::ExprGeneralFunction;
use crate::expr::handles::deref;
use crate::expr::{
    ExprBinaryFunction, ExprFunctionDerivative, ExprFunctionHandle, ExprHandle, ExprNamedUnknown,
    ExprPolyFunction, ExprUnaryFunction, ExprUnknownIterator,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;

impl ExprIntrpParseContext<'_> {
    /// remember the name of the derived function.
    pub(super) fn start_derivate(&mut self) -> Result<(), Failure> {
        let name = self.result();
        self.recept.push_name(&name);
        Ok(())
    }
    /// stack the number of quotes as the degree.
    pub(super) fn end_derivate(&mut self) -> Result<(), Failure> {
        let degree: i32 = self.scanner.get_degree();
        self.recept.push_value(degree);
        Ok(())
    }
    /// degree 1 and the variable of `Deriv[e, x]`, a new
    /// unknown (not made known) when the name is new; a syntax failure for a
    /// constant.
    pub(super) fn derivation(&mut self) -> Result<(), Failure> {
        self.recept.push_value(1);
        let name = self.result();
        let mut unknown: Handle<ExprNamedUnknown> = self.named_unknown(&name)?;
        if unknown.is_null() {
            unknown = ExprNamedUnknown::new(&name);
        }
        self.recept.push(&unknown.cast());
        Ok(())
    }
    /// replace the degree 1 by the value read, through
    /// `IntegerValue` (a numeric failure without leading digits).
    pub(super) fn derivation_value(&mut self) -> Result<(), Failure> {
        let text = self.result();
        self.recept.pop_value();
        self.recept.push_value(text.integer_value()?);
        Ok(())
    }
    /// the stacked expression derived on the variable.
    pub(super) fn end_derivation(&mut self) -> Result<(), Failure> {
        let degree: i32 = self.recept.pop_value();
        let variable: Handle<ExprNamedUnknown> = self.recept.pop().cast();
        let expression: ExprHandle = self.recept.pop();
        let derived: ExprHandle =
            deref(&expression, "ExprIntrp_EndDerivation")?.n_derivative(&variable, degree)?;
        self.recept.push(&derived);
        Ok(())
    }
    /// remember the function name, no differential yet.
    pub(super) fn start_differential(&mut self) -> Result<(), Failure> {
        self.start_derivate()?;
        self.nbdiff = 0;
        Ok(())
    }
    /// stack the rank `n` of the variable `Xn` (or `xn`);
    /// a syntax failure for another name.
    pub(super) fn diff_degree_var(&mut self) -> Result<(), Failure> {
        let text = self.result();
        if !matches!(text.as_bytes().first(), Some(b'X' | b'x')) {
            return Err(syntax_error(&format!(
                "differential variable {}; expected X<rank>",
                String::from_utf8_lossy(text.as_bytes())
            )));
        }
        self.recept.push_value(atoi_after_first(text.as_bytes()));
        self.nbdiff += 1;
        Ok(())
    }
    /// degree 1, then the variable rank.
    pub(super) fn diff_var(&mut self) -> Result<(), Failure> {
        self.recept.push_value(1);
        self.diff_degree_var()
    }
    /// stack the degree of `@n f`.
    pub(super) fn diff_degree(&mut self) -> Result<(), Failure> {
        let degree: i32 = self.result().integer_value()?;
        self.recept.push_value(degree);
        Ok(())
    }
    /// the degree of `@n Xk` must equal the stacked one;
    /// a syntax failure otherwise.
    pub(super) fn ver_diff_degree(&mut self) -> Result<(), Failure> {
        let degree: i32 = self.result().integer_value()?;
        let stacked: i32 = self.recept.pop_value();
        if degree != stacked {
            return Err(syntax_error(&format!(
                "differential degree {degree}; expected {stacked}"
            )));
        }
        self.recept.push_value(degree);
        Ok(())
    }
    /// the named function derived once per differential,
    /// on the variables of the stacked ranks; a syntax failure for an unknown
    /// function or a rank outside its variables.
    pub(super) fn end_differential(&mut self) -> Result<(), Failure> {
        let name: AsciiString = self.recept.pop_name();
        let mut function: ExprFunctionHandle = self.known_function(&name)?.cast();
        let count: i32 = deref(&function, "ExprIntrp_EndDifferential")?.nb_of_variables();
        for _ in 0..self.nbdiff {
            let rank: i32 = self.recept.pop_value();
            let degree: i32 = self.recept.pop_value();
            if rank > count || rank < 1 {
                return Err(syntax_error(&format!(
                    "differential rank {rank}; expected in [1, {count}]"
                )));
            }
            let variable = deref(&function, "ExprIntrp_EndDifferential")?.variable(rank)?;
            function = ExprFunctionDerivative::new(&function, &variable, degree)?.cast();
        }
        self.recept.push_function(&function);
        Ok(())
    }
    /// the derived function applied to the stacked
    /// arguments; a syntax failure when an argument is missing.
    pub(super) fn end_diff_function(&mut self) -> Result<(), Failure> {
        let function: ExprFunctionHandle = self.recept.pop_function();
        let count: i32 = deref(&function, "ExprIntrp_EndDiffFunction")
            .map_err(|_| {
                syntax_error("ExprIntrp_EndDiffFunction: no function; expected a derived function")
            })?
            .nb_of_variables();
        let application: ExprHandle = match count {
            1 => {
                let operand: ExprHandle = self.recept.pop();
                ExprUnaryFunction::new(&function, &operand)?.cast()
            }
            2 => {
                let second: ExprHandle = self.recept.pop();
                let first: ExprHandle = self.pop_required("ExprIntrp_EndDiffFunction")?;
                ExprBinaryFunction::new(&function, &first, &second)?.cast()
            }
            _ => {
                let mut arguments: Vec<ExprHandle> = Vec::new();
                for _ in 0..count.max(0) {
                    arguments.push(self.pop_required("ExprIntrp_EndDiffFunction")?);
                }
                arguments.reverse();
                ExprPolyFunction::new(&function, &arguments).cast()
            }
        };
        self.recept.push(&application);
        Ok(())
    }
    /// `f'(u)`. A standard function is derived on the one
    /// free unknown of `f(u)` (a syntax failure when there is none or more than
    /// one); a defined function of one variable becomes its derivative applied to `u`.
    pub(super) fn end_der_function(&mut self) -> Result<(), Failure> {
        let name: AsciiString = self.recept.pop_name();
        let operand: ExprHandle = self.recept.pop();
        let standard: ExprHandle = standard_function(&name, &operand);
        if !standard.is_null() {
            let variable: Handle<ExprNamedUnknown> = single_free_unknown(&standard)?;
            let degree: i32 = self.recept.pop_value();
            let derived: ExprHandle =
                deref(&standard, "ExprIntrp_EndDerFunction")?.n_derivative(&variable, degree)?;
            self.recept.push(&derived);
            return Ok(());
        }
        let function = self.known_function(&name)?;
        let count: i32 = deref(&function, "ExprIntrp_EndDerFunction")?.nb_of_variables();
        if count != 1 {
            return Err(syntax_error(&format!(
                "{}' has {count} variables; expected 1",
                String::from_utf8_lossy(name.as_bytes())
            )));
        }
        let function: ExprFunctionHandle = function.cast();
        let variable = deref(&function, "ExprIntrp_EndDerFunction")?.variable(1)?;
        let degree: i32 = self.recept.pop_value();
        let derivative: ExprFunctionHandle =
            ExprFunctionDerivative::new(&function, &variable, degree)?.cast();
        let application: ExprHandle = ExprUnaryFunction::new(&derivative, &operand)?.cast();
        self.recept.push(&application);
        Ok(())
    }
}

/// The unknown of `expression` when it has exactly one, free: the loop of
/// `ExprIntrpEndDerFunction`, which forgets an assigned unknown and refuses a second
/// one met while it holds the first; a syntax failure otherwise.
fn single_free_unknown(expression: &ExprHandle) -> Result<Handle<ExprNamedUnknown>, Failure> {
    let mut unknowns: ExprUnknownIterator = ExprUnknownIterator::new(expression)?;
    let mut variable: Handle<ExprNamedUnknown> = Handle::default();
    while unknowns.more() {
        if !variable.is_null() {
            return Err(syntax_error(
                "f'(u): u has more than one unknown; expected one",
            ));
        }
        variable = unknowns.value()?;
        if deref(&variable, "ExprIntrp_EndDerFunction")?.is_assigned() {
            variable = Handle::default();
        }
        unknowns.next()?;
    }
    if variable.is_null() {
        return Err(syntax_error("f'(u): u has no free unknown; expected one"));
    }
    Ok(variable)
}
