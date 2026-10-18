//! The foundational hashers: points, directions, vectors and `Axis2` placements.
//! Hashes round each coordinate to `HashTolerance`; equality compares each
//! coordinate within `CompTolerance`.
use martensite_tkernel::ncollection::HashPolicy;
use martensite_tkg2d::geom2d_hash::hash_key::{combined_hash, leading_and_rounded, values_within};
use martensite_tkg2d::hasher_tolerances;
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Vector3, Xyz};

hasher_tolerances!(
    /// `GeomHashPointHasher`: hashes and compares `Point3` values;
    /// `GeomHashPointHasher::default().hash(Point3::new(1.0, 2.0, 3.0))`.
    GeomHashPointHasher
);
hasher_tolerances!(
    /// `GeomHashDirectionHasher`: hashes and compares `Direction3` values;
    /// `GeomHashDirectionHasher::default().hash(direction)`.
    GeomHashDirectionHasher
);
hasher_tolerances!(
    /// `GeomHashVectorHasher`: hashes and compares `Vector3` values;
    /// `GeomHashVectorHasher::default().hash(Vector3::new(1.0, 0.0, 0.0))`.
    GeomHashVectorHasher
);
hasher_tolerances!(
    /// `GeomHashAxisPlacement`: hashes and compares `Axis2` placements by location,
    /// main direction and X direction; `GeomHashAxisPlacement::default().hash(frame)`.
    GeomHashAxisPlacement
);

// The coordinate triple hash and comparison that points, directions and vectors share.
fn xyz_hash(xyz: Xyz, hash_tolerance: f64) -> usize {
    leading_and_rounded(&[], &[xyz.x(), xyz.y(), xyz.z()], hash_tolerance)
}
fn xyz_equal(first: Xyz, second: Xyz, comp_tolerance: f64) -> bool {
    let pairs = [
        (first.x(), second.x()),
        (first.y(), second.y()),
        (first.z(), second.z()),
    ];
    values_within(comp_tolerance, &pairs)
}

impl GeomHashPointHasher {
    /// The rounded-coordinate hash (`operator()(const Point3&)`); `hasher.hash(p)`.
    pub fn hash(&self, point: Point3) -> usize {
        xyz_hash(point.xyz(), self.hash_tolerance)
    }
    /// Coordinates within `comp_tolerance` (`operator()(const Point3&, const Point3&)`);
    /// `hasher.equal(p, q)`.
    pub fn equal(&self, first: Point3, second: Point3) -> bool {
        xyz_equal(first.xyz(), second.xyz(), self.comp_tolerance)
    }
}
impl GeomHashDirectionHasher {
    /// The rounded-component hash (`operator()(const Direction3&)`); `hasher.hash(d)`.
    pub fn hash(&self, direction: Direction3) -> usize {
        xyz_hash(direction.xyz(), self.hash_tolerance)
    }
    /// Components within `comp_tolerance` (`operator()(const Direction3&, const Direction3&)`);
    /// `hasher.equal(d, e)`.
    pub fn equal(&self, first: Direction3, second: Direction3) -> bool {
        xyz_equal(first.xyz(), second.xyz(), self.comp_tolerance)
    }
}
impl GeomHashVectorHasher {
    /// The rounded-component hash (`operator()(const Vector3&)`); `hasher.hash(v)`.
    pub fn hash(&self, vector: Vector3) -> usize {
        xyz_hash(*vector.xyz(), self.hash_tolerance)
    }
    /// Components within `comp_tolerance` (`operator()(const Vector3&, const Vector3&)`);
    /// `hasher.equal(v, w)`.
    pub fn equal(&self, first: Vector3, second: Vector3) -> bool {
        xyz_equal(*first.xyz(), *second.xyz(), self.comp_tolerance)
    }
}
impl GeomHashAxisPlacement {
    fn parts(&self) -> (GeomHashPointHasher, GeomHashDirectionHasher) {
        (
            GeomHashPointHasher::new(self.comp_tolerance, self.hash_tolerance),
            GeomHashDirectionHasher::new(self.comp_tolerance, self.hash_tolerance),
        )
    }
    /// Location, main and X direction hashes combined (`operator()(const Axis2&)`);
    /// the Y direction is not hashed; `hasher.hash(frame)`.
    pub fn hash(&self, placement: Axis2) -> usize {
        let (points, directions) = self.parts();
        combined_hash(&[
            points.hash(placement.location()),
            directions.hash(placement.direction()),
            directions.hash(placement.x_direction()),
        ])
    }
    /// Location, main and X directions equal (`operator()(const Axis2&, const Axis2&)`);
    /// `hasher.equal(a, b)`.
    pub fn equal(&self, first: Axis2, second: Axis2) -> bool {
        let (points, directions) = self.parts();
        points.equal(first.location(), second.location())
            && directions.equal(first.direction(), second.direction())
            && directions.equal(first.x_direction(), second.x_direction())
    }
}

impl HashPolicy<Point3> for GeomHashPointHasher {
    fn hash(&self, key: &Point3) -> usize {
        GeomHashPointHasher::hash(self, *key)
    }
    fn equal(&self, first: &Point3, second: &Point3) -> bool {
        GeomHashPointHasher::equal(self, *first, *second)
    }
}
impl HashPolicy<Direction3> for GeomHashDirectionHasher {
    fn hash(&self, key: &Direction3) -> usize {
        GeomHashDirectionHasher::hash(self, *key)
    }
    fn equal(&self, first: &Direction3, second: &Direction3) -> bool {
        GeomHashDirectionHasher::equal(self, *first, *second)
    }
}
impl HashPolicy<Vector3> for GeomHashVectorHasher {
    fn hash(&self, key: &Vector3) -> usize {
        GeomHashVectorHasher::hash(self, *key)
    }
    fn equal(&self, first: &Vector3, second: &Vector3) -> bool {
        GeomHashVectorHasher::equal(self, *first, *second)
    }
}
impl HashPolicy<Axis2> for GeomHashAxisPlacement {
    fn hash(&self, key: &Axis2) -> usize {
        GeomHashAxisPlacement::hash(self, *key)
    }
    fn equal(&self, first: &Axis2, second: &Axis2) -> bool {
        GeomHashAxisPlacement::equal(self, *first, *second)
    }
}
