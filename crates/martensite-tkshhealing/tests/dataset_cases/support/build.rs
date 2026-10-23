//! The construction and count commands of the scripts that build their inputs: BRepTest
//! `polyline` and `polyvertex` (BRepBuilderAPI_MakePolygon), `psphere`
//! (BRepPrimAPI_MakeSphere), and DBRep `nbshapes shape -t` with the `checknbshapes -t`
//! shape-count consistency check.
use super::log;
use super::logged_checks::nbshapes_count;
use martensite_tkbrep::{
    topexp,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape};

/// `polyline name x1 y1 z1 x2 y2 z2 ...`: the polygon wire through the points.
pub fn polyline(points: &[[f64; 3]]) -> Shape {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for [x, y, z] in points {
        polygon.add_point(Point3::new(*x, *y, *z)).unwrap();
    }
    polygon
        .wire()
        .expect("polyline: wire not done")
        .into_shape()
}

/// `polyvertex name v1 v2 ...`: the polygon wire through the vertices.
pub fn polyvertex(vertices: &[&Shape]) -> Shape {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for vertex in vertices {
        let vertex = topods::vertex(vertex).expect("polyvertex: a vertex");
        polygon.add_vertex(&vertex).unwrap();
    }
    polygon
        .wire()
        .expect("polyvertex: wire not done")
        .into_shape()
}

/// `psphere name r [angle1 angle2 [angle]]` (degrees): the sphere, the latitude band,
/// or the band over a longitude sector.
pub fn psphere(radius: f64, latitudes: Option<(f64, f64)>, angle: Option<f64>) -> Shape {
    let maker = match (latitudes, angle) {
        (None, None) => BRepPrimAPIMakeSphere::new(radius),
        (None, Some(angle)) => BRepPrimAPIMakeSphere::with_angle(radius, angle.to_radians()),
        (Some((a1, a2)), None) => {
            BRepPrimAPIMakeSphere::with_latitudes(radius, a1.to_radians(), a2.to_radians())
        }
        (Some((a1, a2)), Some(angle)) => BRepPrimAPIMakeSphere::with_latitudes_angle(
            radius,
            a1.to_radians(),
            a2.to_radians(),
            angle.to_radians(),
        ),
    };
    let mut maker = maker.unwrap_or_else(|failure| panic!("psphere: {failure:?}"));
    maker.shape().unwrap().clone()
}

/// The type labels of `DumpExtent`, from COMPOUND down to VERTEX.
const EXTENT_TYPES: [(ShapeEnum, &str); 8] = [
    (ShapeEnum::Compound, " COMPOUND  : "),
    (ShapeEnum::CompSolid, " COMPSOLID : "),
    (ShapeEnum::Solid, " SOLID     : "),
    (ShapeEnum::Shell, " SHELL     : "),
    (ShapeEnum::Face, " FACE      : "),
    (ShapeEnum::Wire, " WIRE      : "),
    (ShapeEnum::Edge, " EDGE      : "),
    (ShapeEnum::Vertex, " VERTEX    : "),
];

/// `nbshapes name -t`: per type the `TopExp::MapShapes` extent (sub-shapes
/// counted once per location), from VERTEX up to COMPOUND, then their sum as SHAPE.
pub fn nbshapes_t(shape: &Shape, name: &str) -> String {
    let mut text = format!("Number of shapes in {name}\n");
    let mut total = 0;
    for (kind, label) in EXTENT_TYPES.iter().rev() {
        let mut map = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(shape, *kind, &mut map).unwrap();
        text.push_str(&format!("{label}{}\n", map.extent()));
        total += map.extent();
    }
    text.push_str(&format!(" SHAPE     : {total}\n"));
    text
}

/// `checknbshapes name -t [-vertex n] ... [-shape n] [-ref text] [-m message]`: each
/// entity with an expected count, or found in the `-ref` text, must have it in
/// `nbshapes name -t`.
pub fn checknbshapes_t(
    shape: &Shape,
    name: &str,
    expected: &[(&str, usize)],
    reference: Option<&str>,
    message: &str,
) {
    log::puts(&format!("checknbshapes {name}"));
    let nb_info = nbshapes_t(shape, name);
    log::result(&nb_info);
    for entity in super::checks::NBSHAPES_ENTITIES {
        let found = nbshapes_count(&nb_info, entity)
            .unwrap_or_else(|| panic!("Error : command \"nbshapes {name}\" gives an empty result"));
        let mut compare = vec![found];
        compare.extend(
            expected
                .iter()
                .filter(|(e, _)| *e == entity)
                .map(|(_, n)| *n),
        );
        compare.extend(reference.and_then(|text| nbshapes_count(text, entity)));
        if compare.len() == 1 {
            continue;
        }
        compare.sort_unstable();
        compare.dedup();
        if compare.len() != 1 {
            log::puts(&format!(
                "Error : {message} is WRONG because number of {entity} entities in shape \"{name}\" is {found}"
            ));
        } else {
            log::puts(&format!(
                "OK : {message} is GOOD because number of {entity} entities is equal to number of expected {entity} entities"
            ));
        }
    }
}

/// `brestore file name`: BRepTools::Read of the whole located file.
pub fn brestore(file: &str) -> Shape {
    let mut shape = Shape::default();
    let mut stream =
        martensite_tkernel::storage::IoStream::from_bytes(super::dataset::located_bytes(file));
    martensite_tkbrep::brep_tools::read(
        &mut shape,
        &mut stream,
        &martensite_tkbrep::brep::BRepBuilder::default(),
        &martensite_tkernel::message::ProgressRange::default(),
    )
    .unwrap_or_else(|failure| panic!("brestore {file}: {failure:?}"));
    shape
}

/// A shape summary: its type, orientation and
/// set TShape flags (Free, Modified, Orientable, Closed, Infinite, Convex).
pub fn whatis(name: &str, shape: &Shape) -> String {
    use martensite_tkg3d::top_abs::{shape_orientation_to_string, shape_type_to_string};
    let tshape = shape.tshape().get().expect("whatis: a non-null shape");
    let mut text = format!(
        "{name} is shape {} {}",
        shape_type_to_string(shape.shape_type().unwrap()),
        shape_orientation_to_string(shape.orientation())
    );
    let flags = [
        (tshape.free(), " Free"),
        (tshape.modified(), " Modified"),
        (tshape.orientable(), " Orientable"),
        (tshape.closed(), " Closed"),
        (tshape.infinite(), " Infinite"),
        (tshape.convex(), " Convex"),
    ];
    for (set, word) in flags {
        if set {
            text.push_str(word);
        }
    }
    text
}

/// `set t [maxtolerance name]` and the scripts' `regexp { +Face +: +Min +[-0-9.+eE]+ +Max
/// +([-0-9.+eE]+)}` (then Edge, Vertex): the printed `%.5e` maxima of the faces, edges and
/// vertices; a kind the shape lacks makes the regexp fail and the variable undefined.
pub fn maxtolerance_printed(name: &str, shape: &Shape) -> [f64; 3] {
    let text = super::topology::maxtolerance_text(name, shape);
    ["Face", "Edge", "Vertex"].map(|kind| {
        let line = text
            .lines()
            .find(|line| line.trim_start().starts_with(kind))
            .unwrap_or_else(|| panic!("can't read variable: no {kind} tolerance"));
        let max = line.split("Max").nth(1).expect("Max");
        max.split_whitespace().next().unwrap().parse().unwrap()
    })
}

/// BRepTest `edge name v1 v2`: BRepBuilderAPI_MakeEdge between the vertices.
pub fn edge(v1: &Shape, v2: &Shape) -> Shape {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
    let v1 = topods::vertex(v1).expect("edge: a vertex");
    let v2 = topods::vertex(v2).expect("edge: a vertex");
    BRepBuilderAPIMakeEdge::from_vertices(&v1, &v2)
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}

/// BRepTest `wire name -unsorted s1 s2 ...`: every edge of the shapes added at once
/// (BRepBuilderAPI_MakeWire::Add of the list); the command's error text when not done.
pub fn wire_unsorted(parts: &[&Shape]) -> Result<Shape, String> {
    use martensite_tkbrep::topexp::Explorer;
    use martensite_tkernel::ncollection::List;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeWire;
    let mut edges = List::default();
    for part in parts {
        let mut explorer = Explorer::new(part, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
        while explorer.more() {
            edges.append(explorer.current().clone());
            explorer.next().unwrap();
        }
    }
    let mut maker = BRepBuilderAPIMakeWire::new();
    maker.add_list(&edges).unwrap();
    match maker.shape() {
        Ok(shape) => Ok(shape.clone()),
        Err(_) => Err(format!(
            "Wire not done with an error:\n{:?}\n",
            maker.error()
        )),
    }
}

/// DBRep `nexplode name f|e|v`: the distinct sub-shapes in exploration order, sorted by
/// `999 X + 99 Y + 0.9 Z` of their centre of mass (a vertex: its point) with the native
/// bubble sort; the `Warning!` line when two keys are equal.
pub fn nexplode(shape: &Shape, kind: ShapeEnum) -> (Vec<Shape>, String) {
    let parts = super::commands::explode_type(shape, kind);
    let keys: Vec<f64> = parts.iter().map(nexplode_key).collect();
    let mut order: Vec<usize> = (0..parts.len()).collect();
    let mut unsorted = true;
    while unsorted {
        unsorted = false;
        for i in 0..order.len().saturating_sub(1) {
            if keys[order[i]] > keys[order[i + 1]] {
                order.swap(i, i + 1);
                unsorted = true;
            }
        }
    }
    let mut text = String::new();
    for pair in order.windows(2) {
        if keys[pair[1]] == keys[pair[0]] {
            text.push_str("Warning! For this shape the results may be incorrect.\n");
        }
    }
    (order.into_iter().map(|i| parts[i].clone()).collect(), text)
}

fn nexplode_key(part: &Shape) -> f64 {
    let point = if part.shape_type().unwrap() == ShapeEnum::Vertex {
        martensite_tkbrep::brep::BRepTool::pnt(&topods::vertex(part).unwrap()).unwrap()
    } else {
        let mut properties = martensite_tkgeombase::gprop::GProps::default();
        martensite_tktopalgo::brep_gprop::BRepGProp::linear_properties(
            part,
            &mut properties,
            false,
            false,
        )
        .unwrap();
        properties.centre_of_mass()
    };
    point.x() * 999. + point.y() * 99. + point.z() * 0.9
}
