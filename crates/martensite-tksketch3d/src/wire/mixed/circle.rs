// SPDX-License-Identifier: GPL-2.0-only
//! Exact authored quarter-circle domain and its analytic terminal tube.
use super::{LineDirection, WireFailure};
use crate::{ArcDefinition, ArcGeometry, PointId};
use martensite_tkmath::{
    gp::{Direction3, Point3, Vector3},
    predicates::roots::metric_bounds::{
        BINARY64_QUARTER_ANGLE_ERROR, cardinal_quarter_terminal_bound, exact_affine_coordinate,
    },
};
use std::f64::consts::FRAC_PI_2;

/// Why an authored arc could not prove the first mixed-wire circle domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireArcScope {
    /// The input does not identify an original center control.
    CenterControl,
    /// The radial axes and normal do not define an exact cardinal frame.
    Frame,
    /// The original retained interval does not encode a positive quarter turn.
    Range,
    /// The authored points are inconsistent with the exact cardinal carrier.
    Placement,
    /// The certified native trim deviation has no finite enclosure.
    Unrepresentable,
}

/// Detached proof of an original center-defined cardinal quarter arc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardinalWireArc {
    authored: ArcGeometry,
    center: PointId,
    terminal_error_upper: f64,
}

impl CardinalWireArc {
    /// Read the immutable original geometry. Example: `arc.authored().range()`.
    pub const fn authored(self) -> ArcGeometry {
        self.authored
    }

    /// Original center-control identity. Example: `arc.center()`.
    pub const fn center(self) -> PointId {
        self.center
    }

    /// Certified upper position deviation across the normalized real quarter and stored trim.
    /// Example: `arc.terminal_error_upper()`.
    pub const fn terminal_error_upper(self) -> f64 {
        self.terminal_error_upper
    }

    /// Ideal signed tangents for the original or reversed traversal.
    /// Example: `arc.tangents(LineDirection::Reversed)`.
    pub fn tangents(self, direction: LineDirection) -> [Direction3; 2] {
        let frame = self.authored.frame();
        match direction {
            LineDirection::Forward => [frame.y_direction(), frame.x_direction().reversed()],
            LineDirection::Reversed => [frame.x_direction(), frame.y_direction().reversed()],
        }
    }

    pub(super) fn points(self) -> [Point3; 2] {
        let [first, last, _] = self.authored.points();
        [first, last]
    }

    pub(super) fn tangent_errors(direction: LineDirection) -> [f64; 2] {
        match direction {
            LineDirection::Forward => [0., BINARY64_QUARTER_ANGLE_ERROR],
            LineDirection::Reversed => [BINARY64_QUARTER_ANGLE_ERROR, 0.],
        }
    }

    pub(super) fn capture(authored: ArcGeometry) -> Result<Self, WireFailure> {
        let arc = authored.recipe().id;
        let refuse = |cause| WireFailure::ArcDomain { arc, cause };
        let ArcDefinition::Center { center, normal } = authored.recipe().definition else {
            return Err(refuse(WireArcScope::CenterControl));
        };
        check_frame(authored, normal).map_err(refuse)?;
        if authored.range() != [0., FRAC_PI_2] {
            return Err(refuse(WireArcScope::Range));
        }
        check_placement(authored).map_err(refuse)?;
        let bound = cardinal_quarter_terminal_bound(authored.radius())
            .map_err(|_| refuse(WireArcScope::Unrepresentable))?;
        let limit = authored
            .recipe()
            .fit_tolerance
            .value()
            .min(martensite_tkernel::precision::confusion());
        if bound > limit {
            return Err(WireFailure::NativeArcErrorBound { arc, bound, limit });
        }
        Ok(Self {
            authored,
            center,
            terminal_error_upper: bound,
        })
    }
}

fn check_frame(arc: ArcGeometry, normal: Direction3) -> Result<(), WireArcScope> {
    let frame = arc.frame();
    let [x, y] = [frame.x_direction(), frame.y_direction()];
    let cross = Vector3::from(x).crossed(Vector3::from(y));
    if !cardinal(x)
        || !cardinal(y)
        || frame.direction() != normal
        || [cross.x(), cross.y(), cross.z()] != *normal.coordinates()
    {
        return Err(WireArcScope::Frame);
    }
    Ok(())
}

fn cardinal(direction: Direction3) -> bool {
    let coords = direction.coordinates();
    coords.iter().filter(|c| c.abs() == 1.).count() == 1
        && coords.iter().all(|c| matches!(*c, -1. | 0. | 1.))
}

fn check_placement(arc: ArcGeometry) -> Result<(), WireArcScope> {
    let [a, b, center] = arc.points();
    let frame = arc.frame();
    if frame.location() != center {
        return Err(WireArcScope::Placement);
    }
    check_endpoint(a, center, frame.x_direction(), arc.radius())?;
    check_endpoint(b, center, frame.y_direction(), arc.radius())
}

fn check_endpoint(
    point: Point3,
    center: Point3,
    radial: Direction3,
    radius: f64,
) -> Result<(), WireArcScope> {
    for ((&p, &c), &d) in point
        .coordinates()
        .iter()
        .zip(center.coordinates())
        .zip(radial.coordinates())
    {
        if exact_affine_coordinate(c, d, radius).ok().flatten() != Some(p) {
            return Err(WireArcScope::Placement);
        }
    }
    Ok(())
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(CardinalWireArc, WireArcScope)>();
};
