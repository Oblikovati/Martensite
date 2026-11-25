//! Bound the normal map over the source, reference, and paths between their points.
use super::*;

pub(super) enum Support {
    Cylinder {
        radius: f64,
        depth: f64,
    },
    Cone {
        x_lower: f64,
        radius_lower: f64,
        depth: f64,
    },
}

pub(super) fn cylinder(request: &WrappedNormalRequest) -> Checked<Support> {
    let depth = validate_request(request.depths, request.options.spatial_error)?;
    let radius = request.support.radius;
    check_radius(radius, depth)?;
    Ok(Support::Cylinder { radius, depth })
}

pub(super) fn cone(
    request: &WrappedConeNormalRequest,
    source: &input::Source<'_>,
) -> Checked<Support> {
    let depth = validate_request(request.depths, request.options.spatial_error)?;
    let angle = request.support.semi_angle;
    if !(angle > 0. && angle < std::f64::consts::FRAC_PI_2) {
        return Err(invalid(
            format!("semi-angle={angle}"),
            "a positive acute semi-angle",
        ));
    }
    let hull = Hull::capture(source, &[]);
    // Half the source separation leaves room for the cubic controls. On the
    // acute interval sin(alpha) >= 2 alpha/pi > alpha/2.
    let x_lower = (hull.lower[0] * 0.5).next_down();
    let radius_lower = (x_lower * (angle * 0.5).next_down()).next_down();
    check_radius(radius_lower, depth)?;
    Ok(Support::Cone {
        x_lower,
        radius_lower,
        depth,
    })
}

fn validate_request(depths: [f64; 2], spatial: f64) -> Checked<f64> {
    let [first, last] = depths;
    if !(first.is_finite()
        && last.is_finite()
        && first < last
        && spatial.is_finite()
        && spatial > 0.)
    {
        return Err(invalid(
            format!("depths={depths:?}, spatial limit={spatial}"),
            "a finite positive spatial limit and finite increasing depths",
        ));
    }
    Ok(first.abs().max(last.abs()))
}

fn check_radius(radius: f64, depth: f64) -> Checked<()> {
    if !radius.is_finite() || radius <= depth {
        return Err(invalid(
            format!("physical radius lower={radius}, signed depth magnitude={depth}"),
            "a finite physical radius strictly above both signed depth magnitudes",
        ));
    }
    Ok(())
}

impl Support {
    pub(super) fn scale(&self) -> f64 {
        let (radius, depth) = match *self {
            Self::Cylinder { radius, depth } => (radius, depth),
            Self::Cone {
                radius_lower,
                depth,
                ..
            } => (radius_lower, depth),
        };
        // Both developed charts are isometric at depth zero. At depth d the
        // angular tangent scales by 1 + d cos(alpha)/rho for the cone and
        // 1 + d/radius for the cylinder. The other tangent is unchanged.
        (1. + (depth / radius).next_up()).next_up()
    }

    pub(super) fn inverse_slant_error(&self, spans: &[RationalReferenceSpan]) -> f64 {
        match *self {
            Self::Cylinder { .. } => 0.,
            Self::Cone { x_lower, .. } => measure::inverse_slant_error(x_lower, spans),
        }
    }

    pub(super) fn check(
        &self,
        source: &input::Source<'_>,
        spans: &[RationalReferenceSpan],
    ) -> Checked<()> {
        let hull = Hull::capture(source, spans);
        let width = match *self {
            Self::Cylinder { radius, .. } => (hull.width(0) / radius).next_up(),
            Self::Cone {
                x_lower,
                radius_lower,
                ..
            } => cone_width(hull, x_lower, radius_lower)?,
        };
        if width >= 3. || !width.is_finite() {
            return Err(invalid(
                format!("developed angular width upper={width}"),
                "a supported single chart narrower than three physical radians",
            ));
        }
        Ok(())
    }
}

fn cone_width(hull: Hull, x_lower: f64, radius_lower: f64) -> Checked<f64> {
    if hull.lower[0] < x_lower {
        return Err(invalid(
            format!("developed chart x lower={}", hull.lower[0]),
            "source and reference controls within the certified positive cone halfplane",
        ));
    }
    // The gradient of developed argument has length 1/r. Dividing by
    // sin(alpha) converts its variation to the physical azimuth.
    let diameter = (hull.width(0) + hull.width(1)).next_up();
    Ok((diameter / radius_lower).next_up())
}

struct Hull {
    lower: [f64; 2],
    upper: [f64; 2],
}

impl Hull {
    fn capture(source: &input::Source<'_>, spans: &[RationalReferenceSpan]) -> Self {
        let points = source
            .poles()
            .chain(source.lines.iter().flat_map(|line| line.points))
            .chain(spans.iter().flat_map(|span| span.poles));
        let mut hull = Self {
            lower: [f64::INFINITY; 2],
            upper: [f64::NEG_INFINITY; 2],
        };
        for point in points {
            hull.lower = std::array::from_fn(|i| hull.lower[i].min(point[i]));
            hull.upper = std::array::from_fn(|i| hull.upper[i].max(point[i]));
        }
        hull
    }

    fn width(&self, axis: usize) -> f64 {
        (self.upper[axis] - self.lower[axis]).next_up()
    }
}
