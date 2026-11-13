//! Checked coefficient retention for binary geometry capture (#432).
use super::super::{Axis3, Direction3, Failure, Point3, construction};
use super::stored_frame_interval::{Bounds, VectorBounds, cross, dot, normalize};

impl Axis3 {
    /// Retains all coefficients after outward orthonormal-frame validation (#432).
    /// The roundoff enclosure contract is documented in `docs/features/stored-surface-frames/contract.md`.
    /// `Axis3::from_stored_coefficients(Point3::default(), [0.,0.,1.], [1.,0.,0.], [0.,1.,0.])?`.
    pub fn from_stored_coefficients(
        location: Point3,
        direction: [f64; 3],
        x_direction: [f64; 3],
        y_direction: [f64; 3],
    ) -> Result<Self, Failure> {
        let stored = [direction, x_direction, y_direction];
        validate_origin(location)?;
        validate(stored).map_err(|reason| {
            construction(&format!(
                "stored frame {stored:?}: expected PreserveStoredV1 frame; {reason}"
            ))
        })?;
        let [main, x, y] = stored.map(|coordinates| Direction3 { coordinates });
        Ok(Self::from_stored(location, main, x, y))
    }
}
fn validate_origin(location: Point3) -> Result<(), Failure> {
    if location.coordinates().iter().all(|value| value.is_finite()) {
        return Ok(());
    }
    Err(construction(&format!(
        "stored frame origin {:?}: expected finite coordinates",
        location.coordinates()
    )))
}
fn validate(stored: [[f64; 3]; 3]) -> Result<(), &'static str> {
    let [main, x, y] = stored.map(|vector| vector.map(Bounds::point));
    for vector in [main, x, y] {
        check_norm(vector)?;
    }
    let normal = conditioned_normal(main, x)?;
    let rebuilt_x = cross(x, normal)
        .and_then(|inner| cross(normal, inner))
        .and_then(normalize)
        .ok_or("inconclusive X reconstruction")?;
    let rebuilt_y = cross(normal, rebuilt_x)
        .and_then(normalize)
        .ok_or("inconclusive Y reconstruction")?;
    let sign = cross(x, y)
        .and_then(|product| dot(product, main))
        .ok_or("inconclusive handedness")?;
    let oriented_y = orient_y(rebuilt_y, sign)?;
    check_membership(stored, [normal, rebuilt_x, oriented_y])
}
fn conditioned_normal(main: VectorBounds, x: VectorBounds) -> Result<VectorBounds, &'static str> {
    let normal = normalize(main).ok_or("inconclusive main normalization")?;
    let unit_x = normalize(x).ok_or("inconclusive X normalization")?;
    let cosine = dot(normal, unit_x).ok_or("inconclusive conditioning")?;
    if cosine.lower < -0.5 || cosine.upper > 0.5 {
        return Err("main/X conditioning outside [-1/2,1/2]");
    }
    Ok(normal)
}
fn check_norm(vector: VectorBounds) -> Result<(), &'static str> {
    if vector.iter().any(|axis| !axis.lower.is_finite()) {
        return Err("nonfinite direction");
    }
    let norm = dot(vector, vector).ok_or("inconclusive squared norm")?;
    if norm.lower < 0.5 || norm.upper > 2.0 {
        return Err("squared norm outside [1/2,2]");
    }
    Ok(())
}
fn orient_y(y: VectorBounds, sign: Bounds) -> Result<VectorBounds, &'static str> {
    if sign.lower > 0.0 {
        return Ok(y);
    }
    if sign.upper < 0.0 {
        return Ok(y.map(Bounds::negate));
    }
    Err("handedness enclosure includes zero")
}
fn check_membership(stored: [[f64; 3]; 3], rebuilt: [VectorBounds; 3]) -> Result<(), &'static str> {
    for (vector, enclosure) in stored.into_iter().zip(rebuilt) {
        if vector
            .into_iter()
            .zip(enclosure)
            .any(|(value, bound)| !bound.contains(value))
        {
            return Err("stored coefficient outside orthonormal reconstruction enclosure");
        }
    }
    Ok(())
}
