// SPDX-License-Identifier: GPL-2.0-only
//! General Fuse solid assembly for fillets and chamfers (#44).
//!
//! Splits a source solid with blend faces through `BRepAlgoAPIBuilderAlgo` (General Fuse)
//! without invoking `TopOpeBRepBuild`, selects the kept solid cell(s), translates the
//! General Fuse history into [`HistoryGraph`], and seals the result as [`OperationResult`].
use martensite_tkbo::brep_algo_api::BRepAlgoAPIBuilderAlgo;
use martensite_tkbrep::operation::{HistoryGraph, OperationError, OperationKind, OperationResult};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// Assembles a blended solid by splitting `source` with `blend_faces` via General Fuse
/// (`BRepAlgoAPIBuilderAlgo`), keeping the primary material solid cell, and returning a
/// sealed [`OperationResult`] with complete [`HistoryGraph`] (#44).
pub struct GeneralFuseBlendAssembler;

impl GeneralFuseBlendAssembler {
    /// Splits `source` with `blend_faces` using General Fuse and seals the kept solid (#44).
    pub fn assemble_solid(
        operation: OperationKind,
        source: &Shape,
        blend_faces: &[Shape],
    ) -> Result<OperationResult, OperationError> {
        if source.is_null() {
            return Err(OperationError::bad_input(
                operation,
                "blend source shape is null",
            ));
        }
        if blend_faces.is_empty() || blend_faces.iter().any(Shape::is_null) {
            return Err(
                OperationError::bad_input(operation, "blend faces must be non-empty and non-null")
                    .with_entity(source.clone()),
            );
        }
        let mut args = List::default();
        args.append(source.clone());
        for face in blend_faces {
            args.append(face.clone());
        }
        let mut gf = BRepAlgoAPIBuilderAlgo::new();
        gf.set_non_destructive(true);
        gf.set_to_fill_history(true);
        gf.set_arguments(&args);
        gf.build().map_err(|err| {
            OperationError::kernel_failure(operation, err.to_string())
                .with_entity(source.clone())
                .with_cause(err)
        })?;
        if gf.has_errors() {
            return Err(OperationError::kernel_failure(
                operation,
                "General Fuse blend assembly reported errors",
            )
            .with_entity(source.clone()));
        }
        let split_compound = gf.shape().clone();
        let kept_solid = select_kept_solid(operation, source, &split_compound)?;
        let history_inputs: Vec<Shape> = std::iter::once(source.clone())
            .chain(blend_faces.iter().cloned())
            .collect();
        let history = if let Some(h) = gf.history().and_then(|handle| handle.get()) {
            HistoryGraph::from_brep_history(&history_inputs, h).map_err(|err| {
                OperationError::kernel_failure(operation, err.to_string()).with_cause(err)
            })?
        } else {
            HistoryGraph::empty()
        };
        OperationResult::seal(operation, kept_solid, history)
    }
}

fn select_kept_solid(
    operation: OperationKind,
    source: &Shape,
    split_compound: &Shape,
) -> Result<Shape, OperationError> {
    let mut explorer = Explorer::new(split_compound, ShapeEnum::Solid, ShapeEnum::Shape).map_err(
        |err| {
            OperationError::kernel_failure(operation, err.to_string())
                .with_entity(source.clone())
                .with_cause(err)
        },
    )?;
    let mut best: Option<(f64, Shape)> = None;
    while explorer.more() {
        let candidate = explorer.current().clone();
        let mut props = GProps::default();
        if BRepGProp::volume_properties(&candidate, &mut props, true, false, false).is_ok() {
            let volume = props.mass().abs();
            if best.as_ref().is_none_or(|(v, _)| volume > *v) {
                best = Some((volume, candidate));
            }
        }
        explorer.next().map_err(|err| {
            OperationError::kernel_failure(operation, err.to_string()).with_cause(err)
        })?;
    }
    best.map(|(_, solid)| solid).ok_or_else(|| {
        OperationError::kernel_failure(
            operation,
            "General Fuse blend assembly produced no valid solid cell",
        )
        .with_entity(source.clone())
    })
}
