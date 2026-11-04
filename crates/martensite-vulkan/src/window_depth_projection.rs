//! Admission for an actual homogeneous triangle, before any command can change its depth.
use crate::{RenderViewportGridPrecision, window_depth_plane::*};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum WindowDepthProjectionFailure {
    Viewport {
        words: [u32; 4],
    },
    Clip {
        vertex: usize,
        words: [u32; 4],
    },
    WindowRange {
        vertex: usize,
        precision: RenderViewportGridPrecision,
    },
    Plane(WindowDepthPlaneFailure),
    CorrectedClip {
        vertex: usize,
        word: u32,
    },
}

pub(crate) fn compensated_clip_triangle(
    clip: [[f32; 4]; 3],
    viewport: [f32; 4],
    negative_one_to_one: bool,
    reference: RenderViewportGridPrecision,
    raster: RenderViewportGridPrecision,
) -> Result<[[f32; 4]; 3], WindowDepthProjectionFailure> {
    require_viewport(viewport)?;
    require_clip_triangle(clip, negative_one_to_one)?;
    let window = window_triangle(clip, viewport, negative_one_to_one);
    require_grid_range(window, reference)?;
    require_grid_range(window, raster)?;
    let reference_vertices =
        snapped_window_vertices(window, reference).map_err(WindowDepthProjectionFailure::Plane)?;
    let plane = WindowDepthPlane::through(reference_vertices)
        .map_err(WindowDepthProjectionFailure::Plane)?;
    let corrected = plane
        .compensated_vertices(window, raster)
        .map_err(WindowDepthProjectionFailure::Plane)?;
    corrected_clip_depths(clip, corrected, negative_one_to_one)
}

fn require_viewport(viewport: [f32; 4]) -> Result<(), WindowDepthProjectionFailure> {
    if viewport.iter().all(|value| value.is_finite()) && viewport[2] != 0. && viewport[3] != 0. {
        return Ok(());
    }
    Err(WindowDepthProjectionFailure::Viewport {
        words: viewport.map(f32::to_bits),
    })
}

fn require_clip_triangle(
    clip: [[f32; 4]; 3],
    negative_one_to_one: bool,
) -> Result<(), WindowDepthProjectionFailure> {
    for (vertex, position) in clip.into_iter().enumerate() {
        let [x, y, z, w] = position;
        let minimum_z = if negative_one_to_one { -w } else { 0. };
        if position.iter().all(|v| v.is_finite())
            && w > 0.
            && x.abs() <= w
            && y.abs() <= w
            && z >= minimum_z
            && z <= w
        {
            continue;
        }
        return Err(WindowDepthProjectionFailure::Clip {
            vertex,
            words: position.map(f32::to_bits),
        });
    }
    Ok(())
}

fn window_triangle(
    clip: [[f32; 4]; 3],
    viewport: [f32; 4],
    negative_one_to_one: bool,
) -> [[f64; 3]; 3] {
    clip.map(|[x, y, z, w]| {
        let window_x = ((x / w) * 0.5 + 0.5) * viewport[2] + viewport[0];
        let window_y = ((y / w) * 0.5 + 0.5) * viewport[3] + viewport[1];
        let depth = if negative_one_to_one {
            (z / w) * 0.5 + 0.5
        } else {
            z / w
        };
        [f64::from(window_x), f64::from(window_y), f64::from(depth)]
    })
}

fn require_grid_range(
    window: [[f64; 3]; 3],
    precision: RenderViewportGridPrecision,
) -> Result<(), WindowDepthProjectionFailure> {
    let maximum = f64::from((1_u32 << 24) - 1) / f64::from(1_u32 << precision.fractional_bits());
    for (vertex, position) in window.into_iter().enumerate() {
        if position.iter().all(|value| value.is_finite())
            && position[0].abs() <= maximum
            && position[1].abs() <= maximum
        {
            continue;
        }
        return Err(WindowDepthProjectionFailure::WindowRange { vertex, precision });
    }
    Ok(())
}

fn corrected_clip_depths(
    mut clip: [[f32; 4]; 3],
    corrected: [[f64; 3]; 3],
    negative_one_to_one: bool,
) -> Result<[[f32; 4]; 3], WindowDepthProjectionFailure> {
    for (vertex, (position, window)) in clip.iter_mut().zip(corrected).enumerate() {
        let depth = window[2] as f32;
        let normalized = if negative_one_to_one {
            depth * 2. - 1.
        } else {
            depth
        };
        let candidate = normalized * position[3];
        let minimum = if negative_one_to_one {
            -position[3]
        } else {
            0.
        };
        if !candidate.is_finite() || candidate < minimum || candidate > position[3] {
            return Err(WindowDepthProjectionFailure::CorrectedClip {
                vertex,
                word: candidate.to_bits(),
            });
        }
        position[2] = candidate;
    }
    Ok(clip)
}

#[cfg(test)]
#[path = "window_depth_projection_tests.rs"]
mod tests;
