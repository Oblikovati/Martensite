//! Symbolic expressions, functions, relations and a text interpreter.
//!
//! Expression handles share mutable nodes. Constructors return handles and each
//! node keeps a weak self-reference so operations can retain its identity.
//! Parsing owns its scanner, stacks and named-object context per invocation.
//!
//! # Rust-native object model (#592)
//!
//! Move fresh nodes into an expression interface with `into_upcast()` and borrow
//! their concrete type with `downcast_ref()`. Class descriptors keep the existing
//! `Expr_*` and `ExprIntrp_*` names and ancestry.
//!
//! A parser's generator is a `Handle<dyn ExprIntrpGeneratorObject>`. Both a
//! standalone generator and an interpreter implement that interface; its
//! `generator_state()` borrows their original definitions. Convert an existing
//! interpreter with `upcast::<dyn ExprIntrpGeneratorObject>()`. A concrete
//! `downcast::<ExprIntrpGenerator>()` only accepts a standalone generator.
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
