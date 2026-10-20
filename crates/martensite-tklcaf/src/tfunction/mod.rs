//! The TFunction package: the function mechanism of the data framework (functions, their drivers, the
//! dependency graph, the logbook of modified labels and the execution iterator).
mod driver;
mod function;
mod graph_node;
mod ifunction;
mod iterator;
mod logbook;
mod scope;
pub use driver::{
    TFunctionDriver, TFunctionDriverBase, TFunctionDriverTable, TFunctionExecutionStatus,
};
pub use function::TFunctionFunction;
pub use graph_node::TFunctionGraphNode;
pub use ifunction::TFunctionIFunction;
pub use iterator::TFunctionIterator;
pub use logbook::TFunctionLogbook;
pub use scope::{TFunctionFunctionMap, TFunctionScope};
