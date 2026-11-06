//! A parallel shell closed by the splits of a cap keeps the orientation of its offset faces,
//! whatever order the shape addresses give the quilt (`offset/compshape/A3`).
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::brep_offset::{BRepOffsetMakeOffset, BRepOffsetMode, MakeOffsetOptions};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;

const SOLID: &[u8] =
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/CCV_2_c1_gsv.rle");
/// The volume `checkprops result -v` expects of `offset/compshape/A3`, and its relative tolerance.
const EXPECTED_VOLUME: f64 = 3.0764e+06;
const RELATIVE_TOLERANCE: f64 = 0.01;
/// Runs in one process. Without the fix some run inverted the shell in 12 of 12 test runs, by the
/// seventh run at the latest.
const RUNS: usize = 16;

fn read_solid() -> Shape {
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(SOLID.to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    shape
}

/// `offsetcompshape result s -10 s_8 s_10`.
fn thick_solid(solid: &Shape) -> Shape {
    let mut faces = IndexedMapOfShape::default();
    for face in Explorer::new(solid, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        faces.add(face);
    }
    let options = MakeOffsetOptions {
        mode: BRepOffsetMode::Skin,
        intersection: true,
        self_inter: false,
        join: JoinType::Arc,
        thickening: false,
        remove_int_edges: false,
    };
    let mut builder = BRepOffsetMakeOffset::new();
    builder.initialize(solid, -10.0, 1.0e-7, options);
    for index in [8, 10] {
        builder.add_face(&topods::face(faces.find_key(index).unwrap()).unwrap());
    }
    builder.make_thick_solid(&ProgressRange::default()).unwrap();
    assert!(builder.is_done());
    builder.shape().clone()
}

fn volume(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
    properties.mass()
}

#[test]
fn the_offset_faces_orient_the_parallel_shell_at_every_address_layout() {
    // Results and padding stay alive so that every run allocates its shapes elsewhere.
    let mut kept = Vec::new();
    for run in 0..RUNS {
        let solid = read_solid();
        let result = thick_solid(&solid);
        let valid = BRepCheckAnalyzer::new(&result, true, false, false)
            .and_then(|analyzer| analyzer.is_valid())
            .unwrap();
        assert!(valid, "run {run}: invalid solid");
        let measured = volume(&result);
        assert!(
            ((measured - EXPECTED_VOLUME) / EXPECTED_VOLUME).abs() <= RELATIVE_TOLERANCE,
            "run {run}: volume {measured}, expected {EXPECTED_VOLUME}"
        );
        kept.push((solid, result, vec![0_u8; 4096 + 1024 * run]));
    }
}
