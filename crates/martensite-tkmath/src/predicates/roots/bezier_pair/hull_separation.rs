//! Exact Bernstein hull witnesses establish disjointness or one declared endpoint.
use super::super::certificate_arithmetic::ExactRatio;
use std::cmp::Ordering;
type ExactPoint = [ExactRatio; 2];
type ExactHull = [ExactPoint; 3];

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::predicates::roots) enum HullSeparation {
    Disjoint {
        normal: ExactPoint,
    },
    EndpointOnly {
        point: ExactPoint,
        normals: [ExactPoint; 2],
    },
    Unresolved,
}

pub(in crate::predicates::roots) fn certify_hulls<E>(
    left: &ExactHull,
    right: &ExactHull,
    endpoint: Option<&ExactPoint>,
    checkpoint: &mut impl FnMut() -> Result<(), E>,
) -> Result<HullSeparation, E> {
    let candidates = normal_candidates(left, right);
    for normal in &candidates {
        checkpoint()?;
        if strict_separation(left, right, normal) {
            return Ok(HullSeparation::Disjoint {
                normal: normal.clone(),
            });
        }
    }
    let Some(point) = endpoint.filter(|point| endpoint_member(left, right, point)) else {
        return Ok(HullSeparation::Unresolved);
    };
    endpoint_separation(left, right, point, &candidates, checkpoint)
}

fn endpoint_separation<E>(
    left: &ExactHull,
    right: &ExactHull,
    point: &ExactPoint,
    candidates: &[ExactPoint],
    checkpoint: &mut impl FnMut() -> Result<(), E>,
) -> Result<HullSeparation, E> {
    let mut weak = Vec::<ExactPoint>::new();
    for normal in candidates {
        checkpoint()?;
        if !weak_separation(left, right, point, normal) {
            continue;
        }
        if let Some(witness) = independent_witness(point, normal, &weak) {
            return Ok(witness);
        }
        weak.push(normal.clone());
    }
    Ok(HullSeparation::Unresolved)
}

fn endpoint_member(left: &ExactHull, right: &ExactHull, point: &ExactPoint) -> bool {
    left.contains(point) && right.contains(point)
}

fn independent_witness(
    point: &ExactPoint,
    normal: &ExactPoint,
    weak: &[ExactPoint],
) -> Option<HullSeparation> {
    let other = weak
        .iter()
        .find(|other| cross(normal, other).sign() != Ordering::Equal)?;
    Some(HullSeparation::EndpointOnly {
        point: point.clone(),
        normals: [other.clone(), normal.clone()],
    })
}

fn normal_candidates(left: &ExactHull, right: &ExactHull) -> Vec<ExactPoint> {
    [left, right].into_iter().flat_map(hull_normals).collect()
}

fn hull_normals(hull: &ExactHull) -> Vec<ExactPoint> {
    let mut result = Vec::new();
    for i in 0..3 {
        for j in 0..i {
            let difference = subtract(&hull[i], &hull[j]);
            result.push(difference.clone());
            result.push([difference[1].negated(), difference[0].clone()]);
        }
    }
    result
}

fn strict_separation(left: &ExactHull, right: &ExactHull, normal: &ExactPoint) -> bool {
    let left = left.each_ref().map(|point| dot(normal, point));
    let right = right.each_ref().map(|point| dot(normal, point));
    left.iter().max() < right.iter().min() || right.iter().max() < left.iter().min()
}

fn weak_separation(
    left: &ExactHull,
    right: &ExactHull,
    point: &ExactPoint,
    normal: &ExactPoint,
) -> bool {
    let left = left
        .each_ref()
        .map(|p| dot(normal, &subtract(p, point)).sign());
    let right = right
        .each_ref()
        .map(|p| dot(normal, &subtract(p, point)).sign());
    opposite_weak_sides(&left, &right) || opposite_weak_sides(&right, &left)
}

fn opposite_weak_sides(left: &[Ordering; 3], right: &[Ordering; 3]) -> bool {
    left.iter().all(|v| *v != Ordering::Greater) && right.iter().all(|v| *v != Ordering::Less)
}

fn subtract(left: &ExactPoint, right: &ExactPoint) -> ExactPoint {
    std::array::from_fn(|i| left[i].subtract(&right[i]))
}

pub(in crate::predicates::roots) fn cross(left: &ExactPoint, right: &ExactPoint) -> ExactRatio {
    left[0]
        .multiply(&right[1])
        .subtract(&left[1].multiply(&right[0]))
}

fn dot(left: &ExactPoint, right: &ExactPoint) -> ExactRatio {
    left[0]
        .multiply(&right[0])
        .add(&left[1].multiply(&right[1]))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn hull(points: [[f64; 2]; 3]) -> ExactHull {
        points.map(|p| p.map(ExactRatio::from_finite))
    }
    #[test]
    fn two_independent_weak_lines_prove_only_the_actual_shared_endpoint() {
        let a = hull([[0., 0.], [1., 0.], [2., 0.]]);
        let b = hull([[2., 0.], [2., 1.], [2., 2.]]);
        let actual = [2., 0.].map(ExactRatio::from_finite);
        let result = certify_hulls(&a, &b, Some(&actual), &mut || Ok::<(), ()>(())).unwrap();
        assert!(matches!(result, HullSeparation::EndpointOnly { .. }));
        let forged = [99., 99.].map(ExactRatio::from_finite);
        assert_eq!(
            certify_hulls(&a, &b, Some(&forged), &mut || Ok::<(), ()>(())).unwrap(),
            HullSeparation::Unresolved
        );
    }
    #[test]
    fn overlapping_backtracking_hulls_and_missing_seam_do_not_exclude() {
        let a = hull([[0., 0.], [1., 0.], [2., 0.]]);
        let b = hull([[1., 0.], [2., 0.], [3., 0.]]);
        let point = [2., 0.].map(ExactRatio::from_finite);
        assert_eq!(
            certify_hulls(&a, &b, Some(&point), &mut || Ok::<(), ()>(())).unwrap(),
            HullSeparation::Unresolved
        );
        let corner = hull([[2., 0.], [2., 1.], [2., 2.]]);
        assert_eq!(
            certify_hulls(&a, &corner, None, &mut || Ok::<(), ()>(())).unwrap(),
            HullSeparation::Unresolved
        );
    }
}
