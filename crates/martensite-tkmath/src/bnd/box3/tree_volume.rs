//! `BoundingBox3` as the bounding volume of `UBTree<int, BoundingBox3>`.
use crate::bnd::BoundingBox3;
use martensite_tkernel::ncollection::BoundingVolume;

/// The members that `UBTree` calls on its bound type: `Add`, `IsOut(BoundingBox3)`
/// and `SquareExtent`. It lives here, beside `BoundingBox3`, so that every toolkit
/// instantiates the tree on the same type (the solid classifier explorer, for
/// one).
///
/// ```
/// use martensite_tkernel::ncollection::BoundingVolume;
/// use martensite_tkmath::bnd::BoundingBox3;
/// use martensite_tkmath::gp::Point3;
/// let mut unit = BoundingBox3::new(Point3::new(0., 0., 0.), Point3::new(1., 1., 1.));
/// let far = BoundingBox3::new(Point3::new(5., 5., 5.), Point3::new(6., 6., 6.));
/// assert!(BoundingVolume::is_out(&unit, &far));
/// BoundingVolume::add(&mut unit, &far);
/// assert_eq!(BoundingVolume::square_extent(&unit), 108.0);
/// ```
impl BoundingVolume for BoundingBox3 {
    fn add(&mut self, other: &Self) {
        self.add_box(other);
    }
    fn is_out(&self, other: &Self) -> bool {
        self.is_out_box(other)
    }
    fn square_extent(&self) -> f64 {
        BoundingBox3::square_extent(self)
    }
}
