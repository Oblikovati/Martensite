//! Axis-aligned box/frustum SAT and native full-inside flag refinement.
use super::{SelectMgrFrustumCore, frustum_ranges::refine_frustum_inside};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::standard::{Failure, exceptions::program_error};
use martensite_tkmath::{
    bvh::{std_max, std_min},
    gp::Xyz,
};

/// The coordinate of a vector along an axis index.
fn axis_of(v: &Vec3<f64>, axis: usize) -> f64 {
    match axis {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    }
}
/// The projections of a box on a direction: the minimum and maximum corners chosen by sign.
/// An unordered pair fails with the direction; `has_box_overlap` builds the failure from it,
/// so the hot loop carries a small error (#594).
#[inline]
fn box_projection(dir: &Xyz, min: &Vec3<f64>, max: &Vec3<f64>) -> Result<(f64, f64), Xyz> {
    // The native comparison is against `0.f`.
    let pick = |d: f64, lo: f64, hi: f64| if d < 0.0 { hi } else { lo };
    let low = dir.x() * pick(dir.x(), min.x, max.x)
        + dir.y() * pick(dir.y(), min.y, max.y)
        + dir.z() * pick(dir.z(), min.z, max.z);
    let high = dir.x() * pick(dir.x(), max.x, min.x)
        + dir.y() * pick(dir.y(), max.y, min.y)
        + dir.z() * pick(dir.z(), max.z, min.z);
    // Unordered (NaN) or reversed projections; the picking loop keeps the message out of line.
    if high.partial_cmp(&low).is_none_or(std::cmp::Ordering::is_lt) {
        return Err(*dir);
    }
    Ok((low, high))
}
#[cold]
#[inline(never)]
fn unordered_projection(dir: &Xyz, min: &Vec3<f64>, max: &Vec3<f64>) -> Failure {
    program_error(&format!(
        "box corners {min:?}..{max:?}, direction {dir:?}: expected ordered finite projections"
    ))
}
/// The cross product of box axis `dim` with a frustum edge, as native code writes it: the
/// coordinate `dim` stays zero.
#[inline]
fn cross_box_axis(dim: usize, edge: Xyz) -> Xyz {
    match dim {
        0 => Xyz::new(0.0, -edge.z(), edge.y()),
        1 => Xyz::new(edge.z(), 0.0, -edge.x()),
        _ => Xyz::new(-edge.y(), edge.x(), 0.0),
    }
}
impl<const N: usize> SelectMgrFrustumCore<N> {
    fn box_axis_range(
        &self,
        direction: &Xyz,
        low: f64,
        high: f64,
        inclusion: bool,
    ) -> Option<(f64, f64)> {
        let (mut min, mut max) = (f64::MAX, -f64::MAX);
        // A frustum keeps `2 N` vertices; the fixed count lets the compiler unroll the loop (#594).
        let vertices = self.vertices.get(..2 * N).unwrap_or(&self.vertices);
        for vertex in vertices {
            let projection = vertex.xyz().dot(*direction);
            min = std_min(min, projection);
            max = std_max(max, projection);
            if min <= high && max >= low && !inclusion {
                return None;
            }
        }
        Some((min, max))
    }
    /// `isSeparated(box, direction, inside)`: refines the full-inside flag.
    fn is_separated_box(
        &self,
        min: &Vec3<f64>,
        max: &Vec3<f64>,
        direction: &Xyz,
        inside: &mut Option<&mut bool>,
    ) -> Result<bool, Xyz> {
        let (low, high) = box_projection(direction, min, max)?;
        let inclusion = inside.as_ref().is_some_and(|flag| **flag);
        let Some((min_f, max_f)) = self.box_axis_range(direction, low, high, inclusion) else {
            return Ok(false);
        };
        if min_f > high || max_f < low {
            return Ok(true);
        }
        refine_frustum_inside(inside, (low, high), (min_f, max_f));
        Ok(false)
    }
    /// The axis-aligned test of `hasBoxOverlap`; false when separated.
    fn box_ortho_overlap(
        &self,
        min: &Vec3<f64>,
        max: &Vec3<f64>,
        inside: &mut Option<&mut bool>,
    ) -> bool {
        for axis in 0..3 {
            let (lo, hi) = (axis_of(min, axis), axis_of(max, axis));
            let (lower, upper) = (
                self.min_ortho_verts_projections[axis],
                self.max_ortho_verts_projections[axis],
            );
            if lo > upper || hi < lower {
                return false;
            }
            refine_frustum_inside(inside, (lo, hi), (lower, upper));
        }
        true
    }
    /// The frustum plane test of `hasBoxOverlap`; false when separated.
    fn box_planes_overlap(
        &self,
        min: &Vec3<f64>,
        max: &Vec3<f64>,
        ortho: bool,
        inside: &mut Option<&mut bool>,
    ) -> Result<bool, Xyz> {
        for index in (0..N + 1).step_by(self.inc_factor(ortho)) {
            let (lo, hi) = box_projection(&self.planes[index], min, max)?;
            let (lower, upper) = (
                self.min_verts_projections[index],
                self.max_verts_projections[index],
            );
            if lo > upper || hi < lower {
                return Ok(false);
            }
            refine_frustum_inside(inside, (lo, hi), (lower, upper));
        }
        Ok(true)
    }
    fn box_cross_axes_overlap(
        &self,
        min: &Vec3<f64>,
        max: &Vec3<f64>,
        ortho: bool,
        inside: &mut Option<&mut bool>,
    ) -> Result<bool, Xyz> {
        let edges = &self.edge_dirs[..Self::directions_nb(ortho)];
        // Native order: each box axis against every frustum edge.
        for dim in 0..3 {
            for edge in edges {
                let direction = cross_box_axis(dim, *edge);
                if self.is_separated_box(min, max, &direction, inside)? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
    /// `hasBoxOverlap`: the separating axis test, refining full-inside; bad projections raise ProgramError.
    /// Example: `core.has_box_overlap(&min, &max, false, None)?`.
    pub fn has_box_overlap(
        &self,
        min: &Vec3<f64>,
        max: &Vec3<f64>,
        ortho: bool,
        inside: Option<&mut bool>,
    ) -> Result<bool, Failure> {
        self.has_box_overlap_deferred(min, max, || ortho, inside)
    }
    /// `has_box_overlap` that asks for the projection kind only after the axis-aligned test
    /// passes, as native code reads `Camera()->IsOrthographic()` there. The intersectors read
    /// it through two locks, and most nodes of a picking walk fail the axis test first (#594).
    pub(crate) fn has_box_overlap_deferred(
        &self,
        min: &Vec3<f64>,
        max: &Vec3<f64>,
        ortho: impl FnOnce() -> bool,
        mut inside: Option<&mut bool>,
    ) -> Result<bool, Failure> {
        self.box_overlap(min, max, ortho, &mut inside)
            .map_err(|direction| unordered_projection(&direction, min, max))
    }
    fn box_overlap(
        &self,
        min: &Vec3<f64>,
        max: &Vec3<f64>,
        ortho: impl FnOnce() -> bool,
        inside: &mut Option<&mut bool>,
    ) -> Result<bool, Xyz> {
        if !self.box_ortho_overlap(min, max, inside) {
            return Ok(false);
        }
        let ortho = ortho();
        if !self.box_planes_overlap(min, max, ortho, inside)? {
            return Ok(false);
        }
        self.box_cross_axes_overlap(min, max, ortho, inside)
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectMgrFrustumCore, cross_box_axis};
    use martensite_tkernel::ncollection::Vec3;
    use martensite_tkmath::gp::Xyz;
    use std::cell::Cell;

    /// A zeroed frustum: every projection range is the point 0.
    fn deferred_overlap(min: f64, max: f64, asked: &Cell<u32>) -> bool {
        let core = SelectMgrFrustumCore::<4>::new();
        let (min, max) = (Vec3::new(min, min, min), Vec3::new(max, max, max));
        let ortho = || {
            asked.set(asked.get() + 1);
            false
        };
        let deferred = core.has_box_overlap_deferred(&min, &max, ortho, None);
        let eager = core.has_box_overlap(&min, &max, false, None);
        assert_eq!(
            deferred.as_ref().ok(),
            eager.as_ref().ok(),
            "deferred and eager results"
        );
        deferred.expect("ordered projections")
    }

    #[test]
    fn deferred_overlap_asks_for_the_projection_only_past_the_axis_test() {
        let asked = Cell::new(0);
        assert!(!deferred_overlap(1.0, 2.0, &asked), "separated on the axes");
        assert_eq!(
            asked.get(),
            0,
            "an axis-separated box never reads the camera"
        );
        assert!(
            deferred_overlap(-1.0, 1.0, &asked),
            "a box around the frustum"
        );
        assert_eq!(
            asked.get(),
            1,
            "one read for a box that passes the axis test"
        );
    }

    /// Native hasBoxOverlap writes `-edge[(dim + 2) % 3]` and `edge[(dim + 1) % 3]` into a
    /// zero vector, at those two indices.
    fn indexed_cross_axis(dim: usize, edge: Xyz) -> [u64; 3] {
        let (next, last) = ((dim + 1) % 3, (dim + 2) % 3);
        let mut direction = [0.0_f64; 3];
        direction[next] = -edge.coordinates()[last];
        direction[last] = edge.coordinates()[next];
        direction.map(f64::to_bits)
    }

    #[test]
    fn cross_axes_match_the_indexed_native_form_bit_for_bit() {
        for edge in [Xyz::new(1.5, -2.25, 0.0), Xyz::new(-0.0, 3.0, -7.5)] {
            for dim in 0..3 {
                let axis = cross_box_axis(dim, edge);
                let bits = [axis.x(), axis.y(), axis.z()].map(f64::to_bits);
                assert_eq!(
                    bits,
                    indexed_cross_axis(dim, edge),
                    "axis {dim} of {edge:?}"
                );
            }
        }
    }
}
