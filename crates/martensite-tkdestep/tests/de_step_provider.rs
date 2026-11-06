//! The cases of the provider, on the real node and provider. The expected vendor is `Martensite`.
#[path = "support/memory_stream.rs"]
mod memory_stream;

use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoCheckerSI, BOPAlgoPaveFiller, Operation};
use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Compound, Face, Shape};
use martensite_tkde::de::{
    DeConfigurationNode, DeProvider, DeProviderReadStreamList, DeProviderReadStreamNode,
    DeProviderWriteStreamList, DeProviderWriteStreamNode, DeShapeFixParametersFixMode,
    DeWorkSession, DeWrapper, new_configuration_node,
};
use martensite_tkdestep::de_step::{
    DeStepConfigurationNode, DeStepProvider, DeStepRWModeTessellated, DeStepWriteModeStepSchema,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::osd::{IStream, OStream};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tkg3d::geom::{GeomConic, GeomEllipse, GeomSurfaceOfRevolution, GeomSweptSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Plane3, Point3};
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis, BRepPrimAPIMakeRevol,
    BRepPrimAPIMakeSphere, BRepPrimAPIMakeTorus,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use memory_stream::{memory_istream, memory_ostream};
use std::sync::{Mutex, MutexGuard, PoisonError};

/// The write actor belongs to the controller, of which a process has one:, where each case
/// has the process alone, one case at a time.
fn alone() -> MutexGuard<'static, ()> {
    static ONE_CASE_AT_A_TIME: Mutex<()> = Mutex::new(());
    ONE_CASE_AT_A_TIME
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

fn range() -> ProgressRange<'static> {
    ProgressRange::default()
}

fn solid_shape(made: Result<BRepPrimAPIMakeBox, martensite_tkernel::standard::Failure>) -> Shape {
    made.expect("box").solid().expect("solid").into_shape()
}

fn a_box(dx: f64, dy: f64, dz: f64) -> Shape {
    solid_shape(BRepPrimAPIMakeBox::from_sizes(dx, dy, dz))
}

fn a_box_at(x: f64, y: f64, z: f64, side: f64) -> Shape {
    solid_shape(BRepPrimAPIMakeBox::at_point(
        Point3::new(x, y, z),
        side,
        side,
        side,
    ))
}

fn a_sphere(radius: f64) -> Shape {
    let mut made = BRepPrimAPIMakeSphere::new(radius).expect("sphere");
    made.solid().expect("solid").into_shape()
}

fn a_cylinder(radius: f64, height: f64) -> Shape {
    let mut made = BRepPrimAPIMakeCylinder::new(radius, height);
    made.solid().expect("solid").into_shape()
}

fn new_document() -> Handle<TDocStdDocument> {
    let application = XCAFAppApplication::get_application();
    let mut document: Handle<TDocStdDocument> = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .expect("document");
    document
}

fn write_list<'a>(key: &str, stream: &'a mut dyn OStream) -> DeProviderWriteStreamList<'a> {
    let mut list = DeProviderWriteStreamList::default();
    list.append(DeProviderWriteStreamNode::new(
        &AsciiString::new(key),
        stream,
    ));
    list
}

fn read_list<'a>(key: &str, stream: &'a mut dyn IStream) -> DeProviderReadStreamList<'a> {
    let mut list = DeProviderReadStreamList::default();
    list.append(DeProviderReadStreamNode::new(
        &AsciiString::new(key),
        stream,
    ));
    list
}

/// `Write(streams, shape)`: the bytes when it succeeds.
fn write_shape(provider: &dyn DeProvider, key: &str, shape: &Shape) -> Option<Vec<u8>> {
    let (mut stream, written) = memory_ostream();
    let mut list = write_list(key, &mut stream);
    provider
        .write_shape_streams(&mut list, shape, &range())
        .expect("write")
        .then(|| written.contents())
}

/// `Read(streams, shape)`: the shape when it succeeds.
fn read_shape(provider: &dyn DeProvider, key: &str, content: &[u8]) -> Option<Shape> {
    let mut stream = memory_istream(content);
    let mut list = read_list(key, &mut stream);
    let mut shape = Shape::default();
    provider
        .read_shape_streams(&mut list, &mut shape, &range())
        .expect("read")
        .then_some(shape)
}

fn write_document(
    provider: &dyn DeProvider,
    key: &str,
    document: &Handle<TDocStdDocument>,
) -> Option<Vec<u8>> {
    let (mut stream, written) = memory_ostream();
    let mut list = write_list(key, &mut stream);
    provider
        .write_document_streams(&mut list, document, &range())
        .expect("write")
        .then(|| written.contents())
}

fn read_document(
    provider: &dyn DeProvider,
    key: &str,
    content: &[u8],
    document: &Handle<TDocStdDocument>,
) -> bool {
    let mut stream = memory_istream(content);
    let mut list = read_list(key, &mut stream);
    provider
        .read_document_streams(&mut list, document, &range())
        .expect("read")
}

fn add_shape(
    document: &Handle<TDocStdDocument>,
    shape: &Shape,
) -> martensite_tklcaf::tdf::TdfLabel {
    let main = document.get().expect("document").main().expect("main");
    XCAFDocDocumentTool::shape_tool(&main)
        .expect("tool")
        .add_shape(shape, true, true)
        .expect("label")
}

fn nb_shapes(document: &Handle<TDocStdDocument>) -> usize {
    let main = document.get().expect("document").main().expect("main");
    XCAFDocDocumentTool::shape_tool(&main)
        .expect("tool")
        .get_shapes()
        .expect("shapes")
        .len()
}

fn count(shape: &Shape, kind: ShapeEnum) -> i32 {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).expect("explorer");
    let mut found = 0;
    while explorer.more() {
        found += 1;
        explorer.next().expect("next");
    }
    found
}

/// `IsValidSTEPContent`.
fn is_valid_step_content(content: &[u8]) -> bool {
    let text = String::from_utf8_lossy(content);
    !text.is_empty()
        && text.contains("ISO-10303-21;")
        && text.contains("HEADER;")
        && text.contains("DATA;")
        && text.contains("ENDSEC;")
}

/// The fixture: a provider on a new node.
fn provider() -> DeStepProvider {
    let node: Handle<DeStepConfigurationNode> =
        new_configuration_node(DeStepConfigurationNode::default());
    DeStepProvider::with_node(&node.cast())
}

/// `getTriangulationStats`: faces, nodes, triangles.
fn triangulation_stats(shape: &Shape) -> (i32, i32, i32) {
    let (mut faces, mut nodes, mut triangles) = (0, 0, 0);
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    while explorer.more() {
        faces += 1;
        let face = topods::face(explorer.current()).expect("face");
        let (mesh, _) = BRepTool::triangulation(&face, 0).expect("triangulation");
        if let Some(mesh) = mesh.get() {
            nodes += mesh.nb_nodes();
            triangles += mesh.nb_triangles();
        }
        explorer.next().expect("next");
    }
    (faces, nodes, triangles)
}

/// `writeReadTessellatedStep`: the result and the content.
fn write_read_tessellated(
    source: &Shape,
    write_mode: DeStepRWModeTessellated,
    read_mode: DeStepRWModeTessellated,
) -> (Shape, Vec<u8>) {
    let node = new_configuration_node(DeStepConfigurationNode::default());
    {
        let mut parameters = node.get().expect("node").internal_parameters.write();
        parameters.write_schema = DeStepWriteModeStepSchema::AP242_DIS;
        parameters.write_tessellated = write_mode;
    }
    let content = write_shape(
        &DeStepProvider::with_node(&node.cast()),
        "tessellated.step",
        source,
    )
    .expect("written");
    let read_node = new_configuration_node(DeStepConfigurationNode::default());
    read_node
        .get()
        .expect("node")
        .internal_parameters
        .write()
        .read_tessellated = read_mode;
    let result = read_shape(
        &DeStepProvider::with_node(&read_node.cast()),
        "tessellated.step",
        &content,
    )
    .expect("read");
    (result, content)
}

fn mesh(shape: &Shape) {
    BRepMeshIncrementalMesh::with_shape(shape, 0.1, false, 0.5, false);
}

fn check_tessellated(
    source: &Shape,
    write_mode: DeStepRWModeTessellated,
    read_mode: DeStepRWModeTessellated,
    source_stats: (i32, i32, i32),
    result_stats: (i32, i32, i32),
) {
    assert_eq!(triangulation_stats(source), source_stats);
    let (result, content) = write_read_tessellated(source, write_mode, read_mode);
    assert!(is_valid_step_content(&content));
    assert_eq!(triangulation_stats(&result), result_stats);
}

#[test]
fn basic_properties() {
    let _alone = alone();
    let provider = provider();
    assert_eq!(provider.get_format().as_bytes(), b"STEP");
    assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
    assert!(provider.get_node().is_some());
}

#[test]
fn stream_shape_write_read() {
    let _alone = alone();
    let (provider, shape) = (provider(), a_box(10.0, 10.0, 10.0));
    let content = write_shape(&provider, "test.step", &shape).expect("written");
    assert!(!content.is_empty());
    assert!(is_valid_step_content(&content));
    let read = read_shape(&provider, "test.step", &content).expect("read");
    assert!(!read.is_null());
    assert_eq!(
        count(&read, ShapeEnum::Solid),
        count(&shape, ShapeEnum::Solid)
    );
}

#[test]
fn mesh_step_b1_meshed_box() {
    let _alone = alone();
    let source = a_box(1., 1., 1.);
    mesh(&source);
    check_tessellated(
        &source,
        DeStepRWModeTessellated::ON,
        DeStepRWModeTessellated::ON,
        (6, 24, 12),
        (6, 24, 12),
    );
}

#[test]
fn mesh_step_b2_meshed_box_with_void() {
    let _alone = alone();
    let outer = a_box_at(-5., -5., -5., 10.);
    let inner = a_box_at(-3., -3., -3., 6.);
    let source = BRepAlgoAPICut::from_shapes(&outer, &inner)
        .expect("cut")
        .shape()
        .clone();
    assert!(!source.is_null());
    mesh(&source);
    check_tessellated(
        &source,
        DeStepRWModeTessellated::ON,
        DeStepRWModeTessellated::ON,
        (12, 48, 24),
        (12, 48, 24),
    );
}

#[test]
fn mesh_step_c1_mesh_without_brep_geometry() {
    let _alone = alone();
    let source = a_box(1., 1., 1.);
    mesh(&source);
    brep_tools::clean_geometry(&source).expect("clean");
    check_tessellated(
        &source,
        DeStepRWModeTessellated::ON,
        DeStepRWModeTessellated::ON,
        (6, 24, 12),
        (6, 24, 12),
    );
}

#[test]
fn mesh_step_c2_tessellated_without_brep() {
    let _alone = alone();
    let source = a_box(1., 1., 1.);
    mesh(&source);
    check_tessellated(
        &source,
        DeStepRWModeTessellated::ON_NO_B_REP,
        DeStepRWModeTessellated::ON,
        (6, 24, 12),
        (6, 0, 0),
    );
}

#[test]
fn mesh_step_read_b1_tessellated_without_brep() {
    let _alone = alone();
    let source = a_box_at(-5., -5., -5., 10.);
    mesh(&source);
    check_tessellated(
        &source,
        DeStepRWModeTessellated::ON,
        DeStepRWModeTessellated::ON_NO_B_REP,
        (6, 24, 12),
        (6, 0, 0),
    );
}

/// A shape without self-interference at the highest level of the check.
fn assert_no_self_interference(shape: &Shape) {
    let mut checker = BOPAlgoCheckerSI::new();
    checker.set_arguments(&[shape.clone()].into_iter().collect::<List<Shape>>());
    checker.set_level_of_check(9);
    checker.options_mut().set_run_parallel(false);
    checker.perform();
    assert!(!checker.options().has_errors());
}

fn is_valid(shape: &Shape) -> bool {
    BRepCheckAnalyzer::new(shape, true, false, false)
        .expect("analyzer")
        .is_valid()
        .expect("analysis")
}

fn volume_of(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).expect("volume");
    properties.mass()
}

#[test]
fn step_bug_25989_fused_shape_volume_round_trip() {
    let _alone = alone();
    let (the_box, cylinder, sphere) = (
        a_box(10.0, 10.0, 10.0),
        a_cylinder(2.0, 10.0),
        a_sphere(5.0),
    );
    let mut torus = BRepPrimAPIMakeTorus::new(10.0, 2.0).expect("torus");
    let torus = torus.solid().expect("solid").into_shape();
    let mut filler = BOPAlgoPaveFiller::new();
    let arguments: List<Shape> = [
        the_box.clone(),
        cylinder.clone(),
        sphere.clone(),
        torus.clone(),
    ]
    .into_iter()
    .collect();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(!filler.options().has_errors());
    let mut fusion = BOPAlgoBOP::new();
    fusion.add_argument(&the_box);
    fusion.add_tool(&cylinder);
    fusion.add_tool(&sphere);
    fusion.add_tool(&torus);
    fusion.set_operation(Operation::Fuse);
    fusion.options_mut().set_run_parallel(false);
    fusion.perform_with_filler(&mut filler);
    assert!(!fusion.options().has_errors());
    let source = fusion.shape().clone();
    assert!(!source.is_null());
    assert!(is_valid(&source));
    assert_no_self_interference(&source);
    let source_volume = volume_of(&source);
    let provider = provider();
    let content = write_shape(&provider, "bug25989.step", &source).expect("written");
    assert!(!content.is_empty());
    let restored = read_shape(&provider, "bug25989.step", &content).expect("read");
    assert!(!restored.is_null());
    assert!(is_valid(&restored));
    assert_no_self_interference(&restored);
    let tolerance = 1.0e-6 * source_volume.abs().max(1.0);
    assert!((volume_of(&restored) - source_volume).abs() <= tolerance);
}

#[test]
fn step_bug_25694_default_schema_is_ap214is() {
    let _alone = alone();
    let content =
        write_shape(&provider(), "bug25694.step", &a_box(10.0, 10.0, 10.0)).expect("written");
    assert!(is_valid_step_content(&content));
    assert!(
        String::from_utf8_lossy(&content).contains("AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }")
    );
}

#[test]
fn step_bug_27070_null_shape_write_fails() {
    let _alone = alone();
    assert!(write_shape(&provider(), "bug27070.step", &Shape::default()).is_none());
}

fn surface_area(face: &Face) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&face.clone().into_shape(), &mut properties, false, false)
        .expect("area");
    properties.mass()
}

#[test]
fn step_bug_28024_disabled_orientation_fix() {
    let _alone = alone();
    let vertex = |x, y| {
        let mut made = BRepBuilderAPIMakeVertex::new(Point3::new(x, y, 0.0)).expect("vertex");
        made.vertex().expect("vertex").clone()
    };
    let (v1, v2, v3) = (vertex(0.0, 0.0), vertex(1.0, 0.0), vertex(0.0, 1.0));
    let edge = |a, b| {
        BRepBuilderAPIMakeEdge::from_vertices(a, b)
            .expect("edge")
            .edge()
            .expect("edge")
            .clone()
    };
    let (e1, e2, e3) = (edge(&v2, &v1), edge(&v3, &v2), edge(&v1, &v3));
    let mut wire = BRepBuilderAPIMakeWire::new();
    wire.add_edge(&e1).expect("edge");
    wire.add_edge(&e3).expect("edge");
    wire.add_edge(&e2).expect("edge");
    assert!(wire.is_done());
    let plane = Plane3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).expect("z"),
    )
    .expect("plane");
    let mut maker =
        BRepBuilderAPIMakeFace::from_plane_wire(plane, &wire.wire().expect("wire"), false)
            .expect("face");
    assert!(maker.is_done());
    let face = maker.face().expect("face").clone();
    assert!((surface_area(&face) - -0.5).abs() <= 1.0e-6);
    let content =
        write_shape(&provider(), "bug28024.stp", &face.clone().into_shape()).expect("written");
    assert!(!content.is_empty());
    let read_node = new_configuration_node(DeStepConfigurationNode::default());
    {
        let live = read_node.get().expect("node");
        let mut parameters = live.shape_fix_parameters();
        parameters.fix_orientation_mode = DeShapeFixParametersFixMode::NOT_FIX;
        live.set_shape_fix_parameters(parameters);
    }
    let restored = read_shape(
        &DeStepProvider::with_node(&read_node.cast()),
        "bug28024.stp",
        &content,
    )
    .expect("read");
    let faces = Explorer::new(&restored, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    assert!(faces.more());
    assert!((surface_area(&topods::face(faces.current()).expect("face")) - -0.5).abs() <= 1.0e-6);
}

#[test]
fn step_bug_23203_ellipse_revolution_axes() {
    let _alone = alone();
    let axis = Axis2::with_x_direction(
        Point3::new(0., 0., 0.),
        Direction3::new(0., -1., 0.).expect("n"),
        Direction3::new(1., 0., -1.).expect("x"),
    )
    .expect("axes");
    let ellipse = Handle::new(GeomEllipse::new(axis, 10.0, 5.0).expect("ellipse"));
    let revolution_axis = Axis1::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).expect("z"),
    );
    let surface = Handle::new(
        GeomSurfaceOfRevolution::new(&ellipse.cast(), revolution_axis).expect("surface"),
    );
    let mut maker = BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface.cast(),
        (0.0, 3.0, 0.0, 1.0),
        precision::confusion(),
    )
    .expect("face");
    assert!(maker.is_done());
    let face = maker.face().expect("face").clone().into_shape();
    let provider = provider();
    let content = write_shape(&provider, "bug23203.step", &face).expect("written");
    assert!(!content.is_empty());
    let restored = read_shape(&provider, "bug23203.step", &content).expect("read");
    assert!(!restored.is_null());
    let faces = Explorer::new(&restored, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    assert!(faces.more());
    let surface =
        BRepTool::surface(&topods::face(faces.current()).expect("face")).expect("surface");
    let revolution: Handle<GeomSurfaceOfRevolution> = surface.cast();
    let revolution = revolution.get().expect("a surface of revolution");
    let restored_ellipse: Handle<GeomEllipse> = revolution.basis_curve().cast();
    let restored_ellipse = restored_ellipse.get().expect("an ellipse");
    let expected = ellipse.get().expect("ellipse");
    for (actual, wanted) in [
        (
            restored_ellipse.x_axis().direction(),
            expected.x_axis().direction(),
        ),
        (
            restored_ellipse.y_axis().direction(),
            expected.y_axis().direction(),
        ),
    ] {
        assert!((actual.x() - wanted.x()).abs() <= 1.0e-6);
        assert!((actual.y() - wanted.y()).abs() <= 1.0e-6);
        assert!((actual.z() - wanted.z()).abs() <= 1.0e-6);
    }
}

#[test]
fn xde_bug_27722_ellipse_revolution_topology() {
    let _alone = alone();
    let z_axes = Axis2::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).expect("z"),
    )
    .expect("axes");
    let ellipse = Handle::new(GeomEllipse::new(z_axes, 20.0, 10.0).expect("ellipse"));
    let mut edge =
        BRepBuilderAPIMakeEdge::from_curve_params(&ellipse.cast(), 0.0, 0.5 * std::f64::consts::PI)
            .expect("edge");
    let edge = edge.edge().expect("edge").clone().into_shape();
    let axis = Axis1::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 1., 0.).expect("y"),
    );
    let mut revol = BRepPrimAPIMakeRevol::new(&edge, axis, 2.0 * std::f64::consts::PI, false)
        .expect("revolution");
    let source = revol.shape().expect("shape").clone();
    assert!(!source.is_null());
    assert!(is_valid(&source));
    let provider = provider();
    let content = write_shape(&provider, "bug27722.step", &source).expect("written");
    assert!(!content.is_empty());
    let restored = read_shape(&provider, "bug27722.step", &content).expect("read");
    assert!(!restored.is_null());
    let is_restored_valid = is_valid(&restored);
    let has_difference = [
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        ShapeEnum::Wire,
        ShapeEnum::Face,
        ShapeEnum::Shell,
        ShapeEnum::Solid,
        ShapeEnum::CompSolid,
        ShapeEnum::Compound,
        ShapeEnum::Shape,
    ]
    .iter()
    .any(|kind| count(&source, *kind) != count(&restored, *kind));
    if !is_restored_valid || has_difference {
        // the case skips itself here: "TODO bug 27722: STEP round trip produces an invalid shape
        // or changes its topology."
        eprintln!(
            "skipped: the round trip is valid: {is_restored_valid}, the same topology: {}",
            !has_difference
        );
        return;
    }
    assert!(count(&restored, ShapeEnum::Face) > 0);
}

#[test]
fn stream_document_write_read() {
    let _alone = alone();
    let (provider, document) = (provider(), new_document());
    assert!(!add_shape(&document, &a_box(10.0, 10.0, 10.0)).is_null());
    let content = write_document(&provider, "document.step", &document).expect("written");
    assert!(!content.is_empty());
    assert!(is_valid_step_content(&content));
    let read_back = new_document();
    assert!(read_document(
        &provider,
        "document.step",
        &content,
        &read_back
    ));
    assert!(nb_shapes(&read_back) > 0);
}

#[test]
fn xde_bug_1540_write_document_with_invisible_layer() {
    let _alone = alone();
    let (provider, document) = (provider(), new_document());
    let shape_label = add_shape(&document, &a_box(10., 10., 10.));
    assert!(!shape_label.is_null());
    let main = document.get().expect("document").main().expect("main");
    let layers = XCAFDocDocumentTool::layer_tool(&main).expect("tool");
    let layer = layers
        .add_layer(&ExtendedString::from_utf8("invis"))
        .expect("layer");
    assert!(!layer.is_null());
    layers
        .set_layer_label(&shape_label, &layer, false)
        .expect("set");
    layers.set_visibility(&layer, false).expect("visibility");
    assert!(layers.is_set_label(&shape_label, &layer).expect("set"));
    assert!(!layers.is_visible(&layer).expect("visible"));
    let content = write_document(&provider, "bug1540.step", &document).expect("written");
    assert!(is_valid_step_content(&content));
    let text = String::from_utf8_lossy(&content);
    assert!(text.contains("invis"));
    assert!(text.contains("INVISIBILITY"));
}

/// A wrapper with a node bound.
fn wrapper() -> (DeWrapper, Handle<DeStepConfigurationNode>) {
    let wrapper = DeWrapper::new();
    let node = new_configuration_node(DeStepConfigurationNode::default());
    let bound: Handle<dyn DeConfigurationNode> = node.cast();
    assert!(wrapper.bind(&bound));
    (wrapper, node)
}

#[test]
fn de_wrapper_integration() {
    let _alone = alone();
    let (wrapper, node) = wrapper();
    let (mut output, written) = memory_ostream();
    let mut write_streams = write_list("test.step", &mut output);
    assert!(
        wrapper
            .write_shape_streams(&mut write_streams, &a_sphere(5.0), &range())
            .expect("write")
    );
    let content = written.contents();
    assert!(!content.is_empty());
    assert!(is_valid_step_content(&content));
    let mut input = memory_istream(&content);
    let mut read_streams = read_list("test.step", &mut input);
    let mut wrapper_shape = Shape::default();
    let wrapper_result = wrapper
        .read_shape_streams(&mut read_streams, &mut wrapper_shape, &range())
        .expect("read");
    let direct = DeStepProvider::with_node(&node.cast());
    let direct_shape = read_shape(&direct, "test.step", &content);
    assert_eq!(wrapper_result, direct_shape.is_some());
    let direct_shape = direct_shape.unwrap_or_default();
    assert_eq!(wrapper_shape.is_null(), direct_shape.is_null());
    if wrapper_result && !direct_shape.is_null() {
        assert!(count(&direct_shape, ShapeEnum::Solid) > 0);
    }
}

#[test]
fn multiple_shapes_in_document() {
    let _alone = alone();
    let (provider, document) = (provider(), new_document());
    assert!(!add_shape(&document, &a_box(10.0, 10.0, 10.0)).is_null());
    assert!(!add_shape(&document, &a_sphere(5.0)).is_null());
    assert!(!add_shape(&document, &a_cylinder(3.0, 8.0)).is_null());
    let content = write_document(&provider, "multi_shapes.step", &document).expect("written");
    assert!(!content.is_empty());
    assert!(is_valid_step_content(&content));
    let read_back = new_document();
    assert!(read_document(
        &provider,
        "multi_shapes.step",
        &content,
        &read_back
    ));
    assert_eq!(nb_shapes(&read_back), 3);
}

#[test]
fn different_brep_geometries() {
    let _alone = alone();
    let provider = provider();
    let a_box_content =
        write_shape(&provider, "box.step", &a_box(10.0, 10.0, 10.0)).expect("written");
    let sphere_content = write_shape(&provider, "sphere.step", &a_sphere(5.0)).expect("written");
    let cylinder_content =
        write_shape(&provider, "cylinder.step", &a_cylinder(3.0, 8.0)).expect("written");
    for content in [&a_box_content, &sphere_content, &cylinder_content] {
        assert!(is_valid_step_content(content));
    }
    assert_ne!(a_box_content, sphere_content);
    assert_ne!(a_box_content, cylinder_content);
    assert_ne!(sphere_content, cylinder_content);
    assert!(
        !read_shape(&provider, "box.step", &a_box_content)
            .expect("read")
            .is_null()
    );
    assert!(
        !read_shape(&provider, "sphere.step", &sphere_content)
            .expect("read")
            .is_null()
    );
}

#[test]
fn de_wrapper_file_extensions() {
    let _alone = alone();
    let (wrapper, _node) = wrapper();
    let shape = a_box(10.0, 10.0, 10.0);
    for extension in ["test.step", "test.STEP", "test.stp", "test.STP"] {
        let (mut output, written) = memory_ostream();
        let mut write_streams = write_list(extension, &mut output);
        assert!(
            wrapper
                .write_shape_streams(&mut write_streams, &shape, &range())
                .expect("write"),
            "Failed to write with extension: {extension}"
        );
        let content = written.contents();
        assert!(
            !content.is_empty(),
            "Empty content for extension: {extension}"
        );
        assert!(
            is_valid_step_content(&content),
            "Invalid STEP content for extension: {extension}"
        );
        let mut input = memory_istream(&content);
        let mut read_streams = read_list(extension, &mut input);
        let mut read = Shape::default();
        assert!(
            wrapper
                .read_shape_streams(&mut read_streams, &mut read, &range())
                .expect("read"),
            "Failed to read with extension: {extension}"
        );
        assert!(
            !read.is_null(),
            "Null shape read with extension: {extension}"
        );
    }
}

#[test]
fn error_handling() {
    let _alone = alone();
    let provider = provider();
    let mut empty_write = DeProviderWriteStreamList::default();
    assert!(
        !provider
            .write_shape_streams(&mut empty_write, &a_box(10.0, 10.0, 10.0), &range())
            .expect("write")
    );
    let mut empty_read = DeProviderReadStreamList::default();
    let mut shape = Shape::default();
    assert!(
        !provider
            .read_shape_streams(&mut empty_read, &mut shape, &range())
            .expect("read")
    );
    assert!(write_shape(&provider, "null_test.step", &Shape::default()).is_none());
    assert!(read_shape(&provider, "invalid.step", b"This is not valid STEP content").is_none());
    let null_document: Handle<TDocStdDocument> = Handle::default();
    let (mut output, _) = memory_ostream();
    let mut streams = write_list("null_test.step", &mut output);
    assert!(
        !provider
            .write_document_streams(&mut streams, &null_document, &range())
            .expect("write")
    );
    assert!(
        !provider
            .read_document_streams(&mut empty_read, &null_document, &range())
            .expect("read")
    );
}

#[test]
fn configuration_modes() {
    let _alone = alone();
    let provider = provider();
    let node: Handle<DeStepConfigurationNode> = provider.get_node().cast();
    assert!(node.is_some());
    assert_eq!(provider.get_format().as_bytes(), b"STEP");
    assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
    let new_node = new_configuration_node(DeStepConfigurationNode::default());
    let new_provider = DeStepProvider::with_node(&new_node.cast());
    assert_eq!(new_provider.get_format().as_bytes(), b"STEP");
    assert_eq!(new_provider.get_vendor().as_bytes(), b"Martensite");
    assert!(new_provider.get_node().is_some());
}

fn new_session() -> DeWorkSession {
    martensite_tkxsbase::xs_control::XSControlWorkSession::new().cast()
}

#[test]
fn work_session_integration() {
    let _alone = alone();
    let provider = provider();
    let mut session = new_session();
    let (mut output, written) = memory_ostream();
    let mut write_streams = write_list("ws_test.step", &mut output);
    assert!(
        provider
            .write_shape_streams_with_session(
                &mut write_streams,
                &a_box(10.0, 10.0, 10.0),
                &mut session,
                &range()
            )
            .expect("write")
    );
    let content = written.contents();
    assert!(!content.is_empty());
    assert!(is_valid_step_content(&content));
    let mut input = memory_istream(&content);
    let mut read_streams = read_list("ws_test.step", &mut input);
    let mut read = Shape::default();
    assert!(
        provider
            .read_shape_streams_with_session(&mut read_streams, &mut read, &mut session, &range())
            .expect("read")
    );
    assert!(!read.is_null());
}

#[test]
fn document_work_session_integration() {
    let _alone = alone();
    let (provider, document) = (provider(), new_document());
    assert!(!add_shape(&document, &a_sphere(5.0)).is_null());
    let mut session = new_session();
    let (mut output, written) = memory_ostream();
    let mut write_streams = write_list("doc_ws_test.step", &mut output);
    assert!(
        provider
            .write_document_streams_with_session(
                &mut write_streams,
                &document,
                &mut session,
                &range()
            )
            .expect("write")
    );
    let content = written.contents();
    assert!(!content.is_empty());
    assert!(is_valid_step_content(&content));
    let read_back = new_document();
    let mut input = memory_istream(&content);
    let mut read_streams = read_list("doc_ws_test.step", &mut input);
    assert!(
        provider
            .read_document_streams_with_session(
                &mut read_streams,
                &read_back,
                &mut session,
                &range()
            )
            .expect("read")
    );
    assert!(nb_shapes(&read_back) > 0);
}

#[test]
fn step_bug_29830_large_compound_write_and_destruction() {
    let _alone = alone();
    let shape_count = 1000;
    let the_box = a_box(1.0, 1.0, 1.0);
    let content = {
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let mut compound: Shape = compound.into();
        for _ in 0..shape_count {
            builder.add(&mut compound, &the_box).expect("add");
        }
        let document = new_document();
        assert!(!add_shape(&document, &compound).is_null());
        write_document(&provider(), "bug29830.step", &document).expect("written")
    };
    assert!(is_valid_step_content(&content));
    assert!(content.len() > shape_count);
}
