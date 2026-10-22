//! Full face composition preserves a free internal vertex or incorporates it into a seam.
use super::*;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{Edge, Face, Shape, ShapeIterator, Vertex},
};
use martensite_tkg3d::{geom::GeomRectangularTrimmedSurface, top_abs::ShapeEnum};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

fn source_with_vertex(on_joint: bool) -> (Face, Vertex) {
    let mut face = bounded_plane_face();
    let vertex = free_internal_vertex(on_joint);
    let x = BRepTool::pnt(&vertex).unwrap().x();
    BRepBuilder::default()
        .update_vertex_on_face(&vertex, x, 1., &face, 1.0e-7)
        .unwrap();
    Builder
        .add(&mut face, &vertex.clone().into_shape())
        .unwrap();
    (face, vertex)
}

fn free_internal_vertex(on_joint: bool) -> Vertex {
    let point = Point3::new(if on_joint { 1. } else { 0.5 }, 1., 0.);
    let mut vertex = BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap();
    vertex.set_orientation(Orientation::Internal);
    vertex
}

fn finite_patch(index: i32, split: bool) -> Handle<dyn GeomSurface> {
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    Handle::new(
        GeomRectangularTrimmedSurface::new(
            &plane,
            ((index - 1) as f64, if split { index as f64 } else { 2. }),
            (0., 2.),
            (true, true),
        )
        .unwrap(),
    )
    .cast()
}

fn finite_grid(split: bool) -> Handle<ShapeExtendCompositeSurface> {
    let count = if split { 2 } else { 1 };
    let cells = HArray2::with_value(1, count, 1, 1, Handle::<dyn GeomSurface>::default()).unwrap();
    for index in 1..=count {
        cells
            .change_array2()
            .set_value(index, 1, finite_patch(index, split))
            .unwrap();
    }
    Handle::new(
        ShapeExtendCompositeSurface::with_grid(
            &Handle::new(cells),
            ShapeExtendParametrisation::Natural,
        )
        .unwrap(),
    )
}

fn compose_vertex_case(split: bool, on_joint: bool, closed: bool) {
    let (source, vertex) = source_with_vertex(on_joint);
    let context = Handle::new(ShapeBuildReShape::new());
    let compose = ShapeFixComposeShell::new();
    compose.set_context(&context);
    compose
        .init(&finite_grid(split), &Location::default(), &source, 1.0e-7)
        .unwrap();
    compose.set_closed_mode(closed);
    assert_completion(&compose);
    assert_valid(&source.clone().into_shape());
    assert_valid(&compose.result());
    assert_result_topology(&compose.result(), split, on_joint);
    assert_face_consistency(&compose.result());
    assert_vertex_history(&vertex, &context, &compose.result());
    assert_source_children(&source, &vertex);
    assert_source_geometry(&source, &vertex, on_joint);
}

fn assert_completion(compose: &ShapeFixComposeShell) {
    assert!(compose.perform().unwrap());
    assert!(compose.status(ShapeExtendStatus::Done1));
    assert!(!compose.status(ShapeExtendStatus::Fail));
}

fn assert_valid(shape: &Shape) {
    assert!(
        BRepCheckAnalyzer::new(shape, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}

fn assert_result_topology(result: &Shape, split: bool, on_joint: bool) {
    let faces: Vec<_> = Explorer::new(result, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .collect();
    assert_eq!(faces.len(), if split { 2 } else { 1 });
    let direct_vertices = faces
        .iter()
        .map(|face| {
            ShapeIterator::new(face, true, true)
                .filter(|child| child.shape_type().unwrap() == ShapeEnum::Vertex)
                .count()
        })
        .sum::<usize>();
    assert_eq!(direct_vertices, usize::from(!(split && on_joint)));
}

fn assert_vertex_history(vertex: &Vertex, context: &Handle<ShapeBuildReShape>, result: &Shape) {
    let replacement = context.value(&vertex.clone().into_shape()).unwrap();
    assert!(!replacement.is_null());
    let point = BRepTool::pnt(vertex).unwrap();
    assert_eq!(
        BRepTool::pnt(&topods::vertex(&replacement).unwrap()).unwrap(),
        point
    );
    assert!(
        Explorer::new(result, ShapeEnum::Vertex, ShapeEnum::Shape)
            .unwrap()
            .any(|current| current.is_same(&replacement))
    );
}

fn assert_source_children(source: &Face, vertex: &Vertex) {
    let children: Vec<_> = ShapeIterator::new(&source.clone().into_shape(), true, true).collect();
    assert_eq!(children.len(), 2);
    assert!(
        children
            .iter()
            .any(|child| child.is_same(&vertex.clone().into_shape()))
    );
}

fn assert_source_geometry(source: &Face, vertex: &Vertex, on_joint: bool) {
    assert_eq!(
        BRepTool::pnt(vertex).unwrap(),
        Point3::new(if on_joint { 1. } else { 0.5 }, 1., 0.)
    );
    assert_eq!(
        source_boundary_points(source),
        [
            Point3::new(0., 0., 0.),
            Point3::new(0., 2., 0.),
            Point3::new(2., 0., 0.),
            Point3::new(2., 2., 0.)
        ]
    );
}

fn source_boundary_points(source: &Face) -> Vec<Point3> {
    let source_shape = source.clone().into_shape();
    let wire = ShapeIterator::new(&source_shape, true, true)
        .value()
        .unwrap()
        .clone();
    let mut points: Vec<_> = Explorer::new(&wire, ShapeEnum::Vertex, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| BRepTool::pnt(&topods::vertex(&shape).unwrap()).unwrap())
        .collect();
    points.sort_by(|a, b| a.x().total_cmp(&b.x()).then(a.y().total_cmp(&b.y())));
    points.dedup();
    points
}

fn assert_face_consistency(result: &Shape) {
    for shape in Explorer::new(result, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        let face = topods::face(&shape).unwrap();
        for edge in Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
            assert_edge_consistency(&topods::edge(&edge).unwrap(), &face);
        }
    }
}

fn assert_edge_consistency(edge: &Edge, face: &Face) {
    let (curve, first, last) = BRepTool::curve(edge).unwrap();
    let pcurve = BRepTool::curve_on_face(edge, face).unwrap();
    assert_eq!((first, last), (pcurve.first, pcurve.last));
    let surface = BRepTool::surface(face).unwrap();
    for parameter in [first, (first + last) / 2., last] {
        let uv = pcurve.pcurve.get().unwrap().value(parameter).unwrap();
        let actual = curve.get().unwrap().value(parameter).unwrap();
        let expected = surface.get().unwrap().value(uv.x(), uv.y()).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn internal_vertex_composition_completes_with_valid_topology_and_history() {
    for mode in 0..8 {
        compose_vertex_case(mode & 4 != 0, mode & 2 != 0, mode & 1 != 0);
    }
}
