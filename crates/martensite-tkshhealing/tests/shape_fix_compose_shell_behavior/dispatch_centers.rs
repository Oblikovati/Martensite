//! Patch placement uses the precomputed wire center even when occurrences share storage.
use super::*;
use martensite_tkbrep::topods::{Face, Shape, ShapeIterator};
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkg3d::geom::GeomBezierSurface;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

const UNIT_ENDPOINTS: [[f64; 4]; 4] = [
    [0., 0., 0., 1.],
    [0., 0., 1., 0.],
    [1., 0., 1., 1.],
    [0., 1., 1., 1.],
];

fn unit_bezier_patch() -> Handle<dyn GeomSurface> {
    let mut poles = Array2::<Point3>::with_bounds(1, 2, 1, 2).unwrap();
    for u in 1..=2 {
        for v in 1..=2 {
            poles
                .set_value(u, v, Point3::new((u - 1) as f64, (v - 1) as f64, 0.))
                .unwrap();
        }
    }
    Handle::new(GeomBezierSurface::new(&poles).unwrap()).cast()
}

fn reparameterized_grid(surface: &Handle<dyn GeomSurface>) -> Handle<ShapeExtendCompositeSurface> {
    let grid = surface_grid(surface.clone());
    let joints = Array1::from_shared(&[0., 2.], 1);
    assert!(grid.set_u_joint_values(&joints).unwrap());
    assert!(grid.set_v_joint_values(&joints).unwrap());
    grid
}

fn patch_and_source() -> (Handle<ShapeExtendCompositeSurface>, Face) {
    let patch = unit_bezier_patch();
    let face = BRepBuilderAPIMakeFace::from_surface(&patch, 1.0e-7)
        .unwrap()
        .face()
        .unwrap();
    (reparameterized_grid(&patch), face)
}

fn original_boundary(face: &Face) -> ShapeFixWireSegment {
    let shape = face.clone().into_shape();
    let child = ShapeIterator::new(&shape, true, true)
        .value()
        .unwrap()
        .clone();
    let wire = topods::wire(&child).unwrap();
    let data = Handle::new(ShapeExtendWireData::from_wire(&wire, true, true).unwrap());
    ShapeFixWireSegment::from_wire_data(&data, Orientation::Forward).unwrap()
}

fn compose_patch(face: &Face, grid: &Handle<ShapeExtendCompositeSurface>) -> ShapeFixComposeShell {
    let composer = ShapeFixComposeShell::new();
    composer.set_context(&Handle::new(ShapeBuildReShape::new()));
    composer
        .init(grid, &Location::default(), face, 1.0e-7)
        .unwrap();
    composer
}

fn dispatch_repeated(
    face: &Face,
    grid: &Handle<ShapeExtendCompositeSurface>,
    count: usize,
) -> ShapeFixWireSegment {
    let composer = compose_patch(face, grid);
    let segment = original_boundary(face);
    let mut wires = shared_occurrences(&segment, count);
    let mut faces: Sequence<Shape> = Sequence::default();
    composer.dispatch_wires(&mut faces, &mut wires).unwrap();
    assert_eq!(wires.length() as usize, count);
    assert_eq!(faces.length() as usize, count);
    assert!(composer.result().is_null());
    assert!(!composer.status(ShapeExtendStatus::Done));
    segment
}

fn shared_occurrences(
    segment: &ShapeFixWireSegment,
    count: usize,
) -> Sequence<ShapeFixWireSegment> {
    let mut wires = Sequence::default();
    for _ in 0..count {
        wires.append(segment.clone());
    }
    assert!(wires.value(1).unwrap().wire_data() == wires.value(count).unwrap().wire_data());
    wires
}

fn endpoint_coordinates(segment: &ShapeFixWireSegment, face: &Face) -> Vec<[f64; 4]> {
    (1..=segment.nb_edges())
        .map(|index| {
            let edge = segment.edge(index).unwrap();
            let record = BRepTool::curve_on_face(&edge, face).unwrap();
            let curve = record.pcurve.get().unwrap();
            let first = curve.value(record.first).unwrap();
            let last = curve.value(record.last).unwrap();
            [first.x(), first.y(), last.x(), last.y()]
        })
        .collect()
}

#[test]
fn shared_occurrences_retain_the_original_center_for_each_patch_mapping() {
    for count in 1..=3 {
        assert_repeated_mapping(count);
    }
}

fn assert_repeated_mapping(count: usize) {
    let (grid, source) = patch_and_source();
    let before = endpoint_coordinates(&original_boundary(&source), &source);
    assert_eq!(before, UNIT_ENDPOINTS);
    let segment = dispatch_repeated(&source, &grid, count);
    let scale = martensite_cmath::powi(0.5, count as i32);
    let expected: Vec<_> = before
        .iter()
        .map(|row| row.map(|value| value * scale))
        .collect();
    assert_eq!(endpoint_coordinates(&segment, &source), expected);
    assert_source_unmodified(&source);
}

fn assert_source_unmodified(source: &Face) {
    assert_eq!(
        endpoint_coordinates(&original_boundary(source), source),
        UNIT_ENDPOINTS
    );
    assert!(
        BRepCheckAnalyzer::new(&source.clone().into_shape(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}
