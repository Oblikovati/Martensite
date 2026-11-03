//! Existing geometry constructors rebuild owned fixture inputs with explicit bindings.
use super::{registry::FixtureOwners, splines, types::*};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::geom::{GeomLine, GeomPlane, GeomSurfaceOfLinearExtrusion, GeomSweptSurface};
use martensite_tkmath::gp::{
    Axis1, Axis2d, Axis3, Direction2, Direction3, Point2, Point3, StandardDirection3,
};
pub(super) fn construct(input: &GraphInput, owners: &mut FixtureOwners) {
    for (key, field) in input.owners.iter().copied().enumerate() {
        primitive(key, field, owners);
    }
    for (key, field) in input.owners.iter().copied().enumerate() {
        if let OwnerInput::Extrusion { basis, direction } = field {
            extrusion(key, basis, direction, owners);
        }
    }
}
fn primitive(key: usize, input: OwnerInput, owners: &mut FixtureOwners) {
    match input {
        OwnerInput::Spline3(input) => {
            owners.curves3.insert(key, splines::curve3(input).cast());
        }
        OwnerInput::Line3 { origin, direction } => {
            owners.curves3.insert(key, line3(origin, direction).cast());
        }
        _ => primitive2(key, input, owners),
    }
}
fn primitive2(key: usize, input: OwnerInput, owners: &mut FixtureOwners) {
    match input {
        OwnerInput::Spline2(input) => {
            owners.curves2.insert(key, splines::curve2(input).cast());
        }
        OwnerInput::Line2 { origin, direction } => {
            owners.curves2.insert(key, line2(origin, direction).cast());
        }
        _ => plane_owner(key, input, owners),
    }
}
fn plane_owner(key: usize, input: OwnerInput, owners: &mut FixtureOwners) {
    if let OwnerInput::Plane {
        origin,
        x,
        y,
        normal,
    } = input
    {
        owners
            .surfaces
            .insert(key, plane(origin, x, y, normal).cast());
    }
}

fn extrusion(key: usize, basis: usize, direction: [u64; 3], owners: &mut FixtureOwners) {
    let surface = Handle::new(
        GeomSurfaceOfLinearExtrusion::new(&owners.curve3(basis), direction3(direction)).unwrap(),
    );
    // The constructor copies its basis; the binding names that actual retained copy.
    owners.curves3.insert(basis, surface.basis_curve());
    owners.surfaces.insert(key, surface.cast());
}
fn line3(origin: [u64; 3], direction: [u64; 3]) -> Handle<GeomLine> {
    let p = scalars(origin);
    Handle::new(GeomLine::new(Axis1::new(
        Point3::new(p[0], p[1], p[2]),
        direction3(direction),
    )))
}
fn line2(origin: [u64; 2], direction: [u64; 2]) -> Handle<Geom2dLine> {
    let p = scalars(origin);
    let d = scalars(direction);
    let axis = Axis2d::new(
        Point2::new(p[0], p[1]),
        Direction2::new(d[0], d[1]).unwrap(),
    );
    Handle::new(Geom2dLine::new(axis))
}
fn direction3(input: [u64; 3]) -> Direction3 {
    let d = scalars(input);
    Direction3::new(d[0], d[1], d[2]).unwrap()
}
fn plane(origin: [u64; 3], x: [u64; 3], y: [u64; 3], normal: [u64; 3]) -> Handle<GeomPlane> {
    let p = scalars(origin);
    // The standard-frame constructor preserves the explicitly recorded zero signs.
    let axis =
        Axis3::with_standard_frame(Point3::new(p[0], p[1], p[2]), standard(normal), standard(x));
    let actual_y = axis.y_direction();
    assert_eq!(
        [actual_y.x(), actual_y.y(), actual_y.z()].map(f64::to_bits),
        y
    );
    Handle::new(GeomPlane::new(axis))
}

fn standard(input: [u64; 3]) -> StandardDirection3 {
    use StandardDirection3::*;
    [X, Y, Z, NegX, NegY, NegZ]
        .into_iter()
        .find(|axis| {
            let d = Direction3::from_standard(*axis);
            [d.x(), d.y(), d.z()].map(f64::to_bits) == input
        })
        .unwrap_or_else(|| panic!("direction {input:?}: expected an exactly stored standard axis"))
}
