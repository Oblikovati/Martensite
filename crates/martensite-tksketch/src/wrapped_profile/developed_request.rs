//! Keep developed material inputs independent of the support geometry.
use super::*;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub(super) struct DevelopedRegionRequest {
    pub region: SurfaceRegionId,
    pub loops: Vec<Vec<AuthoredPiece>>,
    pub depths: [f64; 2],
    pub tolerance: f64,
    pub max_fit_level: u8,
}

impl Deref for Request {
    type Target = DevelopedRegionRequest;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.source
    }
}
impl DerefMut for Request {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}
impl AsRef<DevelopedRegionRequest> for Request {
    #[inline(always)]
    fn as_ref(&self) -> &DevelopedRegionRequest {
        &self.source
    }
}
impl AsRef<DevelopedRegionRequest> for DevelopedRegionRequest {
    #[inline(always)]
    fn as_ref(&self) -> &DevelopedRegionRequest {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn developed_material_certifies_without_cylinder_geometry() {
        let source = super::super::tests::request().source;
        let planar = planar::certify(&source).unwrap();
        assert!(planar.matches_request(&source));
        assert!(planar.area.encloses(3.525));
        let mut changed = source.clone();
        changed.loops[0][0].poles[0][0] += 0.1;
        assert!(!planar.matches_request(&changed));
        assert!(planar.matches_request(&source));
    }
}
