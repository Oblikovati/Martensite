//! A named unit, its symbols, SI factor and owning quantity.
//!
//! Symbols are never edited after
//! creation, so the symbol sequence holds `AsciiString` values.
use super::quantity::Quantity;
use super::support::{c_str, item, length, new_sequence, object, stream_real, write_bytes};
use super::token::Token;
use crate::ncollection::HSequence;
use crate::standard::{Failure, Handle, StandardTransient};
use crate::tcollection::AsciiString;
use crate::units::TokenObject;
use std::io::Write;
use std::sync::{PoisonError, RwLock};

mod shifted;
pub use shifted::ShiftedUnit;

/// A unit of a quantity; `Unit::with_symbol(b"metre", b"m")`.
pub struct Unit {
    name: AsciiString,
    symbols: Handle<HSequence<AsciiString>>,
    value: RwLock<f64>,
    quantity: RwLock<Handle<Quantity>>,
}
crate::impl_transient!(Unit, "Units_Unit", StandardTransient, interfaces: [dyn UnitObject]);

impl Unit {
    fn build(name: &[u8], symbol: Option<&[u8]>, value: f64, quantity: Handle<Quantity>) -> Self {
        let symbols = new_sequence::<AsciiString>();
        if let Some(symbol) = symbol {
            symbols.get().unwrap().change_sequence(|items| {
                items.prepend(AsciiString::from_bytes(c_str(symbol)));
            });
        }
        Self {
            name: AsciiString::from_bytes(c_str(name)),
            symbols,
            value: RwLock::new(value),
            quantity: RwLock::new(quantity),
        }
    }
    /// Name, first symbol, SI factor and quantity; `Unit::new(b"metre", b"m", 1.0, &q)`.
    pub fn new(name: &[u8], symbol: &[u8], value: f64, quantity: &Handle<Quantity>) -> Self {
        Self::build(name, Some(symbol), value, quantity.clone())
    }
    /// Name and first symbol with a zero factor; `Unit::with_symbol(b"metre", b"m")`.
    pub fn with_symbol(name: &[u8], symbol: &[u8]) -> Self {
        Self::build(name, Some(symbol), 0.0, Handle::default())
    }
    /// A name without symbols; `Unit::with_name(b"metre")`.
    pub fn with_name(name: &[u8]) -> Self {
        Self::build(name, None, 0.0, Handle::default())
    }
    /// The unit name; `unit.name()`.
    pub fn name(&self) -> AsciiString {
        self.name.clone()
    }
    /// Appends an alternative symbol; `unit.symbol(b"mtr")`.
    pub fn symbol(&self, symbol: &[u8]) {
        let symbol = AsciiString::from_bytes(c_str(symbol));
        self.symbols.get().unwrap().append(symbol);
    }
    /// The SI factor; `unit.value()`.
    pub fn value(&self) -> f64 {
        *self.value.read().unwrap_or_else(PoisonError::into_inner)
    }
    /// Replaces the SI factor; `unit.set_value(1e-3)`.
    pub fn set_value(&self, value: f64) {
        *self.value.write().unwrap_or_else(PoisonError::into_inner) = value;
    }
    /// The owning quantity (null until set); `unit.quantity()`.
    pub fn quantity(&self) -> Handle<Quantity> {
        self.quantity
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
    /// Replaces the owning quantity; `unit.set_quantity(&quantity)`.
    pub fn set_quantity(&self, quantity: &Handle<Quantity>) {
        *self
            .quantity
            .write()
            .unwrap_or_else(PoisonError::into_inner) = quantity.clone();
    }
    /// The shared symbol sequence; `unit.symbols_sequence()`.
    pub fn symbols_sequence(&self) -> Handle<HSequence<AsciiString>> {
        self.symbols.clone()
    }
    /// True when any symbol equals the C string; `unit.is_equal(b"m")?`.
    pub fn is_equal(&self, text: &[u8]) -> Result<bool, Failure> {
        let text = c_str(text);
        for index in 1..=length(&self.symbols)? {
            if item(&self.symbols, index)?.as_bytes() == text {
                return Ok(true);
            }
        }
        Ok(false)
    }
    /// The base `Token()`: first symbol, blank meaning, factor, quantity dimensions.
    pub(super) fn plain_token(&self) -> Result<Handle<dyn TokenObject>, Failure> {
        let symbol = item(&self.symbols, 1)?;
        let quantity = self.quantity();
        let dimensions = object(&quantity, "Units_Unit::Token")?.dimensions();
        Ok(Handle::new(Token::with_dimensions(
            symbol.as_bytes(),
            b" ",
            self.value(),
            &dimensions,
        ))
        .into_upcast::<dyn TokenObject>())
    }
    /// The quoted symbol list shared by both `Dump` overrides.
    pub(super) fn dump_symbols(&self, out: &mut dyn Write) -> Result<(), Failure> {
        for index in 1..=length(&self.symbols)? {
            let separator: &[u8] = if index != 1 { b" or " } else { b"" };
            let symbol = item(&self.symbols, index)?;
            let quoted = [separator, b"\"", symbol.as_bytes(), b"\""];
            write_bytes(out, &quoted.concat())?;
        }
        Ok(())
    }
    /// The base dump text; `shift` and `level` are unused.
    pub(super) fn dump_plain(&self, out: &mut dyn Write) -> Result<(), Failure> {
        self.dump_symbols(out)?;
        let tail = format!("\t\t(= {} SI)\n", stream_real(self.value()));
        let line = [
            b"\t\tName:  ".as_slice(),
            self.name.as_bytes(),
            tail.as_bytes(),
        ];
        write_bytes(out, &line.concat())
    }
}

/// Virtual members of units, dispatched on the handle's dynamic type.
pub trait UnitHandle {
    /// `Token()`: a (shifted) token for the first symbol; `unit.token()?`.
    fn token(&self) -> Result<Handle<dyn TokenObject>, Failure>;
    /// Example: `unit.dump(0, 0, &mut std::io::stdout())?`.
    fn dump(&self, shift: i32, level: i32, out: &mut dyn Write) -> Result<(), Failure>;
}

impl UnitHandle for Handle<dyn UnitObject> {
    fn token(&self) -> Result<Handle<dyn TokenObject>, Failure> {
        object(self, "Units_Unit::Token")?.first_symbol_token()
    }
    fn dump(&self, _shift: i32, _level: i32, out: &mut dyn Write) -> Result<(), Failure> {
        object(self, "Units_Unit::Dump")?.dump_unit(out)
    }
}

/// A polymorphic unit with access to its shared state.
pub trait UnitObject: crate::standard::Transient {
    /// Borrow this object's unit state; `object.unit_state()`.
    fn unit_state(&self) -> &Unit;

    /// Build a token from this unit's first symbol; `object.first_symbol_token()?`.
    fn first_symbol_token(&self) -> Result<Handle<dyn TokenObject>, Failure> {
        self.unit_state().plain_token()
    }

    /// Write the unit's symbol, name and SI conversion; `object.dump_unit(&mut output)?`.
    fn dump_unit(&self, out: &mut dyn Write) -> Result<(), Failure> {
        self.unit_state().dump_plain(out)
    }
}

impl crate::standard::RuntimeTyped for dyn UnitObject {
    const TYPE_NAME: &'static str = <Unit as crate::standard::RuntimeTyped>::TYPE_NAME;
    fn static_type() -> &'static crate::standard::RuntimeType {
        <Unit as crate::standard::RuntimeTyped>::static_type()
    }
}
crate::impl_transient_interface_target!(dyn UnitObject);

impl UnitObject for Unit {
    fn unit_state(&self) -> &Unit {
        self
    }
}

impl std::ops::Deref for dyn UnitObject {
    type Target = Unit;
    fn deref(&self) -> &Self::Target {
        self.unit_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::Dimensions;

    fn length_quantity() -> Handle<Quantity> {
        Handle::new(Quantity::new(
            b"LENGTH",
            &Dimensions::a_length(),
            &new_sequence(),
        ))
    }

    #[test]
    fn symbols_values_and_token() {
        let quantity = length_quantity();
        let unit = Handle::new(Unit::new(b"millimetre", b"mm", 1e-3, &quantity))
            .into_upcast::<dyn UnitObject>();
        let object = unit.get().unwrap();
        object.symbol(b"mmx");
        assert!(object.is_equal(b"mmx").unwrap() && !object.is_equal(b"m").unwrap());
        let token = unit.token().unwrap();
        assert_eq!(token.get().unwrap().word(), "mm");
        assert_eq!(token.get().unwrap().value(), 1e-3);
        assert_eq!(
            token.get().unwrap().dimensions().get().unwrap().length(),
            1.0
        );
        let mut out = Vec::new();
        unit.dump(0, 0, &mut out).unwrap();
        assert_eq!(
            out,
            b"\"mm\" or \"mmx\"\t\tName:  millimetre\t\t(= 0.001 SI)\n"
        );
    }

    #[test]
    fn missing_symbol_or_quantity_is_reported() {
        let bare: Handle<dyn UnitObject> =
            Handle::new(Unit::with_name(b"bare")).into_upcast::<dyn UnitObject>();
        let failure = bare.token().err().unwrap();
        assert_eq!(failure.message(), "NCollection_Sequence::Value");
        let orphan: Handle<dyn UnitObject> =
            Handle::new(Unit::with_symbol(b"orphan", b"o")).into_upcast::<dyn UnitObject>();
        assert!(orphan.token().is_err());
        orphan.get().unwrap().set_value(2.0);
        assert_eq!(orphan.get().unwrap().value(), 2.0);
        assert!(orphan.get().unwrap().quantity().is_null());
    }
}
