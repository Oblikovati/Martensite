//! The AdvApp2Var package: approximation of a function of two parameters by
//! polynomial patches and B-spline surfaces.
//!
//! The package has two layers. The C++ classes drive the cutting of the domain
//! into patches and iso-curves; the numerical work is f2c translations of
//! Fortran routines in [`math_base`], [`approx_f2var`] and [`sys_base`], over the
//! common-block tables of [`data`].
//!
//! The kernels keep the retained routine names, argument order and loop structure,
//! so every floating-point operation happens in the retained order. Scalars that the
//! retained code only reads are passed by value, scalars it writes are `&mut`, and an
//! array argument is the slice that starts at the retained pointer; inside, the f2c
//! pointer shift is kept by the views of the `f2c` module. The calls to the trace
//! routines (`mgenmsg_`, `mgsomsg_`, `maermsg_`) are left out of the kernels: the
//! debug level [`sys_base::mnfndeb_`] is 0 and the routines do nothing.
mod approx_a_func2var;
mod approx_a_func2var_perform;
mod approx_a_func2var_results;
mod arrays;
mod caller_limits;
mod caller_orders;
mod caller_sampling;
pub use caller_limits::AdvApp2VarCallerLimitField;
pub use caller_sampling::AdvApp2VarCallerSamplingField;
mod combined_provider;
mod context;
mod criterion;
mod evaluator;
pub use caller_orders::{AdvApp2VarCallerOrderField, AdvApp2VarIsoCallerIntegers};
mod evaluator_slots;
mod integer_arguments;
mod integer_frame;
mod operation_frame_lease;
pub use combined_provider::{
    AdvApp2VarCombinedBufferEvaluation, AdvApp2VarCombinedBufferEvaluator,
    AdvApp2VarCombinedEvaluation, AdvApp2VarCombinedEvaluator, AdvApp2VarIsoIntegerOutput,
    AdvApp2VarIsoOperationFrame, AdvApp2VarOwnedIsoEvaluator,
};
mod real_provider;
mod real_slots;
mod scalar_incidence;
mod scalar_provider;
mod square_frame;
mod square_intervals;
mod square_roots;
pub use square_intervals::AdvApp2VarSquareIntervalAxis;
mod square_tables;
pub use square_tables::AdvApp2VarSquareTable;
mod square_sample_frame;
pub use square_roots::AdvApp2VarSquareRootAxis;
pub use square_sample_frame::AdvApp2VarSquareSampleFrame;
mod square_provider;
pub use evaluator_slots::{AdvApp2VarDerivativeOrders, AdvApp2VarEvaluationSlots};
pub use real_provider::{AdvApp2VarRealEvaluation, AdvApp2VarRealEvaluator};
pub use real_slots::{
    AdvApp2VarCallerCurveSlots, AdvApp2VarCallerRealArray, AdvApp2VarEndpointOutput,
    AdvApp2VarErrorOutput, AdvApp2VarFoldedOutput, AdvApp2VarIsoRealFrame, AdvApp2VarRealBindings,
    AdvApp2VarRealField,
};
pub use scalar_provider::{
    AdvApp2VarIntegerBindings, AdvApp2VarIntegerField, AdvApp2VarScalarEvaluation,
    AdvApp2VarScalarEvaluator, AdvApp2VarScalarProvider,
};
pub use square_frame::{
    AdvApp2VarSquareIntegerBindings, AdvApp2VarSquareIntegerField, AdvApp2VarSquareOperationFrame,
};
pub use square_provider::{
    AdvApp2VarSquareBufferEvaluation, AdvApp2VarSquareBufferEvaluator, AdvApp2VarSquareEvaluation,
    AdvApp2VarSquareEvaluator, AdvApp2VarSquareStatusEvaluator,
};
mod f2c;
mod framework;
mod framework_update;
mod iso;
mod iso_approx;
mod network;
mod node;
mod patch;
mod patch_constraints;
mod patch_discretise;
mod patch_make;

pub mod approx_f2var;
pub mod data;
pub mod math_base;
pub mod sys_base;

pub use approx_a_func2var::AdvApp2VarApproxAFunc2Var;
pub use context::{AdvApp2VarContext, FrontierTolerances, SubspaceTolerances};
pub use criterion::{AdvApp2VarCriterion, AdvApp2VarCriterionRepartition, AdvApp2VarCriterionType};
pub use evaluator::{AdvApp2VarEvaluation, AdvApp2VarEvaluationMut, AdvApp2VarEvaluatorFunc2Var};
pub use framework::{AdvApp2VarFramework, AdvApp2VarIsoStrip};
pub use iso::AdvApp2VarIso;
pub use network::AdvApp2VarNetwork;
pub use node::AdvApp2VarNode;
pub use patch::AdvApp2VarPatch;

mod evaluator_buffers;
pub use evaluator_buffers::{AdvApp2VarBufferEvaluation, AdvApp2VarEvaluationBuffers};

#[cfg(test)]
mod slice_callback_fixture;
