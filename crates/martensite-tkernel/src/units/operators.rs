//! The free operators on handles, each forwarding to the
//! member of the same meaning. A null left handle is `Standard_NullObject`.
use super::dimensions::Dimensions;
use super::quantity::Quantity;
use super::support::object;
use super::token::{Token, TokenHandle};
use super::unit::Unit;
use crate::standard::{Failure, Handle};

/// Quantity name equality; `quantity_equals(&q, b"MASS")?`.
pub fn quantity_equals(quantity: &Handle<Quantity>, text: &[u8]) -> Result<bool, Failure> {
    Ok(object(quantity, "operator==")?.is_equal(text))
}
/// Token word equality; `token_equals(&t, b"m")?`.
pub fn token_equals(token: &Handle<Token>, text: &[u8]) -> Result<bool, Failure> {
    Ok(object(token, "operator==")?.is_equal(text))
}
/// Unit symbol equality: any symbol matches; `unit_equals(&u, b"m")?`.
pub fn unit_equals(unit: &Handle<Unit>, text: &[u8]) -> Result<bool, Failure> {
    object(unit, "operator==")?.is_equal(text)
}
/// Multiplies dimensions; `dimensions_multiply(&a, &b)?`.
pub fn dimensions_multiply(
    left: &Handle<Dimensions>,
    right: &Handle<Dimensions>,
) -> Result<Handle<Dimensions>, Failure> {
    object(left, "operator*")?.multiply(right)
}
/// Divides dimensions; `dimensions_divide(&a, &b)?`.
pub fn dimensions_divide(
    left: &Handle<Dimensions>,
    right: &Handle<Dimensions>,
) -> Result<Handle<Dimensions>, Failure> {
    object(left, "operator/")?.divide(right)
}
/// Dimensions raised to a real power; `dimensions_pow(&a, 2.0)?`.
pub fn dimensions_pow(
    dimensions: &Handle<Dimensions>,
    exponent: f64,
) -> Result<Handle<Dimensions>, Failure> {
    Ok(object(dimensions, "pow")?.power(exponent))
}
/// A token raised to a token's value; `token_pow_token(&a, &b)?`.
pub fn token_pow_token(
    left: &Handle<Token>,
    right: &Handle<Token>,
) -> Result<Handle<Token>, Failure> {
    object(left, "pow")?.power_token(right)
}
/// A token raised to a real power; `token_pow(&a, 2.0)?`.
pub fn token_pow(token: &Handle<Token>, exponent: f64) -> Result<Handle<Token>, Failure> {
    object(token, "pow")?.power(exponent)
}
/// Adding an integer gives a blank token; `token_add_integer(&a, 1)?`.
pub fn token_add_integer(token: &Handle<Token>, integer: i32) -> Result<Handle<Token>, Failure> {
    Ok(object(token, "operator+")?.add_integer(integer))
}
/// Adds tokens; `token_add(&a, &b)?`.
pub fn token_add(left: &Handle<Token>, right: &Handle<Token>) -> Result<Handle<Token>, Failure> {
    object(left, "operator+")?.add(right)
}
/// Subtracts tokens; `token_subtract(&a, &b)?`.
pub fn token_subtract(
    left: &Handle<Token>,
    right: &Handle<Token>,
) -> Result<Handle<Token>, Failure> {
    object(left, "operator-")?.subtract(right)
}
/// Multiplies tokens; `token_multiply(&a, &b)?`.
pub fn token_multiply(
    left: &Handle<Token>,
    right: &Handle<Token>,
) -> Result<Handle<Token>, Failure> {
    object(left, "operator*")?.multiply(right)
}
/// Divides tokens (the left token itself for a near-zero divisor);
/// `token_divide(&a, &b)?`.
pub fn token_divide(left: &Handle<Token>, right: &Handle<Token>) -> Result<Handle<Token>, Failure> {
    left.divide(right)
}
/// Token word inequality; `token_not_equals(&a, b"m")?`.
pub fn token_not_equals(token: &Handle<Token>, text: &[u8]) -> Result<bool, Failure> {
    Ok(object(token, "operator!=")?.is_not_equal(text))
}
/// Token word less-or-equal; `token_less_or_equal(&a, b"mm")?`.
pub fn token_less_or_equal(token: &Handle<Token>, text: &[u8]) -> Result<bool, Failure> {
    Ok(object(token, "operator<=")?.is_less_or_equal(text))
}
/// Token word greater; `token_greater(&a, b"a")?`.
pub fn token_greater(token: &Handle<Token>, text: &[u8]) -> Result<bool, Failure> {
    Ok(object(token, "operator>")?.is_greater(text))
}
/// Compares two tokens with `>`; `token_greater_token(&a, &b)?`.
pub fn token_greater_token(left: &Handle<Token>, right: &Handle<Token>) -> Result<bool, Failure> {
    object(left, "operator>")?.is_greater_token(right)
}
/// Compares two tokens with `>=`; `token_greater_or_equal(&a, &b)?`.
pub fn token_greater_or_equal(
    left: &Handle<Token>,
    right: &Handle<Token>,
) -> Result<bool, Failure> {
    object(left, "operator>=")?.is_greater_or_equal(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators_forward_to_members() {
        let metre = Handle::new(Token::with_dimensions(
            b"m",
            b"U",
            1.0,
            &Dimensions::a_length(),
        ));
        assert!(token_equals(&metre, b"m").unwrap() && !token_not_equals(&metre, b"m").unwrap());
        assert!(
            token_less_or_equal(&metre, b"mm").unwrap() && token_greater(&metre, b"a").unwrap()
        );
        let area = token_pow(&metre, 2.0).unwrap();
        assert!(!token_greater_token(&area, &metre).unwrap());
        assert!(token_greater_or_equal(&metre, &area).unwrap());
        let sum = token_add(&metre, &metre).unwrap();
        assert_eq!(sum.get().unwrap().value(), 2.0);
        assert_eq!(
            token_subtract(&metre, &metre)
                .unwrap()
                .get()
                .unwrap()
                .value(),
            0.0
        );
        assert_eq!(
            token_multiply(&metre, &metre)
                .unwrap()
                .get()
                .unwrap()
                .word(),
            "(m)*(m)"
        );
        assert_eq!(
            token_divide(&metre, &sum).unwrap().get().unwrap().value(),
            0.5
        );
        assert_eq!(
            token_pow_token(&metre, &sum)
                .unwrap()
                .get()
                .unwrap()
                .value(),
            1.0
        );
        assert_eq!(
            token_add_integer(&metre, 4).unwrap().get().unwrap().word(),
            " "
        );
        let length = Dimensions::a_length();
        let time = Dimensions::a_time();
        let speed = dimensions_divide(&length, &time).unwrap();
        let back = dimensions_multiply(&speed, &time).unwrap();
        assert!(back.get().unwrap().is_equal(&length).unwrap());
        assert_eq!(
            dimensions_pow(&length, 3.0)
                .unwrap()
                .get()
                .unwrap()
                .length(),
            3.0
        );
        assert!(token_equals(&Handle::default(), b"m").is_err());
    }
}
