//! Polygon points retain triangle clipping, winding, attributes and the face shader.
use crate::{PrimitiveDraw, PrimitiveTopology, RenderError};
use martensite_tkservice::{
    aspect::AspectInteriorStyle,
    graphic3d::{Graphic3dAspectsMembers, Graphic3dAspectsSource},
};
use std::fmt;

/// A requested polygon-point raster mode was unavailable; inspect `error.polygon_point_failure()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderPolygonPointFailure {
    /// The requested device feature is absent; `None` means no portability subset extension.
    DeviceCapabilities {
        /// The selected physical device's core non-solid polygon feature.
        fill_mode_non_solid: bool,
        /// The advertised subset's queried point-polygons support.
        subset_point_polygons: Option<bool>,
    },
    /// A registered custom program has no independently established polygon-point size contract.
    CustomProgramPointSizeUndefined {
        /// The registered program whose polygon-point size behavior has not been established.
        key: crate::user_program::UserProgramKey,
    },
}
impl fmt::Display for RenderPolygonPointFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeviceCapabilities {
                fill_mode_non_solid,
                subset_point_polygons,
            } => write!(
                output,
                "polygon-point raster requested with fillModeNonSolid={fill_mode_non_solid}, subsetPointPolygons={subset_point_polygons:?}; expected enabled fillModeNonSolid and supported pointPolygons when a subset is advertised"
            ),
            Self::CustomProgramPointSizeUndefined { key } => write!(
                output,
                "polygon-point raster requested for custom program {key:?}; expected an independently established polygon-point PointSize contract"
            ),
        }
    }
}
impl std::error::Error for RenderPolygonPointFailure {}

pub(crate) fn require_polygon_points(
    requested: bool,
    fill_mode_non_solid: bool,
    subset_point_polygons: Option<bool>,
) -> Result<(), RenderError> {
    if requested && (!fill_mode_non_solid || subset_point_polygons == Some(false)) {
        return Err(RenderPolygonPointFailure::DeviceCapabilities {
            fill_mode_non_solid,
            subset_point_polygons,
        }
        .into());
    }
    Ok(())
}

pub(crate) fn apply_polygon_points(
    draws: &mut [PrimitiveDraw],
    aspect: &dyn Graphic3dAspectsSource,
) -> Result<(), RenderError> {
    if aspect.interior_style() != AspectInteriorStyle::Point {
        return Ok(());
    }
    for draw in draws {
        // Polygon edge passes restore their own LINE mode in the native renderer.
        if draw.topology() == PrimitiveTopology::Triangles && !draw.is_polygon_edge_pass() {
            require_polygon_point_program(draw, true)?;
            draw.polygon_points = true;
        }
    }
    Ok(())
}

pub(crate) fn require_polygon_point_program(
    draw: &PrimitiveDraw,
    requested: bool,
) -> Result<(), RenderError> {
    if !requested {
        return Ok(());
    }
    let Some(program) = &draw.user_program else {
        return Ok(());
    };
    Err(RenderPolygonPointFailure::CustomProgramPointSizeUndefined {
        key: program.modules.key,
    }
    .into())
}

#[cfg(test)]
#[path = "polygon_point_raster_tests.rs"]
mod tests;
