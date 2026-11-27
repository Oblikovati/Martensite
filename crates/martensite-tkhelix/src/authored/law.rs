// SPDX-License-Identifier: GPL-2.0-only
//! Finite immutable authored helix and its ordinary-radian derivatives.
use super::{HelixFailure, HelixQuantity as Q};
use crate::helix_geom::local::LocalHelix;
use martensite_tkmath::{
    gp::{Axis2, Point3, Vector3},
    predicates::roots::metric_bounds::{HelixTraceLaw, HelixTraceLead},
};
use std::f64::consts::TAU;

/// Select the two independent axial/rotation quantities of one constant lead.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HelixLead {
    /// Supply axial rise per turn and number of turns to resolve height.
    PitchAndTurns {
        /// Axial model units per turn.
        pitch: f64,
        /// Positive count, including a fractional final turn.
        turns: f64,
    },
    /// Supply total axial rise and number of turns to resolve pitch.
    HeightAndTurns {
        /// Total positive axial displacement in model units.
        height: f64,
        /// Positive count, including a fractional final turn.
        turns: f64,
    },
    /// Supply axial rise per turn and total axial rise to resolve turns.
    PitchAndHeight {
        /// Axial model units per turn.
        pitch: f64,
        /// Total positive axial displacement in model units.
        height: f64,
    },
}

/// The radius is constant or affine in the turn parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HelixRadius {
    /// Circular cylinder with one positive finite radius.
    Cylinder {
        /// Constant radius in model units.
        radius: f64,
    },
    /// Conical span with two positive finite radii, including increasing or decreasing taper.
    Cone {
        /// Opening radius in model units.
        opening: f64,
        /// Closing radius in model units.
        closing: f64,
    },
}

/// Rotation about the placement axis, independent of positive axial rise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelixRotation {
    /// Positive rotation from frame X toward Y with increasing parameter.
    Positive,
    /// Negative rotation from frame X toward minus Y with increasing parameter.
    Negative,
}

/// Author a finite single-span helix in a right-handed model frame.
#[derive(Clone, Copy, Debug)]
pub struct ConstantHelixRequest {
    /// Model-space frame, with the origin on the opening axis station.
    pub placement: Axis2,
    /// Requested constant or affine radius.
    pub radius: HelixRadius,
    /// Two independent positive finite axial/rotation scalars.
    pub lead: HelixLead,
    /// Independent direction of traversal about the frame Z axis.
    pub rotation: HelixRotation,
}

/// Effective binary64 axial choices and actual native parameter domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixResolved {
    /// Axial rise per turn in model units.
    pub pitch: f64,
    /// Total axial rise in model units.
    pub height: f64,
    /// Number of turns, including a possible remainder.
    pub turns: f64,
    /// Rounded upper end of the finite radian domain `[0, parameter_end]`.
    pub parameter_end: f64,
}

/// Model-space position and derivatives at an admitted native-radian parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixEvaluation {
    /// Evaluated position.
    pub position: Point3,
    /// First derivative per radian.
    pub first: Vector3,
    /// Second derivative per radian squared.
    pub second: Vector3,
}

/// Finite immutable analytic view independent from its optional native fit.
#[derive(Clone, Copy, Debug)]
pub struct ConstantHelix {
    request: ConstantHelixRequest,
    resolved: HelixResolved,
    local: LocalHelix,
}

impl ConstantHelix {
    /// Admit a right-handed placement and resolve the effective scalar geometry.
    /// Use `ConstantHelix::new(request)?` before querying or fitting the law.
    pub fn new(request: ConstantHelixRequest) -> Result<Self, HelixFailure> {
        let mut request = request;
        request.placement = checked_frame(request.placement)?;
        let resolved = request.lead.resolve()?;
        let radii = request.radius.radii();
        let local = local_law(radii, resolved, request.rotation)?;
        Ok(Self {
            request,
            resolved,
            local,
        })
    }

    /// Return the original typed choices, such as `law.request().rotation`.
    pub fn request(self) -> ConstantHelixRequest {
        self.request
    }

    /// Return the effective scalars and native domain, including `law.resolved().parameter_end`.
    pub fn resolved(self) -> HelixResolved {
        self.resolved
    }

    /// Evaluate the finite analytic helix and its correct first two derivatives.
    /// Use `law.evaluate(0.)?` to read the opening station.
    pub fn evaluate(self, parameter: f64) -> Result<HelixEvaluation, HelixFailure> {
        let [x, y, z] = self.component(parameter, 0)?;
        let [dx, dy, dz] = self.component(parameter, 1)?;
        let [ddx, ddy, ddz] = self.component(parameter, 2)?;
        Ok(HelixEvaluation {
            position: Point3::new(x, y, z),
            first: Vector3::new(dx, dy, dz),
            second: Vector3::new(ddx, ddy, ddz),
        })
    }

    pub(super) fn component(self, parameter: f64, order: u8) -> Result<[f64; 3], HelixFailure> {
        let end = self.resolved.parameter_end;
        if !(0. ..=end).contains(&parameter) {
            return Err(HelixFailure::Parameter { parameter, end });
        }
        let (sine, cosine) = martensite_cmath::sincos(parameter);
        let local = match order {
            0 => self.local.point(parameter, cosine, sine),
            1 => self.local.first(parameter, cosine, sine),
            2 => self.local.second(parameter, cosine, sine),
            _ => return Err(HelixFailure::Evaluation { parameter, order }),
        };
        let xyz = self.world(local, order == 0);
        if xyz.into_iter().all(f64::is_finite) {
            return Ok(xyz);
        }
        Err(HelixFailure::Evaluation { parameter, order })
    }

    fn world(self, [x, y, z]: [f64; 3], point: bool) -> [f64; 3] {
        place(self.request.placement, [x, y, z], point)
    }

    pub(super) fn trace_law(self) -> HelixTraceLaw {
        let frame = self.request.placement;
        HelixTraceLaw {
            origin: *frame.location().coordinates(),
            basis: [frame.x_direction(), frame.y_direction(), frame.direction()]
                .map(|direction| *direction.coordinates()),
            radii: self.request.radius.radii(),
            lead: self.request.lead.trace(),
            parameter_end: self.resolved.parameter_end,
            positive_rotation: self.local.positive,
        }
    }
}

pub(super) fn checked_frame(frame: Axis2) -> Result<Axis2, HelixFailure> {
    Axis2::from_stored_coefficients(
        frame.location(),
        *frame.direction().coordinates(),
        *frame.x_direction().coordinates(),
        *frame.y_direction().coordinates(),
    )
    .map_err(HelixFailure::InvalidFrame)
}

fn local_law(
    [opening, closing]: [f64; 2],
    resolved: HelixResolved,
    rotation: HelixRotation,
) -> Result<LocalHelix, HelixFailure> {
    positive(opening, Q::OpeningRadius)?;
    positive(closing, Q::ClosingRadius)?;
    let end = resolved.parameter_end;
    let radial = (closing - opening) / end;
    if !radial.is_finite() || (opening != closing && radial == 0.) {
        return Err(HelixFailure::DerivedScalar {
            quantity: Q::RadialRate,
            value: radial,
        });
    }
    let axial = derived_positive(resolved.height / end, Q::AxialRate)?;
    let local = LocalHelix {
        opening_radius: opening,
        radial_rate: radial,
        axial_rate: axial,
        positive: rotation == HelixRotation::Positive,
    };
    derived_positive(local.radius(end), Q::ClosingRadius)?;
    Ok(local)
}

impl HelixRadius {
    fn radii(self) -> [f64; 2] {
        match self {
            Self::Cylinder { radius } => [radius; 2],
            Self::Cone { opening, closing } => [opening, closing],
        }
    }
}

impl HelixLead {
    fn trace(self) -> HelixTraceLead {
        match self {
            Self::PitchAndTurns { pitch, turns } => HelixTraceLead::PitchAndTurns([pitch, turns]),
            Self::HeightAndTurns { height, turns } => {
                HelixTraceLead::HeightAndTurns([height, turns])
            }
            Self::PitchAndHeight { pitch, height } => {
                HelixTraceLead::PitchAndHeight([pitch, height])
            }
        }
    }

    fn resolve(self) -> Result<HelixResolved, HelixFailure> {
        let (pitch, height, turns) = match self {
            Self::PitchAndTurns { pitch, turns } => {
                let pitch = positive(pitch, Q::Pitch)?;
                let turns = positive(turns, Q::Turns)?;
                (pitch, derived_positive(pitch * turns, Q::Height)?, turns)
            }
            Self::HeightAndTurns { height, turns } => {
                let height = positive(height, Q::Height)?;
                let turns = positive(turns, Q::Turns)?;
                (derived_positive(height / turns, Q::Pitch)?, height, turns)
            }
            Self::PitchAndHeight { pitch, height } => {
                let pitch = positive(pitch, Q::Pitch)?;
                let height = positive(height, Q::Height)?;
                (pitch, height, derived_positive(height / pitch, Q::Turns)?)
            }
        };
        let parameter_end = derived_positive(TAU * turns, Q::ParameterEnd)?;
        Ok(HelixResolved {
            pitch,
            height,
            turns,
            parameter_end,
        })
    }
}

pub(super) fn positive(value: f64, quantity: Q) -> Result<f64, HelixFailure> {
    if value.is_finite() && value > 0. {
        return Ok(value);
    }
    Err(HelixFailure::InvalidScalar { quantity, value })
}

fn derived_positive(value: f64, quantity: Q) -> Result<f64, HelixFailure> {
    if value.is_finite() && value > 0. {
        return Ok(value);
    }
    Err(HelixFailure::DerivedScalar { quantity, value })
}

pub(super) fn place(frame: Axis2, [x, y, z]: [f64; 3], point: bool) -> [f64; 3] {
    let [i, j, k] = [frame.x_direction(), frame.y_direction(), frame.direction()]
        .map(|direction| *direction.coordinates());
    let origin = if point {
        *frame.location().coordinates()
    } else {
        [0.; 3]
    };
    std::array::from_fn(|axis| origin[axis] + x * i[axis] + y * j[axis] + z * k[axis])
}
