//! Source relations use retained handles; geometry ordering is only a diagnostic multiset.
use super::FillGap;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_tools::{History, uv_bounds},
    topexp,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
fn unique_trim_shapes(face: &Shape, kind: ShapeEnum) -> usize {
    let mut shapes = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(face, kind, &mut shapes).unwrap();
    shapes.extent()
}
fn trim_geometry(face: &Shape) -> Vec<u64> {
    let typed = topods::face(face).unwrap();
    let (a, b, c, d) = uv_bounds(&typed).unwrap();
    vec![
        face.orientation() as u64,
        BRepTool::face_tolerance(&typed).unwrap().to_bits(),
        a.to_bits(),
        b.to_bits(),
        c.to_bits(),
        d.to_bits(),
        unique_trim_shapes(face, ShapeEnum::Edge) as u64,
        unique_trim_shapes(face, ShapeEnum::Vertex) as u64,
        unique_trim_shapes(face, ShapeEnum::Wire) as u64,
    ]
}
pub(super) fn source_history_flags(gap: &FillGap, history: &History, source: &Shape) -> Vec<usize> {
    let faces = gap.faces.seek(source);
    let modified = history.modified(source).unwrap();
    let matched = faces
        .into_iter()
        .flat_map(|faces| faces.iter())
        .map(|face| modified.iter().filter(|image| image.is_same(face)).count())
        .sum();
    vec![
        usize::from(faces.is_some()),
        faces.map_or(0, |faces| faces.len()),
        modified.len(),
        history.generated(source).unwrap().len(),
        usize::from(history.is_removed(source).unwrap()),
        matched,
    ]
}
pub(super) fn source_geometry(gap: &FillGap, source: &Shape) -> Vec<Vec<u64>> {
    let mut rows: Vec<_> = gap
        .faces
        .seek(source)
        .into_iter()
        .flat_map(|faces| faces.iter())
        .map(trim_geometry)
        .collect();
    rows.sort_unstable();
    rows
}
