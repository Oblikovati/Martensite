use martensite_tkbrep::brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    top_abs::ShapeEnum,
};
use martensite_tkmath::geom_abs::{CurveType, SurfaceType};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

pub(super) fn operands() -> [Shape; 2] {
    [box_at(0.), box_at(1.)]
}

fn box_at(x: f64) -> Shape {
    BRepPrimAPIMakeBox::from_points(Point3::new(x, 0., 0.), Point3::new(x + 2., 2., 2.))
        .unwrap()
        .solid()
        .unwrap()
        .into_shape()
}

pub(super) fn entities(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    let mut unique = Vec::<Shape>::new();
    for entity in Explorer::new(shape, kind, ShapeEnum::Shape).unwrap() {
        if !unique.iter().any(|known| known.is_same(&entity)) {
            unique.push(entity);
        }
    }
    unique
}

pub(super) fn bounds(shape: &Shape) -> [f64; 6] {
    let mut result = [
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];
    for vertex in entities(shape, ShapeEnum::Vertex) {
        let point = BRepTool::pnt(&topods::vertex(&vertex).unwrap()).unwrap();
        for (axis, value) in [point.x(), point.y(), point.z()].into_iter().enumerate() {
            result[2 * axis] = result[2 * axis].min(value);
            result[2 * axis + 1] = result[2 * axis + 1].max(value);
        }
    }
    result
}

pub(super) fn signature(shape: &Shape) -> Vec<[u64; 6]> {
    let mut faces: Vec<_> = entities(shape, ShapeEnum::Face)
        .iter()
        .map(|face| bounds(face).map(f64::to_bits))
        .collect();
    faces.sort();
    faces
}

pub(super) fn require_box(shape: &Shape, expected: [f64; 6]) {
    assert_eq!(bounds(shape), expected, "expected box bounds={expected:?}");
    require_counts(shape);
    let checker = BRepCheckAnalyzer::new(shape, true, false, true).unwrap();
    assert!(
        checker.is_valid().unwrap(),
        "exact geometry/topology check failed"
    );
    require_faces(shape, expected);
}

fn require_counts(shape: &Shape) {
    let counts = [
        (ShapeEnum::Solid, 1),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Face, 6),
        (ShapeEnum::Edge, 12),
        (ShapeEnum::Vertex, 8),
    ];
    for (kind, count) in counts {
        assert_eq!(
            entities(shape, kind).len(),
            count,
            "unexpected {kind:?} count"
        );
    }
}

fn require_faces(shape: &Shape, expected: [f64; 6]) {
    let mut remaining = entities(shape, ShapeEnum::Face);
    for axis in 0..3 {
        for end in 0..2 {
            let mut rectangle = expected;
            rectangle[2 * axis] = expected[2 * axis + end];
            rectangle[2 * axis + 1] = expected[2 * axis + end];
            let index = remaining
                .iter()
                .position(|face| bounds(face) == rectangle)
                .unwrap();
            let face = topods::face(&remaining.remove(index)).unwrap();
            require_rectangle(&face, rectangle);
        }
    }
    assert!(remaining.is_empty());
}

fn require_rectangle(face: &topods::Face, rectangle: [f64; 6]) {
    let surface = BRepAdaptorSurface::with_face(face, true).unwrap();
    assert_eq!(surface.get_type().unwrap(), SurfaceType::Plane);
    let shape = face.clone().into_shape();
    require_corners(&shape, rectangle);
    let edges = entities(&shape, ShapeEnum::Edge);
    assert_eq!(edges.len(), 4);
    for edge in edges {
        let edge = topods::edge(&edge).unwrap();
        let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
        assert_eq!(curve.get_type().unwrap(), CurveType::Line);
        assert!(BRepTool::edge_tolerance(&edge).unwrap().is_finite());
    }
    assert!(BRepTool::face_tolerance(face).unwrap().is_finite());
}

fn require_corners(shape: &Shape, rectangle: [f64; 6]) {
    let mut actual: Vec<_> = entities(shape, ShapeEnum::Vertex)
        .iter()
        .map(|vertex| {
            let point = BRepTool::pnt(&topods::vertex(vertex).unwrap()).unwrap();
            [point.x(), point.y(), point.z()]
        })
        .collect();
    actual.sort_by(|left, right| left.partial_cmp(right).unwrap());
    let mut expected = corners(rectangle);
    expected.sort_by(|left, right| left.partial_cmp(right).unwrap());
    expected.dedup();
    assert_eq!(
        expected.len(),
        4,
        "analytic face must have four distinct corners"
    );
    assert_eq!(
        actual, expected,
        "face vertices must match all four analytic corners"
    );
}

fn corners(rectangle: [f64; 6]) -> Vec<[f64; 3]> {
    let [xmin, xmax, ymin, ymax, zmin, zmax] = rectangle;
    [xmin, xmax]
        .into_iter()
        .flat_map(|x| {
            [ymin, ymax]
                .into_iter()
                .flat_map(move |y| [zmin, zmax].map(|z| [x, y, z]))
        })
        .collect()
}
