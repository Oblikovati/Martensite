//! Rebind the certified local +Z chart to the exact native cylinder and edge frames.
use super::super::edge_bezier_boundary::literal_translation;
use super::CylinderMetricSource;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomSurface};
use martensite_tkmath::gp::Transform3;

struct Frame {
    origin: [f64; 3],
    radius: f64,
    height_sign: f64,
}

pub(super) struct Datum {
    pub radius: f64,
    pub height_sign: f64,
}

pub(super) fn admitted(source: &CylinderMetricSource<'_>) -> Option<Datum> {
    if !literal_translation(source.reference_transform)
        || !literal_translation(source.surface_transform)
    {
        return None;
    }
    let raw = frame(source.surface)?;
    let moved = frame(source.moved_surface)?;
    let origin = exact_origin(raw.origin, source.surface_transform)?;
    let rail_origin = source.reference_transform.translation_part();
    if origin != *rail_origin.coordinates()
        || origin != moved.origin
        || raw.radius != moved.radius
        || raw.height_sign != moved.height_sign
    {
        return None;
    }
    Some(Datum {
        radius: raw.radius,
        height_sign: raw.height_sign,
    })
}

fn frame(surface: &Handle<dyn GeomSurface>) -> Option<Frame> {
    let surface = surface.downcast::<GeomCylindricalSurface>();
    let cylinder = surface.get()?.cylinder().ok()?;
    let axes = cylinder.position();
    let [x, y, z] = *axes.direction().coordinates();
    if x != 0.
        || y != 0.
        || ![1., -1.].contains(&z)
        || *axes.x_direction().coordinates() != [1., 0., 0.]
        || *axes.y_direction().coordinates() != [0., 1., 0.]
    {
        return None;
    }
    Some(Frame {
        origin: *axes.location().coordinates(),
        radius: cylinder.radius(),
        height_sign: z,
    })
}

fn exact_origin(raw: [f64; 3], location: Transform3) -> Option<[f64; 3]> {
    let mut moved = raw;
    for (coord, &offset) in moved
        .iter_mut()
        .zip(location.translation_part().coordinates())
    {
        let (sum, residual) = add_residual(*coord, offset)?;
        if residual != 0. {
            return None;
        }
        *coord = sum;
    }
    Some(moved)
}

pub(super) fn pole_shift_error(raw: [f64; 3], moved: [f64; 3], offset: [f64; 3]) -> Option<f64> {
    raw.into_iter()
        .zip(moved)
        .zip(offset)
        .try_fold(0., |error, ((raw, moved), offset)| {
            let (sum, residual) = add_residual(raw, offset)?;
            if sum != moved {
                return None;
            }
            Some((error + residual.abs()).next_up())
        })
}

fn add_residual(first: f64, second: f64) -> Option<(f64, f64)> {
    let (big, small) = if first.abs() >= second.abs() {
        (first, second)
    } else {
        (second, first)
    };
    let sum = big + small;
    // With the larger operand first, FastTwoSum gives the exact discarded bits.
    sum.is_finite().then(|| (sum, small - (sum - big)))
}
