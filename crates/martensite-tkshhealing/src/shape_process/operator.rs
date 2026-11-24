//! The operators of ShapeProcess: the abstract `ShapeProcessOperator` and the operator
//! `ShapeProcessUOperator` that calls a function.
use super::ShapeProcessContextSource;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle, Transient};
use martensite_tkernel::{impl_transient, impl_transient_interface};

/// `ShapeProcessOperFunc`: the function of a user operator; true when it changed the
/// shape of the context. A exception is the `Err` value.
pub type ShapeProcessOperFunc =
    fn(&Handle<dyn ShapeProcessContextSource>, &ProgressRange<'_>) -> Result<bool, Failure>;

/// `ShapeProcessOperator`: one named step of a ShapeProcess sequence.
///
/// ```
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkshhealing::shape_process::{ShapeProcessContext, ShapeProcessOperator, ShapeProcessUOperator};
/// let op = ShapeProcessUOperator::new(|_, _| Ok(true));
/// let context = Handle::new(ShapeProcessContext::new()).into_upcast();
/// assert!(op.perform(&context, &Default::default()).unwrap());
/// ```
pub trait ShapeProcessOperator: Transient {
    /// runs the operator on the context.
    fn perform(
        &self,
        context: &Handle<dyn ShapeProcessContextSource>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure>;
}
impl_transient_interface!(
    dyn ShapeProcessOperator,
    "ShapeProcess_Operator",
    dyn Transient
);

/// `ShapeProcessUOperator`: an operator that calls its function.
pub struct ShapeProcessUOperator {
    function: ShapeProcessOperFunc,
}
impl_transient!(
    ShapeProcessUOperator,
    "ShapeProcess_UOperator",
    dyn ShapeProcessOperator,
    interfaces: [dyn ShapeProcessOperator]
);

impl ShapeProcessUOperator {
    /// Wrap the processing callback; `ShapeProcessUOperator::new(my_operator)`.
    pub fn new(function: ShapeProcessOperFunc) -> Self {
        Self { function }
    }
}

impl ShapeProcessOperator for ShapeProcessUOperator {
    /// `Perform`: the result of the function.
    fn perform(
        &self,
        context: &Handle<dyn ShapeProcessContextSource>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        (self.function)(context, progress)
    }
}
