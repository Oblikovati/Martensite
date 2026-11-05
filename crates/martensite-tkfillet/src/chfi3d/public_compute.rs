//! An explicit dynamic receiver for compiler inventories that omit inherent dyn impls.
use super::ChFi3dBuilderOps;
use martensite_tkernel::standard::Failure;

/// Compute through the existing dynamic builder contract; `compute_builder(&mut chamfer)?`.
/// The forwarding function preserves the receiver explicitly because pinned rustdoc JSON omits dyn impls.
pub fn compute_builder(operation: &mut dyn ChFi3dBuilderOps) -> Result<(), Failure> {
    operation.compute()?;
    Ok(())
}
