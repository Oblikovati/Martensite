//! Raw occurrences and state words are restored only after local assembly is complete.
use super::{registry::FixtureOwners, types::*};
use martensite_tkbrep::{
    brep::{BRepTEdge, BRepTFace, BRepTVertex},
    topods::{Builder, Shape, TWire},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::{gp::Point3, toploc::Location};
pub(super) fn construct(input: &GraphInput, owners: &mut FixtureOwners) {
    create_owners(input, owners);
    append_all_children(input, owners);
    for (key, field) in input.owners.iter().copied().enumerate() {
        if let Some((state, _)) = fields(field) {
            restore_state(key, state, owners);
        }
    }
}
fn create_owners(input: &GraphInput, owners: &mut FixtureOwners) {
    for (key, field) in input.owners.iter().copied().enumerate() {
        let shape = match field {
            OwnerInput::Face {
                surface,
                location,
                tolerance,
                natural,
                ..
            } => Some(face(owners, surface, location, tolerance, natural)),
            OwnerInput::Wire { .. } => Some(shape(Handle::new(TWire::new()).cast())),
            OwnerInput::Edge {
                tolerance,
                flags,
                representations,
                ..
            } => Some(edge(owners, tolerance, flags, representations)),
            OwnerInput::Vertex {
                point, tolerance, ..
            } => Some(vertex(point, tolerance)),
            _ => None,
        };
        if let Some(shape) = shape {
            owners.shapes.insert(key, shape);
        }
    }
}
fn shape(owner: Handle<dyn martensite_tkbrep::topods::TShape>) -> Shape {
    Shape::from_parts(owner, Location::default(), Orientation::Forward)
}
fn face(
    owners: &FixtureOwners,
    surface: usize,
    location: LocationInput,
    tolerance: u64,
    natural: bool,
) -> Shape {
    let result = Handle::new(BRepTFace::new());
    result.set_surface(&owners.surface(surface));
    result.set_location(&owners.location(location));
    result.set_tolerance(f64::from_bits(tolerance));
    result.set_natural_restriction(natural);
    shape(result.cast())
}
fn edge(owners: &FixtureOwners, tolerance: u64, flags: i32, representations: &[usize]) -> Shape {
    assert_eq!(
        flags & !7,
        0,
        "edge flags {flags}: expected the three stored geometry flags"
    );
    let result = Handle::new(BRepTEdge::new());
    result.set_tolerance(f64::from_bits(tolerance));
    result.set_same_parameter(flags & 1 != 0);
    result.set_same_range(flags & 2 != 0);
    result.set_degenerated(flags & 4 != 0);
    result.change_curves(|curves| {
        curves.extend(
            representations
                .iter()
                .map(|key| owners.representations[key].clone()),
        )
    });
    shape(result.cast())
}
fn vertex(point: [u64; 3], tolerance: u64) -> Shape {
    let result = Handle::new(BRepTVertex::new());
    let p = scalars(point);
    result.set_pnt(Point3::new(p[0], p[1], p[2]));
    result.set_tolerance(f64::from_bits(tolerance));
    shape(result.cast())
}
fn append_children(key: usize, children: &[OccurrenceInput], owners: &FixtureOwners) {
    let mut parent = owners.shape(key);
    for child in children {
        Builder::default()
            .add(&mut parent, &owners.occurrence(*child))
            .unwrap();
    }
}
fn restore_state(key: usize, state: u16, owners: &FixtureOwners) {
    let shape = owners.shape(key);
    let base = shape.tshape().get().unwrap().tshape_base();
    // All assembly is local; no reader can observe this temporary cleared word.
    base.set_bit(u16::MAX, false);
    base.set_bit(state, true);
    assert_eq!(base.state(), state);
}
pub(super) fn fields(input: OwnerInput) -> Option<(u16, &'static [OccurrenceInput])> {
    match input {
        OwnerInput::Face {
            state, children, ..
        }
        | OwnerInput::Wire { state, children }
        | OwnerInput::Edge {
            state, children, ..
        }
        | OwnerInput::Vertex {
            state, children, ..
        } => Some((state, children)),
        _ => None,
    }
}

fn append_all_children(input: &GraphInput, owners: &FixtureOwners) {
    // Attaching an owner freezes it; its own children must already be installed.
    for level in 0..4 {
        for (key, field) in input.owners.iter().copied().enumerate() {
            if assembly_level(field) == Some(level) {
                append_children(key, fields(field).unwrap().1, owners);
            }
        }
    }
}
fn assembly_level(input: OwnerInput) -> Option<usize> {
    match input {
        OwnerInput::Vertex { .. } => Some(0),
        OwnerInput::Edge { .. } => Some(1),
        OwnerInput::Wire { .. } => Some(2),
        OwnerInput::Face { .. } => Some(3),
        _ => None,
    }
}
