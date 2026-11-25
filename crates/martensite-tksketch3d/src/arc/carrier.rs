// SPDX-License-Identifier: GPL-2.0-only
//! Calculate the circle in local coordinates, then verify its directed trim.
use super::{ArcDefinition, ArcFailure, SketchArc, distance_comparison};
use crate::admission::estimated_norm;
use martensite_tkmath::{
    elclib,
    gp::{Axis2, Direction3, Point3, Vector3},
    predicates::{
        compare_triangle_altitude_3d, compare_triangle_orientation_3d,
        compare_triangle_projected_dot_3d,
    },
};
use std::{
    cmp::Ordering,
    f64::consts::{PI, TAU},
};

pub(super) fn construct(
    recipe: SketchArc,
    [start, end, placement]: [Point3; 3],
    minimum: f64,
) -> Result<Carrier, ArcFailure> {
    match recipe.definition {
        ArcDefinition::Through { .. } => through(start, end, placement, minimum),
        ArcDefinition::Center { normal, .. } => {
            centered(recipe, start, end, placement, normal, minimum)
        }
    }
}

fn through(
    start: Point3,
    end: Point3,
    witness: Point3,
    minimum: f64,
) -> Result<Carrier, ArcFailure> {
    let altitude = compare_triangle_altitude_3d(
        *start.coordinates(),
        *end.coordinates(),
        *witness.coordinates(),
        minimum,
    )
    .map_err(|_| ArcFailure::UnrepresentableCarrier)?;
    if altitude == Ordering::Less {
        return Err(ArcFailure::SmallWitnessAltitude);
    }
    let (scale, [w, e]) = scaled_offsets(start, [witness, end])?;
    let (center, normal) = circumcenter(w, e)?;
    if orientation(start, witness, end, normal)? != Ordering::Greater {
        return Err(ArcFailure::IndeterminateBranch);
    }
    let location = start.translated(center * scale);
    let circle = LocalCircle::new(location, normal, -center, scale)?;
    let (via, sweep) = (circle.angle(w - center), circle.angle(e - center));
    if !(via > 0. && via < sweep && sweep < TAU) {
        return Err(ArcFailure::IndeterminateBranch);
    }
    Ok(circle.trim(sweep, Some(via)))
}

fn circumcenter(w: Vector3, e: Vector3) -> Result<(Vector3, Direction3), ArcFailure> {
    let cross = w.crossed(e);
    let normal = direction(cross)?;
    let normal_vector = Vector3::from(normal);
    let divisor = 2. * cross.dot(normal_vector);
    // The two perpendicular bisectors give this local linear combination.
    let numerator = e.crossed(normal_vector) * w.square_magnitude()
        + normal_vector.crossed(w) * e.square_magnitude();
    Ok((numerator / divisor, normal))
}

fn centered(
    recipe: SketchArc,
    start: Point3,
    end: Point3,
    center: Point3,
    normal: Direction3,
    minimum: f64,
) -> Result<Carrier, ArcFailure> {
    for (point, position) in recipe.endpoints.into_iter().zip([start, end]) {
        if distance_comparison(center, position, minimum)? == Ordering::Less {
            return Err(ArcFailure::ShortRadius(point));
        }
    }
    let (scale, [a, b]) = scaled_offsets(center, [start, end])?;
    let selected_normal = direction(Vector3::from(normal))?;
    let circle = LocalCircle::new(center, selected_normal, a, scale)?;
    let sign = orientation(center, start, end, normal)?;
    let selected = if sign == Ordering::Equal {
        semicircle(center, start, end, normal)?
    } else {
        center_sweep(sign, circle.angle(b))?
    };
    Ok(circle.trim(selected, None))
}

fn center_sweep(sign: Ordering, sweep: f64) -> Result<f64, ArcFailure> {
    match sign {
        Ordering::Greater if sweep > 0. && sweep < PI => Ok(sweep),
        Ordering::Less if sweep > PI && sweep < TAU => Ok(sweep),
        _ => Err(ArcFailure::IndeterminateBranch),
    }
}

fn semicircle(
    center: Point3,
    start: Point3,
    end: Point3,
    normal: Direction3,
) -> Result<f64, ArcFailure> {
    // Reconstructing the oblique frame can move the rounded angle to either side of pi.
    let projected = compare_triangle_projected_dot_3d(
        *center.coordinates(),
        *start.coordinates(),
        *end.coordinates(),
        *normal.coordinates(),
    )
    .map_err(|_| ArcFailure::UnrepresentableCarrier)?;
    if projected == Ordering::Less {
        return Ok(PI);
    }
    Err(ArcFailure::IndeterminateBranch)
}

fn orientation(
    origin: Point3,
    first: Point3,
    second: Point3,
    normal: Direction3,
) -> Result<Ordering, ArcFailure> {
    compare_triangle_orientation_3d(
        *origin.coordinates(),
        *first.coordinates(),
        *second.coordinates(),
        *normal.coordinates(),
    )
    .map_err(|_| ArcFailure::UnrepresentableCarrier)
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Carrier {
    pub(super) frame: Axis2,
    pub(super) radius: f64,
    pub(super) sweep: f64,
    pub(super) witness: Option<f64>,
}

impl PartialEq for Carrier {
    fn eq(&self, other: &Self) -> bool {
        self.frame.location() == other.frame.location()
            && self.frame.direction() == other.frame.direction()
            && self.frame.x_direction() == other.frame.x_direction()
            && self.frame.y_direction() == other.frame.y_direction()
            && self.radius == other.radius
            && self.sweep == other.sweep
            && self.witness == other.witness
    }
}

impl Carrier {
    pub(super) fn value(self, angle: f64) -> Point3 {
        elclib::circle_value(angle, self.frame, self.radius)
    }
}

struct LocalCircle {
    frame: Axis2,
    radius: f64,
}

impl LocalCircle {
    fn new(
        center: Point3,
        normal: Direction3,
        start: Vector3,
        scale: f64,
    ) -> Result<Self, ArcFailure> {
        let normal_vector = Vector3::from(normal);
        let planar = start - normal_vector * start.dot(normal_vector);
        let radius = estimated_norm(*planar.xyz()) * scale;
        // Finite radius alone is insufficient to evaluate the entire trim at this location.
        if radius <= 0.
            || !center
                .coordinates()
                .iter()
                .all(|coordinate| (coordinate.abs() + 2. * radius).is_finite())
        {
            return Err(ArcFailure::UnrepresentableCarrier);
        }
        let frame = Axis2::with_x_direction(center, normal, direction(planar)?)
            .map_err(|_| ArcFailure::UnrepresentableCarrier)?;
        Ok(Self { frame, radius })
    }

    fn trim(self, sweep: f64, witness: Option<f64>) -> Carrier {
        Carrier {
            frame: self.frame,
            radius: self.radius,
            sweep,
            witness,
        }
    }

    fn angle(&self, offset: Vector3) -> f64 {
        let mut local = self.frame;
        local.set_location(Point3::default());
        let scale = max_component(offset);
        elclib::circle_parameter(local, Point3::from(*(offset / scale).xyz()))
    }
}

fn scaled_offsets(origin: Point3, points: [Point3; 2]) -> Result<(f64, [Vector3; 2]), ArcFailure> {
    let [a, b] = points.map(|point| Vector3::between(origin, point));
    let scale = max_component(a).max(max_component(b));
    if scale <= 0. || !scale.is_finite() {
        return Err(ArcFailure::UnrepresentableCarrier);
    }
    Ok((scale, [a / scale, b / scale]))
}

fn direction(vector: Vector3) -> Result<Direction3, ArcFailure> {
    let scale = max_component(vector);
    if scale <= 0. || !scale.is_finite() {
        return Err(ArcFailure::UnrepresentableCarrier);
    }
    let [x, y, z] = *(vector / scale).xyz().coordinates();
    if ![x, y, z].into_iter().all(f64::is_finite) {
        return Err(ArcFailure::UnrepresentableCarrier);
    }
    Direction3::new(x, y, z).map_err(|_| ArcFailure::UnrepresentableCarrier)
}

fn max_component(vector: Vector3) -> f64 {
    vector
        .xyz()
        .coordinates()
        .iter()
        .map(|coordinate| coordinate.abs())
        .fold(0., f64::max)
}
