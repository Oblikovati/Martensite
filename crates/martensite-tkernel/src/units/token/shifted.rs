//! A token whose SI conversion adds a move before scaling
//! (temperatures such as `deg.C`).
use super::Token;
use crate::standard::Handle;
use crate::units::TokenObject;
use crate::units::dimensions::Dimensions;

/// A token with an additive offset; `ShiftedToken::new(b"deg.C", b" ", 1.0, 273.15, &dims)`.
pub struct ShiftedToken {
    token: Token,
    movement: f64,
}
crate::impl_transient!(ShiftedToken, "Units_ShiftedToken", Token, interfaces: [dyn TokenObject]);

impl ShiftedToken {
    /// Builds the base token (dimensions copied) and stores the move;
    /// `ShiftedToken::new(b"deg.C", b"U", 1.0, 273.15, &Dimensions::a_thermodynamic_temperature())`.
    pub fn new(
        word: &[u8],
        mean: &[u8],
        value: f64,
        movement: f64,
        dimensions: &Handle<Dimensions>,
    ) -> Self {
        Self {
            token: Token::with_dimensions(word, mean, value, dimensions),
            movement,
        }
    }
    /// The inherited token part; `shifted.token().word()`.
    pub fn token(&self) -> &Token {
        &self.token
    }
    /// The additive offset (`Move`); `shifted.movement()`.
    pub fn movement(&self) -> f64 {
        self.movement
    }
    /// `(value + move) * factor` (`Multiplied` override); `shifted.multiplied(0.0)`.
    pub fn multiplied(&self, value: f64) -> f64 {
        (value + self.movement) * self.token.value()
    }
    /// `value / factor - move` (`Divided` override); `shifted.divided(273.15)`.
    pub fn divided(&self, value: f64) -> f64 {
        (value / self.token.value()) - self.movement
    }
    /// A new shifted copy with copied dimensions (`Creates` override);
    /// `shifted.creates()`.
    pub fn creates(&self) -> Handle<dyn TokenObject> {
        let token = &self.token;
        let copy = ShiftedToken::new(
            token.word().as_bytes(),
            token.mean().as_bytes(),
            token.value(),
            self.movement,
            &token.dimensions(),
        );
        Handle::new(copy).into_upcast::<dyn TokenObject>()
    }
}

impl crate::units::TokenObject for ShiftedToken {
    fn token_state(&self) -> &Token {
        &self.token
    }

    fn copy_token(&self) -> Handle<dyn TokenObject> {
        self.creates()
    }

    fn token_multiplied(&self, value: f64) -> f64 {
        self.multiplied(value)
    }

    fn token_divided(&self, value: f64) -> f64 {
        self.divided(value)
    }

    fn dump_token_tail(&self, shift: i32, out: &mut dyn std::io::Write) -> std::io::Result<()> {
        let indent = "  ".repeat(shift.max(0) as usize);
        let movement = crate::units::support::stream_real(self.movement());
        writeln!(out, "{indent}  move  : {movement}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_is_applied_before_scaling_and_copied() {
        let dims = Dimensions::a_thermodynamic_temperature();
        let celsius = ShiftedToken::new(b"deg.C", b"U", 1.0, 273.15, &dims);
        assert_eq!(celsius.multiplied(0.0), 273.15);
        assert_eq!(celsius.divided(273.15), 0.0);
        let copy = celsius.creates();
        assert!(copy.is_instance_name("Units_ShiftedToken"));
        assert_eq!(
            copy.downcast_ref::<ShiftedToken>().unwrap().movement(),
            273.15
        );
        assert_eq!(celsius.token().word(), "deg.C");
    }
}
