//! Operation-local bounds for the depth shader; this is not a device-rasterization model.
use crate::RenderViewportGridPrecision;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DepthShaderBound {
    minimum: f64,
    maximum: f64,
}

impl DepthShaderBound {
    pub(crate) fn source(value: f32) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let actual = f64::from(value);
        let may_flush = value != 0. && value.abs() < f32::MIN_POSITIVE;
        Some(Self {
            minimum: if may_flush { actual.min(0.) } else { actual },
            maximum: if may_flush { actual.max(0.) } else { actual },
        })
    }

    pub(crate) fn endpoints(self) -> [f64; 2] {
        [self.minimum, self.maximum]
    }

    pub(crate) fn add(self, other: Self) -> Option<Self> {
        // Finite add-zero is exact under every rounding mode; source/operation hulls already cover FTZ.
        if other.is_point(0.) {
            return Some(self);
        }
        if self.is_point(0.) {
            return Some(other);
        }
        Self::rounded(
            self.minimum + other.minimum,
            self.maximum + other.maximum,
            1.,
        )
    }

    pub(crate) fn subtract(self, other: Self) -> Option<Self> {
        if other.is_point(0.) {
            return Some(self);
        }
        Self::rounded(
            self.minimum - other.maximum,
            self.maximum - other.minimum,
            1.,
        )
    }

    pub(crate) fn multiply(self, other: Self) -> Option<Self> {
        if let Some(exact) = self.identity_product(other) {
            return Some(exact);
        }
        let values = [
            self.minimum * other.minimum,
            self.minimum * other.maximum,
            self.maximum * other.minimum,
            self.maximum * other.maximum,
        ];
        Self::rounded(
            values.into_iter().fold(f64::INFINITY, f64::min),
            values.into_iter().fold(f64::NEG_INFINITY, f64::max),
            1.,
        )
    }

    pub(crate) fn divide(self, other: Self) -> Option<Self> {
        if other.minimum <= 0. && other.maximum >= 0. {
            return None;
        }
        let minimum_absolute = other.minimum.abs().min(other.maximum.abs());
        if minimum_absolute < f64::from(f32::MIN_POSITIVE)
            || other.minimum.abs().max(other.maximum.abs()) > martensite_cmath::powi(2_f64, 126)
        {
            return None;
        }
        let values = [
            self.minimum / other.minimum,
            self.minimum / other.maximum,
            self.maximum / other.minimum,
            self.maximum / other.maximum,
        ];
        // Vulkan's normal-denominator FDiv permits 2.5 ULP, unlike add/multiply.
        Self::rounded(
            values.into_iter().fold(f64::INFINITY, f64::min),
            values.into_iter().fold(f64::NEG_INFINITY, f64::max),
            3.,
        )
    }

    pub(crate) fn snap(self, precision: RenderViewportGridPrecision) -> Option<Self> {
        let scale = f64::from(1_u32 << precision.fractional_bits());
        if self.minimum.abs().max(self.maximum.abs()) * scale > f64::from((1_u32 << 24) - 1) {
            return None;
        }
        Some(Self {
            minimum: precision.snap(self.minimum).ok()?,
            maximum: precision.snap(self.maximum).ok()?,
        })
    }

    pub(crate) fn excludes_zero(self) -> bool {
        self.minimum > 0. || self.maximum < 0.
    }

    fn is_point(self, value: f64) -> bool {
        self.minimum == value && self.maximum == value
    }

    fn identity_product(self, other: Self) -> Option<Self> {
        // These products are exact in binary32; input bounds already include optional FTZ.
        if self.is_point(0.) || other.is_point(0.) {
            return Self::source(0.);
        }
        if self.is_point(1.) {
            return Some(other);
        }
        other.is_point(1.).then_some(self)
    }

    fn rounded(minimum: f64, maximum: f64, ulps: f64) -> Option<Self> {
        let magnitude = minimum.abs().max(maximum.abs());
        let radius =
            (magnitude * f64::from(f32::EPSILON) * ulps + f64::from(f32::MIN_POSITIVE)).next_up();
        let minimum = (minimum.next_down() - radius).next_down();
        let maximum = (maximum.next_up() + radius).next_up();
        if !minimum.is_finite()
            || !maximum.is_finite()
            || minimum > maximum
            || minimum < -f64::from(f32::MAX)
            || maximum > f64::from(f32::MAX)
        {
            return None;
        }
        Some(Self { minimum, maximum })
    }
}

pub(crate) fn clip_transform_bounds(
    matrix: [f32; 16],
    vertex: crate::RenderVertex,
) -> Option<[DepthShaderBound; 4]> {
    Some([
        clip_row_bound(matrix, vertex, 0)?,
        clip_row_bound(matrix, vertex, 1)?,
        clip_row_bound(matrix, vertex, 2)?,
        clip_row_bound(matrix, vertex, 3)?,
    ])
}

fn clip_row_bound(
    matrix: [f32; 16],
    vertex: crate::RenderVertex,
    row: usize,
) -> Option<DepthShaderBound> {
    let x = DepthShaderBound::source(vertex.position[0])?;
    let y = DepthShaderBound::source(vertex.position[1])?;
    let z = DepthShaderBound::source(vertex.position[2])?;
    let weight = DepthShaderBound::source(vertex.weight)?;
    let first = DepthShaderBound::source(matrix[4 + row])?.multiply(y)?;
    let second = DepthShaderBound::source(matrix[row])?.multiply(x)?;
    let third = DepthShaderBound::source(matrix[8 + row])?.multiply(z)?;
    let fourth = DepthShaderBound::source(matrix[12 + row])?.multiply(weight)?;
    first.add(second)?.add(third)?.add(fourth)
}

#[cfg(test)]
#[path = "window_depth_bounds_tests.rs"]
mod tests;
