//! `StepToTopoDS`: the tessellated face cases of `face translation`, and the state
//! the translators share.
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Edge, Shape, Vertex};
use martensite_tkdestep::geom_to_step::GeomToStepMakeLine;
use martensite_tkdestep::step_data::StepDataFactors;
use martensite_tkdestep::step_geom::StepGeomCartesianPoint;
use martensite_tkdestep::step_shape::{StepShapeEdgeCurve, StepShapeVertex};
use martensite_tkdestep::step_to_topo_ds::{
    StepToTopoDS, StepToTopoDSBuilder, StepToTopoDSBuilderError, StepToTopoDSNMTool,
    StepToTopoDSPointPair, StepToTopoDSRoot, StepToTopoDSTool, StepToTopoDSTranslateEdge,
    StepToTopoDSTranslateFace, StepToTopoDSTranslateShell, StepToTopoDSTranslateSolid,
    StepToTopoDSTranslateVertex,
};
use martensite_tkdestep::step_visual::{
    StepVisualComplexTriangulatedFace, StepVisualCoordinatesList, StepVisualFaceOrSurface,
    StepVisualTessellatedFace, StepVisualTessellatedSurfaceSet, StepVisualTriangulatedFace,
    StepVisualTriangulatedSurfaceSet,
};
use martensite_tkernel::ncollection::{DataMap, HArray1, HArray2};
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::{Direction3, Line3, Point3, Xyz};
use martensite_tkmath::poly::{Triangulation, mesh_purpose};
use martensite_tkxsbase::transfer::TransferTransientProcess;

fn name(text: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(text))
}

fn shared<T: Clone + Default + Send + Sync + 'static>(values: &[T]) -> Handle<HArray1<T>> {
    let array = HArray1::with_bounds(1, values.len() as i32).expect("bounds");
    for (index, value) in values.iter().enumerate() {
        array
            .change_array1()
            .set_value(index as i32 + 1, value.clone())
            .expect("set");
    }
    Handle::new(array)
}

fn coords(points: &[[f64; 3]]) -> Handle<StepVisualCoordinatesList> {
    let values: Vec<Xyz> = points.iter().map(|p| Xyz::new(p[0], p[1], p[2])).collect();
    let list = Handle::new(StepVisualCoordinatesList::new());
    list.init(&name("coords"), &shared(&values));
    list
}

fn rows<T: Clone + Default + Send + Sync + 'static>(data: &[[T; 3]]) -> Handle<HArray2<T>> {
    let array = HArray2::with_bounds(1, data.len() as i32, 1, 3).expect("bounds");
    for (row, values) in data.iter().enumerate() {
        for (column, value) in values.iter().enumerate() {
            array
                .change_array2()
                .set_value(row as i32 + 1, column as i32 + 1, value.clone())
                .expect("set");
        }
    }
    Handle::new(array)
}

/// `set_up`: the tools of the fixture.
fn tools() -> (StepToTopoDSTool, StepToTopoDSNMTool) {
    let mut tool = StepToTopoDSTool::new();
    tool.init(&DataMap::default(), &TransferTransientProcess::new(100));
    let mut nm_tool = StepToTopoDSNMTool::new();
    nm_tool.set_active(false);
    (tool, nm_tool)
}

fn triangulated_face(
    points: &[[f64; 3]],
    pnmax: i32,
    normals: &[[f64; 3]],
    pnindex: &[i32],
    triangles: &[[i32; 3]],
) -> Handle<StepVisualTessellatedFace> {
    let face = Handle::new(StepVisualTriangulatedFace::new());
    let normals = if normals.is_empty() {
        Handle::default()
    } else {
        rows(normals)
    };
    let pnindex = if pnindex.is_empty() {
        Handle::default()
    } else {
        shared(pnindex)
    };
    face.init(
        &name("face"),
        &coords(points),
        pnmax,
        &normals,
        false,
        &StepVisualFaceOrSurface::default(),
        &pnindex,
        &rows(triangles),
    );
    face.cast()
}

fn surface_set(
    points: &[[f64; 3]],
    pnmax: i32,
    pnindex: &[i32],
    triangles: &[[i32; 3]],
) -> Handle<StepVisualTessellatedSurfaceSet> {
    let set = Handle::new(StepVisualTriangulatedSurfaceSet::new());
    let pnindex = if pnindex.is_empty() {
        Handle::default()
    } else {
        shared(pnindex)
    };
    set.init(
        &name("surfset"),
        &coords(points),
        pnmax,
        &Handle::default(),
        &pnindex,
        &rows(triangles),
    );
    set.cast()
}

fn complex_face(
    points: &[[f64; 3]],
    strips: &[&[i32]],
    fans: &[&[i32]],
) -> Handle<StepVisualTessellatedFace> {
    let lists = |values: &[&[i32]]| -> Handle<HArray1<Handle<StandardTransient>>> {
        if values.is_empty() {
            return Handle::default();
        }
        let items: Vec<Handle<StandardTransient>> =
            values.iter().map(|list| shared(list).cast()).collect();
        shared(&items)
    };
    let face = Handle::new(StepVisualComplexTriangulatedFace::new());
    face.init(
        &name("complex_face"),
        &coords(points),
        points.len() as i32,
        &Handle::default(),
        false,
        &StepVisualFaceOrSurface::default(),
        &Handle::default(),
        &lists(strips),
        &lists(fans),
    );
    face.cast()
}

fn mesh_of(translator: &StepToTopoDSTranslateFace) -> Handle<Triangulation> {
    assert!(translator.is_done());
    let face = topods::face(translator.value().expect("value")).expect("face");
    let (mesh, _) = BRepTool::triangulation(&face, mesh_purpose::NONE).expect("triangulation");
    assert!(mesh.is_some());
    mesh
}

fn translate_face(face: &Handle<StepVisualTessellatedFace>) -> Handle<Triangulation> {
    let (mut tool, mut nm_tool) = tools();
    let mut translator = StepToTopoDSTranslateFace::new();
    let mut has_geom = false;
    translator
        .init_tessellated_face(
            face,
            &mut tool,
            &mut nm_tool,
            false,
            &mut has_geom,
            &StepDataFactors::new(),
        )
        .expect("init");
    mesh_of(&translator)
}

fn translate_set(
    set: &Handle<StepVisualTessellatedSurfaceSet>,
    factors: &StepDataFactors,
) -> Handle<Triangulation> {
    let (mut tool, mut nm_tool) = tools();
    let mut translator = StepToTopoDSTranslateFace::new();
    translator
        .init_tessellated_surface_set(set, &mut tool, &mut nm_tool, factors)
        .expect("init");
    mesh_of(&translator)
}

fn assert_near(found: f64, expected: f64, tolerance: f64) {
    assert!(
        (found - expected).abs() <= tolerance,
        "{found} expected {expected}"
    );
}

const UNIT_SQUARE: [[f64; 3]; 4] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
];
const UNIT_TRIANGLE: [[f64; 3]; 3] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];

#[test]
fn triangulated_face_direct_nodes() {
    let mesh = translate_face(&triangulated_face(
        &UNIT_SQUARE,
        4,
        &[],
        &[],
        &[[1, 2, 3], [1, 3, 4]],
    ));
    assert_eq!(mesh.nb_nodes(), 4);
    assert_eq!(mesh.nb_triangles(), 2);
    assert!(!mesh.has_normals());
    assert_near(mesh.node(1).unwrap().x(), 0.0, 1e-12);
    assert_near(mesh.node(2).unwrap().x(), 1.0, 1e-12);
    assert_near(mesh.node(3).unwrap().y(), 1.0, 1e-12);
    let triangle = mesh.triangle(1).unwrap();
    assert_eq!(
        (
            triangle.value(1).unwrap(),
            triangle.value(2).unwrap(),
            triangle.value(3).unwrap()
        ),
        (1, 2, 3)
    );
}

#[test]
fn triangulated_face_with_pnindex() {
    // 4 points defined but pnindex selects only 3 of them.
    let points = [
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0],
        [10.0, 10.0, 0.0],
        [0.0, 10.0, 0.0],
    ];
    let mesh = translate_face(&triangulated_face(
        &points,
        4,
        &[],
        &[2, 3, 4],
        &[[1, 2, 3]],
    ));
    assert_eq!(mesh.nb_nodes(), 3);
    assert_eq!(mesh.nb_triangles(), 1);
    assert_near(mesh.node(1).unwrap().x(), 10.0, 1e-12);
    assert_near(mesh.node(1).unwrap().y(), 0.0, 1e-12);
    assert_near(mesh.node(2).unwrap().x(), 10.0, 1e-12);
    assert_near(mesh.node(2).unwrap().y(), 10.0, 1e-12);
    assert_near(mesh.node(3).unwrap().x(), 0.0, 1e-12);
    assert_near(mesh.node(3).unwrap().y(), 10.0, 1e-12);
}

#[test]
fn triangulated_surface_set_direct_nodes() {
    let points = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [1.0, 2.0, 0.0]];
    let mesh = translate_set(
        &surface_set(&points, 3, &[], &[[1, 2, 3]]),
        &StepDataFactors::new(),
    );
    assert_eq!(mesh.nb_nodes(), 3);
    assert_eq!(mesh.nb_triangles(), 1);
    assert_near(mesh.node(1).unwrap().x(), 0.0, 1e-12);
    assert_near(mesh.node(2).unwrap().x(), 2.0, 1e-12);
    assert_near(mesh.node(3).unwrap().y(), 2.0, 1e-12);
}

#[test]
fn triangulated_surface_set_with_pnindex() {
    let points = [
        [0.0, 0.0, 0.0],
        [5.0, 0.0, 0.0],
        [5.0, 5.0, 0.0],
        [0.0, 5.0, 0.0],
    ];
    let mesh = translate_set(
        &surface_set(&points, 4, &[1, 3, 4], &[[1, 2, 3]]),
        &StepDataFactors::new(),
    );
    assert_eq!(mesh.nb_nodes(), 3);
    assert_near(mesh.node(1).unwrap().x(), 0.0, 1e-12);
    assert_near(mesh.node(2).unwrap().x(), 5.0, 1e-12);
    assert_near(mesh.node(2).unwrap().y(), 5.0, 1e-12);
    assert_near(mesh.node(3).unwrap().x(), 0.0, 1e-12);
    assert_near(mesh.node(3).unwrap().y(), 5.0, 1e-12);
}

#[test]
fn triangulated_face_with_normals() {
    let normals = [[0.0, 0.0, 1.0]; 3];
    let mesh = translate_face(&triangulated_face(
        &UNIT_TRIANGLE,
        3,
        &normals,
        &[],
        &[[1, 2, 3]],
    ));
    assert!(mesh.has_normals());
    assert_near(mesh.normal(1).unwrap().z(), 1.0, 1e-12);
}

#[test]
fn triangulated_face_single_normal() {
    let mesh = translate_face(&triangulated_face(
        &UNIT_TRIANGLE,
        3,
        &[[0.0, 1.0, 0.0]],
        &[],
        &[[1, 2, 3]],
    ));
    assert!(mesh.has_normals());
    // All 3 nodes should share the same normal (0,1,0).
    for rank in 1..=3 {
        let normal = mesh.normal(rank).unwrap();
        assert_near(normal.x(), 0.0, 1e-12);
        assert_near(normal.y(), 1.0, 1e-12);
        assert_near(normal.z(), 0.0, 1e-12);
    }
}

#[test]
fn triangulated_surface_set_length_factor() {
    let mut factors = StepDataFactors::new();
    factors.initialize_factors(25.4, 1.0, 1.0);
    let mesh = translate_set(&surface_set(&UNIT_TRIANGLE, 3, &[], &[[1, 2, 3]]), &factors);
    // Coordinates should be scaled by length factor 25.4.
    assert_near(mesh.node(2).unwrap().x(), 25.4, 1e-10);
    assert_near(mesh.node(3).unwrap().y(), 25.4, 1e-10);
}

const STRIP_POINTS: [[f64; 3]; 4] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
];

#[test]
fn complex_triangulated_face_triangle_strips() {
    let mesh = translate_face(&complex_face(&STRIP_POINTS, &[&[1, 2, 3, 4]], &[]));
    assert_eq!(mesh.nb_nodes(), 4);
    assert_eq!(mesh.nb_triangles(), 2);
}

#[test]
fn complex_triangulated_face_triangle_fans() {
    let points = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
    ];
    let mesh = translate_face(&complex_face(&points, &[], &[&[1, 2, 3, 4, 5]]));
    assert_eq!(mesh.nb_nodes(), 5);
    assert_eq!(mesh.nb_triangles(), 3);
}

#[test]
fn null_input_tessellated_face() {
    let (mut tool, mut nm_tool) = tools();
    let mut translator = StepToTopoDSTranslateFace::new();
    let mut has_geom = false;
    translator
        .init_tessellated_face(
            &Handle::default(),
            &mut tool,
            &mut nm_tool,
            false,
            &mut has_geom,
            &StepDataFactors::new(),
        )
        .expect("init");
    assert!(!translator.is_done());
}

#[test]
fn null_input_tessellated_surface_set() {
    let (mut tool, mut nm_tool) = tools();
    let mut translator = StepToTopoDSTranslateFace::new();
    translator
        .init_tessellated_surface_set(
            &Handle::default(),
            &mut tool,
            &mut nm_tool,
            &StepDataFactors::new(),
        )
        .expect("init");
    assert!(!translator.is_done());
}

#[test]
fn complex_triangulated_face_degenerate_strip() {
    // Only 1 non-degenerate triangle from strip [1,2,3,3].
    let mesh = translate_face(&complex_face(&STRIP_POINTS, &[&[1, 2, 3, 3]], &[]));
    assert_eq!(mesh.nb_triangles(), 1);
}

#[test]
fn a_value_before_a_result_raises_not_done() {
    assert_eq!(
        StepToTopoDSBuilder::new()
            .value()
            .err()
            .map(|failure| failure.exception_type()),
        Some("StdFail_NotDone")
    );
    assert!(StepToTopoDSTranslateShell::new().value().is_err());
    assert!(StepToTopoDSTranslateSolid::new().value().is_err());
    assert!(StepToTopoDSTranslateEdge::new().value().is_err());
    assert!(StepToTopoDSTranslateVertex::new().value().is_err());
}

#[test]
fn an_edge_is_made_on_a_3d_curve_between_its_vertices() {
    let factors = StepDataFactors::new();
    let axis = Line3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    );
    let line = GeomToStepMakeLine::from_lin(&axis, &factors)
        .unwrap()
        .value()
        .unwrap();
    let builder = BRepBuilder::default();
    let (mut start, mut end) = (Vertex::default(), Vertex::default());
    builder
        .make_vertex_at(&mut start, Point3::new(2.0, 0.0, 0.0), 1e-7)
        .unwrap();
    builder
        .make_vertex_at(&mut end, Point3::new(10.0, 0.0, 0.0), 1e-7)
        .unwrap();
    let (mut tool, _) = tools();
    let mut translator = StepToTopoDSTranslateEdge::new();
    let mut edge = Edge::default();
    translator
        .make_from_curve_3d(
            &line.cast(),
            &Handle::new(StepShapeEdgeCurve::new()),
            &Handle::new(StepShapeVertex::new()),
            1e-7,
            &mut edge,
            &mut start,
            &mut end,
            &mut tool,
            &factors,
        )
        .unwrap();
    assert!(!edge.is_null());
    let (first, last) = BRepTool::range(&edge).unwrap();
    assert_near(first, 2.0, 1e-9);
    assert_near(last, 10.0, 1e-9);
}

#[test]
fn a_new_root_has_the_confusion_tolerances() {
    let mut root = StepToTopoDSRoot::new();
    assert!(!root.is_done());
    assert_eq!(root.precision(), 1.0e-7);
    assert_eq!(root.max_tol(), 1.0e-7);
    root.set_precision(0.5);
    root.set_max_tol(2.0);
    assert_eq!((root.precision(), root.max_tol()), (0.5, 2.0));
}

#[test]
fn a_point_pair_is_the_same_either_way() {
    let (a, b, c) = (
        Handle::new(StepGeomCartesianPoint::new()),
        Handle::new(StepGeomCartesianPoint::new()),
        Handle::new(StepGeomCartesianPoint::new()),
    );
    assert!(StepToTopoDSPointPair::new(&a, &b) == StepToTopoDSPointPair::new(&b, &a));
    assert!(StepToTopoDSPointPair::new(&a, &b) != StepToTopoDSPointPair::new(&a, &c));
}

#[test]
fn the_tool_keeps_what_it_is_given() {
    let (mut tool, _) = tools();
    let entity = Handle::new(martensite_tkdestep::step_shape::StepShapeVertexPoint::new());
    assert!(!tool.is_bound(&entity.cast()));
    assert!(tool.find(&entity.cast()).is_err());
    let mut shape = martensite_tkbrep::topods::Vertex::default();
    martensite_tkbrep::brep::BRepBuilder::default().make_vertex(&mut shape);
    tool.bind(&entity.cast(), &shape.clone().into_shape())
        .expect("bind");
    assert!(tool.is_bound(&entity.cast()));
    assert!(tool.find(&entity.cast()).expect("find").is_same(&shape));
}

#[test]
fn the_error_codes_decode() {
    let text = StepToTopoDS::decode_builder_error(StepToTopoDSBuilderError::Done);
    assert_eq!(text.string().as_bytes(), b"Builder Done");
    let _: Shape = Shape::default();
}
