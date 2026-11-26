// SPDX-License-Identifier: GPL-2.0-only
//! Explicit non-destructive shape repair on a detached copy with history remap (#59).
use crate::shape_fix::{FixRoot, ShapeFixShape};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools::ReShape;
use martensite_tkbrep::operation::{HistoryGraph, OperationError, OperationKind};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

/// Options controlling explicit repair on a detached copy (#59).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RepairOnCopyOptions {
    /// Target working precision.
    pub precision: f64,
    /// Upper cap on tolerances during repair.
    pub max_tolerance: f64,
}

impl Default for RepairOnCopyOptions {
    fn default() -> Self {
        Self {
            precision: 1.0e-6,
            max_tolerance: 1.0e-3,
        }
    }
}

/// Output of [`repair_on_copy`]: detached repaired shape, history remap from source
/// entities to repaired entities, achieved max tolerance, and validity status (#59).
#[derive(Clone, Debug)]
pub struct RepairOnCopyResult {
    repaired_shape: Shape,
    history: HistoryGraph,
    achieved_max_tolerance: f64,
    is_valid: bool,
}

impl RepairOnCopyResult {
    /// Detached repaired shape.
    pub fn repaired_shape(&self) -> &Shape {
        &self.repaired_shape
    }
    /// Remap from original source entities to repaired entities.
    pub fn history(&self) -> &HistoryGraph {
        &self.history
    }
    /// Largest tolerance observed on vertices, edges, or faces of the repaired shape.
    pub fn achieved_max_tolerance(&self) -> f64 {
        self.achieved_max_tolerance
    }
    /// Whether `BRepCheckAnalyzer` certified the repaired shape valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

/// Runs explicit shape healing (`ShapeFixShape`) on a detached deep copy of `source`,
/// leaving `source` untouched and returning a history remap from original to repaired
/// entities (#59).
pub fn repair_on_copy(
    source: &Shape,
    options: RepairOnCopyOptions,
) -> Result<RepairOnCopyResult, OperationError> {
    if source.is_null() {
        return Err(OperationError::kernel_failure(
            OperationKind::Custom("RepairOnCopy"),
            "cannot repair a null shape",
        ));
    }
    if !(options.precision.is_finite()
        && options.precision > 0.0
        && options.max_tolerance.is_finite()
        && options.max_tolerance >= options.precision)
    {
        return Err(OperationError::kernel_failure(
            OperationKind::Custom("RepairOnCopy"),
            "invalid repair precision or max_tolerance",
        ));
    }

    let op = OperationKind::Custom("RepairOnCopy");
    let mut copy = BRepBuilderAPICopy::with_shape(source, true, true)
        .map_err(|f| OperationError::kernel_failure(op, f.message().to_string()))?;
    let detached = copy
        .shape()
        .map_err(|f| OperationError::kernel_failure(op, f.message().to_string()))?
        .clone();

    let fixer = ShapeFixShape::new(detached);
    fixer.set_precision(options.precision);
    fixer.set_max_tolerance(options.max_tolerance);
    let _ = fixer
        .perform()
        .map_err(|f| OperationError::kernel_failure(op, f.message().to_string()))?;
    let repaired_shape = fixer.shape();

    // Build history remap from source entities -> copy -> fixer context -> repaired entities.
    let mut builder = HistoryGraph::builder();
    let mut src_entities = IndexedMapOfShape::default();
    topexp::map_shapes(source, &mut src_entities, true, true);
    let context = fixer.context();
    for idx in 1..=src_entities.extent() {
        let orig = src_entities
            .find_key(idx)
            .map_err(|f| OperationError::kernel_failure(op, f.message().to_string()))?
            .clone();
        if let Ok(copied) = copy.modified_shape(&orig) {
            let mapped = if let Some(ctx) = context.get() {
                ctx.apply(&copied, ShapeEnum::Shape).unwrap_or(copied)
            } else {
                copied
            };
            if mapped.is_null() {
                builder.record_deleted(orig);
            } else {
                builder.record_derived(vec![orig], vec![mapped], false);
            }
        }
    }

    let achieved_max_tolerance = max_entity_tolerance(&repaired_shape)
        .map_err(|f| OperationError::kernel_failure(op, f.message().to_string()))?;
    let analyzer = BRepCheckAnalyzer::new(&repaired_shape, true, false, false)
        .map_err(|f| OperationError::kernel_failure(op, f.message().to_string()))?;
    let is_valid = analyzer
        .is_valid()
        .map_err(|f| OperationError::kernel_failure(op, f.message().to_string()))?;

    Ok(RepairOnCopyResult {
        repaired_shape,
        history: builder.build(),
        achieved_max_tolerance,
        is_valid,
    })
}

fn max_entity_tolerance(shape: &Shape) -> Result<f64, martensite_tkernel::standard::Failure> {
    let v = BRepTool::max_tolerance(shape, ShapeEnum::Vertex)?;
    let e = BRepTool::max_tolerance(shape, ShapeEnum::Edge)?;
    let f = BRepTool::max_tolerance(shape, ShapeEnum::Face)?;
    Ok(v.max(e).max(f))
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;

    #[test]
    fn repair_on_copy_leaves_source_untouched_and_remaps_history() {
        let source = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0)
            .expect("box")
            .shape()
            .expect("shape")
            .clone();
        let result =
            repair_on_copy(&source, RepairOnCopyOptions::default()).expect("repair on copy");
        assert!(result.is_valid());
        assert!(!result.repaired_shape().is_same(&source));
        assert!(result.achieved_max_tolerance() <= 1e-3);
        assert!(!result.history().is_empty());
    }
}
