//! Numeric rendering and integer conversion for symbolic text.
use super::handles::{ExprHandle, deref};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::AsciiString;

/// The text of an operand, in parentheses when it has more than one sub-expression:
/// the rule of every infix `String()`; `operand_text(&sum)? == "(x+y)"`.
pub fn operand_text(operand: &ExprHandle) -> Result<AsciiString, Failure> {
    let expression = deref(operand, "String")?;
    let text: AsciiString = expression.string()?;
    if expression.nb_sub_expressions() > 1 {
        let mut grouped: AsciiString = AsciiString::new("(");
        grouped += &text;
        grouped += ")";
        return Ok(grouped);
    }
    Ok(text)
}

/// The operands' texts joined by `separator`, each by [`operand_text`]: the `String()`
/// of a sum, a product, a difference, a quotient and a power;
/// `infix_text(&[x, y], "+")? == "x+y"`.
pub fn infix_text(operands: &[ExprHandle], separator: &str) -> Result<AsciiString, Failure> {
    let mut text: AsciiString = AsciiString::new("");
    for (index, operand) in operands.iter().enumerate() {
        if index > 0 {
            text += separator;
        }
        text += &operand_text(operand)?;
    }
    Ok(text)
}

/// `prefix` + the operand's text + `)`: the `String()` of the standard functions
/// (`Sin(x)`, `Sqrt(x)`, ...); `call_text("Sin(", &x)? == "Sin(x)"`.
pub fn call_text(prefix: &str, operand: &ExprHandle) -> Result<AsciiString, Failure> {
    let mut text: AsciiString = AsciiString::new(prefix);
    text += &deref(operand, "String")?.string()?;
    text += ")";
    Ok(text)
}
