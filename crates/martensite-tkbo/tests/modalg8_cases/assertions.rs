use super::*;

pub(super) fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BREP fixture header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("source restore bug32721.brep");
    shape
}

pub(super) fn check_valid(source: &str, shape: &Shape) {
    let checker = BRepCheckAnalyzer::new(shape, true, false, false).expect("source checkshape");
    assert!(
        checker.is_valid().expect("source validity"),
        "{source}: invalid result"
    );
}
pub(super) fn count_located(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut indexed = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut indexed).expect("source checknbshapes -t");
    indexed.extent()
}
pub(super) fn count_entries(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut entries = BRepToolsShapeSet::default();
    entries.add(shape).expect("source nbshapes entries");
    (1..=entries.nb_shapes())
        .filter(|index| entries.shape(*index).unwrap().shape_type().unwrap() == kind)
        .count()
}
pub(super) fn maximum_tolerance(shape: &Shape) -> f64 {
    [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face]
        .into_iter()
        .map(|kind| BRepTool::max_tolerance(shape, kind).expect("source checkmaxtol"))
        .fold(0.0, f64::max)
}
pub(super) fn check_mass(label: &str, actual: f64, expected: f64) {
    let printed: f64 = format_general(actual, 6)
        .parse()
        .expect("source numeric mass");
    assert_ne!(
        printed, 0.0,
        "{label}: source checkprops requires nonzero mass"
    );
    assert!(
        (printed - expected).abs() / expected <= 0.01,
        "{label}: mass {printed}, expected {expected}"
    );
}
pub(super) fn check_area(label: &str, shape: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties_eps(shape, &mut properties, 1.0e-4, false)
        .expect("source adaptive sprops");
    check_mass(label, properties.mass(), expected);
}
pub(super) fn check_volume(label: &str, shape: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::volume_properties_eps(shape, &mut properties, 1.0e-4, false, false)
        .expect("source adaptive vprops");
    check_mass(label, properties.mass(), expected);
}
