//! Adjacent-face extension retains the original face/history association.
use super::{ExtendedFaces, FillGap};
use crate::bop_algo::ProgressCancellation;
use martensite_tkbrep::toptools::{IndexedMapOfShape, ShapeMapHasher};
use martensite_tkbrep::{brep_tools::History, topods};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::{ncollection::IndexedDataMap, standard::Failure};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tktopalgo::{brep_bnd_lib::BRepBndLib, brep_lib::extend_face};

impl FillGap {
    /// Each adjacent face extended in all directions by the diagonal of the feature box,
    /// recorded as modified into its extension (`ExtendAdjacentFaces`).
    #[cfg(test)]
    pub(super) fn extend_adjacent_faces(
        &self,
        adjacent: &IndexedMapOfShape,
        history: &History,
    ) -> Result<ExtendedFaces, Failure> {
        self.extend_adjacent_faces_on_progress(
            adjacent,
            history,
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        )
    }

    /// Extend actual adjacent faces in caller scope; cancellation retains partial raw history.
    /// Example: `gap.extend_adjacent_faces_on_progress(adjacent, history, range, observed)?` needs parent completion checks.
    pub(super) fn extend_adjacent_faces_on_progress(
        &self,
        adjacent: &IndexedMapOfShape,
        history: &History,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<ExtendedFaces, Failure> {
        let length = self.adjacent_extension_length()?;
        let scope = ProgressScope::with_literal(
            range,
            "Extending adjacent faces",
            adjacent.extent() as f64,
            false,
        );
        extend_in_scope(adjacent, history, length, &scope, observed)
    }

    fn adjacent_extension_length(&self) -> Result<f64, Failure> {
        let mut bounds = BoundingBox3::default();
        BRepBndLib::add(&self.feature, &mut bounds, true)?;
        Ok(bounds.square_extent().sqrt())
    }
}

#[cfg(test)]
#[path = "extension_tests.rs"]
mod tests;

fn extend_in_scope(
    adjacent: &IndexedMapOfShape,
    history: &History,
    length: f64,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<ExtendedFaces, Failure> {
    let mut extended = IndexedDataMap::with_hasher(ShapeMapHasher, 1);
    for face in adjacent.iter() {
        if !scope.more() {
            observed.record_cancelled();
            break;
        }
        bind_extended_face(face, length, history, &mut extended)?;
        drop(scope.next(1.));
    }
    Ok(extended)
}
fn bind_extended_face(
    face: &martensite_tkbrep::topods::Shape,
    length: f64,
    history: &History,
    extended: &mut ExtendedFaces,
) -> Result<(), Failure> {
    let extension = extend_face(&topods::face(face)?, length, [true; 4])?.into_shape();
    extended.add(face.clone(), extension.clone());
    history.add_modified(face, &extension)
}
