//! `IFGraph`: tools over the graph of a model (shared entities, comparisons, counts,
//! external sources, sub-parts and strong components); the IFGraph classes derived from
//! `InterfaceGraphContent` implement its `Evaluate`, those derived from
//! `IFGraphSubPartsIterator` implement [`IFGraphSubPartsEvaluate`].
mod contents;
mod strong_componants;
mod sub_parts;
#[cfg(test)]
mod tests;

pub use contents::{IFGraphAllShared, IFGraphCompare, IFGraphCumulate, IFGraphExternalSources};
pub use strong_componants::{IFGraphSCRoots, IFGraphStrongComponants};
pub use sub_parts::{IFGraphSubPartsEvaluate, IFGraphSubPartsIterator};

#[cfg(test)]
mod sub_parts_copy_tests;
