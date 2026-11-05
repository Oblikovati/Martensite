//! Joined final diagnostics use exact source-handle incidence rather than inferred coordinate identity.
use super::fixture::WholeCase;
use super::*;
use crate::bop_algo::AlertUserBreak;
use martensite_tkbrep::{
    topexp,
    toptools::{IndexedMapOfShape, ShapeMapHasher},
};
use martensite_tkernel::standard::RuntimeTyped;
use martensite_tkg3d::top_abs::ShapeEnum;
/// Observe final raw state after execution; `whole_snapshot(&case, observed, queries)`.
pub(super) fn whole_snapshot(case: &WholeCase, observed: bool, queries: usize) -> String {
    builder_snapshot(case.receiver.builder(), &case.sources, observed, queries)
}
/// Snapshot actual receiver state using retained construction handles; `builder_snapshot(b, sources, stopped, queries)`.
pub(crate) fn builder_snapshot(
    builder: &BOPAlgoBuilder,
    sources: &[Shape],
    observed: bool,
    queries: usize,
) -> String {
    let structure = builder
        .pave_filler()
        .and_then(BOPAlgoPaveFiller::data_structure);
    builder_snapshot_with_structure(builder, sources, structure, observed, queries)
}
/// Observe borrowed data without claiming filler ownership; `builder_snapshot_with_structure(b, sources, ds, observed, queries)`.
pub(crate) fn builder_snapshot_with_structure(
    builder: &BOPAlgoBuilder,
    sources: &[Shape],
    structure: Option<&crate::bop_ds::DataStructure>,
    observed: bool,
    queries: usize,
) -> String {
    let core = whole_core(builder, structure, observed, queries);
    let counts = result_counts(builder.shape());
    let mut all = IndexedMapOfShape::with_hasher(ShapeMapHasher, 1);
    if !builder.shape().is_null() {
        topexp::map_shapes(builder.shape(), &mut all, true, true);
    }
    let rows: Vec<_> = sources
        .iter()
        .map(|source| source_row(builder, source, &all, sources))
        .collect();
    format!("{core};{counts};[{}]", rows.join(","))
}
fn whole_core(
    builder: &BOPAlgoBuilder,
    structure: Option<&crate::bop_ds::DataStructure>,
    observed: bool,
    queries: usize,
) -> String {
    let filler = builder.pave_filler();
    let flags = format!(
        "{},{},{}",
        builder.options().has_errors(),
        builder.options().has_warnings(),
        builder.options().has_error(AlertUserBreak::static_type())
    );
    let source = format!(
        "{},{},{},{}",
        filler.is_some(),
        structure.is_some(),
        structure.map_or(-1, |s| s.nb_shapes()),
        structure.map_or(-1, |s| s.nb_source_shapes())
    );
    let result = format!(
        "{},{}",
        builder.shape().is_null(),
        if builder.shape().is_null() {
            -1
        } else {
            builder.shape().shape_type().unwrap() as i32
        }
    );
    format!(
        "[{flags},{source},{result},{},{},{},{},{},{queries},{observed}]",
        builder.has_history(),
        builder.has_filled_history(),
        builder.has_modified(),
        builder.has_generated(),
        builder.has_deleted()
    )
}
/// Observe typed topology without deriving identity; `result_counts(shape)` returns eight counts.
pub(crate) fn result_counts(shape: &Shape) -> String {
    let kinds = [
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        ShapeEnum::Wire,
        ShapeEnum::Face,
        ShapeEnum::Shell,
        ShapeEnum::Solid,
        ShapeEnum::CompSolid,
        ShapeEnum::Compound,
    ];
    let counts: Vec<_> = kinds
        .into_iter()
        .map(|kind| {
            let mut mapped = IndexedMapOfShape::with_hasher(ShapeMapHasher, 1);
            if !shape.is_null() {
                topexp::map_shapes_of_type(shape, kind, &mut mapped)
                    .expect("result topology: expected readable typed incidence map");
            }
            mapped.len().to_string()
        })
        .collect();
    format!("[{}]", counts.join(","))
}
fn source_row(
    builder: &BOPAlgoBuilder,
    source: &Shape,
    all: &IndexedMapOfShape,
    sources: &[Shape],
) -> String {
    let images = builder.images().seek(source);
    let rows: Vec<_> = images
        .into_iter()
        .flat_map(|images| images.iter())
        .map(|image| image_row(builder, image, source, sources))
        .collect();
    format!(
        "[{},{},{},{},{},[{}]]",
        images.is_some(),
        builder.modified(source).unwrap().len(),
        builder.generated(source).unwrap().len(),
        builder.is_deleted(source).unwrap(),
        all.contains(source),
        rows.join(",")
    )
}
fn image_row(builder: &BOPAlgoBuilder, image: &Shape, source: &Shape, sources: &[Shape]) -> String {
    let origins = builder.origins().seek(image);
    let mut parents = Vec::new();
    if let Some(origins) = origins {
        for (index, source) in sources.iter().enumerate() {
            for origin in origins.iter() {
                if origin.is_same(source) {
                    parents.push(index.to_string());
                }
            }
        }
    }
    format!(
        "[{},{},{},[{}]]",
        image.shape_type().unwrap() as i32,
        image.is_same(source),
        origins.map_or(0, |o| o.len()),
        parents.join(",")
    )
}
