//! Owned geometry and appearance controls do not claim image, font-raster or GPU evidence.
use super::*;
use martensite_tkernel::ncollection::List;
use martensite_tkservice::{
    aspect::AspectInteriorStyle, graphic3d::*, null_driver::NullGraphicDriver,
};
use martensite_tkv3d::{
    ais::{AisInteractiveObject, AisShape, AisShapeParameters as _},
    prs_mgr::*,
    select_mgr::*,
};
use std::collections::HashSet;

fn null_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap()
}

#[test]
fn named_highlight_comparison_keeps_the_exact_source_colour() {
    assert_eq!(highlight::highlight_colour_error("TURQUOISE3"), None);
    assert_eq!(
        highlight::highlight_colour_error("TURQUOISE2"),
        Some("Error: Wrong dynamic highlight color")
    );
    assert!(highlight::highlight_colour_error("turquoise3").is_some());
}

#[test]
fn source_highlight_retains_own_modes_and_an_actual_detected_object() {
    let mut session = null_session();
    let shape = crate::primitive_shapes::box_sized([1., 2., 3.]).unwrap();
    highlight::highlight_geometry(&mut session, &shape).unwrap();
    let object = session.object("b").unwrap();
    assert_eq!(object.display_mode(), 1);
    assert_eq!(object.hilight_mode(), 1);
    assert_eq!(session.context().detected_interactive(), *object);
}

fn displayed_count(session: &ViewerSession) -> usize {
    let mut objects: List<Handle<dyn AisInteractiveObject>> = List::default();
    session.context().displayed_objects(&mut objects).unwrap();
    objects.len()
}

fn actual_box_corners(shape: &martensite_tkbrep::topods::Shape) -> Vec<[f64; 3]> {
    use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
    use martensite_tkg3d::top_abs::ShapeEnum;
    let mut explorer = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
    let mut positions = Vec::new();
    while explorer.more() {
        let vertex = topods::vertex(explorer.current()).unwrap();
        let point = BRepTool::pnt(&vertex).unwrap();
        let position = [point.x(), point.y(), point.z()];
        if !positions.contains(&position) {
            positions.push(position);
        }
        explorer.next().unwrap();
    }
    positions
}

fn assert_lattice_box(session: &ViewerSession, [i, j, k]: [i32; 3]) -> usize {
    let name = format!("b{i}{j}{k}");
    let object = session.object(&name).unwrap();
    let typed = object.cast::<AisShape>();
    assert!(!typed.is_null());
    assert_eq!(object.display_mode(), 1);
    let positions = actual_box_corners(&typed.shape());
    assert_eq!(positions.len(), 8);
    assert_lattice_corners(&name, &positions, [i, j, k]);
    object.object_address() as usize
}

fn assert_lattice_corners(name: &str, positions: &[[f64; 3]], [i, j, k]: [i32; 3]) {
    for bits in 0..8 {
        let expected = [
            f64::from(3 * i + (bits & 1)),
            f64::from(3 * j + ((bits >> 1) & 1)),
            f64::from(3 * k + ((bits >> 2) & 1)),
        ];
        assert!(
            positions.contains(&expected),
            "box {name} corners {positions:?}; expected {expected:?}"
        );
    }
}

#[test]
fn all125_authored_names_resolve_to_distinct_source_boxes() {
    let mut session = null_session();
    camera_boxes::authored_boxes(&mut session).unwrap();
    let mut identities = HashSet::new();
    for index in 0..125 {
        identities.insert(assert_lattice_box(
            &session,
            [index / 25, (index / 5) % 5, index % 5],
        ));
    }
    assert_eq!(identities.len(), 125);
    assert_eq!(displayed_count(&session), 125);
}

#[test]
fn frustum_is_the_actual_snapshot_while_later_camera_changes() {
    let mut session = null_session();
    let frustum = camera_boxes::frustum_scene(&mut session).unwrap();
    let points = frustum.fields.read().points;
    let triangles = frustum.fields.read().triangles.clone();
    let camera = session.view().camera();
    camera.set_scale(camera.scale() * 2.);
    assert_eq!(frustum.fields.read().points, points);
    frustum.set_camera_frustum(&camera).unwrap();
    assert_eq!(frustum.fields.read().triangles, triangles);
    assert_ne!(frustum.fields.read().points, points);
    assert_eq!(displayed_count(&session), 126);
}

fn assert_frame_vertices(array: &Graphic3dArrayOfTriangles) {
    for (index, expected) in [[-1., 0., 0.], [-1., 1., 0.], [3., 1., 0.], [3., 0., 0.]]
        .into_iter()
        .enumerate()
    {
        let point = array.vertice(index as i32 + 1).unwrap();
        assert_eq!([point.x(), point.y(), point.z()], expected);
    }
}

#[test]
fn frame_contains_exact_source_vertices_and_triangle_indices() {
    let array = point_frame::frame_triangles().unwrap();
    assert_eq!(array.vertex_number(), 4);
    assert_eq!(array.edge_number(), 6);
    assert_frame_vertices(&array);
    assert_eq!(
        (1..=6).map(|i| array.edge(i).unwrap()).collect::<Vec<_>>(),
        [1, 2, 3, 2, 3, 4]
    );
}

fn displayed_frame() -> (ViewerSession, Handle<point_frame::PointFrameText>) {
    let mut session = null_session();
    let object = point_frame::display_point_frame(&mut session).unwrap();
    (session, object)
}

#[test]
fn point_aspect_copies_the_live_drawer_without_mutating_it() {
    let (_session, object) = displayed_frame();
    let original = object.attributes().shading_aspect().aspect();
    let style = original.interior_style();
    let copied = object.frame_aspect();
    assert_ne!(copied.object_address(), original.object_address());
    assert_eq!(copied.interior_style(), AspectInteriorStyle::Point);
    assert_eq!(original.interior_style(), style);
    copied.set_interior_style(style);
    assert!(copied.is_equal(&*original));
}

fn assert_label_pose(text: &Graphic3dText) {
    assert!(text.has_plane() && text.has_own_anchor_point());
    assert_eq!(
        text.orientation().location(),
        martensite_tkmath::gp::Point3::default()
    );
    assert_eq!(
        text.orientation().direction(),
        martensite_tkmath::gp::Direction3::new(0., 0., 1.).unwrap()
    );
}

#[test]
fn text_has_its_separate_group_exact_content_plane_and_anchor() {
    let (_session, object) = displayed_frame();
    let presentation = object
        .presentations()
        .into_iter()
        .find(|prs| prs.mode() == 0)
        .unwrap();
    let groups = presentation.groups();
    assert_eq!(groups.len(), 2);
    assert_eq!(
        groups[0].aspects().interior_style(),
        AspectInteriorStyle::Point
    );
    let text_aspect: Handle<dyn Graphic3dAspectsSource> =
        object.attributes().text_aspect().aspect().cast();
    assert_eq!(groups[1].aspects(), text_aspect);
    let text = object.label_group(&presentation.cast()).unwrap();
    assert_eq!(text.text().to_c_string(), b"YOU SHOULD SEE THIS TEXT\0");
    assert_label_pose(&text);
}

#[test]
fn custom_qa_object_keeps_the_source_empty_selection() {
    let (session, object) = displayed_frame();
    assert!(session.object("point_frame_text").is_none());
    let selection = Handle::new(SelectMgrSelection::new(0));
    object.compute_selection(&selection, 0).unwrap();
    assert!(selection.entities().is_empty());
    assert_eq!(object.dynamic_type().unwrap().name(), "OCC27700_Text");
}
