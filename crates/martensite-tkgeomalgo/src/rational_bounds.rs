// SPDX-License-Identifier: GPL-2.0-only
//! Conservative convex-hull bounds and homogeneous subdivision isolation for
//! bounded rational Bézier surface patches (#37, #464).
use martensite_tkmath::gp::Point3;

/// Axis-aligned bounding box guaranteed to enclose a rational Bézier patch with
/// strictly positive weights (#37).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RationalPatchAabb {
    /// Minimum corner `[x, y, z]`.
    pub min: [f64; 3],
    /// Maximum corner `[x, y, z]`.
    pub max: [f64; 3],
}

impl RationalPatchAabb {
    /// True when `self` and `other` overlap after inflating by `margin`.
    pub fn overlaps(&self, other: &Self, margin: f64) -> bool {
        (0..3).all(|axis| {
            self.min[axis] - margin <= other.max[axis]
                && other.min[axis] - margin <= self.max[axis]
        })
    }
}

/// Bounded rational Bézier surface patch preserving exact non-constant weights (#37, #464).
#[derive(Clone, Debug, PartialEq)]
pub struct RationalBezierPatch {
    /// Control-point grid `[u_index][v_index]`.
    poles: Vec<Vec<Point3>>,
    /// Strictly positive weight grid `[u_index][v_index]`.
    weights: Vec<Vec<f64>>,
}

impl RationalBezierPatch {
    /// Builds a validated rational Bézier patch; rejects empty grids or non-positive weights.
    pub fn new(poles: Vec<Vec<Point3>>, weights: Vec<Vec<f64>>) -> Option<Self> {
        if poles.is_empty() || poles.len() != weights.len() {
            return None;
        }
        let v_len = poles[0].len();
        if v_len == 0 {
            return None;
        }
        for (p_row, w_row) in poles.iter().zip(weights.iter()) {
            if p_row.len() != v_len || w_row.len() != v_len {
                return None;
            }
            if w_row.iter().any(|&w| !(w.is_finite() && w > 0.0)) {
                return None;
            }
        }
        Some(Self { poles, weights })
    }

    /// True if weights vary across the patch (never demoted for near-unit weights, #464).
    pub fn is_rational(&self) -> bool {
        let first = self.weights[0][0];
        self.weights
            .iter()
            .flatten()
            .any(|&w| w.to_bits() != first.to_bits())
    }

    /// Exact conservative AABB via the convex-hull property of positive-weight rational Béziers (#37).
    pub fn conservative_aabb(&self) -> RationalPatchAabb {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        for p in self.poles.iter().flatten() {
            let coords = [p.x(), p.y(), p.z()];
            for axis in 0..3 {
                min[axis] = min[axis].min(coords[axis]);
                max[axis] = max[axis].max(coords[axis]);
            }
        }
        RationalPatchAabb { min, max }
    }

    /// Subdivides the patch at `u = 0.5` in homogeneous coordinates `(w*P, w)` (#37).
    pub fn subdivide_u(&self) -> (Self, Self) {
        let u_len = self.poles.len();
        let v_len = self.poles[0].len();
        let mut left_poles = vec![vec![Point3::new(0.0, 0.0, 0.0); v_len]; u_len];
        let mut right_poles = vec![vec![Point3::new(0.0, 0.0, 0.0); v_len]; u_len];
        let mut left_w = vec![vec![1.0; v_len]; u_len];
        let mut right_w = vec![vec![1.0; v_len]; u_len];

        for j in 0..v_len {
            let mut h: Vec<[f64; 4]> = (0..u_len)
                .map(|i| {
                    let p = self.poles[i][j];
                    let w = self.weights[i][j];
                    [p.x() * w, p.y() * w, p.z() * w, w]
                })
                .collect();
            for level in 0..u_len {
                let lw = h[0][3];
                left_poles[level][j] = Point3::new(h[0][0] / lw, h[0][1] / lw, h[0][2] / lw);
                left_w[level][j] = lw;
                let last = h.len() - 1;
                let rw = h[last][3];
                right_poles[u_len - 1 - level][j] =
                    Point3::new(h[last][0] / rw, h[last][1] / rw, h[last][2] / rw);
                right_w[u_len - 1 - level][j] = rw;
                if h.len() > 1 {
                    let mut next = Vec::with_capacity(h.len() - 1);
                    for k in 0..(h.len() - 1) {
                        next.push([
                            0.5 * (h[k][0] + h[k + 1][0]),
                            0.5 * (h[k][1] + h[k + 1][1]),
                            0.5 * (h[k][2] + h[k + 1][2]),
                            0.5 * (h[k][3] + h[k + 1][3]),
                        ]);
                    }
                    h = next;
                }
            }
        }
        (
            Self {
                poles: left_poles,
                weights: left_w,
            },
            Self {
                poles: right_poles,
                weights: right_w,
            },
        )
    }

    /// Recursive conservative intersection isolation up to `max_depth` subdivisions (#37).
    pub fn may_intersect(&self, other: &Self, margin: f64, max_depth: usize) -> bool {
        if !self
            .conservative_aabb()
            .overlaps(&other.conservative_aabb(), margin)
        {
            return false;
        }
        if max_depth == 0 {
            return true;
        }
        let (a0, a1) = self.subdivide_u();
        let (b0, b1) = other.subdivide_u();
        a0.may_intersect(&b0, margin, max_depth - 1)
            || a0.may_intersect(&b1, margin, max_depth - 1)
            || a1.may_intersect(&b0, margin, max_depth - 1)
            || a1.may_intersect(&b1, margin, max_depth - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_patch(z: f64, mid_weight: f64) -> RationalBezierPatch {
        let poles = vec![
            vec![Point3::new(0.0, 0.0, z), Point3::new(0.0, 1.0, z)],
            vec![Point3::new(0.5, 0.0, z), Point3::new(0.5, 1.0, z)],
            vec![Point3::new(1.0, 0.0, z), Point3::new(1.0, 1.0, z)],
        ];
        let weights = vec![
            vec![1.0, 1.0],
            vec![mid_weight, mid_weight],
            vec![1.0, 1.0],
        ];
        RationalBezierPatch::new(poles, weights).expect("valid patch")
    }

    #[test]
    fn conservative_bounds_certify_separation_and_preserve_near_unit_weights() {
        let lower = unit_patch(0.0, 1.0 + 1e-7);
        let upper = unit_patch(2.0, 1.0);
        let touching = unit_patch(0.0, 0.999_999);

        assert!(
            lower.is_rational(),
            "near-unit non-constant weight must not be demoted (#464)"
        );
        assert!(!lower.may_intersect(&upper, 1e-6, 3));
        assert!(lower.may_intersect(&touching, 1e-6, 3));
    }
}
