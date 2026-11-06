//! The commands that report on a shape, with the text each writes and the value a script reads back.
use crate::support::{
    commands::bound_shape, section_check, session::Session, tolerance_check::tolerance_report,
};
use martensite_tkbo::bop_tools::compute_curve_tolerance;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_tools,
    topexp::Explorer,
    topods::{self, Shape},
    toptools::MapOfShape,
};
use martensite_tkernel::{standard::Handle, storage::format_general};
use martensite_tkg3d::{
    geom::{GeomCurve, GeomSurface},
    geom_adaptor::GeomAdaptorCurve,
    top_abs::ShapeEnum,
};
use martensite_tkgeomalgo::geom_api::ProjectPointOnSurf;
use martensite_tkgeombase::{extrema::ExtAlgo, gcpnts::AbscissaPoint};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::{
    brep_check::{BRepCheckAnalyzer, BRepCheckEdge},
    brep_extrema::BRepExtremaDistShapeShape,
};

pub use super::check_text::*;

use crate::boolean_section_endpoints as section_endpoints;

/// `value` as reference `std::scientific` with five decimals: `1.00000e-07`.
pub fn scientific_five(value: f64) -> String {
    let text = format!("{value:.5e}");
    let (mantissa, exponent) = text.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{mantissa}e{sign}{:02}", exponent.abs())
}

fn tolerance_range(shape: &Shape, kind: ShapeEnum) -> (f64, f64, usize) {
    let (mut min, mut max) = (f64::MAX, -f64::MAX);
    let mut seen = MapOfShape::default();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let current = explorer.current().clone();
        let tolerance = match kind {
            ShapeEnum::Face => BRepTool::face_tolerance(&topods::face(&current).unwrap()),
            ShapeEnum::Edge => BRepTool::edge_tolerance(&topods::edge(&current).unwrap()),
            _ => BRepTool::vertex_tolerance(&topods::vertex(&current).unwrap()),
        }
        .unwrap();
        max = max.max(tolerance);
        min = min.min(tolerance);
        seen.add(current);
        explorer.next().unwrap();
    }
    (min, max, seen.extent())
}

/// The largest tolerances `maxtolerance` prints, as a script's regexp reads them back.
#[derive(Clone, Copy, Debug, Default)]
pub struct PrintedMaxima {
    /// `Face : ... Max`, when the shape has a face.
    pub face: Option<f64>,
    /// `Edge : ... Max`, when the shape has an edge.
    pub edge: Option<f64>,
    /// `Vertex : ... Max`, when the shape has a vertex.
    pub vertex: Option<f64>,
}

fn maxtolerance_text(shape: &Shape, name: &str) -> (String, PrintedMaxima) {
    let ranges = [ShapeEnum::Face, ShapeEnum::Edge, ShapeEnum::Vertex]
        .map(|kind| tolerance_range(shape, kind));
    let mut text = format!(
        "\n## Tolerances on the shape {name}  (nbFaces:{}  nbEdges:{} nbVtx:{})\n",
        ranges[0].2, ranges[1].2, ranges[2].2
    );
    let mut printed = [None; 3];
    for (index, label) in ["Face  ", "Edge  ", "Vertex"].iter().enumerate() {
        let (min, max, _) = ranges[index];
        if min <= max {
            let (low, high) = (scientific_five(min), scientific_five(max));
            text.push_str(&format!(
                "\n    {label} : Min {low:>8}    Max  {high:>8} \n "
            ));
            printed[index] = Some(high.parse().unwrap());
        }
    }
    text.push('\n');
    (
        text,
        PrintedMaxima {
            face: printed[0],
            edge: printed[1],
            vertex: printed[2],
        },
    )
}

/// `maxtolerance name`: writes its report and returns the maxima it prints.
/// Example: `maxtolerance(s, "cut1").face`.
pub fn maxtolerance(s: &dyn Session, name: &str) -> PrintedMaxima {
    let (text, maxima) = maxtolerance_text(&bound_shape(s, name), name);
    print!("{text}");
    maxima
}

/// `maxtolerance vertex`, read back by the script's regexp: the number and its text.
/// Example: `vertex_max_tolerance(s, "result_3").0`.
pub fn vertex_max_tolerance(s: &dyn Session, name: &str) -> (f64, String) {
    let shape = bound_shape(s, name);
    let (text, maxima) = maxtolerance_text(&shape, name);
    print!("{text}");
    let value = maxima
        .vertex
        .unwrap_or_else(|| panic!("maxtolerance {name}: expected a vertex"));
    let (_, max, _) = tolerance_range(&shape, ShapeEnum::Vertex);
    (value, scientific_five(max))
}

/// `puts [maxtolerance name]`: the command's report and its value written again.
/// Example: `maxtolerance_put(s, "cut1")`.
pub fn maxtolerance_put(s: &dyn Session, name: &str) {
    let (text, _) = maxtolerance_text(&bound_shape(s, name), name);
    print!("{text}{text}");
}

/// `checkmaxtol name`: writes the `tolerance` rows and returns the largest, as it prints.
/// Example: `checkmaxtol_value(s, "r1") > 0.0002`.
pub fn checkmaxtol_value(s: &dyn Session, name: &str) -> f64 {
    let printed = tolerance_report(s, name);
    printed
        .parse()
        .unwrap_or_else(|cause| panic!("tolerance {name}: {printed:?}: {cause}"))
}

/// `checkmaxtol name -min_tol limit`, with the procedure's message; `limit_text` is as the script
/// spells it after `expr`. Example: `checkmaxtol_min(s, "result", 1.0e-4, "0.0001")`.
pub fn checkmaxtol_min(s: &dyn Session, name: &str, limit: f64, limit_text: &str) {
    let printed = tolerance_report(s, name);
    let value: f64 = printed.parse().unwrap();
    if value > limit {
        println!(
            "Error: tolerance of \"{name}\" ({printed}) is greater than checking tolerance ({limit_text})"
        );
    }
}

/// `checkshape name r`: writes and returns the command's one-line report.
/// Example: `checkshape_report_line(s, "cut1").contains("OK")`.
pub fn checkshape_report_line(s: &dyn Session, name: &str) -> String {
    let shape = bound_shape(s, name);
    let analyzer = BRepCheckAnalyzer::new(&shape, true, false, false)
        .unwrap_or_else(|failure| panic!("checkshape {name} r: {failure:?}"));
    let valid = analyzer.is_valid().unwrap();
    let line = if valid {
        format!(" -- The Shape {name} looks OK")
    } else {
        format!("Faulty : the shape {name} is invalid")
    };
    println!("{line}");
    line
}

/// `puts [computetolerance name]`: each distinct edge's tolerance, written by the command and by
/// `puts`. The native text prints the edge's address; it is written as 0 here.
/// Example: `computetolerance_put(s, "res1")`.
pub fn computetolerance_put(s: &dyn Session, name: &str) {
    let shape = bound_shape(s, name);
    let mut seen = MapOfShape::default();
    let mut text = String::new();
    let mut explorer = Explorer::new(&shape, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let current = explorer.current().clone();
        if seen.add(current.clone()) {
            let tolerance = BRepCheckEdge::new(&topods::edge(&current).unwrap())
                .unwrap()
                .tolerance()
                .unwrap();
            text += &format!("Tolerance de 0x0 : {}\n", format_general(tolerance, 6));
        }
        explorer.next().unwrap();
    }
    print!("{text}\n{text}\n");
}

/// `checklength curve -l expected` of a 3D curve: the procedure's lines, an error beyond 1%.
/// Example: `checklength_curve("c", &c, 125.66)`.
pub fn checklength_curve(name: &str, curve: &Handle<dyn GeomCurve>, expected: f64) {
    let adaptor = GeomAdaptorCurve::with_curve(curve).unwrap();
    let length = AbscissaPoint::length_with_tolerance(&adaptor, 1.0e-4).unwrap();
    println!("The length {name} is {}", format_general(length, 17));
    println!("The expected length is {}", format_general(expected, 17));
    if length == 0. || ((expected - length) / expected).abs() > 0.01 {
        println!(
            "Error : The length of result shape is {}",
            format_general(length, 17)
        );
    }
}

/// `proj surface x y z`: one line per orthogonal projection; a failure stops the case.
/// Example: `proj_on_surface(&su1, [0., 100., 300.])`.
pub fn proj_on_surface(surface: &Handle<dyn GeomSurface>, at: [f64; 3]) {
    let point = Point3::new(at[0], at[1], at[2]);
    let bounds = surface.get().unwrap().bounds();
    let projection = ProjectPointOnSurf::with_bounds(point, surface, bounds, ExtAlgo::Grad)
        .unwrap_or_else(|failure| panic!("proj {at:?}: {failure:?}"));
    for index in 1..=projection.nb_points().unwrap() {
        let (u, v) = projection.parameters(index).unwrap();
        let label = if index == 1 { "Point on surface  " } else { "" };
        println!(
            "ext_{index}  {label}Parameters: {} {}",
            format_general(u, 17),
            format_general(v, 17)
        );
    }
    println!();
}

/// `distmini d first second`: writes the command's lines and returns the value of `d_val`.
/// Example: `distmini(s, "d", "result_3", "b_5")`.
pub fn distmini(s: &dyn Session, name: &str, first: &str, second: &str) -> f64 {
    let (first, second) = (bound_shape(s, first), bound_shape(s, second));
    let measure = BRepExtremaDistShapeShape::with_shapes(&first, &second, Default::default())
        .unwrap_or_else(|failure| panic!("distmini {name}: {failure:?}"));
    assert!(measure.is_done(), "distmini {name}: expected completion");
    println!("\"distmini\" command returns:\n{name}_val {name} \nOutput is complete.\n");
    measure.value().unwrap()
}

/// `dump name` of a real variable: writes it and returns its text, as a script's regexp reads it.
/// Example: `dump_real("d_val", 1.5)`.
pub fn dump_real(name: &str, value: f64) -> String {
    let text = format_fifteen(value);
    println!("\n\n*********** Dump of {name} *************\n{text}\n");
    text
}

/// A real variable as `dump` writes it, fifteen significant digits; `format_fifteen(1.5)`.
pub fn format_fifteen(value: f64) -> String {
    format_general(value, 15)
}

/// `checksection name` as a script reads it back: writes its text and returns the count.
/// Example: `if checksection_count(s, "result") != 0 { println!("Faulty"); }`.
pub fn checksection_count(s: &mut dyn Session, name: &str) -> usize {
    section_check::checksection(s, name);
    match s.shape(name) {
        Some(shape) => section_endpoints::odd_incidence_vertices(shape).len(),
        None => 0,
    }
}

/// `puts [checksection name]`: the report written again by `puts`.
/// Example: `checksection_put(s, "result")`.
pub fn checksection_put(s: &mut dyn Session, name: &str) {
    section_check::checksection(s, name);
    let names = checksection_names(s, name);
    println!(
        " nb alone Vertices : {}\n{}\n",
        names.len(),
        names.join(" ")
    );
}

fn checksection_names(s: &dyn Session, name: &str) -> Vec<String> {
    let found = s.shape(name).map_or(0, |shape| {
        section_endpoints::odd_incidence_vertices(shape).len()
    });
    (1..=found).map(|index| format!("alone_{index}")).collect()
}

/// `statshape name`: writes the command's count table and says whether it names a vertex.
/// Example: `statshape_has_vertex(s, "r1")`.
pub fn statshape_has_vertex(s: &dyn Session, name: &str) -> bool {
    let table = statshape_table(&bound_shape(s, name));
    println!("Count    Item\n-----    ----");
    for (count, item) in &table {
        println!("{count}\t {item}");
    }
    println!();
    table.iter().any(|(_, item)| item.contains("VERTEX"))
}

fn statshape_table(shape: &Shape) -> Vec<(usize, String)> {
    let mut table = Vec::new();
    for kind in [ShapeEnum::Edge, ShapeEnum::Vertex] {
        let word = if kind == ShapeEnum::Edge {
            "EDGE"
        } else {
            "VERTEX"
        };
        let mut oriented = 0;
        let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
        let mut shared = MapOfShape::default();
        while explorer.more() {
            oriented += 1;
            shared.add(explorer.current().clone());
            explorer.next().unwrap();
        }
        if oriented > 0 {
            table.push((oriented, format!("{word} (Oriented)")));
            table.push((shared.extent(), format!("{word} (Shared)")));
        }
        if kind == ShapeEnum::Edge && oriented > 0 {
            let mut free = 0;
            let mut outside = Explorer::new(shape, kind, ShapeEnum::Wire).unwrap();
            while outside.more() {
                free += 1;
                outside.next().unwrap();
            }
            table.push((free, "EDGE (Free)".to_owned()));
        }
    }
    table
}

/// What `dump` writes after the shapes: the shape set's empty tables.
const DUMP_TAIL: &str = "\n\n -------\nDump of 0 Curve2ds \n -------\n\n\n -------\nDump of 0 Curves \n -------\n\n -------\nDump of 0 Polygon3Ds\n -------\n -------\nDump of 0 PolygonOnTriangulations\n -------\n\n -------\nDump of 0 surfaces \n -------\n\n -------\nDump of 0 Triangulations\n -------\n\n\n\n -------\n Dump of 0 Locations\n -------\n\n";

/// The eight flags of a shape set's dump, in the order of its legend.
fn flag_text(shape: &Shape) -> String {
    let tshape = shape.tshape();
    let tshape = tshape.get().expect("dump: expected a shape");
    [
        tshape.free(),
        tshape.modified(),
        tshape.checked(),
        tshape.orientable(),
        tshape.closed(),
        tshape.infinite(),
        tshape.convex(),
        tshape.locked(),
    ]
    .map(|flag| if flag { '1' } else { '0' })
    .iter()
    .collect()
}

/// The head of `dump`, for the root at `index` of `count` shapes.
fn dump_head(name: &str, shape: &Shape, index: usize, count: usize) {
    let orientation = format!("{:?}", shape.orientation()).to_uppercase();
    println!(
        "\n\n*********** Dump of {name} *************\nShape : {index}, {orientation}\n\nDump of {count} TShapes\n\n-----------------\n\nFlags : Free, Modified, Checked, Orientable, Closed, Infinite, Convex, Locked\n"
    );
}

/// The dump's entry of a vertex, and its point and tolerance as a script's regexps read them back.
fn dump_vertex_entry(index: usize, shape: &Shape) -> ([f64; 3], f64) {
    let vertex = topods::vertex(shape).unwrap();
    let point = BRepTool::pnt(&vertex).unwrap();
    let tolerance = format_general(BRepTool::vertex_tolerance(&vertex).unwrap(), 15);
    let texts = [point.x(), point.y(), point.z()].map(|value| format_general(value, 15));
    println!(
        "TShape # {index} : VERTEX    {} 0x0\n    \n    Tolerance : {tolerance}\n    - Point 3D : {}, {}, {}\n",
        flag_text(shape),
        texts[0],
        texts[1],
        texts[2]
    );
    (
        texts.map(|text| text.parse().unwrap()),
        tolerance.parse().unwrap(),
    )
}

/// `dump name` of a vertex or of a compound of one vertex: the command's report, and the values a
/// script's regexps read from it. The address native prints is written as 0.
/// Example: `let (at, tolerance) = vertex_dump(s, "result_2_1");`.
pub fn vertex_dump(s: &dyn Session, name: &str) -> ([f64; 3], f64) {
    let shape = bound_shape(s, name);
    if shape.shape_type().unwrap() == ShapeEnum::Vertex {
        dump_head(name, &shape, 1, 1);
        let values = dump_vertex_entry(1, &shape);
        print!("{DUMP_TAIL}");
        return values;
    }
    let parts = crate::support::commands::distinct_parts(&shape, ShapeEnum::Vertex);
    let [vertex] = parts.as_slice() else {
        panic!("dump {name}: expected one vertex, got {}", parts.len());
    };
    assert_eq!(
        shape.shape_type().unwrap(),
        ShapeEnum::Compound,
        "dump {name}"
    );
    dump_head(name, &shape, 2, 2);
    println!(
        "TShape # 1 : COMPOUND  {} 0x0\n    +2 \n",
        flag_text(&shape)
    );
    let values = dump_vertex_entry(2, vertex);
    print!("{DUMP_TAIL}");
    values
}

/// `whatis name`: the words the command answers; a harness echo carries them, so nothing is written.
/// Example: `whatis(s, "r_1").contains("Closed")`.
pub fn whatis(s: &dyn Session, name: &str) -> String {
    let shape = bound_shape(s, name);
    let kind = format!("{:?}", shape.shape_type().unwrap()).to_uppercase();
    let orientation = format!("{:?}", shape.orientation()).to_uppercase();
    let tshape = shape.tshape();
    let Some(tshape) = tshape.get() else {
        panic!("whatis {name}: expected a shape");
    };
    let mut text = format!("{name} is a shape {kind} {orientation}");
    for (on, word) in [
        (tshape.free(), "Free"),
        (tshape.modified(), "Modified"),
        (tshape.checked(), "Checked"),
        (tshape.orientable(), "Orientable"),
        (tshape.closed(), "Closed"),
        (tshape.infinite(), "Infinite"),
        (tshape.convex(), "Convex"),
    ] {
        if on {
            text.push(' ');
            text.push_str(word);
        }
    }
    text
}

/// `regexp kind [whatis name]`: writes the command's line and says whether it names the kind.
/// Example: `whatis_has(s, "r_1", "SHELL")`.
pub fn whatis_has(s: &dyn Session, name: &str, kind: &str) -> bool {
    whatis(s, name).split(' ').any(|word| word == kind)
}

/// `xbounds face`: writes and returns the four numbers, as the command prints them.
/// Example: `xbounds(s, "pryz_1")[2]`.
pub fn xbounds(s: &dyn Session, name: &str) -> [String; 4] {
    let face = topods::face(&bound_shape(s, name)).unwrap();
    let (u1, u2, v1, v2) = brep_tools::uv_bounds(&face).unwrap();
    let texts = [u1, u2, v1, v2].map(|value| format_general(value, 6));
    println!("{}\n", texts.join("\n"));
    texts
}

/// `xdistef edge face`: writes the command's line and returns the distance a script's regexp reads.
/// Example: `tolerance_read_edge_face(&edge, &face)`.
pub fn tolerance_read_edge_face(edge: &Shape, face: &Shape) -> f64 {
    let (mut distance, mut parameter) = (0.0, 0.0);
    let found = compute_curve_tolerance(
        &topods::face(face).unwrap(),
        &topods::edge(edge).unwrap(),
        &mut distance,
        &mut parameter,
    )
    .unwrap();
    assert!(found, "xdistef: expected a tolerance");
    println!(
        "Max Distance = {}; Parameter on curve = {}",
        format_general(distance, 17),
        format_general(parameter, 17)
    );
    distance
}

/// `compare a b`: writes and returns the answer; `compare(s, "r_1_1", "r_2_1")`.
pub fn compare(s: &dyn Session, first: &str, second: &str) -> String {
    let (first, second) = (bound_shape(s, first), bound_shape(s, second));
    let text = if !first.is_same(&second) {
        "shapes are not same\n"
    } else if first.is_equal(&second) {
        "same shapes\nequal shapes\n"
    } else {
        "same shapes\n"
    };
    print!("{text}");
    text.to_owned()
}
