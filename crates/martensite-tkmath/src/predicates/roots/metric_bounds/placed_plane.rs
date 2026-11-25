// SPDX-License-Identifier: GPL-2.0-only
//! Exact affine transport and normal classification for the original planar chart.
use super::super::certificate_arithmetic::{ExactRatio, rounding::outward_binary64};
use crate::{
    gp::{Direction3, Line3, Plane3, Point3},
    predicates::NonFiniteCoordinate,
};
use std::cmp::Ordering;

pub(super) type ExactVector = [ExactRatio; 3];

/// Original planar chart after exact application of the stored affine placement.
#[derive(Clone, Debug)]
pub struct PlacedPlane {
    center: ExactVector,
    normal: ExactVector,
}

impl PlacedPlane {
    /// Capture the plane's parameter basis and original `[[linear, translation]; 3]` rows.
    /// `None` denotes a degenerate placed tangent pair.
    /// Example: `let support = PlacedPlane::capture(plane, placement)?;`.
    pub fn capture(
        plane: Plane3,
        rows: [[f64; 4]; 3],
    ) -> Result<Option<Self>, NonFiniteCoordinate> {
        let placement = ExactPlacement::capture(rows)?;
        let frame = plane.position();
        let u = placement.vector(exact_vector(*frame.x_direction().coordinates(), 0)?);
        let v = placement.vector(exact_vector(*frame.y_direction().coordinates(), 1)?);
        let normal = cross(&u, &v);
        if normal.iter().all(|value| value.sign() == Ordering::Equal) {
            return Ok(None);
        }
        let center = placement.point(exact_point(plane.location(), 2)?);
        Ok(Some(Self { center, normal }))
    }

    /// Exact sign of pull dotted with the placed parameter normal. Example: `plane.pull_sign(pull)?`.
    pub fn pull_sign(&self, pull: Direction3) -> Result<Ordering, NonFiniteCoordinate> {
        let pull = exact_vector(*pull.coordinates(), 3)?;
        Ok(dot(&pull, &self.normal).sign())
    }

    /// Exact world coordinate and normal sign of an axis-aligned side, if representable.
    /// Example: `let side = plane.axis_side();`.
    pub fn axis_side(&self) -> Option<(usize, f64, Ordering)> {
        let mut axes = self
            .normal
            .iter()
            .enumerate()
            .filter(|(_, component)| component.sign() != Ordering::Equal);
        let (axis, component) = axes.next()?;
        if axes.next().is_some() {
            return None;
        }
        let [lo, hi] = outward_binary64(&self.center[axis])?;
        (lo == hi).then_some((axis, lo, component.sign()))
    }
}

/// Evaluate exact source line endpoints after its stored placement, returning `None`
/// if the finite increasing interval or any placed endpoint lacks an exact binary64 value.
/// Example: `let endpoints = exact_placed_line_ends(line, placement, range)?;`.
pub fn exact_placed_line_ends(
    line: Line3,
    rows: [[f64; 4]; 3],
    range: [f64; 2],
) -> Result<Option<[[f64; 3]; 2]>, NonFiniteCoordinate> {
    if !range.into_iter().all(f64::is_finite) || range[0] >= range[1] {
        return Ok(None);
    }
    let placement = ExactPlacement::capture(rows)?;
    let origin = placement.point(exact_point(line.location(), 0)?);
    let direction = placement.vector(exact_vector(*line.direction().coordinates(), 1)?);
    let [a, b] = range
        .map(|parameter| exact_line_point(&origin, &direction, ExactRatio::from_finite(parameter)));
    Ok(a.zip(b).map(|(a, b)| [a, b]))
}

/// Apply the original placement and require exact binary64 world coordinates.
/// Example: `let placed = exact_placed_point(point, placement)?;`.
pub fn exact_placed_point(
    point: Point3,
    rows: [[f64; 4]; 3],
) -> Result<Option<[f64; 3]>, NonFiniteCoordinate> {
    let placement = ExactPlacement::capture(rows)?;
    Ok(exact_binary64_point(
        placement.point(exact_point(point, 0)?),
    ))
}

fn exact_line_point(
    origin: &ExactVector,
    direction: &ExactVector,
    parameter: ExactRatio,
) -> Option<[f64; 3]> {
    exact_binary64_point(std::array::from_fn(|axis| {
        origin[axis].add(&direction[axis].multiply(&parameter))
    }))
}

fn exact_binary64_point(position: ExactVector) -> Option<[f64; 3]> {
    let mut point = [0.; 3];
    for (coordinate, value) in position.iter().zip(&mut point) {
        let [lo, hi] = outward_binary64(coordinate)?;
        if lo != hi {
            return None;
        }
        *value = lo;
    }
    Some(point)
}

pub(super) struct ExactPlacement {
    linear: [ExactVector; 3],
    translation: ExactVector,
}

impl ExactPlacement {
    pub(super) fn capture(rows: [[f64; 4]; 3]) -> Result<Self, NonFiniteCoordinate> {
        let [a, b, c] = rows;
        let linear = [linear_row(a, 5)?, linear_row(b, 6)?, linear_row(c, 7)?];
        let translation = exact_vector([a[3], b[3], c[3]], 7)?;
        Ok(Self {
            linear,
            translation,
        })
    }

    pub(super) fn vector(&self, source: ExactVector) -> ExactVector {
        self.linear.each_ref().map(|row| dot(row, &source))
    }

    pub(super) fn point(&self, source: ExactVector) -> ExactVector {
        let placed = self.vector(source);
        std::array::from_fn(|i| placed[i].add(&self.translation[i]))
    }
}

fn linear_row([a, b, c, _]: [f64; 4], index: usize) -> Result<ExactVector, NonFiniteCoordinate> {
    exact_vector([a, b, c], index)
}

pub(super) fn exact_point(point: Point3, index: usize) -> Result<ExactVector, NonFiniteCoordinate> {
    exact_vector([point.x(), point.y(), point.z()], index)
}

pub(super) fn exact_vector(
    coordinates: [f64; 3],
    index: usize,
) -> Result<ExactVector, NonFiniteCoordinate> {
    for (axis, value) in coordinates.into_iter().enumerate() {
        if !value.is_finite() {
            return Err(NonFiniteCoordinate {
                point: index,
                axis,
                value,
            });
        }
    }
    Ok(coordinates.map(ExactRatio::from_finite))
}

pub(super) fn dot(a: &ExactVector, b: &ExactVector) -> ExactRatio {
    a.iter()
        .zip(b)
        .fold(ExactRatio::from_finite(0.), |sum, (a, b)| {
            sum.add(&a.multiply(b))
        })
}

pub(super) fn triple(a: &ExactVector, b: &ExactVector, c: &ExactVector) -> ExactRatio {
    dot(a, &cross(b, c))
}

pub(super) fn cross(b: &ExactVector, c: &ExactVector) -> ExactVector {
    std::array::from_fn(|i| {
        let [j, k] = [(i + 1) % 3, (i + 2) % 3];
        b[j].multiply(&c[k]).subtract(&b[k].multiply(&c[j]))
    })
}

#[cfg(test)]
mod tests {
    use super::{PlacedPlane, exact_placed_line_ends, exact_placed_point};
    use crate::gp::{Axis1, Direction3, Line3, Plane3, Point3, Transform3};
    use std::cmp::Ordering;

    #[test]
    fn exact_placed_point_refuses_a_binary64_rounding_cell() {
        let point = Point3::new(1., 2., 3.);
        let mut placement = [[1., 0., 0., 2.], [0., 1., 0., 0.], [0., 0., 1., 0.]];
        assert_eq!(
            exact_placed_point(point, placement).unwrap(),
            Some([3., 2., 3.])
        );
        placement[0][3] = 2_f64.powi(53);
        assert_eq!(exact_placed_point(point, placement).unwrap(), None);
    }

    #[test]
    fn stored_placement_sign_survives_rebuilt_frame_rounding() {
        let axis = Axis1::new(Point3::default(), Direction3::new(1., 2., 3.).unwrap());
        let transform = Transform3::rotation(axis, 0.3).unwrap();
        let plane = Plane3::default();
        let rebuilt = plane.transformed(transform).position();
        let pull = rebuilt.x_direction();
        let rows =
            [1, 2, 3].map(|row| [1, 2, 3, 4].map(|column| transform.value(row, column).unwrap()));
        let source = PlacedPlane::capture(plane, rows).unwrap().unwrap();
        assert_eq!(source.pull_sign(pull), Ok(Ordering::Less));
        assert_eq!(
            crate::predicates::compare_scalar_triple_product_3d(
                *pull.coordinates(),
                *rebuilt.x_direction().coordinates(),
                *rebuilt.y_direction().coordinates()
            ),
            Ok(Ordering::Equal)
        );
    }

    #[test]
    fn box_side_needs_an_exactly_representable_placed_level() {
        let plane = Plane3::new(Point3::new(1., 0., 0.), Direction3::X).unwrap();
        let mut rows = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];
        let source = PlacedPlane::capture(plane, rows).unwrap().unwrap();
        assert_eq!(source.axis_side(), Some((0, 1., Ordering::Greater)));
        rows[0][3] = 9_007_199_254_740_992.;
        let source = PlacedPlane::capture(plane, rows).unwrap().unwrap();
        assert_eq!(source.axis_side(), None);
    }
    #[test]
    fn box_edge_checks_the_original_placement_of_both_ends() {
        let line = Line3::new(Point3::new(1., 0., 0.), Direction3::Z);
        let mut rows = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];
        assert_eq!(
            exact_placed_line_ends(line, rows, [0., 2.]),
            Ok(Some([[1., 0., 0.], [1., 0., 2.]]))
        );
        rows[0][3] = 9_007_199_254_740_992.;
        assert_eq!(exact_placed_line_ends(line, rows, [0., 2.]), Ok(None));
    }
}
