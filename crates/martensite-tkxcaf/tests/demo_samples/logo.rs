//! A logo of three solids: a cut sphere, a toroidal loop and a connector closed by a plate surface,
//! assembled, named, colored and shown from the top on a white background.
use super::document_display::{
    DocumentDisplay, SavedView, fit_view, named, opened_view, redraw_document, save_view,
};
use super::solid_steps::{
    assert_counts, assert_valid_measures, boolean, distinct, mirrored, rotated, translated,
};
use super::support::{new_document, shape_document::ensure_document_unit};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbool::brep_fill::BRepFillCurveConstraint;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_adaptor::{BRepAdaptorCurve2d, BRepAdaptorSurface};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Edge, Face, Shape, Solid, Wire};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::Sequence;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve as _, Adaptor3dCurveOnSurface, Adaptor3dSurface};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkgeomalgo::geom_plate::{
    GeomPlateBuildPlateSurface, GeomPlateCurveConstraint, GeomPlateMakeApprox,
    GeomPlatePlateG0Criterion,
};
use martensite_tkgeombase::adv_app2var::{AdvApp2VarCriterionRepartition, AdvApp2VarCriterionType};
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeOneAxis as _;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere, BRepPrimAPIMakeTorus,
};
use martensite_tktopalgo::brep_builder_api::{
    self, BRepBuilderAPICopy, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace,
    BRepBuilderAPIMakeShape as _, BRepBuilderAPIMakeWire, BRepBuilderAPISewing,
};
use martensite_tktopalgo::brep_class3d::BRepClass3dSolidClassifier;
use martensite_tktopalgo::brep_lib;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool, XCAFDocShapeTool,
};

const X_AXIS: [f64; 3] = [1., 0., 0.];
const Z_AXIS: [f64; 3] = [0., 0., 1.];

const EXPECTED_DUMP: &str = "ASSEMBLY COMPOUND 0:1:1:1 \"Martensite Logo 2019\" 
\tINSTANCE COMPOUND 0:1:1:1:1 (refers to 0:1:1:2) \"=>[0:1:1:2]\" 
\tINSTANCE COMPOUND 0:1:1:1:2 (refers to 0:1:1:3) \"=>[0:1:1:3]\" 
\tINSTANCE SOLID 0:1:1:1:3 (refers to 0:1:1:4) \"=>[0:1:1:4]\" 

PART COMPOUND 0:1:1:2 \"Core\" 

PART COMPOUND 0:1:1:3 \"Loop\" 

PART SOLID 0:1:1:4 \"Connector\" 


Free Shapes: 1
ASSEMBLY COMPOUND  0:1:1:1 \"Martensite Logo 2019\" 
";

struct LogoParts {
    core: Shape,
    looped: Shape,
    connector: Shape,
}

fn box_at(origin: [f64; 3], size: [f64; 3]) -> Shape {
    BRepPrimAPIMakeBox::at_point(
        Point3::new(origin[0], origin[1], origin[2]),
        size[0],
        size[1],
        size[2],
    )
    .unwrap()
    .solid()
    .unwrap()
    .into_shape()
}

fn half_cylinder_end(turn_z: Option<f64>, x: f64, y: f64) -> Shape {
    let cylinder = BRepPrimAPIMakeCylinder::new(0.5, 0.5)
        .solid()
        .unwrap()
        .into_shape();
    let cylinder = rotated(&cylinder, Z_AXIS, 180.);
    let mut cylinder = rotated(&cylinder, X_AXIS, 90.);
    if let Some(degrees) = turn_z {
        cylinder = rotated(&cylinder, Z_AXIS, degrees);
    }
    translated(&cylinder, x, y, 0.)
}

fn loop_solid(cutter: &Shape) -> Shape {
    let torus = BRepPrimAPIMakeTorus::new(1., 0.5)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    let torus = rotated(&torus, Z_AXIS, -90.);
    let torus = translated(&torus, 1.5, 1.5, 0.);
    let trimmed = boolean(&torus, cutter, Operation::Cut);
    let with_first = boolean(
        &trimmed,
        &half_cylinder_end(None, 0.5, 1.5),
        Operation::Fuse,
    );
    boolean(
        &with_first,
        &half_cylinder_end(Some(90.), 1., 0.5),
        Operation::Fuse,
    )
}

fn curve_on_face(edge: &Edge, face: &Face) -> Handle<Adaptor3dCurveOnSurface> {
    let surface = BRepAdaptorSurface::default();
    surface.initialize(face, true).unwrap();
    let surface: Handle<dyn Adaptor3dSurface> = Handle::new(surface).cast();
    let curve = BRepAdaptorCurve2d::default();
    curve.initialize(edge, face).unwrap();
    let curve: Handle<dyn Adaptor2dCurve2d> = Handle::new(curve).cast();
    Handle::new(Adaptor3dCurveOnSurface::with_curve(&curve, &surface).unwrap())
}

fn add_front(
    plate: &mut GeomPlateBuildPlateSurface,
    edge: &Shape,
    face: &Shape,
) -> Handle<Adaptor3dCurveOnSurface> {
    let front = curve_on_face(&topods::edge(edge).unwrap(), &topods::face(face).unwrap());
    let constraint: Handle<GeomPlateCurveConstraint> =
        BRepFillCurveConstraint::on_surface(&front, 0, 0, [0.0001, 0.01, 0.1]).unwrap();
    plate.add_curve(&constraint);
    front
}

fn approximated_support(
    plate: &mut GeomPlateBuildPlateSurface,
    g0_error: f64,
) -> Handle<dyn GeomSurface> {
    const TOLERANCE_3D: f64 = 0.00001;
    let mut contour_2d = Sequence::default();
    let mut contour_3d = Sequence::default();
    plate.disc2d_contour(4, &mut contour_2d).unwrap();
    plate.disc3d_contour(4, 0, &mut contour_3d).unwrap();
    let criterion = GeomPlatePlateG0Criterion::new(
        &contour_2d,
        &contour_3d,
        TOLERANCE_3D.max(g0_error * 10.),
        AdvApp2VarCriterionType::Absolute,
        AdvApp2VarCriterionRepartition::Regular,
    );
    GeomPlateMakeApprox::with_criterion(
        &plate.surface(),
        &criterion,
        TOLERANCE_3D,
        100,
        3,
        Continuity::C1,
        1.1,
    )
    .unwrap()
    .surface()
    .cast()
}

fn front_wire(
    plate: &GeomPlateBuildPlateSurface,
    support: &Handle<dyn GeomSurface>,
    fronts: &[Handle<Adaptor3dCurveOnSurface>],
    tolerance: f64,
) -> Wire {
    let builder = BRepBuilder::default();
    let mut wire = BRepBuilderAPIMakeWire::new();
    let order = plate.order().unwrap();
    let sense = plate.sense().unwrap();
    let curves = plate.curves2d().unwrap();
    for position in 1..=fronts.len() as i32 {
        let index = *order.array1().value(position).unwrap();
        let front = fronts[(index - 1) as usize].get().unwrap();
        let (first, last) = (
            front.first_parameter().unwrap(),
            front.last_parameter().unwrap(),
        );
        let reversed = *sense.array1().value(index).unwrap() == 1;
        let (start, end) = if reversed {
            (last, first)
        } else {
            (first, last)
        };
        let curve = curves.array1().value(index).unwrap().clone();
        let edge = BRepBuilderAPIMakeEdge::from_pcurve_params(&curve, support, start, end)
            .unwrap()
            .edge()
            .unwrap();
        builder
            .update_vertex_tolerance(&topexp::first_vertex(&edge, false), tolerance)
            .unwrap();
        builder
            .update_vertex_tolerance(&topexp::last_vertex(&edge, false), tolerance)
            .unwrap();
        brep_lib::build_curve3d(&edge, 1.0e-5, Continuity::C1, 14, 0).unwrap();
        wire.add_edge(&edge).unwrap();
        assert!(wire.is_done(), "the front wire stopped at {position}");
    }
    wire.wire().unwrap()
}

/// The surface closing the gap between the sphere and the loop, approximated by a B-spline.
fn connector_face(core_half: &Shape, loop_half: &Shape) -> Shape {
    let core_face = &distinct(core_half, ShapeEnum::Face)[0];
    let loop_faces = distinct(loop_half, ShapeEnum::Face);
    let (first, second) = (&loop_faces[1], &loop_faces[12]);
    let core_edges = distinct(core_face, ShapeEnum::Edge);
    let mut plate =
        GeomPlateBuildPlateSurface::new(3, 15, 2, 0.00001, 0.0001, 0.01, 0.1, false).unwrap();
    let fronts = vec![
        add_front(&mut plate, &distinct(first, ShapeEnum::Edge)[0], first),
        add_front(&mut plate, &distinct(second, ShapeEnum::Edge)[0], second),
        add_front(&mut plate, &core_edges[1], core_face),
        add_front(&mut plate, &core_edges[3], core_face),
    ];
    plate.perform(&ProgressRange::default()).unwrap();
    let g0_error = plate.g0_error();
    assert!(
        (g0_error - 5.48295258890299e-05).abs() < 1e-9,
        "plate G0 error {g0_error}"
    );
    assert_eq!(plate.g1_error(), 0.);
    let support = approximated_support(&mut plate, g0_error);
    let wire = front_wire(&plate, &support, &fronts, g0_error);
    BRepBuilderAPIMakeFace::from_surface_wire(&support, &wire, true)
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

fn sewn_shell(faces: &[&Shape]) -> Shape {
    const TOLERANCE: f64 = 0.001;
    let mut sewing = BRepBuilderAPISewing::default();
    sewing.init(TOLERANCE, true, true, true, false);
    sewing.set_same_parameter_mode(true);
    sewing.set_floating_edges_mode(false);
    sewing.set_face_mode(true);
    sewing.set_min_tolerance(TOLERANCE * 1e-4);
    sewing.set_max_tolerance(martensite_tkernel::precision::infinite());
    for face in faces {
        sewing.add(face).unwrap();
    }
    sewing.perform(&ProgressRange::default()).unwrap();
    sewing.sewed_shape().clone()
}

/// A solid of the shell, which must already face outward as the sewn faces do.
fn solid_of_shell(shell: &Shape) -> Shape {
    let shell = topods::shell(shell).unwrap();
    if let Some(tshape) = shell.tshape().get() {
        tshape.set_free(true);
    }
    let builder = BRepBuilder::default();
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    builder.add(&mut solid, &shell).unwrap();
    let mut classifier = BRepClass3dSolidClassifier::with_shape(solid.as_ref()).unwrap();
    classifier
        .perform_infinite_point(brep_builder_api::precision())
        .unwrap();
    assert_ne!(classifier.state(), State::In, "the sewn shell faces inward");
    solid.into_shape()
}

fn connector(core: &Shape, looped: &Shape) -> Shape {
    let lower = box_at([-2., -2., -2.], [4., 4., 2.]);
    let core_half = boolean(core, &lower, Operation::Cut);
    let loop_half = boolean(looped, &lower, Operation::Cut);
    let upper = connector_face(&core_half, &loop_half);
    assert_counts("connector surface", &upper, [4, 4, 1, 1, 0, 0, 0, 0, 10]);
    let mut copier = BRepBuilderAPICopy::new();
    copier.perform(&upper, true, false).unwrap();
    let below = mirrored(copier.shape().unwrap(), Z_AXIS);
    let core_faces = distinct(core, ShapeEnum::Face);
    let loop_faces = distinct(looped, ShapeEnum::Face);
    let shell = sewn_shell(&[
        &upper,
        &below,
        &core_faces[1],
        &core_faces[2],
        &loop_faces[1],
        &loop_faces[4],
    ]);
    assert_counts("connector shell", &shell, [5, 9, 6, 6, 1, 0, 0, 0, 27]);
    solid_of_shell(&shell)
}

fn logo_parts() -> LogoParts {
    let sphere = BRepPrimAPIMakeSphere::new(1.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    let cutter = box_at([0., 0., -2.], [1.5, 1.5, 4.]);
    let core = boolean(&sphere, &cutter, Operation::Cut);
    let looped = loop_solid(&cutter);
    let connector = connector(&core, &looped);
    LogoParts {
        core: rotated(&core, Z_AXIS, -45.),
        looped: rotated(&looped, Z_AXIS, -45.),
        connector: rotated(&connector, Z_AXIS, -45.),
    }
}

// Six printed digits of the measured volume of a unit sphere, not an approximation of pi.
#[allow(clippy::approx_constant)]
fn assert_parts(parts: &LogoParts) {
    assert_counts("core", &parts.core, [2, 5, 3, 3, 1, 1, 0, 1, 16]);
    assert_valid_measures("core", &parts.core, 3.14159, 12.5664);
    assert_counts("loop", &parts.looped, [5, 9, 5, 5, 1, 1, 0, 1, 27]);
    assert_valid_measures("loop", &parts.looped, 4.4865, 19.5168);
    assert_counts("connector", &parts.connector, [5, 9, 6, 6, 1, 1, 0, 0, 28]);
    assert_valid_measures("connector", &parts.connector, 1.53619, 7.57167);
}

fn set_name(label: &TdfLabel, name: &str) {
    TDataStdName::set(label, &ExtendedString::from_utf8(name)).unwrap();
}

fn assert_dump(shape_tool: &XCAFDocShapeTool) {
    let mut dump = Vec::new();
    shape_tool.dump_shapes(&mut dump, false).unwrap();
    assert_eq!(String::from_utf8(dump).unwrap().trim_start(), EXPECTED_DUMP);
}

/// Name and color by shape, as a part's color reaches its instance through the shape's label.
fn name_and_color_parts(
    shape_tool: &XCAFDocShapeTool,
    color_tool: &XCAFDocColorTool,
    parts: [(&Shape, &str, &str); 3],
) {
    for (shape, name, hex) in parts {
        set_name(&shape_tool.find_shape_label(shape, false).unwrap(), name);
        let color = ColorRgba::from_hex(hex, false).unwrap();
        let found = color_tool
            .set_shape_color(shape, &color, XCAFDocColorType::ColorGen)
            .unwrap();
        assert!(found, "{name} is not in the document");
    }
}

fn assert_instance_colors(color_tool: &XCAFDocColorTool, assembly: &TdfLabel) {
    let expected = ["BROWN1", "DEEPSKYBLUE3", "DODGERBLUE3"];
    for (tag, name) in (1..).zip(expected) {
        let instance = assembly.find_child(tag, false).unwrap();
        let stored = XCAFDocShapeTool::shape_of(&instance).unwrap();
        let found = color_tool
            .get_shape_color(&stored, XCAFDocColorType::ColorGen)
            .unwrap();
        assert_eq!(named(found.map(|rgba| *rgba.rgb())), Some(name));
    }
}

/// Build, inspect and show the document.
pub fn run() {
    let parts = logo_parts();
    assert_parts(&parts);
    let (_application, document) = new_document("BinXCAF");
    ensure_document_unit(&document);
    let main = document.main().unwrap();
    let shapes = XCAFDocDocumentTool::shape_tool(&main).unwrap();
    let shape_tool = shapes.get().unwrap();
    let assembly = shape_tool.new_shape().unwrap();
    for part in [&parts.core, &parts.looped, &parts.connector] {
        shape_tool
            .add_component_shape(&assembly, part, false)
            .unwrap();
    }
    shape_tool.update_assemblies().unwrap();
    set_name(&assembly, "Martensite Logo 2019");
    let colors = XCAFDocDocumentTool::color_tool(&main).unwrap();
    let color_tool = colors.get().unwrap();
    name_and_color_parts(
        shape_tool,
        color_tool,
        [
            (&parts.core, "Core", "FF3652"),
            (&parts.looped, "Loop", "00AADA"),
            (&parts.connector, "Connector", "0073B0"),
        ],
    );
    assert_dump(shape_tool);
    assert_instance_colors(color_tool, &assembly);
    let root = shape_tool.get_one_shape().unwrap();
    assert_counts("document", &root, [12, 23, 14, 14, 3, 3, 0, 3, 72]);
    assert_image(&show_on_white(shape_tool));
}

fn show_on_white(shape_tool: &XCAFDocShapeTool) -> SavedView {
    let lifecycle = opened_view();
    let view = lifecycle.current_view();
    let view_ref = view.get().unwrap();
    view_ref.set_background_color(Color::named(NamedColor::WHITE));
    let shown = DocumentDisplay::new(false).show_free_shapes(&lifecycle, "D", shape_tool);
    assert_eq!(shown, ["D:Martensite Logo 2019"]);
    redraw_document(&lifecycle);
    view_ref
        .set_proj_orientation(V3dTypeOfOrientation::ZUP_TOP, false)
        .unwrap();
    fit_view(&lifecycle);
    save_view(&lifecycle, "logo")
}

fn assert_image(image: &SavedView) {
    const WHITE: [u8; 3] = [255, 255, 255];
    image.assert_probes([
        [7, 72, 111],
        WHITE,
        [14, 121, 156],
        WHITE,
        [15, 123, 158],
        [8, 108, 139],
        [11, 115, 148],
        [15, 84, 128],
        WHITE,
        WHITE,
        WHITE,
    ]);
}
