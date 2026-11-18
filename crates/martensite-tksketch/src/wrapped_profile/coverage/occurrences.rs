//! Actual frozen incidence remains distinct from canonical semantic binding wrappers (#540).
use super::*;
use martensite_tkg3d::top_abs::compose;
/// An operation-local immutable companion, intentionally absent from the legacy coverage report.
pub(in crate::surface_sketch::wrapped_profile) struct Occurrences {
    faces: Vec<FaceEvidence>,
}
impl Occurrences {
    pub(super) fn new(mut faces: Vec<FaceEvidence>) -> Checked<Self> {
        faces.sort_by_key(|face| face.owner);
        unique_faces(&faces)?;
        for face in &mut faces {
            order_uses(&mut face.edges)?;
        }
        Ok(Self { faces })
    }
    pub(in crate::surface_sketch::wrapped_profile) fn face_orientation(
        &self,
        face: BindingKey,
    ) -> Checked<Orientation> {
        Ok(self.face(face)?.orientation)
    }
    pub(in crate::surface_sketch::wrapped_profile) fn edge_orientation(
        &self,
        edge: BindingKey,
        face: BindingKey,
    ) -> Checked<Orientation> {
        let occurrence = self.face(face)?;
        let local = face_orientation(edge, &occurrence.edges)?;
        // Canonical Forward faces and wires supply local edge uses; shell orientation is composed once.
        Ok(compose(occurrence.orientation, local))
    }
    fn face(&self, face: BindingKey) -> Checked<&FaceEvidence> {
        self.faces
            .binary_search_by_key(&face, |occurrence| occurrence.owner)
            .map(|index| &self.faces[index])
            .map_err(|_| fail(face, CoverageCause::MissingBinding(face)))
    }
}
fn unique_faces(faces: &[FaceEvidence]) -> Checked<()> {
    for pair in faces.windows(2) {
        if pair[0].owner != pair[1].owner {
            continue;
        }
        let owner = pair[0].owner;
        let actual = faces.iter().filter(|face| face.owner == owner).count();
        return count(owner, "actual face occurrences", actual, 1);
    }
    Ok(())
}
pub(super) fn order_uses(uses: &mut [BoundUse]) -> Checked<()> {
    // Index only the already inspected actual uses; no second native graph walk is needed.
    uses.sort_by_key(|use_| use_.key);
    for pair in uses.windows(2) {
        if pair[0].key != pair[1].key {
            continue;
        }
        let owner = pair[0].key;
        let actual = uses.iter().filter(|use_| use_.key == owner).count();
        return count(owner, "actual oriented occurrences", actual, 1);
    }
    Ok(())
}
pub(super) fn face_orientation(face: BindingKey, uses: &[BoundUse]) -> Checked<Orientation> {
    uses.binary_search_by_key(&face, |use_| use_.key)
        .map(|index| uses[index].orientation)
        .map_err(|_| fail(face, CoverageCause::MissingBinding(face)))
}
