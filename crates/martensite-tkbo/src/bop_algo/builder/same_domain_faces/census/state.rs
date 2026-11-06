//! Lookup constructed identities directly; never rank geometry or infer identity from map order.
use super::super::fixtures::SameDomainFixture;
use crate::bop_algo::builder::{SameDomainShapes, ShapeImages};
use crate::bop_algo::{AlertUserBreak, ProgressCancellation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::RuntimeTyped;
type NamedShapes = Vec<(String, Shape)>;

/// Snapshot every final relation and actual report; `snapshot(fixture, observed, polls)`.
pub(super) fn snapshot(
    fixture: &SameDomainFixture,
    observed: &ProgressCancellation,
    polls: usize,
) -> String {
    let builder = &fixture.builder;
    let names = constructed_labels(fixture);
    let header = status_fields(fixture, observed, polls);
    format!(
        "{header}|images={}|origins={}|sd={}",
        image_relations(builder.images(), &names),
        image_relations(builder.origins(), &names),
        representative_relations(builder.shapes_sd(), &names)
    )
}

fn status_fields(
    fixture: &SameDomainFixture,
    observed: &ProgressCancellation,
    polls: usize,
) -> String {
    let options = fixture.builder.options();
    format!(
        "errors={}|break={}|parallel={}|fuzzy={}|context={}|observed={}|queries={polls}",
        options.has_errors(),
        options.has_error(AlertUserBreak::static_type()),
        options.run_parallel(),
        options.fuzzy_value().to_bits(),
        !fixture.builder.context.is_null(),
        observed.is_cancelled()
    )
}

fn constructed_labels(fixture: &SameDomainFixture) -> NamedShapes {
    let mut labels: NamedShapes = fixture
        .sources
        .iter()
        .enumerate()
        .map(|(i, shape)| (format!("source{i}"), shape.clone().into_shape()))
        .collect();
    for (i, shape) in fixture.pieces.iter().enumerate() {
        if !labels.iter().any(|(_, known)| known.is_same(shape)) {
            labels.push((format!("piece{i}"), shape.clone().into_shape()));
        }
    }
    labels
}

fn shape_label<'a>(shape: &Shape, labels: &'a NamedShapes) -> &'a str {
    labels
        .iter()
        .find(|(_, known)| known.is_same(shape))
        .map(|(name, _)| name.as_str())
        .unwrap_or_else(|| {
            panic!(
                "SD shape {:?}; expected a constructed source/piece identity",
                shape.shape_type()
            )
        })
}

fn image_relations(map: &ShapeImages, labels: &NamedShapes) -> String {
    render_mapping(labels, map.len(), |key| {
        map.seek(key).map(|values| {
            let values: Vec<_> = values
                .iter()
                .map(|shape| shape_label(shape, labels))
                .collect();
            format!("[{}]", values.join(","))
        })
    })
}

fn representative_relations(map: &SameDomainShapes, labels: &NamedShapes) -> String {
    render_mapping(labels, map.len(), |key| {
        map.seek(key)
            .map(|shape| format!("[{}]", shape_label(shape, labels)))
    })
}

fn render_mapping(
    labels: &NamedShapes,
    count: usize,
    lookup: impl Fn(&Shape) -> Option<String>,
) -> String {
    let mut found = 0;
    let rows: Vec<_> = labels
        .iter()
        .map(|(name, shape)| {
            let value = lookup(shape);
            found += usize::from(value.is_some());
            format!("{name}:{}", value.unwrap_or_else(|| "missing".to_owned()))
        })
        .collect();
    assert_eq!(
        found, count,
        "SD map has {count} entries; expected only constructed identities"
    );
    rows.join(";")
}
