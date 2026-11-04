//! Independent reference algebra for a window-depth plane before physical raster snapping.
//! This module does not select a device policy or infer its fixed-point precision.
use crate::{RenderViewportGridFailure, RenderViewportGridPrecision};

/// A refusal preserves the exact offending coordinate or precision.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum WindowDepthPlaneFailure {
    Grid(RenderViewportGridFailure),
    NonFinite {
        vertex: usize,
        component: usize,
        actual: f64,
    },
    NonFiniteCoefficient {
        component: usize,
        actual: f64,
    },
    Degenerate {
        determinant: f64,
    },
}

/// Affine depth in window coordinates; coverage and perspective varyings are independent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WindowDepthPlane {
    coefficients: [f64; 3],
}

impl WindowDepthPlane {
    pub(crate) fn through(vertices: [[f64; 3]; 3]) -> Result<Self, WindowDepthPlaneFailure> {
        require_finite(vertices)?;
        let [origin, second, third] = vertices;
        let first_delta = subtract(second, origin);
        let second_delta = subtract(third, origin);
        let determinant = cross(first_delta, second_delta, 0, 1);
        if determinant == 0. || !determinant.is_finite() {
            return Err(WindowDepthPlaneFailure::Degenerate { determinant });
        }
        let dx = cross(first_delta, second_delta, 2, 1) / determinant;
        let dy = cross(first_delta, second_delta, 0, 2) / determinant;
        let intercept = origin[2] - dx * origin[0] - dy * origin[1];
        Self::from_coefficients([dx, dy, intercept])
    }

    fn from_coefficients(coefficients: [f64; 3]) -> Result<Self, WindowDepthPlaneFailure> {
        for (component, actual) in coefficients.iter().copied().enumerate() {
            if !actual.is_finite() {
                return Err(WindowDepthPlaneFailure::NonFiniteCoefficient { component, actual });
            }
        }
        Ok(Self { coefficients })
    }

    pub(crate) fn evaluate(self, position: [f64; 2]) -> f64 {
        let [dx, dy, intercept] = self.coefficients;
        dx * position[0] + dy * position[1] + intercept
    }

    pub(crate) fn maximum_slope(self) -> f64 {
        self.coefficients[0].abs().max(self.coefficients[1].abs())
    }

    pub(crate) fn compensated_vertices(
        self,
        vertices: [[f64; 3]; 3],
        precision: RenderViewportGridPrecision,
    ) -> Result<[[f64; 3]; 3], WindowDepthPlaneFailure> {
        let snapped = snapped_window_vertices(vertices, precision)?;
        let compensated = snapped.map(|[x, y, _]| [x, y, self.evaluate([x, y])]);
        Self::through(compensated)?;
        Ok(compensated)
    }
}

pub(crate) fn snapped_window_vertices(
    vertices: [[f64; 3]; 3],
    precision: RenderViewportGridPrecision,
) -> Result<[[f64; 3]; 3], WindowDepthPlaneFailure> {
    require_finite(vertices)?;
    let mut snapped = vertices;
    for position in &mut snapped {
        position[0] = precision
            .snap(position[0])
            .map_err(WindowDepthPlaneFailure::Grid)?;
        position[1] = precision
            .snap(position[1])
            .map_err(WindowDepthPlaneFailure::Grid)?;
    }
    Ok(snapped)
}

fn require_finite(vertices: [[f64; 3]; 3]) -> Result<(), WindowDepthPlaneFailure> {
    for (vertex, position) in vertices.iter().enumerate() {
        for (component, actual) in position.iter().copied().enumerate() {
            if !actual.is_finite() {
                return Err(WindowDepthPlaneFailure::NonFinite {
                    vertex,
                    component,
                    actual,
                });
            }
        }
    }
    Ok(())
}

fn subtract(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|component| left[component] - right[component])
}

fn cross(left: [f64; 3], right: [f64; 3], first: usize, second: usize) -> f64 {
    left[first] * right[second] - left[second] * right[first]
}

#[cfg(test)]
#[path = "window_depth_plane_tests.rs"]
mod tests;
