//! Live shape fields retain native ownership once; reference adapters only lend their current contents.
use martensite_tkbrep::topods::{Shape, TopoDSShapeReadSource};
use martensite_tkernel::standard::{Failure, SharedState, catch_native_failure};
use std::sync::Arc;
/// Actual shape member storage. Example: `StdSelectShapeFields { shape: Arc::new(shape) }`.
pub struct StdSelectShapeFields {
    /// Native shape field. The read lease keeps the current value alive across replacement.
    pub shape: Arc<Shape>,
}
/// Const reference to the actual field, including later replacement; `reference.clone_shape()`.
pub struct StdSelectShapeRef<'a> {
    pub(super) fields: &'a SharedState<StdSelectShapeFields>,
}
impl StdSelectShapeRef<'_> {
    /// Check stable field reference identity without copying geometry; `reference.is_same_reference(&other)`.
    pub fn is_same_reference(&self, other: &Self) -> bool {
        let fields = self.fields;
        std::ptr::eq(fields, other.fields)
    }
    /// Acquire an explicit native owning copy; `let copy=reference.clone_shape();`.
    pub fn clone_shape(&self) -> Shape {
        let fields = self.fields.read();
        fields.shape.to_shape()
    }
    /// Test current field nullness without acquiring TShape ownership; `reference.is_null()`.
    pub fn is_null(&self) -> bool {
        let fields = self.fields.read();
        fields.shape.is_null()
    }
}
impl TopoDSShapeReadSource for StdSelectShapeRef<'_> {
    fn read_current_shape<R>(
        &self,
        project: impl FnOnce(&Shape) -> Result<R, Failure>,
    ) -> Result<R, Failure> {
        let shape = Arc::clone(&self.fields.read().shape);
        catch_native_failure(move || project(&shape))
    }
}
