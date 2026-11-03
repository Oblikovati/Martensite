//! Every stored representation remains present and in its declared list position.
use super::{registry::FixtureOwners, types::*};
use martensite_tkbrep::brep::{
    BRepCurve3D, BRepCurveOnClosedSurface, BRepCurveOnSurface, BRepCurveRepresentation, BRepGCurve,
};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::{geom_abs::Shape as Continuity, gp::Point2};
pub(super) fn construct(input: &GraphInput, owners: &mut FixtureOwners) {
    for (key, field) in input.owners.iter().copied().enumerate() {
        if let Some((representation, range)) = representation(field, owners) {
            let interval = scalars(range);
            representation
                .cast::<dyn BRepGCurve>()
                .set_range(interval[0], interval[1])
                .unwrap();
            restore_uv(field, &representation);
            owners.representations.insert(key, representation);
        }
    }
}
fn representation(
    input: OwnerInput,
    owners: &FixtureOwners,
) -> Option<(Handle<dyn BRepCurveRepresentation>, [u64; 2])> {
    match input {
        OwnerInput::Spatial {
            location,
            range,
            curve,
        } => Some((
            Handle::new(BRepCurve3D::new(
                &owners.curve3(curve),
                &owners.location(location),
            ))
            .cast(),
            range,
        )),
        OwnerInput::Parametric {
            location,
            range,
            curve,
            surface,
            endpoints,
        } => Some((
            parametric(owners, location, curve, surface, endpoints),
            range,
        )),
        OwnerInput::Seam {
            location,
            range,
            curves,
            surface,
            continuity,
            endpoints,
        } => Some((
            seam(owners, location, curves, surface, continuity, endpoints),
            range,
        )),
        _ => None,
    }
}
fn parametric(
    owners: &FixtureOwners,
    location: LocationInput,
    curve: usize,
    surface: usize,
    endpoints: [[u64; 2]; 2],
) -> Handle<dyn BRepCurveRepresentation> {
    let result = BRepCurveOnSurface::new(
        &owners.curve2(curve),
        &owners.surface(surface),
        &owners.location(location),
    )
    .unwrap();
    result.set_uv_points(point(endpoints[0]), point(endpoints[1]));
    Handle::new(result).cast()
}
fn seam(
    owners: &FixtureOwners,
    location: LocationInput,
    curves: [usize; 2],
    surface: usize,
    continuity: i32,
    endpoints: [[[u64; 2]; 2]; 2],
) -> Handle<dyn BRepCurveRepresentation> {
    let continuity = [
        Continuity::C0,
        Continuity::G1,
        Continuity::C1,
        Continuity::G2,
        Continuity::C2,
        Continuity::C3,
        Continuity::CN,
    ][continuity as usize];
    let result = BRepCurveOnClosedSurface::new(
        &owners.curve2(curves[0]),
        &owners.curve2(curves[1]),
        &owners.surface(surface),
        &owners.location(location),
        continuity,
    )
    .unwrap();
    result.set_uv_points(point(endpoints[0][0]), point(endpoints[0][1]));
    result.set_uv_points2(point(endpoints[1][0]), point(endpoints[1][1]));
    Handle::new(result).cast()
}
fn point(input: [u64; 2]) -> Point2 {
    let p = scalars(input);
    Point2::new(p[0], p[1])
}

fn restore_uv(input: OwnerInput, representation: &Handle<dyn BRepCurveRepresentation>) {
    match input {
        OwnerInput::Parametric { endpoints, .. } => {
            let curve = representation.cast::<BRepCurveOnSurface>();
            curve.set_uv_points(point(endpoints[0]), point(endpoints[1]));
        }
        OwnerInput::Seam { endpoints, .. } => {
            let curve = representation.cast::<BRepCurveOnClosedSurface>();
            curve.set_uv_points(point(endpoints[0][0]), point(endpoints[0][1]));
            curve.set_uv_points2(point(endpoints[1][0]), point(endpoints[1][1]));
        }
        _ => {}
    }
}
