// SPDX-License-Identifier: GPL-2.0-only
//! Exact directed endpoint stencil and physical line/cubic continuity checks.
use super::{DyadicBound, ExactRatio, NonFiniteCoordinate, nearest_binary64};
use crate::predicates::{exact_coordinate, exact_vector3, triple_product::exact_cross};
use num_bigint::BigInt;
use std::{cmp::Ordering, fmt};

// Binary64 coordinates decode into integer multiples of 2^-1074.
const UNIT_EXPONENT: i32 = -1074;
const SQUARED_SCALE: usize = 2148;

/// Neighboring controls in traversal order away from the chosen cubic endpoint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CubicEndStencil {
    /// The endpoint and first inward control.
    Tangent([[f64; 3]; 2]),
    /// The endpoint and first two inward controls.
    Curvature([[f64; 3]; 3]),
}

/// Exact directed derivatives of the authored endpoint controls.
///
/// Traversal starts at the endpoint and moves into the curve. A reverse traversal
/// uses the same stencil on reversed controls, reversing D1 and preserving D2.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CubicEndJet {
    first: [BigInt; 3],
    second: Option<[BigInt; 3]>,
}

/// Each endpoint derivative coordinate rounded once to binary64.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedCubicEndJet {
    /// Velocity in model units per normalized parameter.
    pub first: [f64; 3],
    /// Acceleration in model units per squared normalized parameter, if requested.
    pub second: Option<[f64; 3]>,
}

/// Exact signed angular admission against an incoming line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TangentDirectionBand {
    /// Sign of the directed dot product.
    pub dot: Ordering,
    /// Sine magnitude compared with the supplied angular limit.
    pub sine: Ordering,
}

/// A declined endpoint stencil or metric query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CubicEndJetFailure {
    /// Original finite coordinate requirement.
    NonFinite(NonFiniteCoordinate),
    /// The endpoint and its first neighboring control coincide.
    Stationary,
    /// The incoming line endpoints coincide.
    DegenerateLine,
    /// Curvature requires a second neighboring control.
    MissingSecond,
    /// A speed or curvature magnitude limit must be finite and nonnegative.
    InvalidMagnitude(f64),
    /// A sine limit must be finite and in the inclusive range `[0,1]`.
    InvalidSine(f64),
    /// One derivative coordinate would round to infinity.
    OutOfRange {
        /// Derivative order in the normalized parameter.
        order: usize,
        /// Axis of the refused component.
        axis: usize,
    },
}

impl fmt::Display for CubicEndJetFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite(source) => source.fmt(output),
            Self::Stationary => write!(output, "cubic endpoint velocity must be nonzero"),
            Self::DegenerateLine => write!(output, "incoming endpoints must be distinct"),
            Self::MissingSecond => write!(output, "endpoint curvature requires three controls"),
            Self::InvalidMagnitude(value) => write!(
                output,
                "magnitude limit {value} must be finite and nonnegative"
            ),
            Self::InvalidSine(value) => write!(output, "sine limit {value} must lie in [0,1]"),
            Self::OutOfRange { order, axis } => write!(
                output,
                "endpoint derivative order {order}, axis {axis} must round to finite binary64",
            ),
        }
    }
}

impl std::error::Error for CubicEndJetFailure {}

impl CubicEndJet {
    /// Capture an exact directed linearly evaluated stencil. Example `CubicEndJet::new(stencil)?`.
    pub fn new(stencil: CubicEndStencil) -> Result<Self, CubicEndJetFailure> {
        let (endpoint, adjacent, next) = match stencil {
            CubicEndStencil::Tangent([a, b]) => (a, b, None),
            CubicEndStencil::Curvature([a, b, c]) => (a, b, Some(c)),
        };
        let a = point(endpoint, 0)?;
        let b = point(adjacent, 1)?;
        if a == b {
            return Err(CubicEndJetFailure::Stationary);
        }
        let first = std::array::from_fn(|axis| (&b[axis] - &a[axis]) * 3);
        let second = next
            .map(|control| acceleration(&a, &b, control))
            .transpose()?;
        Ok(Self { first, second })
    }

    /// Round each derivative coordinate once. Example `jet.rounded()?.first`.
    pub fn rounded(&self) -> Result<RoundedCubicEndJet, CubicEndJetFailure> {
        Ok(RoundedCubicEndJet {
            first: rounded_vector(&self.first, 1)?,
            second: self
                .second
                .as_ref()
                .map(|v| rounded_vector(v, 2))
                .transpose()?,
        })
    }

    /// Compare the exact endpoint speed with a model-unit floor. Example `jet.speed(0.001)?`.
    pub fn speed(&self, floor: f64) -> Result<Ordering, CubicEndJetFailure> {
        let floor = magnitude(floor)?;
        Ok(squared(&self.first).cmp(&(&floor * &floor)))
    }

    /// Compare exact tangent direction with the line directed toward the shared endpoint.
    /// Example `jet.tangent(incoming, 1e-8)?.sine`.
    pub fn tangent(
        &self,
        [from, to]: [[f64; 3]; 2],
        sine: f64,
    ) -> Result<TangentDirectionBand, CubicEndJetFailure> {
        if !(0. ..=1.).contains(&sine) {
            return Err(CubicEndJetFailure::InvalidSine(sine));
        }
        let sine = magnitude(sine)?;
        let line = line_direction(from, to)?;
        let normal = exact_cross(&line, &self.first);
        let observed = squared(&normal) << SQUARED_SCALE;
        let allowed = &sine * &sine * squared(&line) * squared(&self.first);
        Ok(TangentDirectionBand {
            dot: dot(&line, &self.first).cmp(&0.into()),
            sine: observed.cmp(&allowed),
        })
    }

    /// Compare exact endpoint curvature magnitude with an inverse-length cap.
    /// A straight line has zero curvature. Example `jet.curvature(1e-6)?`.
    pub fn curvature(&self, cap: f64) -> Result<Ordering, CubicEndJetFailure> {
        let limit = magnitude(cap)?;
        let second = self
            .second
            .as_ref()
            .ok_or(CubicEndJetFailure::MissingSecond)?;
        let squared_speed = squared(&self.first);
        let normal = exact_cross(&self.first, second);
        // |κ|² = |v × a|² / (v·v)^3. Widen the integer grid before comparing.
        let observed = squared(&normal) << (2 * SQUARED_SCALE);
        let allowed = &limit * &limit * &squared_speed * &squared_speed * squared_speed;
        Ok(observed.cmp(&allowed))
    }
}

fn point(position: [f64; 3], index: usize) -> Result<[BigInt; 3], CubicEndJetFailure> {
    exact_vector3(position, index).map_err(CubicEndJetFailure::NonFinite)
}

fn acceleration(
    a: &[BigInt; 3],
    b: &[BigInt; 3],
    control: [f64; 3],
) -> Result<[BigInt; 3], CubicEndJetFailure> {
    let c = point(control, 2)?;
    Ok(std::array::from_fn(|axis| {
        (&c[axis] - (&b[axis] << 1_usize) + &a[axis]) * 6
    }))
}

fn line_direction(from: [f64; 3], to: [f64; 3]) -> Result<[BigInt; 3], CubicEndJetFailure> {
    let from = point(from, 0)?;
    let to = point(to, 1)?;
    if from == to {
        return Err(CubicEndJetFailure::DegenerateLine);
    }
    Ok(std::array::from_fn(|axis| &to[axis] - &from[axis]))
}

fn rounded_vector(vector: &[BigInt; 3], order: usize) -> Result<[f64; 3], CubicEndJetFailure> {
    let [x, y, z] = [0, 1, 2].map(|axis| {
        let exact = DyadicBound::new(vector[axis].clone(), UNIT_EXPONENT);
        nearest_binary64(&ExactRatio::from_dyadic(&exact))
            .ok_or(CubicEndJetFailure::OutOfRange { order, axis })
    });
    Ok([x?, y?, z?])
}

fn magnitude(value: f64) -> Result<BigInt, CubicEndJetFailure> {
    if !value.is_finite() || value < 0. {
        return Err(CubicEndJetFailure::InvalidMagnitude(value));
    }
    exact_coordinate(value, 0, 0).map_err(CubicEndJetFailure::NonFinite)
}

fn dot(first: &[BigInt; 3], second: &[BigInt; 3]) -> BigInt {
    first.iter().zip(second).map(|(a, b)| a * b).sum()
}

fn squared(vector: &[BigInt; 3]) -> BigInt {
    dot(vector, vector)
}

#[cfg(test)]
mod tests;
