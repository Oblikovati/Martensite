//! Fixed native graphs call `is_tangent_faces` on seams, stored continuity and curved, located and
//! spline domains; expected results are the frozen native records, with no native runtime.
//! The 64 C2 cells on the two circular rims stay uncalled: the native constructor leaves the status
//! it reads uninitialized there, so they have no defined result (tkfillet-tangency-native-2026-10-26).
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{self, Edge, Face, Shape, ShapeIterator},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkfillet::chfi3d::is_tangent_faces;
use martensite_tkmath::{geom_abs::Shape as Continuity, gp::Transform3, toploc::Location};
use std::collections::BTreeMap;

const EXPECTED: &str = include_str!("tangency_graphs/fixtures/expected.txt");

macro_rules! shared_graph {
    ($name:literal) => {
        (
            $name,
            include_bytes!(concat!("connection_graphs/fixtures/", $name, ".brep")).as_slice(),
        )
    };
}
macro_rules! own_graph {
    ($name:literal) => {
        (
            $name,
            include_bytes!(concat!("tangency_graphs/fixtures/", $name, ".brep")).as_slice(),
        )
    };
}

const GRAPHS: [(&str, &[u8]); 29] = [
    shared_graph!("bezier_negative"),
    shared_graph!("bezier_negative_uhigh"),
    shared_graph!("bezier_negative_ulow"),
    shared_graph!("bezier_negative_vhigh"),
    shared_graph!("bezier_positive"),
    shared_graph!("bezier_positive_restricted"),
    own_graph!("bezier_positive_stored_C2"),
    own_graph!("bezier_positive_stored_G1"),
    shared_graph!("bezier_positive_uhigh"),
    shared_graph!("bezier_positive_ulow"),
    shared_graph!("bezier_positive_vhigh"),
    shared_graph!("bspline_negative"),
    shared_graph!("bspline_negative_uhigh"),
    shared_graph!("bspline_negative_ulow"),
    shared_graph!("bspline_negative_vhigh"),
    shared_graph!("bspline_positive"),
    shared_graph!("bspline_positive_uhigh"),
    shared_graph!("bspline_positive_ulow"),
    shared_graph!("bspline_positive_vhigh"),
    shared_graph!("cone_rim"),
    shared_graph!("cylinder_rim"),
    shared_graph!("cylinder_seam"),
    own_graph!("distinct_cylinder_seam"),
    shared_graph!("planar_aligned"),
    shared_graph!("planar_orthogonal"),
    own_graph!("planar_orthogonal_stored_C2"),
    own_graph!("planar_orthogonal_stored_G1"),
    own_graph!("smooth_bezier_joint"),
    own_graph!("smooth_bspline_joint"),
];

struct Roles {
    edge: Edge,
    first: Face,
    second: Face,
}

fn read_roles(name: &str, brep: &[u8]) -> Roles {
    let header = b"DBRep_DrawableShape";
    assert!(brep.starts_with(header), "{name}: expected graph header");
    let mut stream = IoStream::from_bytes(brep[header.len()..].to_vec());
    let mut root = Shape::default();
    brep_tools::read(
        &mut root,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    let roles: Vec<Shape> = ShapeIterator::new(&root, true, true).collect();
    assert_eq!(roles.len(), 4, "{name}: expected four authored roles");
    Roles {
        edge: topods::edge(&roles[1]).unwrap(),
        first: topods::face(&roles[2]).unwrap(),
        second: topods::face(&roles[3]).unwrap(),
    }
}

/// The native proper rigid location applied to all three roles of a located variant.
fn rigid_location() -> Location {
    let mut transform = Transform3::default();
    transform
        .set_values([[0., 1., 0., 7.], [0., 0., 1., -11.], [1., 0., 0., 13.]])
        .unwrap();
    Location::new(transform)
}

/// Build a local variant, flags being [located, exchanged, first reversed, second reversed, edge
/// reversed]; the shared authored graph stays untouched.
fn variant(roles: &Roles, flags: [bool; 5]) -> Roles {
    let (mut first, mut second) = (roles.first.clone(), roles.second.clone());
    let mut edge = roles.edge.clone();
    if flags[1] {
        std::mem::swap(&mut first, &mut second);
    }
    if flags[2] {
        first.reverse();
    }
    if flags[3] {
        second.reverse();
    }
    if flags[4] {
        edge.reverse();
    }
    if !flags[0] {
        return Roles {
            edge,
            first,
            second,
        };
    }
    let location = rigid_location();
    Roles {
        edge: topods::edge(&edge.moved(&location, false).unwrap()).unwrap(),
        first: topods::face(&first.moved(&location, false).unwrap()).unwrap(),
        second: topods::face(&second.moved(&location, false).unwrap()).unwrap(),
    }
}

fn continuity(value: &str) -> Continuity {
    match value {
        "1" => Continuity::G1,
        "3" => Continuity::G2,
        "4" => Continuity::C2,
        other => panic!("order={other:?}: expected the native enum value 1, 3 or 4"),
    }
}

/// Compare one frozen row and return its failure, if any.
fn check_row(graphs: &BTreeMap<&str, Roles>, row: &str) -> Option<String> {
    let fields: Vec<&str> = row.split_whitespace().collect();
    assert_eq!(fields.len(), 8, "row={row:?}: expected eight fields");
    let flags = std::array::from_fn(|i| fields[i + 1] == "1");
    let inputs = variant(&graphs[fields[0]], flags);
    let expected = fields[7] == "1";
    match is_tangent_faces(
        &inputs.edge,
        &inputs.first,
        &inputs.second,
        continuity(fields[6]),
    ) {
        Ok(actual) if actual == expected => None,
        Ok(actual) => Some(format!("{row}: returned {actual}")),
        Err(failure) => Some(format!("{row}: failed {failure:?}")),
    }
}

#[test]
fn seam_stored_curved_located_and_spline_graphs_match_the_native_records() {
    let graphs: BTreeMap<&str, Roles> = GRAPHS
        .iter()
        .map(|(name, brep)| (*name, read_roles(name, brep)))
        .collect();
    let rows: Vec<&str> = EXPECTED.lines().collect();
    assert_eq!(rows.len(), 2720, "expected every defined native call");
    let failures: Vec<String> = rows
        .iter()
        .filter_map(|row| check_row(&graphs, row))
        .collect();
    assert!(
        failures.is_empty(),
        "{} of {} differ, first: {:#?}",
        failures.len(),
        rows.len(),
        &failures[..failures.len().min(40)]
    );
}
