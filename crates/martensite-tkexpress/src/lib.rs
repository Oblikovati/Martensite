//! Symbolic expressions, functions, relations and a text interpreter.
//!
//! Expression handles share mutable nodes. Constructors return handles and each
//! node keeps a weak self-reference so operations can retain its identity.
//! Parsing owns its scanner, stacks and named-object context per invocation.
//!
//! ```
//! use martensite_tkexpress::expr_intrp::ExprIntrpGenExp;
//! use martensite_tkernel::tcollection::AsciiString;
//! let interpreter = ExprIntrpGenExp::create();
//! interpreter.process(&AsciiString::new("2+3"));
//! assert_eq!(interpreter.expression()?.evaluate_numeric()?, 5.0);
//! # Ok::<(), martensite_tkernel::standard::Failure>(())
//! ```
pub mod expr;
pub mod expr_intrp;
