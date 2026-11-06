//! Tessellated faces and surface sets translated to faces with a triangulation: node indexing, normals,
//! scaling, strips, fans and null input.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods;
use martensite_tkdestep::step_data::StepDataFactors;
use martensite_tkdestep::step_to_topo_ds::{
    StepToTopoDSNMTool, StepToTopoDSTool, StepToTopoDSTranslateFace,
};
use martensite_tkdestep::step_visual::{
    StepVisualComplexTriangulatedFace, StepVisualCoordinatesList, StepVisualFaceOrSurface,
    StepVisualTriangulatedFace, StepVisualTriangulatedSurfaceSet,
};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Xyz;
use martensite_tkmath::poly::{Triangulation, mesh_purpose};
use martensite_tkxsbase::transfer::TransferTransientProcess;

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// An array from 1, with the values.
fn array<T: Default + Clone + Send + Sync + 'static>(values: &[T]) -> Handle<HArray1<T>> {
    let made = Handle::new(HArray1::with_bounds(1, values.len() as i32).unwrap());
    {
        let mut items = made.get().unwrap().change_array1();
        for (index, value) in values.iter().enumerate() {
            items.set_value(index as i32 + 1, value.clone()).unwrap();
        }
    }
    made
}

fn coords(points: &[[f64; 3]]) -> Handle<StepVisualCoordinatesList> {
    let values: Vec<Xyz> = points
        .iter()
        .map(|[x, y, z]| Xyz::new(*x, *y, *z))
        .collect();
    let list = Handle::new(StepVisualCoordinatesList::new());
    list.init(&text("coords"), &array(&values));
    list
}

/// A table of rows of three, the shape triangles and normals have.
fn rows<T: Default + Clone + Send + Sync + 'static>(data: &[[T; 3]]) -> Handle<HArray2<T>> {
    let made = Handle::new(HArray2::with_bounds(1, data.len() as i32, 1, 3).unwrap());
    {
        let mut items = made.get().unwrap().change_array2();
        for (row, values) in data.iter().enumerate() {
            for (column, value) in values.iter().enumerate() {
                items
                    .set_value(row as i32 + 1, column as i32 + 1, value.clone())
                    .unwrap();
            }
        }
    }
    made
}

fn triangulated_face(
    points: &[[f64; 3]],
    pnmax: i32,
    normals: Handle<HArray2<f64>>,
    pnindex: Handle<HArray1<i32>>,
    triangles: &[[i32; 3]],
) -> Handle<StepVisualTriangulatedFace> {
    let face = Handle::new(StepVisualTriangulatedFace::new());
    face.init(
        &text("face"),
        &coords(points),
        pnmax,
        &normals,
        false,
        &StepVisualFaceOrSurface::default(),
        &pnindex,
        &rows(triangles),
    );
    face
}

fn surface_set(
    points: &[[f64; 3]],
    pnmax: i32,
    pnindex: Handle<HArray1<i32>>,
    triangles: &[[i32; 3]],
) -> Handle<StepVisualTriangulatedSurfaceSet> {
    let set = Handle::new(StepVisualTriangulatedSurfaceSet::new());
    set.init(
        &text("surfset"),
        &coords(points),
        pnmax,
        &Handle::default(),
        &pnindex,
        &rows(triangles),
    );
    set
}

fn strips(list: &[&[i32]]) -> Handle<HArray1<Handle<StandardTransient>>> {
    let entries: Vec<Handle<StandardTransient>> =
        list.iter().map(|indices| array(indices).cast()).collect();
    array(&entries)
}

fn complex_face(
    points: &[[f64; 3]],
    pnmax: i32,
    strips: Handle<HArray1<Handle<StandardTransient>>>,
    fans: Handle<HArray1<Handle<StandardTransient>>>,
) -> Handle<StepVisualComplexTriangulatedFace> {
    let face = Handle::new(StepVisualComplexTriangulatedFace::new());
    face.init(
        &text("complex_face"),
        &coords(points),
        pnmax,
        &Handle::default(),
        false,
        &StepVisualFaceOrSurface::default(),
        &Handle::default(),
        &strips,
        &fans,
    );
    face
}

/// The fixture: a tool over an empty map and an inactive tool of non-manifold topology.
struct Fixture {
    tool: StepToTopoDSTool,
    nm_tool: StepToTopoDSNMTool,
}

impl Fixture {
    fn new() -> Self {
        let mut tool = StepToTopoDSTool::new();
        tool.init(&Default::default(), &TransferTransientProcess::new(10000));
        let mut nm_tool = StepToTopoDSNMTool::new();
        nm_tool.set_active(false);
        Self { tool, nm_tool }
    }
    fn face(&mut self, face: &Handle<StepVisualTriangulatedFace>) -> StepToTopoDSTranslateFace {
        let mut has_geom = false;
        StepToTopoDSTranslateFace::with_tessellated_face(
            &face.cast(),
            &mut self.tool,
            &mut self.nm_tool,
            false,
            &mut has_geom,
            &StepDataFactors::default(),
        )
        .unwrap()
    }
    fn complex(
        &mut self,
        face: &Handle<StepVisualComplexTriangulatedFace>,
    ) -> StepToTopoDSTranslateFace {
        let mut has_geom = false;
        StepToTopoDSTranslateFace::with_tessellated_face(
            &face.cast(),
            &mut self.tool,
            &mut self.nm_tool,
            false,
            &mut has_geom,
            &StepDataFactors::default(),
        )
        .unwrap()
    }
    fn set(
        &mut self,
        set: &Handle<StepVisualTriangulatedSurfaceSet>,
        factors: &StepDataFactors,
    ) -> StepToTopoDSTranslateFace {
        StepToTopoDSTranslateFace::with_tessellated_surface_set(
            &set.cast(),
            &mut self.tool,
            &mut self.nm_tool,
            factors,
        )
        .unwrap()
    }
}

/// `getMesh`: the triangulation of the translated face.
fn mesh_of(translator: &StepToTopoDSTranslateFace) -> Handle<Triangulation> {
    assert!(translator.is_done());
    let face = topods::face(translator.value().unwrap()).unwrap();
    let (mesh, _) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
    assert!(mesh.is_some());
    mesh
}

fn near(value: f64, expected: f64, tolerance: f64) -> bool {
    (value - expected).abs() <= tolerance
}

const SQUARE: [[f64; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [0., 1., 0.]];
const CORNER: [[f64; 3]; 3] = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];
const QUAD: [[f64; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];

#[test]
fn triangulated_face_direct_nodes() {
    let mut fixture = Fixture::new();
    let face = triangulated_face(
        &SQUARE,
        4,
        Handle::default(),
        Handle::default(),
        &[[1, 2, 3], [1, 3, 4]],
    );
    let mesh = mesh_of(&fixture.face(&face));
    let mesh = mesh.get().unwrap();
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (4, 2));
    assert!(!mesh.has_normals());
    assert!(near(mesh.node(1).unwrap().x(), 0.0, 1e-12));
    assert!(near(mesh.node(2).unwrap().x(), 1.0, 1e-12));
    assert!(near(mesh.node(3).unwrap().y(), 1.0, 1e-12));
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
    // 4 points defined but pnindex selects only 3 of them: nodes 1, 2, 3 map to coordinates 2, 3, 4.
    let points = [[0., 0., 0.], [10., 0., 0.], [10., 10., 0.], [0., 10., 0.]];
    let mut fixture = Fixture::new();
    let mesh = mesh_of(&fixture.face(&triangulated_face(
        &points,
        4,
        Handle::default(),
        array(&[2, 3, 4]),
        &[[1, 2, 3]],
    )));
    let mesh = mesh.get().unwrap();
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (3, 1));
    assert!(
        near(mesh.node(1).unwrap().x(), 10.0, 1e-12) && near(mesh.node(1).unwrap().y(), 0.0, 1e-12)
    );
    assert!(
        near(mesh.node(2).unwrap().x(), 10.0, 1e-12)
            && near(mesh.node(2).unwrap().y(), 10.0, 1e-12)
    );
    assert!(
        near(mesh.node(3).unwrap().x(), 0.0, 1e-12) && near(mesh.node(3).unwrap().y(), 10.0, 1e-12)
    );
}

#[test]
fn triangulated_surface_set_direct_nodes() {
    let points = [[0., 0., 0.], [2., 0., 0.], [1., 2., 0.]];
    let mut fixture = Fixture::new();
    let mesh = mesh_of(&fixture.set(
        &surface_set(&points, 3, Handle::default(), &[[1, 2, 3]]),
        &StepDataFactors::default(),
    ));
    let mesh = mesh.get().unwrap();
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (3, 1));
    assert!(near(mesh.node(1).unwrap().x(), 0.0, 1e-12));
    assert!(near(mesh.node(2).unwrap().x(), 2.0, 1e-12));
    assert!(near(mesh.node(3).unwrap().y(), 2.0, 1e-12));
}

#[test]
fn triangulated_surface_set_with_pnindex() {
    // Pnindex selects 3 out of 4 coordinates (indices 1, 3, 4).
    let points = [[0., 0., 0.], [5., 0., 0.], [5., 5., 0.], [0., 5., 0.]];
    let mut fixture = Fixture::new();
    let mesh = mesh_of(&fixture.set(
        &surface_set(&points, 4, array(&[1, 3, 4]), &[[1, 2, 3]]),
        &StepDataFactors::default(),
    ));
    let mesh = mesh.get().unwrap();
    assert_eq!(mesh.nb_nodes(), 3);
    assert!(near(mesh.node(1).unwrap().x(), 0.0, 1e-12));
    assert!(
        near(mesh.node(2).unwrap().x(), 5.0, 1e-12) && near(mesh.node(2).unwrap().y(), 5.0, 1e-12)
    );
    assert!(
        near(mesh.node(3).unwrap().x(), 0.0, 1e-12) && near(mesh.node(3).unwrap().y(), 5.0, 1e-12)
    );
}

#[test]
fn triangulated_face_with_normals() {
    let mut fixture = Fixture::new();
    let normals = rows(&[[0., 0., 1.], [0., 0., 1.], [0., 0., 1.]]);
    let mesh = mesh_of(&fixture.face(&triangulated_face(
        &CORNER,
        3,
        normals,
        Handle::default(),
        &[[1, 2, 3]],
    )));
    let mesh = mesh.get().unwrap();
    assert!(mesh.has_normals());
    assert!(near(mesh.normal(1).unwrap().z(), 1.0, 1e-12));
}

#[test]
fn triangulated_face_single_normal() {
    let mut fixture = Fixture::new();
    let normals = rows(&[[0., 1., 0.]]);
    let mesh = mesh_of(&fixture.face(&triangulated_face(
        &CORNER,
        3,
        normals,
        Handle::default(),
        &[[1, 2, 3]],
    )));
    let mesh = mesh.get().unwrap();
    assert!(mesh.has_normals());
    // All 3 nodes should share the same normal (0,1,0).
    for node in 1..=3 {
        let normal = mesh.normal(node).unwrap();
        assert!(
            near(normal.x(), 0.0, 1e-12)
                && near(normal.y(), 1.0, 1e-12)
                && near(normal.z(), 0.0, 1e-12)
        );
    }
}

#[test]
fn triangulated_surface_set_length_factor() {
    let mut factors = StepDataFactors::new();
    factors.initialize_factors(25.4, 1.0, 1.0);
    let mut fixture = Fixture::new();
    let mesh = mesh_of(&fixture.set(
        &surface_set(&CORNER, 3, Handle::default(), &[[1, 2, 3]]),
        &factors,
    ));
    let mesh = mesh.get().unwrap();
    // Coordinates should be scaled by length factor 25.4.
    assert!(near(mesh.node(2).unwrap().x(), 25.4, 1e-10));
    assert!(near(mesh.node(3).unwrap().y(), 25.4, 1e-10));
}

#[test]
fn complex_triangulated_face_triangle_strips() {
    // Triangle strip: [1, 2, 3, 4] produces 2 triangles.
    let mut fixture = Fixture::new();
    let mesh = mesh_of(&fixture.complex(&complex_face(
        &QUAD,
        4,
        strips(&[&[1, 2, 3, 4]]),
        Handle::default(),
    )));
    let mesh = mesh.get().unwrap();
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (4, 2));
}

#[test]
fn complex_triangulated_face_triangle_fans() {
    // Triangle fan: [1, 2, 3, 4, 5] with center at vertex 1 produces 3 triangles.
    let points = [
        [0., 0., 0.],
        [1., 0., 0.],
        [1., 1., 0.],
        [0., 1., 0.],
        [-1., 0., 0.],
    ];
    let mut fixture = Fixture::new();
    let mesh = mesh_of(&fixture.complex(&complex_face(
        &points,
        5,
        Handle::default(),
        strips(&[&[1, 2, 3, 4, 5]]),
    )));
    let mesh = mesh.get().unwrap();
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (5, 3));
}

#[test]
fn null_input_tessellated_face() {
    let mut fixture = Fixture::new();
    let mut has_geom = false;
    let translator = StepToTopoDSTranslateFace::with_tessellated_face(
        &Handle::default(),
        &mut fixture.tool,
        &mut fixture.nm_tool,
        false,
        &mut has_geom,
        &StepDataFactors::default(),
    )
    .unwrap();
    assert!(!translator.is_done());
}

#[test]
fn null_input_tessellated_surface_set() {
    let mut fixture = Fixture::new();
    let translator = StepToTopoDSTranslateFace::with_tessellated_surface_set(
        &Handle::default(),
        &mut fixture.tool,
        &mut fixture.nm_tool,
        &StepDataFactors::default(),
    )
    .unwrap();
    assert!(!translator.is_done());
}

#[test]
fn complex_triangulated_face_degenerate_strip() {
    // Strip with degenerate: [1, 2, 3, 3] - the second triangle (2,3,3) is degenerate.
    let mut fixture = Fixture::new();
    let mesh = mesh_of(&fixture.complex(&complex_face(
        &QUAD,
        4,
        strips(&[&[1, 2, 3, 3]]),
        Handle::default(),
    )));
    // Only 1 non-degenerate triangle from strip [1,2,3,3].
    assert_eq!(mesh.get().unwrap().nb_triangles(), 1);
}
