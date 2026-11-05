//! Seeded face-intersection regression for the M3 section fixture.
use super::*;
use martensite_tkbo::{brep_algo_api::BRepAlgoAPISection, int_tools::FaceFace};
use martensite_tkbrep::{topexp::Explorer, topods};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;

const SOURCE: &str = "tests/boolean/bsection/M3";
const SOURCE_TOLERANCE: f64 = 2.831_249_850_021_027e-7;

fn source_seed() -> IntSurfPntOn2S {
    let mut seed = IntSurfPntOn2S::default();
    seed.set_parameters(
        0.206_392_063_395_452_24,
        0.692_608_328_433_853,
        0.0,
        0.017_002_507_022_347_624,
    );
    seed
}

fn seeded_face_intersection(first: &Shape, second: &Shape) -> FaceFace {
    let mut intersection = FaceFace::default();
    intersection.set_parameters(true, true, true, 1e-7);
    intersection.set_list(&[source_seed()]);
    intersection
        .perform(
            &topods::face(first).expect("M3 first fixture member is a face"),
            &topods::face(second).expect("M3 second fixture member is a face"),
            false,
        )
        .expect("M3 bopcurves intersection");
    intersection
        .prepare_lines_3d(false)
        .expect("M3 bopcurves preparation");
    intersection
}

fn max_curve_tolerance(intersection: &FaceFace) -> f64 {
    intersection
        .lines()
        .expect("M3 bopcurves completed")
        .iter()
        .map(|curve| curve.tolerance())
        .fold(0.0, f64::max)
}

fn exploded_faces(fixture: &Shape) -> Vec<Shape> {
    Explorer::new(fixture, ShapeEnum::Face, ShapeEnum::Shape)
        .expect("M3 face exploration")
        .collect()
}

fn section_length(first: &Shape, second: &Shape) -> f64 {
    let section = BRepAlgoAPISection::from_shapes(first, second, None, true).expect("M3 bsection");
    assert!(section.is_done(), "{SOURCE}: bsection incomplete");
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false)
        .expect("M3 section length");
    properties.mass()
}

#[test]
fn BSection__M3() {
    let fixture = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/GEN758_nofog.rle"
    ));
    let faces = exploded_faces(&fixture);
    assert_eq!(faces.len(), 2, "{SOURCE}: expected two direct faces");
    let curves = seeded_face_intersection(&faces[0], &faces[1]);
    assert_eq!(
        curves.lines().expect("M3 bopcurves completed").len(),
        1,
        "{SOURCE}: source expects one curve"
    );
    let tolerance = max_curve_tolerance(&curves);
    assert!(
        (tolerance - SOURCE_TOLERANCE).abs() <= SOURCE_TOLERANCE * 0.01,
        "{SOURCE}: tolerance {tolerance}; expected {SOURCE_TOLERANCE} within 1%"
    );
    let length = section_length(&faces[0], &faces[1]);
    assert!(
        (length - 22.2575).abs() / 22.2575 <= 0.01,
        "{SOURCE}: length {length}; expected 22.2575 within 1%"
    );
}
