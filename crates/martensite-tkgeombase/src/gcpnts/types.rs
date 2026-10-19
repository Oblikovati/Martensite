// the implementation at .

//! The curve classifications that select a GCPnts algorithm.

/// How an abscissa search treats the curve (`AbscissaType`);
/// `AbscissaType::LengthParametrized as i32 == 0`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum AbscissaType {
    /// The length is proportional to the parameter (`LengthParametrized`).
    LengthParametrized,
    /// A general parametrization (`Parametrized`).
    Parametrized,
    /// Several continuity intervals (`AbsComposite`).
    AbsComposite,
}

/// How a deflection distribution treats the curve (`DeflectionType`);
/// `DeflectionType::Curved as i32 == 2`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum DeflectionType {
    /// A straight curve: its two ends (`Linear`).
    Linear,
    /// A circle: a constant angular step (`Circular`).
    Circular,
    /// A general curve (`Curved`).
    Curved,
    /// Several continuity intervals (`DefComposite`).
    DefComposite,
}
