//! Prove clipping is unchanged for every permitted shader rounding within the admitted hull.
use crate::{RenderViewportGridPrecision, window_depth_bounds::DepthShaderBound as Bound};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WindowDepthClosureFailure {
    OriginalClip,
    WindowRange,
    ReferenceGrid,
    Arithmetic,
    CorrectedClip,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WindowDepthClosure {
    RasterEmpty,
    ReferencePlane,
    RequiresActualValidation,
}

pub(crate) fn prove_window_depth_closure(
    clip: [[Bound; 4]; 3],
    viewport: [f32; 4],
    negative_one_to_one: bool,
    reference: RenderViewportGridPrecision,
    raster: RenderViewportGridPrecision,
) -> Result<WindowDepthClosure, WindowDepthClosureFailure> {
    prove_operation(
        clip,
        viewport,
        negative_one_to_one,
        reference,
        raster,
        false,
        false,
    )
}

pub(crate) fn prove_fragment_window_depth_closure(
    clip: [[Bound; 4]; 3],
    viewport: [f32; 4],
    negative: bool,
    reference: RenderViewportGridPrecision,
    raster: RenderViewportGridPrecision,
    identical_source_pair: bool,
) -> Result<WindowDepthClosure, WindowDepthClosureFailure> {
    prove_operation(
        clip,
        viewport,
        negative,
        reference,
        raster,
        true,
        identical_source_pair,
    )
}

fn prove_operation(
    clip: [[Bound; 4]; 3],
    viewport: [f32; 4],
    negative_one_to_one: bool,
    reference: RenderViewportGridPrecision,
    raster: RenderViewportGridPrecision,
    fragment: bool,
    identical_source_pair: bool,
) -> Result<WindowDepthClosure, WindowDepthClosureFailure> {
    require_original_clip(clip, negative_one_to_one)?;
    let window = projected_window(clip, viewport, negative_one_to_one)
        .ok_or(WindowDepthClosureFailure::Arithmetic)?;
    let physical = snapped(window, raster).ok_or(WindowDepthClosureFailure::WindowRange)?;
    if identical_source_pair || certified_grid_empty(physical, raster) {
        return Ok(WindowDepthClosure::RasterEmpty);
    }
    let reference = snapped(window, reference).ok_or(WindowDepthClosureFailure::WindowRange)?;
    let Some(gradient) = reference_gradient(reference) else {
        return if fragment {
            Ok(WindowDepthClosure::RequiresActualValidation)
        } else {
            Err(WindowDepthClosureFailure::ReferenceGrid)
        };
    };
    if fragment {
        if require_fragment_evaluation(reference[0], gradient, viewport).is_err() {
            return Ok(WindowDepthClosure::RequiresActualValidation);
        }
    } else {
        require_corrected_clip(clip, reference[0], gradient, physical, negative_one_to_one)?;
    }
    Ok(WindowDepthClosure::ReferencePlane)
}

fn require_fragment_evaluation(
    origin: [Bound; 3],
    gradient: [Bound; 2],
    viewport: [f32; 4],
) -> Result<(), WindowDepthClosureFailure> {
    let [Some(x), Some(y), Some(width), Some(height)] = viewport.map(Bound::source) else {
        return Err(WindowDepthClosureFailure::Arithmetic);
    };
    let right = x.add(width).ok_or(WindowDepthClosureFailure::Arithmetic)?;
    let bottom = y.add(height).ok_or(WindowDepthClosureFailure::Arithmetic)?;
    for sample in [[x, y], [right, y], [x, bottom], [right, bottom]] {
        fragment_sample(origin, gradient, sample).ok_or(WindowDepthClosureFailure::Arithmetic)?;
    }
    Ok(())
}

fn fragment_sample(origin: [Bound; 3], gradient: [Bound; 2], sample: [Bound; 2]) -> Option<Bound> {
    let x = gradient[0].multiply(sample[0].subtract(origin[0])?)?;
    let y = gradient[1].multiply(sample[1].subtract(origin[1])?)?;
    x.add(y)?.add(origin[2])
}

fn require_original_clip(
    clip: [[Bound; 4]; 3],
    negative: bool,
) -> Result<(), WindowDepthClosureFailure> {
    for [x, y, z, w] in clip {
        let minimum_w = w.endpoints()[0];
        let minimum_z = if negative { -minimum_w } else { 0. };
        if minimum_w > 0.
            && [x, y].iter().all(|axis| {
                let [low, high] = axis.endpoints();
                low.abs().max(high.abs()) <= minimum_w
            })
            && z.endpoints()[0] >= minimum_z
            && z.endpoints()[1] <= minimum_w
        {
            continue;
        }
        return Err(WindowDepthClosureFailure::OriginalClip);
    }
    Ok(())
}

fn projected_window(
    clip: [[Bound; 4]; 3],
    viewport: [f32; 4],
    negative: bool,
) -> Option<[[Bound; 3]; 3]> {
    Some([
        window_vertex(clip[0], viewport, negative)?,
        window_vertex(clip[1], viewport, negative)?,
        window_vertex(clip[2], viewport, negative)?,
    ])
}

fn window_vertex(
    [x, y, z, w]: [Bound; 4],
    viewport: [f32; 4],
    negative: bool,
) -> Option<[Bound; 3]> {
    let half = Bound::source(0.5)?;
    let window_x = x
        .divide(w)?
        .multiply(half)?
        .add(half)?
        .multiply(Bound::source(viewport[2])?)?
        .add(Bound::source(viewport[0])?)?;
    let window_y = y
        .divide(w)?
        .multiply(half)?
        .add(half)?
        .multiply(Bound::source(viewport[3])?)?
        .add(Bound::source(viewport[1])?)?;
    let depth = if negative {
        z.divide(w)?.multiply(half)?.add(half)?
    } else {
        z.divide(w)?
    };
    Some([window_x, window_y, depth])
}

fn snapped(
    mut vertices: [[Bound; 3]; 3],
    precision: RenderViewportGridPrecision,
) -> Option<[[Bound; 3]; 3]> {
    for position in &mut vertices {
        position[0] = position[0].snap(precision)?;
        position[1] = position[1].snap(precision)?;
    }
    Some(vertices)
}

fn certified_grid_empty(vertices: [[Bound; 3]; 3], precision: RenderViewportGridPrecision) -> bool {
    let scale = f64::from(1_u32 << precision.fractional_bits());
    let mut integer = [[0_i64; 2]; 3];
    for (target, source) in integer.iter_mut().zip(vertices) {
        for axis in 0..2 {
            let [low, high] = source[axis].endpoints();
            if low != high {
                return false;
            }
            target[axis] = (low * scale) as i64;
        }
    }
    let [a, b, c] = integer;
    (b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1]) == 0
}

fn subtract_vertex(left: [Bound; 3], right: [Bound; 3]) -> Option<[Bound; 3]> {
    Some([
        left[0].subtract(right[0])?,
        left[1].subtract(right[1])?,
        left[2].subtract(right[2])?,
    ])
}

fn cross(a: Bound, b: Bound, c: Bound, d: Bound) -> Option<Bound> {
    a.multiply(b)?.subtract(c.multiply(d)?)
}

fn reference_gradient(vertices: [[Bound; 3]; 3]) -> Option<[Bound; 2]> {
    let first = subtract_vertex(vertices[1], vertices[0])?;
    let second = subtract_vertex(vertices[2], vertices[0])?;
    let determinant = cross(first[0], second[1], second[0], first[1])?;
    if !determinant.excludes_zero() {
        return None;
    }
    Some([
        cross(first[2], second[1], second[2], first[1])?.divide(determinant)?,
        cross(first[0], second[2], second[0], first[2])?.divide(determinant)?,
    ])
}

fn corrected_depth(origin: [Bound; 3], gradient: [Bound; 2], target: [Bound; 3]) -> Option<Bound> {
    let dx = target[0].subtract(origin[0])?;
    let dy = target[1].subtract(origin[1])?;
    gradient[0]
        .multiply(dx)?
        .add(gradient[1].multiply(dy)?)?
        .add(origin[2])
}

fn require_corrected_clip(
    clip: [[Bound; 4]; 3],
    origin: [Bound; 3],
    gradient: [Bound; 2],
    physical: [[Bound; 3]; 3],
    negative: bool,
) -> Result<(), WindowDepthClosureFailure> {
    for (position, target) in clip.into_iter().zip(physical) {
        let depth = corrected_depth(origin, gradient, target)
            .ok_or(WindowDepthClosureFailure::Arithmetic)?;
        let normalized = if negative {
            depth
                .multiply(Bound::source(2.).unwrap())
                .and_then(|v| v.subtract(Bound::source(1.).unwrap()))
        } else {
            Some(depth)
        }
        .ok_or(WindowDepthClosureFailure::Arithmetic)?;
        let corrected = normalized
            .multiply(position[3])
            .ok_or(WindowDepthClosureFailure::Arithmetic)?;
        let [low, high] = corrected.endpoints();
        let minimum_w = position[3].endpoints()[0];
        if low < if negative { -minimum_w } else { 0. } || high > minimum_w {
            return Err(WindowDepthClosureFailure::CorrectedClip);
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "window_depth_closure_tests.rs"]
mod tests;
