//! Keep malformed source values and expected shapes visible at the API boundary.
use super::*;
use std::fmt::{Display, Formatter, Result};

impl Display for ProfileInputFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "wrapped input ")?;
        self.identity_message(formatter)?;
        write!(formatter, "; expected {}", self.expected())
    }
}
impl ProfileInputFailure {
    fn identity_message(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Plane(plane) => write!(f, "plane={plane:?}"),
            Self::DuplicateVertex(vertex) => write!(f, "duplicate vertex={vertex:?}"),
            Self::DuplicateCurve(curve) => write!(f, "duplicate curve={curve:?}"),
            Self::MissingVertex { curve, vertex } => {
                write!(f, "curve={curve:?}, missing vertex={vertex:?}")
            }
            Self::MissingCurve(curve) => write!(f, "missing curve={curve:?}"),
            Self::ReusedCurve(curve) => write!(f, "reused curve={curve:?}"),
            _ => self.representation_message(f),
        }
    }
    fn representation_message(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::EmptyLoop { region, contour } => {
                write!(f, "region={region:?}, contour={contour}, curve uses=0")
            }
            Self::Unsupported(curve) => write!(f, "unsupported curve={curve:?}"),
            Self::Spline { .. } => self.spline_message(f),
            Self::Weight { .. } | Self::Knot { .. } => self.coefficient_message(f),
            _ => self.endpoint_message(f),
        }
    }
    fn coefficient_message(&self, f: &mut Formatter<'_>) -> Result {
        let (curve, index, value) = match self {
            Self::Weight {
                curve,
                index,
                value,
            } => (curve, index, value),
            Self::Knot {
                curve,
                index,
                value,
            } => (curve, index, value),
            _ => return write!(f, "{self:?}"),
        };
        write!(
            f,
            "curve={curve:?}, coefficient index={index}, value={value}"
        )
    }
    fn spline_message(&self, f: &mut Formatter<'_>) -> Result {
        if let Self::Spline {
            curve,
            degree,
            poles,
            weights,
            knots,
        } = self
        {
            return write!(
                f,
                "curve={curve:?}, degree={degree}, poles={poles}, weights={weights:?}, knots={knots}"
            );
        }
        write!(f, "{self:?}")
    }
    fn endpoint_message(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Endpoint { .. } => self.control_message(f),
            Self::NonFinite {
                curve,
                control,
                point,
            } => {
                write!(f, "curve={curve:?}, control={control}, point={point:?}")
            }
            Self::UnknownSource(curve) => write!(f, "unselected source={curve:?}"),
            Self::Interval(span) => write!(f, "source span={span:?}"),
            Self::Point { .. } => self.point_message(f),
            _ => write!(f, "{self:?}"),
        }
    }
    fn point_message(&self, f: &mut Formatter<'_>) -> Result {
        if let Self::Point {
            curve,
            numerator,
            level,
        } = self
        {
            return write!(
                f,
                "curve={curve:?}, point numerator={numerator}, level={level}"
            );
        }
        write!(f, "{self:?}")
    }
    fn control_message(&self, f: &mut Formatter<'_>) -> Result {
        if let Self::Endpoint {
            curve,
            vertex,
            point,
            expected,
        } = self
        {
            return write!(
                f,
                "curve={curve:?}, vertex={vertex:?}, control={point:?}, vertex point={expected:?}"
            );
        }
        write!(f, "{self:?}")
    }
}
