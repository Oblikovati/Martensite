//! `CurveHashKey`: the polymorphic hasher that dispatches on the exact
//! dynamic type of a `GeomCurve` to the typed hashers.
use super::conic_hashers::{
    GeomHashCircleHasher, GeomHashEllipseHasher, GeomHashHyperbolaHasher, GeomHashLineHasher,
    GeomHashParabolaHasher,
};
use super::spline_hashers::{GeomHashBSplineCurveHasher, GeomHashBezierCurveHasher};
use super::wrapped_hashers::{GeomHashOffsetCurveHasher, GeomHashTrimmedCurveHasher};
use crate::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomLine,
    GeomOffsetCurve, GeomParabola, GeomTrimmedCurve,
};
use martensite_tkernel::ncollection::HashPolicy;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d_hash::class_dispatch::{decided_equality, unknown_class_hash};
use martensite_tkg2d::{hasher_class_dispatch, hasher_tolerances};

hasher_tolerances!(
    /// `CurveHashKey`: hashes and compares any `GeomCurve` handle, for
    /// geometry deduplication in hashed containers; a null handle hashes to 0;
    /// `GeomHashCurveHasher::default().hash(&curve)`.
    GeomHashCurveHasher
);

impl GeomHashCurveHasher {
    /// The typed hash of the curve's exact class, 0 for a null handle, and the
    /// StandardCStringHasher hash of the type name for any other class, such as the
    /// GeomEval curves (`operator()(const occ::handle<GeomCurve>&)`);
    /// `hasher.hash(&curve)`.
    pub fn hash(&self, curve: &Handle<dyn GeomCurve>) -> usize {
        let (comp, tol) = (self.comp_tolerance, self.hash_tolerance);
        hasher_class_dispatch!(hash curve, comp, tol;
            GeomLine => GeomHashLineHasher,
            GeomCircle => GeomHashCircleHasher,
            GeomEllipse => GeomHashEllipseHasher,
            GeomHyperbola => GeomHashHyperbolaHasher,
            GeomParabola => GeomHashParabolaHasher,
            GeomBezierCurve => GeomHashBezierCurveHasher,
            GeomBSplineCurve => GeomHashBSplineCurveHasher,
            GeomTrimmedCurve => GeomHashTrimmedCurveHasher,
            GeomOffsetCurve => GeomHashOffsetCurveHasher);
        unknown_class_hash(curve)
    }

    /// Whether two curves are equal for deduplication (`operator()(handle, handle)`):
    /// both null, the same object, or the same exact class compared by its typed
    /// hasher; objects of other classes compare by identity; `hasher.equal(&a, &b)`.
    pub fn equal(&self, first: &Handle<dyn GeomCurve>, second: &Handle<dyn GeomCurve>) -> bool {
        if let Some(decided) = decided_equality(first, second) {
            return decided;
        }
        let (comp, tol) = (self.comp_tolerance, self.hash_tolerance);
        hasher_class_dispatch!(equal first, second, comp, tol;
            GeomLine => GeomHashLineHasher,
            GeomCircle => GeomHashCircleHasher,
            GeomEllipse => GeomHashEllipseHasher,
            GeomHyperbola => GeomHashHyperbolaHasher,
            GeomParabola => GeomHashParabolaHasher,
            GeomBezierCurve => GeomHashBezierCurveHasher,
            GeomBSplineCurve => GeomHashBSplineCurveHasher,
            GeomTrimmedCurve => GeomHashTrimmedCurveHasher,
            GeomOffsetCurve => GeomHashOffsetCurveHasher);
        // Other classes compare by object identity, already found different.
        false
    }
}

impl HashPolicy<Handle<dyn GeomCurve>> for GeomHashCurveHasher {
    fn hash(&self, key: &Handle<dyn GeomCurve>) -> usize {
        GeomHashCurveHasher::hash(self, key)
    }
    fn equal(&self, first: &Handle<dyn GeomCurve>, second: &Handle<dyn GeomCurve>) -> bool {
        GeomHashCurveHasher::equal(self, first, second)
    }
}
