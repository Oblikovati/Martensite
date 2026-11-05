//! A saved view defined on a document reads back its links and every field that was set. Reading
//! the view after a stored document is reopened is not covered here.
use super::*;
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::gp::Direction3;
use martensite_tkxcaf::xcaf_dim_tol_objects::XCAFDimTolObjectsDimensionType;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocDimension, XCAFDocView, XCAFDocViewLinks, XCAFDocViewTool,
};
use martensite_tkxcaf::xcaf_view::{XCAFViewObject, XCAFViewProjectionType};
use support::script_stream::ScriptStream;

const SAVED_VIEW_CASE: &str =
    "document_regressions::saved_view_definition::saved_view_reads_back_links_and_camera_fields";

const EXPECTED_VIEW: &str = "Reference shapes: 0:1:1:1 0:1:1:2 
Reference GD&Ts: 0:1:4:1 
Reference Clipping Planes: 
Name: ALL
Type: parallel
Projection point: 2 3 5
View Direction: 0 0 -1
Up Direction: 0 1 0
Zoom factor: 3
Window Size: width 480,  height 640
Front Plane Distance: 10
Front Back Distance: 0
View VolumeSized Clipping: 1
";

fn entry_of(label: &TdfLabel) -> String {
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

fn entries_of(labels: Option<Vec<TdfLabel>>) -> String {
    let labels = labels.unwrap_or_default();
    labels.iter().map(|label| entry_of(label) + " ").collect()
}

fn first_edge(shape: &Shape) -> Shape {
    let explorer = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    explorer.current().clone()
}

/// A length dimension on one edge, stored with its type and value.
fn dimension_on_edge(context: &XdeShapeRegressionContext, edge: &Shape) -> TdfLabel {
    let main = context.document.main().unwrap();
    let tools = XCAFDocDocumentTool::dim_tol_tool(&main).unwrap();
    let mut edge_label = TdfLabel::default();
    let found = context
        .shape_tool()
        .search(edge, &mut edge_label, [true, true, true])
        .unwrap();
    assert!(found, "the edge is not in the document");
    let dimension = tools.get().unwrap().add_dimension().unwrap();
    tools
        .get()
        .unwrap()
        .set_dimension(&edge_label, &dimension)
        .unwrap();
    let attribute = XCAFDocDimension::set(&dimension).unwrap();
    let object = attribute.get().unwrap().get_object().unwrap();
    let object_ref = object.get().unwrap();
    object_ref.set_type(XCAFDimTolObjectsDimensionType::SizeCurveLength);
    object_ref.set_value(1.);
    attribute.get().unwrap().set_object(&object).unwrap();
    dimension
}

fn defined_view() -> Handle<XCAFViewObject> {
    let view = XCAFViewObject::new();
    view.set_name(&Handle::new(HAsciiString::from(AsciiString::from("ALL"))));
    view.set_type(XCAFViewProjectionType::Parallel);
    view.set_projection_point(&Point3::new(2., 3., 5.));
    view.set_view_direction(&Direction3::new(0., 0., -1.).unwrap());
    view.set_up_direction(&Direction3::new(0., 1., 0.).unwrap());
    view.set_zoom_factor(3.);
    view.set_window_horizontal_size(480.);
    view.set_window_vertical_size(640.);
    view.set_front_plane_distance(10.);
    view.set_view_volume_sides_clipping(true);
    Handle::new(view)
}

fn kind_name(kind: XCAFViewProjectionType) -> &'static str {
    match kind {
        XCAFViewProjectionType::NoCamera => "no_camera",
        XCAFViewProjectionType::Central => "central",
        XCAFViewProjectionType::Parallel => "parallel",
    }
}

fn links_text(tool: &XCAFDocViewTool, label: &TdfLabel) -> String {
    format!(
        "Reference shapes: {}\nReference GD&Ts: {}\nReference Clipping Planes: {}\n",
        entries_of(tool.get_ref_shape_label(label).unwrap()),
        entries_of(tool.get_ref_gdt_label(label).unwrap()),
        entries_of(tool.get_ref_clipping_plane_label(label).unwrap()),
    )
}

fn fields_text(view: &XCAFViewObject) -> String {
    let point = view.projection_point();
    let direction = view.view_direction();
    let up = view.up_direction();
    let name = view.name();
    let name = String::from_utf8_lossy(name.get().unwrap().string().as_bytes()).into_owned();
    format!(
        "Name: {name}\nType: {}\nProjection point: {} {} {}\nView Direction: {} {} {}\n\
         Up Direction: {} {} {}\nZoom factor: {}\nWindow Size: width {},  height {}\n\
         Front Plane Distance: {}\nFront Back Distance: {}\nView VolumeSized Clipping: {}\n",
        kind_name(view.kind()),
        point.x(),
        point.y(),
        point.z(),
        direction.x(),
        direction.y(),
        direction.z(),
        up.x(),
        up.y(),
        up.z(),
        view.zoom_factor(),
        view.window_horizontal_size(),
        view.window_vertical_size(),
        view.front_plane_distance(),
        view.back_plane_distance(),
        u8::from(view.has_view_volume_sides_clipping()),
    )
}

const HOOK_AND_TITLE: [&str; 8] = [
    "cpulimit 1000",
    "CPU and elapsed time limit set to 1000 seconds",
    "pload OCAF XDE",
    "========",
    "OCC28044: Implement data structures in OCAF for Saved Views.",
    "========",
    "pload OCAF",
    "box b 0 0 0 1 1 1",
];

const VIEW_FIELD_COMMANDS: [&str; 10] = [
    "XSetViewName D_First 0:1:7:1 ALL",
    "XSetViewType D_First 0:1:7:1 parallel",
    "XSetViewProjectionPoint D_First 0:1:7:1 2 3 5",
    "XSetViewDir D_First 0:1:7:1 0 0 -1",
    "XSetViewUpDir D_First 0:1:7:1 0 1 0",
    "XSetViewZoom D_First 0:1:7:1 3",
    "XSetViewWindowSize D_First 0:1:7:1 480 640",
    "XSetViewFrontPlaneDistance D_First 0:1:7:1 10",
    "XSetViewVolumeSidesClipping D_First 0:1:7:1 1",
    "XDumpView D_First 0:1:7:1",
];

fn numbered_names(prefix: &str, count: usize) -> String {
    let names: Vec<String> = (1..=count)
        .map(|index| format!("{prefix}_{index}"))
        .collect();
    names.join(" ")
}

/// The boxes and the edge, added in order, with what each command reports.
fn document_shapes(
    context: &XdeShapeRegressionContext,
    stream: &mut ScriptStream,
) -> (TdfLabel, TdfLabel, Shape) {
    stream.line("document D_First created");
    stream.line(entry_of(&context.document.main().unwrap()));
    let unit = support::make_box(1., 1., 1.);
    let far = BRepPrimAPIMakeBox::at_point(Point3::new(2., 2., 2.), 1., 1., 1.)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    stream.line("XAddShape D_First b");
    let first = context.add_shape(&unit, true);
    stream.line(entry_of(&first));
    stream.line("XAddShape D_First bb");
    let second = context.add_shape(&far, true);
    stream.line(entry_of(&second));
    stream.line("explode b e");
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(&unit, ShapeEnum::Edge, &mut edges).unwrap();
    stream.line(numbered_names("b", edges.extent()));
    stream.line("XAddShape D_First b_1");
    let edge = first_edge(&unit);
    stream.line(entry_of(&context.add_shape(&edge, true)));
    (first, second, edge)
}

fn saved_view_scenario() {
    let mut stream = ScriptStream::default();
    stream.lines(HOOK_AND_TITLE);
    stream.lines(["box bb 2 2 2 1 1 1", "NewDocument D_First BinXCAF"]);
    let context = XdeShapeRegressionContext::new();
    let (first, second, edge) = document_shapes(&context, &mut stream);
    stream.line("XAddDimension D_First b_1");
    let dimension = dimension_on_edge(&context, &edge);
    stream.line(entry_of(&dimension));
    assert_eq!(entry_of(&dimension), "0:1:4:1");
    stream.lines([
        "XSetDimensionType D_First 0:1:4:1 14",
        "XSetDimensionValue D_First 0:1:4:1 1",
        "XSetView D_First 0:1:1:1 0:1:1:2 0:1:4:1",
    ]);
    let main = context.document.main().unwrap();
    let views = XCAFDocDocumentTool::view_tool(&main).unwrap();
    let view_tool = views.get().unwrap();
    let label = view_tool.add_view().unwrap();
    stream.line(entry_of(&label));
    assert_eq!(entry_of(&label), "0:1:7:1");
    let links = XCAFDocViewLinks {
        shapes: vec![first, second],
        gdts: vec![dimension],
        ..XCAFDocViewLinks::default()
    };
    view_tool.set_view(&links, &label).unwrap();
    let attribute = XCAFDocView::set(&label).unwrap();
    attribute
        .get()
        .unwrap()
        .set_object(&defined_view())
        .unwrap();
    let stored = attribute.get().unwrap().get_object().unwrap();
    let text = links_text(view_tool, &label) + &fields_text(stored.get().unwrap());
    stream.lines(VIEW_FIELD_COMMANDS);
    text.lines().for_each(|line| stream.line(line));
    stream.save("bug28044");
    assert_eq!(text, EXPECTED_VIEW);
    context.close();
}

#[test]
fn saved_view_reads_back_links_and_camera_fields() {
    case_process_case::run(SAVED_VIEW_CASE, 1000, saved_view_scenario);
}
