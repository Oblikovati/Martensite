//! Shapes transferred through STL files and shown on the real viewer.
use super::checks::{
    CaseFile, assert_area, assert_mesh_counts, assert_shape_counts, assert_valid, input,
    mesh_shape, read_faces, read_mesh_face, write_stl,
};
use super::real_view::{ViewerHost, run_viewer_case};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{self, Face, Shape},
};
use martensite_tkdestl::rw_stl::{DEFAULT_MERGE_ANGLE, RwStl};
use martensite_tkernel::{message::ProgressRange, ncollection::Sequence, standard::Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    gp::{Point3, Transform3},
    poly::{MergeNodesTool, Triangulation, mesh_purpose},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape as _};

const BOX_OF_FACES: [usize; 9] = [8, 18, 12, 12, 0, 0, 0, 1, 51];
const TEXT: i32 = 0;
const BINARY: i32 = 1;

/// `box name size size size`.
pub fn solid_box(size: f64) -> Shape {
    box_at([0., 0., 0.], [size, size, size])
}

/// `box name x y z dx dy dz`.
pub fn box_at(origin: [f64; 3], size: [f64; 3]) -> Shape {
    let [x, y, z] = origin;
    let [dx, dy, dz] = size;
    let first = Point3::new(x, y, z);
    let second = Point3::new(x + dx, y + dy, z + dz);
    let mut maker = BRepPrimAPIMakeBox::from_points(first, second).unwrap();
    maker.solid().unwrap().into_shape()
}

fn assert_box_of_faces_view(result: &Shape, name: &str) {
    assert_shape_counts(result, BOX_OF_FACES);
    assert_area(result, 600.);
    assert_valid(result);
    let host = ViewerHost::open();
    host.assert_shape_view_draws(result, name);
}

fn meshed_box_round_trip(mode: i32, name: &str) {
    let solid = solid_box(10.);
    mesh_shape(&solid, 0.1);
    let file = CaseFile::new(name);
    write_stl(&solid, &file.name(), mode);
    assert!(file.size() != 0, "written file size");
    let result = read_faces(&file.name());
    assert_box_of_faces_view(&result, name);
}

#[test]
fn a_meshed_box_round_trips_through_binary() {
    run_viewer_case(
        "shape_view_cases::a_meshed_box_round_trips_through_binary",
        || meshed_box_round_trip(BINARY, "bug23192_1"),
    );
}

#[test]
fn a_meshed_box_round_trips_through_text() {
    run_viewer_case(
        "shape_view_cases::a_meshed_box_round_trips_through_text",
        || meshed_box_round_trip(TEXT, "bug23192_2"),
    );
}

#[test]
fn a_box_of_faces_round_trips_through_text_and_shows() {
    run_viewer_case(
        "shape_view_cases::a_box_of_faces_round_trips_through_text_and_shows",
        || {
            let read = read_faces(&input("dataset-7.9.0/others/OMF6391_box.stl"));
            mesh_shape(&read, 0.1);
            let file = CaseFile::new("bug22670_2.stl");
            write_stl(&read, &file.name(), TEXT);
            assert_box_of_faces_view(&read_faces(&file.name()), "bug22670_2");
        },
    );
}

/// Both boxes written as text, concatenated by the case with a newline after each: two domains.
#[test]
fn concatenated_text_files_read_as_two_domains() {
    run_viewer_case(
        "shape_view_cases::concatenated_text_files_read_as_two_domains",
        || {
            let [first, second] = [solid_box(5.), box_at([10., 10., 10.], [5., 5., 5.])];
            mesh_shape(&first, 0.1);
            mesh_shape(&second, 0.1);
            let (first_file, second_file, joined) = (
                CaseFile::new("31080_1.stl"),
                CaseFile::new("31080_2.stl"),
                CaseFile::new("31080_cat.stl"),
            );
            write_stl(&first, &first_file.name(), TEXT);
            write_stl(&second, &second_file.name(), TEXT);
            let mut bytes = std::fs::read(&first_file.0).unwrap();
            bytes.push(b'\n');
            bytes.extend(std::fs::read(&second_file.0).unwrap());
            bytes.push(b'\n');
            std::fs::write(&joined.0, bytes).unwrap();
            let result = read_domains(&joined);
            let host = ViewerHost::open();
            host.display_shape(&result, Some(1));
            host.fit();
            assert_shape_counts(&result, [0, 0, 0, 2, 0, 0, 0, 1, 3]);
            assert_mesh_counts(&result, 24, 16);
            assert!(host.dump("bug31080") > 0, "expected a drawn image");
        },
    );
}

/// One face per domain, in a compound for more than one.
fn read_domains(file: &CaseFile) -> Shape {
    let mut list: Sequence<Handle<Triangulation>> = Sequence::default();
    RwStl::read_file_domains(
        &file.name(),
        DEFAULT_MERGE_ANGLE,
        &mut list,
        &ProgressRange::default(),
    )
    .unwrap();
    let builder = BRepBuilder::default();
    let mut compound = martensite_tkbrep::topods::Compound::default();
    builder.make_compound(&mut compound);
    for mesh in list.iter() {
        builder
            .add(&mut compound, &crate::support::shapes::face_of_mesh(mesh))
            .unwrap();
    }
    compound.into()
}

/// `tcopy -mesh from to`.
fn copy_with_mesh(shape: &Shape) -> Shape {
    let mut copy = BRepBuilderAPICopy::with_shape(shape, true, true).unwrap();
    copy.shape().unwrap().clone()
}

/// `trmergenodes shape -angle degrees`: each face's mesh with its nodes merged.
fn merge_nodes_by_angle(shape: &Shape, degrees: f64) {
    let angle = degrees * std::f64::consts::PI / 180.0;
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let face: Face = topods::face(explorer.current()).unwrap();
        let (mesh, _) =
            martensite_tkbrep::brep::BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
        let merged =
            MergeNodesTool::merge_nodes(&mesh, Transform3::default(), false, angle, 0.0, true)
                .unwrap();
        BRepBuilder::default()
            .update_face_triangulation(&face, &merged, true)
            .unwrap();
        explorer.next().unwrap();
    }
}

#[test]
fn a_merge_angle_splits_nodes_at_sharp_corners() {
    run_viewer_case(
        "shape_view_cases::a_merge_angle_splits_nodes_at_sharp_corners",
        || {
            let file = input("occt-source-data/stl/shape.stl");
            let all = read_mesh_face(&file, None);
            assert_mesh_counts(&all, 494, 249);
            let split = read_mesh_face(&file, Some(45.));
            assert_mesh_counts(&split, 494, 413);
            let host = ViewerHost::open();
            let object = host.display_shape(&all, Some(1));
            host.draw_face_boundaries(&object);
            host.fit();
            assert!(host.dump("D2_mergeall") > 0);
            host.context.remove_all(false).unwrap();
            let object = host.display_shape(&split, Some(1));
            host.draw_face_boundaries(&object);
            assert!(host.dump("D2_merge45") > 0);
            let copy = copy_with_mesh(&all);
            merge_nodes_by_angle(&copy, 60.);
            host.context.remove_all(false).unwrap();
            let object = host.display_shape(&copy, Some(1));
            host.draw_face_boundaries(&object);
            assert!(host.dump("D2_merge60") > 0);
            assert_mesh_counts(&copy, 494, 411);
        },
    );
}
