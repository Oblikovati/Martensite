//! The settings that convert what the provider writes: coordinate systems and length units, against
//! values worked by hand.
use crate::provider::{file_in_temp, name_of, node_and_provider, take_text};
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{DeProvider as _, DeSectionGlobal};
use martensite_tkdeply::de_ply::DePlyProvider;
use martensite_tkdeply::rw_ply::RwPlyCafWriter;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Point3;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystem;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;

/// A box from the origin to `(1, 2, 3)` scaled by `size`, meshed into one quad per face.
fn meshed_block(size: f64) -> Shape {
    let high = Point3::new(size, 2.0 * size, 3.0 * size);
    let solid: Shape = BRepPrimAPIMakeBox::from_points(Point3::new(0.0, 0.0, 0.0), high)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    BRepMeshIncrementalMesh::with_shape(&solid, 0.1 * size, false, 0.5, false).unwrap();
    solid
}

fn document_with(shape: &Shape) -> Handle<TDocStdDocument> {
    let document = TDocStdDocument::new(&ExtendedString::new("BinXCAF")).unwrap();
    let tool = XCAFDocDocumentTool::shape_tool(&document.get().unwrap().main().unwrap()).unwrap();
    tool.get().unwrap().add_shape(shape, true, true).unwrap();
    document
}

/// A provider with the coordinate systems and the global units of a case.
fn provider_with(
    system_cs: RwMeshCoordinateSystem,
    file_cs: RwMeshCoordinateSystem,
    global: DeSectionGlobal,
) -> DePlyProvider {
    let (node, provider) = node_and_provider();
    let live = node.get().unwrap();
    {
        let mut parameters = live.internal_parameters.write();
        parameters.system_cs = system_cs;
        parameters.file_cs = file_cs;
    }
    live.configuration_node_base().set_global_parameters(global);
    provider
}

/// Rows `x y z nx ny nz` of the vertices of a file, sorted, without the sign of a zero.
fn vertex_rows(text: &str) -> Vec<String> {
    let (_, records) = text.split_once("end_header\n").unwrap();
    let mut rows: Vec<String> = records
        .lines()
        .take_while(|line| !line.starts_with("3 "))
        .map(|line| {
            let words: Vec<f32> = line.split(' ').map(|word| word.parse().unwrap()).collect();
            let shown: Vec<String> = words[..6]
                .iter()
                .map(|value| format!("{:.4}", value + 0.0))
                .collect();
            shown.join(" ")
        })
        .collect();
    rows.sort();
    rows
}

/// The rows of the faces of a box, each given by its normal and its corners.
fn expected_rows(faces: &[([f32; 3], [[f32; 3]; 4])]) -> Vec<String> {
    let mut rows = Vec::new();
    for (normal, corners) in faces {
        for corner in corners {
            let values = corner.iter().chain(normal.iter());
            let shown: Vec<String> = values.map(|value| format!("{:.4}", value + 0.0)).collect();
            rows.push(shown.join(" "));
        }
    }
    rows.sort();
    rows
}

/// The box from the origin to `(a, b, c)` in the system it was built in, Z up: every corner and
/// normal as they stand, worked by hand.
fn z_up_block(a: f32, b: f32, c: f32) -> Vec<([f32; 3], [[f32; 3]; 4])> {
    vec![
        (
            [-1.0, 0.0, 0.0],
            [[0.0, 0.0, 0.0], [0.0, b, 0.0], [0.0, 0.0, c], [0.0, b, c]],
        ),
        (
            [1.0, 0.0, 0.0],
            [[a, 0.0, 0.0], [a, b, 0.0], [a, 0.0, c], [a, b, c]],
        ),
        (
            [0.0, -1.0, 0.0],
            [[0.0, 0.0, 0.0], [a, 0.0, 0.0], [0.0, 0.0, c], [a, 0.0, c]],
        ),
        (
            [0.0, 1.0, 0.0],
            [[0.0, b, 0.0], [a, b, 0.0], [0.0, b, c], [a, b, c]],
        ),
        (
            [0.0, 0.0, -1.0],
            [[0.0, 0.0, 0.0], [a, 0.0, 0.0], [0.0, b, 0.0], [a, b, 0.0]],
        ),
        (
            [0.0, 0.0, 1.0],
            [[0.0, 0.0, c], [a, 0.0, c], [0.0, b, c], [a, b, c]],
        ),
    ]
}

/// The box from the origin to `(1, 2, 3)` turned to Y up, worked by hand from `(x, y, z)` becoming
/// `(x, z, -y)`, for positions and normals alike. Y up is a turn about X, so the +Z face looks
/// along +Y, the +Y face along -Z, and the Y extent 2 runs down to Z = -2.
fn y_up_block() -> Vec<([f32; 3], [[f32; 3]; 4])> {
    vec![
        (
            [-1.0, 0.0, 0.0],
            [
                [0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0],
                [0.0, 0.0, -2.0],
                [0.0, 3.0, -2.0],
            ],
        ),
        (
            [1.0, 0.0, 0.0],
            [
                [1.0, 0.0, 0.0],
                [1.0, 3.0, 0.0],
                [1.0, 0.0, -2.0],
                [1.0, 3.0, -2.0],
            ],
        ),
        (
            [0.0, 0.0, 1.0],
            [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 3.0, 0.0],
                [1.0, 3.0, 0.0],
            ],
        ),
        (
            [0.0, 0.0, -1.0],
            [
                [0.0, 0.0, -2.0],
                [1.0, 0.0, -2.0],
                [0.0, 3.0, -2.0],
                [1.0, 3.0, -2.0],
            ],
        ),
        (
            [0.0, -1.0, 0.0],
            [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, -2.0],
                [1.0, 0.0, -2.0],
            ],
        ),
        (
            [0.0, 1.0, 0.0],
            [
                [0.0, 3.0, 0.0],
                [1.0, 3.0, 0.0],
                [0.0, 3.0, -2.0],
                [1.0, 3.0, -2.0],
            ],
        ),
    ]
}

const MILLIMETERS: DeSectionGlobal = DeSectionGlobal {
    length_unit: 1.0,
    system_unit: 1.0,
};

#[test]
fn a_metre_document_is_written_to_a_millimetre_file() {
    let provider = provider_with(
        RwMeshCoordinateSystem::Z_UP,
        RwMeshCoordinateSystem::Z_UP,
        MILLIMETERS,
    );
    let document = document_with(&meshed_block(1.0));
    XCAFDocDocumentTool::set_length_unit(&document, 1.0).unwrap();
    let path = file_in_temp("metres.ply");
    let progress = ProgressRange::default();
    assert!(
        provider
            .write_document(&name_of(&path), &document, &progress)
            .unwrap()
    );
    let text = take_text(&path);
    assert_eq!(
        vertex_rows(&text),
        expected_rows(&z_up_block(1000.0, 2000.0, 3000.0)),
        "{text}"
    );
}

#[test]
fn a_millimetre_document_is_written_to_an_inch_file() {
    let global = DeSectionGlobal {
        length_unit: 25.4,
        system_unit: 1.0,
    };
    let provider = provider_with(
        RwMeshCoordinateSystem::Z_UP,
        RwMeshCoordinateSystem::Z_UP,
        global,
    );
    let document = document_with(&meshed_block(25.4));
    XCAFDocDocumentTool::set_length_unit(&document, 0.001).unwrap();
    let path = file_in_temp("inches.ply");
    let progress = ProgressRange::default();
    assert!(
        provider
            .write_document(&name_of(&path), &document, &progress)
            .unwrap()
    );
    let text = take_text(&path);
    assert_eq!(
        vertex_rows(&text),
        expected_rows(&z_up_block(1.0, 2.0, 3.0)),
        "{text}"
    );
}

#[test]
fn a_shape_has_no_unit_of_its_own_and_is_taken_in_the_global_system_unit() {
    let global = DeSectionGlobal {
        length_unit: 1.0,
        system_unit: 1000.0,
    };
    let provider = provider_with(
        RwMeshCoordinateSystem::Z_UP,
        RwMeshCoordinateSystem::Z_UP,
        global,
    );
    let path = file_in_temp("system_unit.ply");
    let progress = ProgressRange::default();
    assert!(
        provider
            .write_shape(&name_of(&path), &meshed_block(1.0), &progress)
            .unwrap()
    );
    let text = take_text(&path);
    assert_eq!(
        vertex_rows(&text),
        expected_rows(&z_up_block(1000.0, 2000.0, 3000.0)),
        "{text}"
    );
}

#[test]
fn z_up_turns_to_y_up_on_positions_and_normals_by_default() {
    let (_node, provider) = node_and_provider();
    let path = file_in_temp("y_up.ply");
    let progress = ProgressRange::default();
    assert!(
        provider
            .write_shape(&name_of(&path), &meshed_block(1.0), &progress)
            .unwrap()
    );
    let text = take_text(&path);
    assert_eq!(vertex_rows(&text), expected_rows(&y_up_block()), "{text}");
}

#[test]
fn y_up_in_the_system_and_z_up_in_the_file_undoes_the_turn() {
    let provider = provider_with(
        RwMeshCoordinateSystem::Y_UP,
        RwMeshCoordinateSystem::Z_UP,
        MILLIMETERS,
    );
    let path = file_in_temp("z_up.ply");
    let progress = ProgressRange::default();
    assert!(
        provider
            .write_shape(&name_of(&path), &meshed_block(1.0), &progress)
            .unwrap()
    );
    let text = take_text(&path);
    // (x, y, z) in a Y-up system is (x, -z, y) in a Z-up file: the block's Y extent 2 becomes Z,
    // and its Z extent 3 runs down to Y = -3.
    let mut expected = Vec::new();
    for (normal, corners) in z_up_block(1.0, 2.0, 3.0) {
        let turn = |[x, y, z]: [f32; 3]| [x, -z, y];
        expected.push((turn(normal), corners.map(turn)));
    }
    assert_eq!(vertex_rows(&text), expected_rows(&expected), "{text}");
}

#[test]
fn a_metre_document_turns_and_scales_together() {
    let (_node, provider) = node_and_provider();
    let document = document_with(&meshed_block(1.0));
    XCAFDocDocumentTool::set_length_unit(&document, 1.0).unwrap();
    let path = file_in_temp("y_up_metres.ply");
    let progress = ProgressRange::default();
    assert!(
        provider
            .write_document(&name_of(&path), &document, &progress)
            .unwrap()
    );
    let text = take_text(&path);
    // Normals keep their length: only positions scale.
    let mut expected = y_up_block();
    for (_, corners) in &mut expected {
        *corners = corners.map(|corner| corner.map(|value| value * 1000.0));
    }
    assert_eq!(vertex_rows(&text), expected_rows(&expected), "{text}");
}

#[test]
fn equal_systems_and_units_write_the_bytes_of_a_writer_without_conversion() {
    let provider = provider_with(
        RwMeshCoordinateSystem::Z_UP,
        RwMeshCoordinateSystem::Z_UP,
        MILLIMETERS,
    );
    let shape = meshed_block(1.0);
    let document = document_with(&shape);
    XCAFDocDocumentTool::set_length_unit(&document, 0.001).unwrap();
    let (through_provider, plain) = (
        file_in_temp("same_system_provider.ply"),
        file_in_temp("same_system_plain.ply"),
    );
    let progress = ProgressRange::default();
    assert!(
        provider
            .write_document(&name_of(&through_provider), &document, &progress)
            .unwrap()
    );
    let information = Default::default();
    assert!(
        RwPlyCafWriter::new(&name_of(&plain))
            .perform(&document, &information, &progress)
            .unwrap()
    );
    let text = std::fs::read(&through_provider).unwrap();
    assert_eq!(text, std::fs::read(&plain).unwrap());
    std::fs::remove_file(&plain).unwrap();
    let text = take_text(&through_provider);
    assert_eq!(
        vertex_rows(&text),
        expected_rows(&z_up_block(1.0, 2.0, 3.0)),
        "{text}"
    );
}
