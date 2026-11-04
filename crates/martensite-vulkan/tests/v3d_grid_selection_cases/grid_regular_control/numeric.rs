//! Public numeric getters preserve bits that human-readable dump formatting rounds away.
use super::*;
use martensite_tkservice::aspect::AspectRectangularGridMembers;
/// Keep exact public grid values; `grid_bits(role)`.
pub(super) fn grid_bits(role: &role::GridRole) -> [u64; 8] {
    let rect: Handle<dyn martensite_tkservice::aspect::AspectRectangularGrid> = role.grid.cast();
    [
        role.grid.x_origin(),
        role.grid.y_origin(),
        role.grid.rotation_angle(),
        rect.x_step(),
        rect.y_step(),
        rect.size_x(),
        rect.size_y(),
        rect.z_offset(),
    ]
    .map(f64::to_bits)
}
/// Keep the privileged plane without dump rounding; `plane_bits(session)`.
pub(super) fn plane_bits(session: &ViewerSession) -> [u64; 12] {
    let plane = session.view().viewer().privileged_plane();
    let point = plane.location();
    let x = plane.x_direction();
    let y = plane.y_direction();
    let normal = plane.direction();
    let triples = [
        [point.x(), point.y(), point.z()],
        [x.x(), x.y(), x.z()],
        [y.x(), y.y(), y.z()],
        [normal.x(), normal.y(), normal.z()],
    ];
    triples
        .concat()
        .into_iter()
        .map(f64::to_bits)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
