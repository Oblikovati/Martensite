//! a syntax failure (a StandardFailure internally), as a [`Failure`] value.
use martensite_tkernel::standard::Failure;

/// a syntax failure; `syntax_error("syntax error")`.
pub fn syntax_error(message: &str) -> Failure {
    Failure::with_type("ExprIntrp_SyntaxError", message, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_failure_preserves_category_and_message() {
        let failure: Failure = syntax_error("syntax error");
        assert_eq!(failure.exception_type(), "ExprIntrp_SyntaxError");
        assert_eq!(failure.message(), "syntax error");
    }
}
