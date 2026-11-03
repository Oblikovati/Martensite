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
type Representation = Handle<dyn BRepCurveRepresentation>;
fn representation(input: OwnerInput, owners: &FixtureOwners) -> Option<(Representation, [u64; 2])> {
    match input {
        OwnerInput::Spatial { range, .. } => Some((spatial_input(owners, input), range)),
        OwnerInput::Parametric { range, .. } => Some((parametric_input(owners, input), range)),
        OwnerInput::Seam { range, .. } => Some((seam_input(owners, input), range)),
        _ => None,
    }
}
fn spatial_input(owners: &FixtureOwners, input: OwnerInput) -> Representation {
    let OwnerInput::Spatial {
        location, curve, ..
    } = input
    else {
        unreachable!()
    };
    Handle::new(BRepCurve3D::new(
        &owners.curve3(curve),
        &owners.location(location),
    ))
    .cast()
}
fn parametric_input(owners: &FixtureOwners, input: OwnerInput) -> Representation {
    let OwnerInput::Parametric {
        location,
        curve,
        surface,
        endpoints,
        ..
    } = input
    else {
        unreachable!()
    };
    parametric(owners, location, curve, surface, endpoints)
}
fn seam_input(owners: &FixtureOwners, input: OwnerInput) -> Representation {
    let OwnerInput::Seam {
        location,
        curves,
        surface,
        continuity,
        endpoints,
        ..
    } = input
    else {
        unreachable!()
    };
    seam(owners, location, curves, surface, continuity, endpoints)
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
    let result = closed_surface(owners, location, curves, surface, continuity);
    result.set_uv_points(point(endpoints[0][0]), point(endpoints[0][1]));
    result.set_uv_points2(point(endpoints[1][0]), point(endpoints[1][1]));
    Handle::new(result).cast()
}
fn closed_surface(
    owners: &FixtureOwners,
    location: LocationInput,
    curves: [usize; 2],
    surface: usize,
    continuity: i32,
) -> BRepCurveOnClosedSurface {
    let continuity = continuity_value(continuity);
    BRepCurveOnClosedSurface::new(
        &owners.curve2(curves[0]),
        &owners.curve2(curves[1]),
        &owners.surface(surface),
        &owners.location(location),
        continuity,
    )
    .unwrap()
}
fn continuity_value(index: i32) -> Continuity {
    [
        Continuity::C0,
        Continuity::G1,
        Continuity::C1,
        Continuity::G2,
        Continuity::C2,
        Continuity::C3,
        Continuity::CN,
    ][index as usize]
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
