//! The shape translators directly, on small shapes: what each gives, as a run of the same
//! calls gave.
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face, Shape, Shell, Solid, Vertex, Wire};
use martensite_tkdestep::step_control::StepControlController;
use martensite_tkdestep::step_data::{StepDataFactors, StepDataStepModel};
use martensite_tkdestep::step_geom::StepGeomPlane;
use martensite_tkdestep::step_shape::{
    StepShapeAdvancedFace, StepShapeClosedShell, StepShapeEdgeCurve, StepShapeEdgeLoop,
    StepShapeVertexPoint,
};
use martensite_tkdestep::topo_ds_to_step::{
    TopoDSToStep, TopoDSToStepBuilder, TopoDSToStepBuilderError, TopoDSToStepMakeBrepWithVoids,
    TopoDSToStepMakeEdgeError, TopoDSToStepMakeFaceError,
    TopoDSToStepMakeFacetedBrepAndBrepWithVoids, TopoDSToStepMakeGeometricCurveSet,
    TopoDSToStepMakeManifoldSolidBrep, TopoDSToStepMakeShellBasedSurfaceModel,
    TopoDSToStepMakeStepEdge, TopoDSToStepMakeStepFace, TopoDSToStepMakeStepVertex,
    TopoDSToStepMakeStepWire, TopoDSToStepMakeTessellatedItem, TopoDSToStepMakeVertexError,
    TopoDSToStepMakeWireError, TopoDSToStepTool,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis as _,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use martensite_tkxsbase::transfer::TransferFinderProcess;

fn range() -> ProgressRange<'static> {
    ProgressRange::default()
}

fn new_process() -> Handle<TransferFinderProcess> {
    StepControlController::init().expect("controller");
    let process = TransferFinderProcess::new(10000);
    process.set_model(&StepDataStepModel::new().cast());
    process
}

fn model_of(process: &Handle<TransferFinderProcess>) -> Handle<StepDataStepModel> {
    process.model().cast()
}

fn box_solid(dx: f64, dy: f64, dz: f64) -> Solid {
    BRepPrimAPIMakeBox::from_sizes(dx, dy, dz)
        .unwrap()
        .solid()
        .unwrap()
}

fn first_of(shape: &Shape, kind: ShapeEnum) -> Shape {
    Explorer::new(shape, kind, ShapeEnum::Shape)
        .unwrap()
        .current()
        .clone()
}

fn first_face(shape: &Shape) -> Face {
    topods::face(&first_of(shape, ShapeEnum::Face)).unwrap()
}

fn first_shell(shape: &Shape) -> Shell {
    topods::shell(&first_of(shape, ShapeEnum::Shell)).unwrap()
}

/// A 10 mm box with a 2 mm box-shaped void at its centre.
fn hollow_box() -> Solid {
    let outer = box_solid(10.0, 10.0, 10.0).into_shape();
    let inner = BRepPrimAPIMakeBox::at_point(Point3::new(4.0, 4.0, 4.0), 2.0, 2.0, 2.0)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    let builder = BRepBuilder::default();
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    let mut whole: Shape = solid.into();
    builder
        .add(&mut whole, &first_shell(&outer).into())
        .unwrap();
    builder
        .add(&mut whole, &first_shell(&inner).reversed())
        .unwrap();
    topods::solid(&whole).unwrap()
}

#[test]
fn a_manifold_brep_of_a_box_holds_its_six_faces() {
    let box_shape = box_solid(1.0, 2.0, 3.0);
    let brep = TopoDSToStepMakeManifoldSolidBrep::with_solid(
        &box_shape,
        &new_process(),
        &StepDataFactors::default(),
        &range(),
    )
    .unwrap();
    assert!(brep.is_done());
    assert_eq!(brep.value().unwrap().outer().nb_cfs_faces().unwrap(), 6);
}

#[test]
fn a_manifold_brep_of_a_shell_holds_its_faces() {
    let shape = box_solid(1.0, 2.0, 3.0).into_shape();
    let brep = TopoDSToStepMakeManifoldSolidBrep::with_shell(
        &first_shell(&shape),
        &new_process(),
        &StepDataFactors::default(),
        &range(),
    )
    .unwrap();
    assert!(brep.is_done());
    assert_eq!(brep.value().unwrap().outer().nb_cfs_faces().unwrap(), 6);
}

#[test]
fn a_brep_with_voids_keeps_the_void_and_its_orientation() {
    let brep = TopoDSToStepMakeBrepWithVoids::new(
        &hollow_box(),
        &new_process(),
        &StepDataFactors::default(),
        &range(),
    )
    .unwrap();
    assert!(brep.is_done());
    let value = brep.value().unwrap();
    assert_eq!(value.nb_voids(), 1);
    assert_eq!(value.outer().nb_cfs_faces().unwrap(), 6);
    let void = value.voids_value(1).unwrap();
    assert_eq!(void.nb_cfs_faces().unwrap(), 6);
    assert!(!void.orientation());
}

/// The `MakeFacetedBrep` crashes on a box and on a sphere when it is called alone, so only
/// the voids class has a reference to hold on to.
#[test]
fn a_faceted_brep_with_voids_keeps_the_void() {
    let brep = TopoDSToStepMakeFacetedBrepAndBrepWithVoids::new(
        &hollow_box(),
        &new_process(),
        &StepDataFactors::default(),
        &range(),
    )
    .unwrap();
    assert!(brep.is_done());
    assert_eq!(brep.value().unwrap().nb_voids().unwrap(), 1);
}

#[test]
fn a_surface_model_of_a_face_is_one_open_shell() {
    let shape = box_solid(1.0, 2.0, 3.0).into_shape();
    let made = TopoDSToStepMakeShellBasedSurfaceModel::with_face(
        &first_face(&shape),
        &new_process(),
        &StepDataFactors::default(),
        &range(),
    )
    .unwrap();
    assert!(made.is_done());
    let value = made.value().unwrap();
    assert_eq!(value.nb_sbsm_boundary().unwrap(), 1);
    assert_eq!(
        value
            .sbsm_boundary_value(1)
            .unwrap()
            .open_shell()
            .nb_cfs_faces()
            .unwrap(),
        1
    );
}

#[test]
fn a_surface_model_of_a_closed_shell_is_one_closed_shell() {
    let shape = box_solid(1.0, 2.0, 3.0).into_shape();
    let made = TopoDSToStepMakeShellBasedSurfaceModel::with_shell(
        &first_shell(&shape),
        &new_process(),
        &StepDataFactors::default(),
        &range(),
    )
    .unwrap();
    assert!(made.is_done());
    let value = made.value().unwrap();
    assert_eq!(value.nb_sbsm_boundary().unwrap(), 1);
    assert_eq!(
        value
            .sbsm_boundary_value(1)
            .unwrap()
            .closed_shell()
            .nb_cfs_faces()
            .unwrap(),
        6
    );
}

#[test]
fn a_surface_model_of_a_hollow_solid_has_a_boundary_per_shell() {
    let made = TopoDSToStepMakeShellBasedSurfaceModel::with_solid(
        &hollow_box(),
        &new_process(),
        &StepDataFactors::default(),
        &range(),
    )
    .unwrap();
    assert!(made.is_done());
    assert_eq!(made.value().unwrap().nb_sbsm_boundary().unwrap(), 2);
}

#[test]
fn a_curve_set_holds_an_edge_once() {
    let edge: Edge =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0))
            .unwrap()
            .edge()
            .unwrap();
    let set = TopoDSToStepMakeGeometricCurveSet::new(
        &edge.into(),
        &new_process(),
        &StepDataFactors::default(),
    )
    .unwrap();
    assert!(set.is_done());
    assert_eq!(set.value().unwrap().nb_elements(), 1);
}

/// Each face gives its four edges: an edge shared by two faces counts twice.
#[test]
fn a_curve_set_of_a_box_counts_its_edges_per_face() {
    let shape = box_solid(1.0, 2.0, 3.0).into_shape();
    let set =
        TopoDSToStepMakeGeometricCurveSet::new(&shape, &new_process(), &StepDataFactors::default())
            .unwrap();
    assert!(set.is_done());
    assert_eq!(set.value().unwrap().nb_elements(), 24);
}

#[test]
fn a_new_tool_starts_empty_and_in_the_pcurve_mode_of_its_model() {
    let process = new_process();
    let tool = TopoDSToStepTool::with_model(&model_of(&process)).unwrap();
    assert!(!tool.faceted());
    assert!(tool.pcurve_mode());
    assert_eq!(tool.lowest_3d_tolerance(), 0.0);
}

#[test]
fn a_vertex_is_a_vertex_point_bound_once() {
    let process = new_process();
    let mut tool = TopoDSToStepTool::with_model(&model_of(&process)).unwrap();
    let factors = StepDataFactors::default();
    let shape = box_solid(1.0, 2.0, 3.0).into_shape();
    let vertex: Vertex = topods::vertex(&first_of(&shape, ShapeEnum::Vertex)).unwrap();
    assert!(!tool.is_bound(&vertex.clone().into()));
    let first =
        TopoDSToStepMakeStepVertex::with_vertex(&vertex, &mut tool, &process, &factors).unwrap();
    assert!(tool.is_bound(&vertex.clone().into()));
    assert!(first.is_done());
    assert_eq!(first.error(), TopoDSToStepMakeVertexError::VertexDone);
    let value = first.value().unwrap();
    assert!(value.is_kind(StepShapeVertexPoint::static_type()));
    let again =
        TopoDSToStepMakeStepVertex::with_vertex(&vertex, &mut tool, &process, &factors).unwrap();
    assert_eq!(
        again.value().unwrap().object_address(),
        value.object_address()
    );
}

#[test]
fn an_edge_of_a_face_is_an_edge_curve_on_a_surface_curve() {
    let process = new_process();
    let mut tool = TopoDSToStepTool::with_model(&model_of(&process)).unwrap();
    let shape = box_solid(1.0, 2.0, 3.0).into_shape();
    tool.set_current_face(&first_face(&shape)).unwrap();
    let edge = topods::edge(&first_of(&shape, ShapeEnum::Edge)).unwrap();
    let made = TopoDSToStepMakeStepEdge::with_edge(
        &edge,
        &mut tool,
        &process,
        &StepDataFactors::default(),
    )
    .unwrap();
    assert!(made.is_done());
    assert_eq!(made.error(), TopoDSToStepMakeEdgeError::EdgeDone);
    let value: Handle<StepShapeEdgeCurve> = made.value().unwrap().cast();
    let live = value.get().expect("an edge_curve");
    assert_eq!(
        live.edge_geometry().dynamic_type().unwrap().name(),
        "StepGeomSurfaceCurve"
    );
    assert!(live.same_sense());
}

#[test]
fn a_wire_of_a_box_is_an_edge_loop_of_four_edges() {
    let process = new_process();
    let mut tool = TopoDSToStepTool::with_model(&model_of(&process)).unwrap();
    let shape = box_solid(1.0, 2.0, 3.0).into_shape();
    tool.set_current_face(&first_face(&shape)).unwrap();
    let wire: Wire = topods::wire(&first_of(&shape, ShapeEnum::Wire)).unwrap();
    let made = TopoDSToStepMakeStepWire::with_wire(
        &wire,
        &mut tool,
        &process,
        &StepDataFactors::default(),
    )
    .unwrap();
    assert!(made.is_done());
    // On success never sets the error: its value there is whatever the memory held.
    let value: Handle<StepShapeEdgeLoop> = made.value().unwrap().cast();
    assert_eq!(value.get().expect("an edge_loop").nb_edge_list(), 4);
}

fn step_face_of(shape: &Shape) -> Handle<StepShapeAdvancedFace> {
    let process = new_process();
    let mut tool = TopoDSToStepTool::with_model(&model_of(&process)).unwrap();
    let made = TopoDSToStepMakeStepFace::with_face(
        &first_face(shape),
        &mut tool,
        &process,
        &StepDataFactors::default(),
    )
    .unwrap();
    assert!(made.is_done());
    assert_eq!(made.error(), TopoDSToStepMakeFaceError::FaceDone);
    made.value().unwrap().cast()
}

#[test]
fn a_planar_face_is_an_advanced_face_on_a_plane() {
    let value = step_face_of(&box_solid(1.0, 2.0, 3.0).into_shape());
    let live = value.get().expect("an advanced_face");
    assert_eq!(live.nb_bounds().unwrap(), 1);
    assert!(live.face_geometry().is_kind(StepGeomPlane::static_type()));
}

#[test]
fn the_first_face_of_a_cylinder_is_on_a_cylindrical_surface() {
    let mut made = BRepPrimAPIMakeCylinder::new(1.0, 2.0);
    let solid = made.solid().unwrap();
    let value = step_face_of(&solid.into_shape());
    let geometry = value.get().expect("an advanced_face").face_geometry();
    assert_eq!(
        geometry.dynamic_type().unwrap().name(),
        "StepGeomCylindricalSurface"
    );
}

#[test]
fn the_builder_makes_a_closed_shell_of_a_closed_shell() {
    let process = new_process();
    let mut tool = TopoDSToStepTool::with_model(&model_of(&process)).unwrap();
    let shape = box_solid(1.0, 2.0, 3.0).into_shape();
    let built = TopoDSToStepBuilder::with_shape(
        &first_shell(&shape).into(),
        &mut tool,
        &process,
        0,
        &StepDataFactors::default(),
        &range(),
    )
    .unwrap();
    assert!(built.is_done());
    // On success never sets the error: its value there is whatever the memory held.
    assert!(
        built
            .value()
            .unwrap()
            .is_kind(StepShapeClosedShell::static_type())
    );
}

#[test]
fn a_meshed_face_and_shell_give_their_tessellated_items() {
    let process = new_process();
    let mut tool = TopoDSToStepTool::with_model(&model_of(&process)).unwrap();
    let factors = StepDataFactors::default();
    let shape = box_solid(1.0, 1.0, 1.0).into_shape();
    BRepMeshIncrementalMesh::with_shape(&shape, 0.5, false, 0.5, false).expect("mesh");
    let face = TopoDSToStepMakeTessellatedItem::with_face(
        &first_face(&shape),
        &mut tool,
        &process,
        true,
        &factors,
        &range(),
    )
    .unwrap();
    assert!(face.is_done());
    assert_eq!(
        face.value().unwrap().dynamic_type().unwrap().name(),
        "StepVisualTriangulatedSurfaceSet"
    );
    let shell = TopoDSToStepMakeTessellatedItem::with_shell(
        &first_shell(&shape),
        &mut tool,
        &process,
        &factors,
        &range(),
    )
    .unwrap();
    assert!(shell.is_done());
    assert_eq!(
        shell.value().unwrap().dynamic_type().unwrap().name(),
        "StepVisualTessellatedShell"
    );
}

#[test]
fn the_decoders_give_the_texts() {
    let text = |decoded: Handle<martensite_tkernel::tcollection::HAsciiString>| {
        String::from_utf8_lossy(decoded.get().unwrap().string().as_bytes()).into_owned()
    };
    assert_eq!(
        text(TopoDSToStep::decode_builder_error(
            TopoDSToStepBuilderError::NoFaceMapped
        )),
        "None of the Shell Faces has been mapped"
    );
    assert_eq!(
        text(TopoDSToStep::decode_face_error(
            TopoDSToStepMakeFaceError::InfiniteFace
        )),
        "The Face has no Outer Wire"
    );
    assert_eq!(
        text(TopoDSToStep::decode_wire_error(
            TopoDSToStepMakeWireError::NonManifoldWire
        )),
        "The Wire is Internal or External"
    );
    assert_eq!(
        text(TopoDSToStep::decode_edge_error(
            TopoDSToStepMakeEdgeError::EdgeOther
        )),
        "Other Error in Make STEP Edge"
    );
    assert_eq!(
        text(TopoDSToStep::decode_vertex_error(
            TopoDSToStepMakeVertexError::VertexOther
        )),
        "Other Error in Make STEP Vertex"
    );
}
