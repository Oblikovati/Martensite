//! The closest ExtPS root is selected exactly as GetPointOn2S in the source.
use super::super::Side;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkgeombase::extrema::{ExtFlag, ExtPS, POnSurf};
use martensite_tkmath::gp::Point3;

pub(super) fn projector(side: Side<'_>, tolerance: f64) -> Result<ExtPS<'static>, Failure> {
    let mut projector = ExtPS::new();
    projector.initialize_shared(
        &side.surface.cast(),
        (
            side.surface.first_u_parameter()?,
            side.surface.last_u_parameter()?,
        ),
        (
            side.surface.first_v_parameter()?,
            side.surface.last_v_parameter()?,
        ),
        (tolerance, tolerance),
    )?;
    projector.set_flag(ExtFlag::Min);
    Ok(projector)
}

pub(super) fn closest(
    projector: &mut ExtPS<'_>,
    point: Point3,
    tolerance: f64,
) -> Result<Option<POnSurf>, Failure> {
    projector.perform(point)?;
    if !projector.is_done() || projector.nb_ext()? == 0 {
        return Ok(None);
    }
    let (mut distance, mut index) = (1e200, 1);
    for current in 1..=projector.nb_ext()? {
        let square = projector.square_distance(current)?;
        if square < distance {
            distance = square;
            index = current;
        }
    }
    if distance <= tolerance * tolerance {
        return projector.point(index).map(Some);
    }
    Ok(None)
}
