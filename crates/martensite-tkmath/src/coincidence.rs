// SPDX-License-Identifier: GPL-2.0-only
//! Transitive coincidence classes with bounded diameter (#34).
//!
//! Pairwise epsilon tests are not transitive (`a ≈ b` and `b ≈ c` does not imply `a ≈ c`).
//! [`CoincidenceClassSet`] maintains union-find equivalence classes whose cluster diameter
//! is strictly capped by `max_diameter`, deciding each incidence once for reuse everywhere.
use crate::gp::Point3;

/// Canonical identifier of a coincidence equivalence class (#34).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoincidenceClassId(usize);

impl CoincidenceClassId {
    /// Underlying canonical root index.
    pub fn raw(self) -> usize {
        self.0
    }
}

/// Error returned when merging two coincidence classes would exceed `max_diameter` (#34).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoincidenceMergeError {
    /// First class identifier.
    pub class_a: CoincidenceClassId,
    /// Second class identifier.
    pub class_b: CoincidenceClassId,
    /// Resulting cluster diameter if merged.
    pub diameter: f64,
    /// Configured maximum allowed cluster diameter.
    pub max_diameter: f64,
}

/// Transitive equivalence-class set with bounded geometric diameter (#34).
#[derive(Clone, Debug)]
pub struct CoincidenceClassSet {
    max_diameter: f64,
    points: Vec<Point3>,
    parent: Vec<usize>,
    members: Vec<Vec<usize>>,
}

impl CoincidenceClassSet {
    /// Creates an empty set enforcing `max_diameter` across every equivalence class.
    pub fn new(max_diameter: f64) -> Self {
        assert!(
            max_diameter.is_finite() && max_diameter >= 0.0,
            "max_diameter {max_diameter}: expected finite non-negative bound"
        );
        Self {
            max_diameter,
            points: Vec::new(),
            parent: Vec::new(),
            members: Vec::new(),
        }
    }

    /// Configured maximum allowed diameter for any coincidence class.
    pub fn max_diameter(&self) -> f64 {
        self.max_diameter
    }

    /// Registers a point in its own singleton coincidence class and returns its entity index.
    pub fn add_point(&mut self, point: Point3) -> usize {
        let id = self.points.len();
        self.points.push(point);
        self.parent.push(id);
        self.members.push(vec![id]);
        id
    }

    /// Canonical coincidence class identifier for `entity`.
    pub fn class_of(&mut self, entity: usize) -> CoincidenceClassId {
        let root = self.find_root(entity);
        CoincidenceClassId(root)
    }

    /// True if `a` and `b` belong to the same coincidence class (decided once and reused).
    pub fn are_coincident(&mut self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    /// Attempts to merge the classes of `a` and `b`, refusing if the combined cluster
    /// diameter would exceed `max_diameter` (#34).
    pub fn try_merge(
        &mut self,
        a: usize,
        b: usize,
    ) -> Result<CoincidenceClassId, CoincidenceMergeError> {
        let ra = self.find_root(a);
        let rb = self.find_root(b);
        if ra == rb {
            return Ok(CoincidenceClassId(ra));
        }
        let mut diameter = 0.0_f64;
        for &ia in &self.members[ra] {
            for &ib in &self.members[rb] {
                let d = self.points[ia].distance(&self.points[ib]);
                if d > diameter {
                    diameter = d;
                }
            }
        }
        if diameter > self.max_diameter {
            return Err(CoincidenceMergeError {
                class_a: CoincidenceClassId(ra),
                class_b: CoincidenceClassId(rb),
                diameter,
                max_diameter: self.max_diameter,
            });
        }
        let (keep, drop) = if ra < rb { (ra, rb) } else { (rb, ra) };
        self.parent[drop] = keep;
        let moved = std::mem::take(&mut self.members[drop]);
        self.members[keep].extend(moved);
        Ok(CoincidenceClassId(keep))
    }

    fn find_root(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coincidence_classes_enforce_transitivity_and_diameter_cap() {
        let mut set = CoincidenceClassSet::new(1.0e-5);
        let a = set.add_point(Point3::new(0.0, 0.0, 0.0));
        let b = set.add_point(Point3::new(6.0e-6, 0.0, 0.0));
        let c = set.add_point(Point3::new(1.2e-5, 0.0, 0.0));
        let d = set.add_point(Point3::new(9.0e-6, 0.0, 0.0));

        let class_ab = set.try_merge(a, b).expect("a and b within 1e-5");
        assert!(set.are_coincident(a, b));
        assert_eq!(set.try_merge(b, d).expect("d keeps diameter <= 1e-5"), class_ab);
        assert!(set.are_coincident(a, d));

        // Merging c with b would make dist(a, c) = 1.2e-5 > 1e-5: refused!
        let err = set.try_merge(b, c).expect_err("must refuse diameter exceedance");
        assert!(err.diameter > err.max_diameter);
        assert!(!set.are_coincident(a, c));
    }
}
