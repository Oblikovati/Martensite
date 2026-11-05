//! Offset analysis interval storage from BRepOffsetInterval.
// leaves default scalars uninitialized.
use martensite_tkernel::standard::Failure;
use martensite_tkfillet::chfi_ds::TypeOfConcavity;

/// Stores bounds without sorting or validation; `BRepOffsetInterval::new(3., 1., TypeOfConcavity::mixed)`.
/// Default construction leaves each field unset, matching reference initialization intent.
/// Reading an unset field is a Rust error; the corresponding reference read is undefined.
#[derive(Clone, Copy, Debug, Default)]
pub struct BRepOffsetInterval {
    first: Option<f64>,
    last: Option<f64>,
    concavity: Option<TypeOfConcavity>,
}
impl BRepOffsetInterval {
    /// Preserve the supplied bounds and classification; `BRepOffsetInterval::new(3., 1., TypeOfConcavity::mixed)`.
    pub fn new(first: f64, last: f64, concavity: TypeOfConcavity) -> Self {
        Self {
            first: Some(first),
            last: Some(last),
            concavity: Some(concavity),
        }
    }
    /// Assign the first bound independently; `interval.set_first(-2.)`.
    pub fn set_first(&mut self, first: f64) {
        self.first = Some(first);
    }
    /// Assign the last bound independently; `interval.set_last(4.)`.
    pub fn set_last(&mut self, last: f64) {
        self.last = Some(last);
    }
    /// Assign the reference type; `interval.set_type(TypeOfConcavity::other)`.
    pub fn set_type(&mut self, concavity: TypeOfConcavity) {
        self.concavity = Some(concavity);
    }
    /// Read an initialized first bound; `let first = interval.first()?`.
    pub fn first(&self) -> Result<f64, Failure> {
        self.first.ok_or_else(|| unset_interval_field("first"))
    }
    /// Read an initialized last bound; `let last = interval.last()?`.
    pub fn last(&self) -> Result<f64, Failure> {
        self.last.ok_or_else(|| unset_interval_field("last"))
    }
    /// Read an initialized reference type; `let concavity = interval.get_type()?`.
    pub fn get_type(&self) -> Result<TypeOfConcavity, Failure> {
        self.concavity.ok_or_else(|| unset_interval_field("type"))
    }
}
fn unset_interval_field(field: &str) -> Failure {
    Failure::with_message(
        "Standard_ConstructionError",
        &format!("BRepOffset_Interval {field}=None; expected an initialized field before reading"),
    )
}
