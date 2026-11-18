//! Authored IDs determine serialization order; the compound has no fabricated identity.
use super::*;
pub(in crate::surface_sketch::wrapped_profile) type Frozen =
    (Vec<u8>, Vec<BindingKey>, Shape, Vec<(BindingKey, Shape)>);
#[cfg(test)]
pub(super) fn construct(requests: &[Request]) -> Checked<Frozen> {
    let mut cancelled = |_| false;
    construct_with_control(requests, &mut Control::new(&mut cancelled))
}
#[cfg(test)]
pub(in crate::surface_sketch::wrapped_profile) fn construct_with_control(
    requests: &[Request],
    control: &mut Control<'_>,
) -> Checked<Frozen> {
    operation::frozen::construct(
        &operation::cylinder::CylinderStages {
            work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
            volume:
                martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit::Report,
        },
        requests,
        control,
    )
}
