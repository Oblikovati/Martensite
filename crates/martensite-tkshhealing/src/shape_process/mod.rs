//! ShapeProcess: the operators and contexts that run sequences of Shape
//! Healing operations.
mod context;
mod oper_library;
mod operator;
mod package;
mod shape_context;

pub use context::{ShapeProcessContext, ShapeProcessResource};
pub use oper_library::ShapeProcessOperLibrary;
pub use operator::{ShapeProcessOperFunc, ShapeProcessOperator, ShapeProcessUOperator};
pub use package::{ShapeProcess, ShapeProcessOperation, ShapeProcessOperationsFlags};
pub use shape_context::{ShapeProcessShapeContext, ShapeProcessShapeMap};
