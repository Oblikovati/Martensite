//! Shared explored-identity rank; no coordinate or geometric ordering determines identity.
use martensite_tkbrep::{topexp::Explorer, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// Return the distinct Explorer/IsSame rank, or None; `occurrence_rank(&parent, &selected)?`.
pub(crate) fn occurrence_rank(parent: &Shape, selected: &Shape) -> Result<Option<usize>, Failure> {
    let mut seen = IndexedMapOfShape::default();
    for shape in Explorer::new(parent, selected.shape_type()?, ShapeEnum::Shape)? {
        if seen.contains(&shape) {
            continue;
        }
        let rank = seen.add(shape.clone());
        if shape.is_same(selected) {
            return Ok(Some(rank));
        }
    }
    Ok(None)
}
