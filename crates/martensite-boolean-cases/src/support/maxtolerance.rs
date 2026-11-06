//! `maxtolerance name`: the smallest and largest tolerance of each kind, as the command writes
//! them. Its answer is written once by the command and once more by a script that prints it.
use super::session::Session;
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The numbers of one kind: distinct parts, smallest and largest tolerance.
struct Range {
    count: usize,
    smallest: f64,
    largest: f64,
}

fn tolerance_of(part: &Shape) -> f64 {
    match part.shape_type().unwrap() {
        ShapeEnum::Face => BRepTool::face_tolerance(&topods::face(part).unwrap()).unwrap(),
        ShapeEnum::Edge => BRepTool::edge_tolerance(&topods::edge(part).unwrap()).unwrap(),
        ShapeEnum::Vertex => BRepTool::vertex_tolerance(&topods::vertex(part).unwrap()).unwrap(),
        other => panic!("maxtolerance: {other:?} has no tolerance"),
    }
}

fn range_of(shape: &Shape, kind: ShapeEnum) -> Range {
    let mut distinct = IndexedMapOfShape::default();
    let (mut smallest, mut largest) = (f64::MAX, -f64::MAX);
    for part in Explorer::new(shape, kind, ShapeEnum::Shape).unwrap() {
        let tolerance = tolerance_of(&part);
        smallest = smallest.min(tolerance);
        largest = largest.max(tolerance);
        distinct.add(part);
    }
    Range {
        count: distinct.extent(),
        smallest,
        largest,
    }
}

/// A value as C's `%.5e` writes it: at least two exponent digits and a sign.
fn scientific(value: f64) -> String {
    let text = format!("{value:.5e}");
    let (mantissa, exponent) = text.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    format!("{mantissa}e{exponent:+03}")
}

fn row(title: &str, range: &Range) -> String {
    let (smallest, largest) = (scientific(range.smallest), scientific(range.largest));
    format!("    {title}: Min {smallest}    Max  {largest} \n \n")
}

/// The answer of `maxtolerance name`; `let tolerance = maxtolerance(s, "b1");`.
pub fn maxtolerance(s: &dyn Session, name: &str) -> String {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("maxtolerance {name}: expected a bound shape"));
    let face = range_of(shape, ShapeEnum::Face);
    let edge = range_of(shape, ShapeEnum::Edge);
    let vertex = range_of(shape, ShapeEnum::Vertex);
    let head = format!(
        "\n## Tolerances on the shape {name}  (nbFaces:{}  nbEdges:{} nbVtx:{})\n\n",
        face.count, edge.count, vertex.count
    );
    let rows = [("Face   ", &face), ("Edge   ", &edge), ("Vertex ", &vertex)];
    let body: String = rows
        .iter()
        .map(|(title, range)| row(title, range))
        .collect();
    head + &body
}
