//! The remaining commands the primitive cases call: `nbshapes -t` and the
//! `checknbshapes` proc with `-t`/`-m`, `compare`, `checkloc`, `scalexyz`, `fscale`/
//! `bscale`, `BRepIntCS`, `bsave`/`brestore` and `binsave`/`binrestore` of a file, and
//! the diagnostic commands `OCC26313`, `BUC60852`, `BUC60889` and `QANullifyShape`.
use super::checks::{NBSHAPES_ENTITIES, nbshapes};
use super::log;
use super::logged_checks::nbshapes_count;
use super::printed::g17;
use super::restore::Restored;
use super::session::Session;
use martensite_tkbrep::{
    bintools,
    brep::BRepBuilder,
    brep_adaptor::BRepAdaptorCurve,
    brep_tools, topexp,
    topods::{self, Compound, Shape, Vertex},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{
    message::ProgressRange, standard::Handle, storage::IoStream, tcollection::AsciiString,
};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve, geom::GeomCurve, geom_adaptor::GeomAdaptorCurve, top_abs::ShapeEnum,
};
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::{GeneralTransform3, Matrix3, Point3, Transform3},
    toploc::Location,
};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIGTransform, BRepBuilderAPIMakeShape},
    brep_int_curve_surface::BRepIntCurveSurfaceInter,
};
use std::path::Path;

/// `nbshapes name -t` (`DumpExtent`): the counts of distinct sub-shapes of each
/// type counting locations (`TopExp::MapShapes` per type), then their sum.
pub fn nbshapes_total(name: &str, shape: &Shape) -> String {
    let names = [
        " COMPOUND  : ",
        " COMPSOLID : ",
        " SOLID     : ",
        " SHELL     : ",
        " FACE      : ",
        " WIRE      : ",
        " EDGE      : ",
        " VERTEX    : ",
    ];
    let kinds = [
        ShapeEnum::Compound,
        ShapeEnum::CompSolid,
        ShapeEnum::Solid,
        ShapeEnum::Shell,
        ShapeEnum::Face,
        ShapeEnum::Wire,
        ShapeEnum::Edge,
        ShapeEnum::Vertex,
    ];
    let mut text = format!("Number of shapes in {name}\n");
    let mut total = 0;
    for index in (0..kinds.len()).rev() {
        let mut map = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(shape, kinds[index], &mut map).unwrap();
        text.push_str(&format!("{}{}\n", names[index], map.extent()));
        total += map.extent();
    }
    text.push_str(&format!(" SHAPE     : {total}\n"));
    text
}

/// `checknbshapes name [-vertex n] ... [-shape n] [-t] [-m message] [-ref text]`: each
/// entity given an expected count, or found in the `-ref` text, must have it.
pub fn checknbshapes(
    name: &str,
    shape: &Shape,
    expected: [Option<usize>; 9],
    reference: Option<&str>,
    total: bool,
    message: &str,
) {
    log::puts(&format!("checknbshapes {name}"));
    let nb_info = if total {
        nbshapes_total(name, shape)
    } else {
        nbshapes(shape)
    };
    log::result(&nb_info);
    for (index, entity) in NBSHAPES_ENTITIES.iter().enumerate() {
        let found = nbshapes_count(&nb_info, entity)
            .unwrap_or_else(|| panic!("Error : command \"nbshapes {name}\" gives an empty result"));
        let mut compare = vec![found];
        compare.extend(expected[index]);
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

/// `BRepTest_BasicCommands` `compare shape1 shape2`, logged.
pub fn compare(first: &Shape, second: &Shape) -> String {
    let text = if first.is_same(second) {
        if first.is_equal(second) {
            "same shapes\nequal shapes\n"
        } else {
            "same shapes\n"
        }
    } else {
        "shapes are not same\n"
    };
    log::result(text);
    text.to_string()
}

/// `BRepTest_BasicCommands` `checkloc name`: `BRepTools::CheckLocations`, each
/// problematic shape set as `name_i`, the names (or `There are no problematic shapes`).
pub fn checkloc(session: &mut dyn Session, name: &str, shape: &Shape) -> String {
    let mut problems = Vec::new();
    brep_tools::check_locations(shape, &mut problems);
    let text = if problems.is_empty() {
        "There are no problematic shapes\n".to_string()
    } else {
        let mut text = String::new();
        for (index, problem) in problems.iter().enumerate() {
            let var = format!("{name}_{}", index + 1);
            session.set(&var, &Restored::Shape(problem.clone()));
            text.push_str(&var);
            text.push(' ');
        }
        text.push('\n');
        text
    };
    log::result(&text);
    text
}

/// `BRepTest_BasicCommands` `scalexyz res shape fx fy fz`:
/// `BRepBuilderAPI_GTransform(S, gp_GTrsf(diag(fx, fy, fz)), Copy = false)`.
pub fn scalexyz(shape: &Shape, factors: [f64; 3]) -> Shape {
    let [x, y, z] = factors;
    let mut gtrsf = GeneralTransform3::default();
    gtrsf.set_vectorial_part(Matrix3::new([[x, 0., 0.], [0., y, 0.], [0., 0., z]]));
    let mut maker = BRepBuilderAPIGTransform::with_shape(shape, gtrsf, false).unwrap();
    assert!(maker.is_done(), "Scaling not done");
    maker.shape().unwrap().clone()
}

/// `BRepTest_BasicCommands` `fscale`/`bscale name x y z scale`: the scale applied as a
/// location, `S.Moved(L, theRaiseExc)` (`bscale` raises on a scaled location, `fscale`
/// does not); `None` is the command's `Operation is not done` warning.
pub fn located_scale(shape: &Shape, center: [f64; 3], scale: f64, forced: bool) -> Option<Shape> {
    let [x, y, z] = center;
    let mut trsf = Transform3::default();
    trsf.set_scale(Point3::new(x, y, z), scale).unwrap();
    match shape.moved(&Location::new(trsf), !forced) {
        Ok(moved) => Some(moved),
        Err(_) => {
            // Message::SendWarning, with the command name after its `b`/`f` prefix.
            log::result(&format!(
                "Operation is not done: scale is not a valid transformation - scale = {}",
                g17(scale)
            ));
            None
        }
    }
}

/// `BRepTest_OtherCommands` `BRepIntCS curve shape`: `BRepIntCurveSurface_Inter::Init(S,
/// GeomAdaptor_Curve(C), 1e-6)`; each point is printed and set as `brics_i`.
pub fn brep_int_cs(
    session: &mut dyn Session,
    curve: &Handle<dyn GeomCurve>,
    shape: &Shape,
) -> Vec<Point3> {
    let adaptor = GeomAdaptorCurve::with_curve(curve).unwrap();
    let mut compound = Compound::default();
    let builder = BRepBuilder::default();
    builder.make_compound(&mut compound);
    let mut inter = BRepIntCurveSurfaceInter::new();
    inter.init_curve(shape, &adaptor, 1e-6).unwrap();
    let mut points = Vec::new();
    let mut text = String::new();
    while inter.more() {
        let p = inter.pnt().unwrap();
        let mut vertex = Vertex::default();
        builder.make_vertex_at(&mut vertex, p, 0.).unwrap();
        builder.add(&mut compound, &vertex).unwrap();
        points.push(p);
        text.push_str(&format!(
            "Point {} : {} {} {}\n",
            points.len(),
            g17(p.x()),
            g17(p.y()),
            g17(p.z())
        ));
        session.set(&format!("brics_{}", points.len()), &Restored::Point(p));
        inter.next().unwrap();
    }
    if points.is_empty() {
        text.push_str("Points of intersections are not found\n");
    }
    log::result(&text);
    points
}

/// `bsave name file` of a shape: the type line
/// `DBRep_DrawableShape`, the shape set (the shape variable save), then `\n0\n\n`.
pub fn bsave(shape: &Shape, path: &Path) {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    brep_tools::write(shape, &mut stream, &ProgressRange::default()).unwrap();
    let body = stream_bytes(stream);
    let mut bytes = b"DBRep_DrawableShape\n".to_vec();
    bytes.extend_from_slice(&body);
    bytes.extend_from_slice(b"\n0\n\n");
    std::fs::write(path, bytes).unwrap_or_else(|error| panic!("bsave {}: {error}", path.display()));
}

fn stream_bytes(stream: IoStream) -> Vec<u8> {
    use std::io::{Read, Seek, SeekFrom};
    let mut device = stream.into_device().expect("in-memory stream");
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut bytes = Vec::new();
    device.read_to_end(&mut bytes).unwrap();
    bytes
}

/// `brestore file name`: the first word names the drawable
/// type; a `DBRep_DrawableShape` reads the shape set that follows.
pub fn brestore(path: &Path) -> Shape {
    let bytes =
        std::fs::read(path).unwrap_or_else(|error| panic!("brestore {}: {error}", path.display()));
    let text = std::str::from_utf8(&bytes).expect("a text drawable");
    let rest = text
        .trim_start()
        .strip_prefix("DBRep_DrawableShape")
        .expect("brestore: a DBRep_DrawableShape");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(rest.as_bytes().to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap_or_else(|failure| panic!("brestore: {failure:?}"));
    shape
}

/// `binsave name file`: `BinTools::Write(S, file, WithTriangles = true)`; the
/// shape name, or `Cannot write to the file <file>` (the command's error).
pub fn binsave(name: &str, shape: &Shape, path: &Path) -> String {
    let file = AsciiString::from(path.to_str().unwrap());
    let written =
        bintools::write_file(shape, &file, Default::default(), &ProgressRange::default()).unwrap();
    let text = if written {
        name.to_string()
    } else {
        format!("Cannot write to the file {}", path.display())
    };
    log::result(&text);
    text
}

/// `binrestore file name` (`readbrep`): a binary file is read with
/// `BinTools::Read`; the name, or `Error: cannot read from the file '<file>'`.
pub fn binrestore(name: &str, path: &Path) -> (String, Shape) {
    let file = AsciiString::from(path.to_str().unwrap());
    let mut shape = Shape::default();
    let read = bintools::read_file(&mut shape, &file, &ProgressRange::default()).unwrap();
    let text = if read {
        name.to_string()
    } else {
        format!("Error: cannot read from the file '{}'", path.display())
    };
    log::result(&text);
    (text, shape)
}

/// `OCC26313 res shape`: `BRepBuilderAPI_GTransform` by diag(1, 2, 3),
/// then `ModifiedShape(S)` (an exception printed as `Error: Exception is thrown`).
pub fn occ26313(shape: &Shape) -> Option<Shape> {
    let mut gtrsf = GeneralTransform3::from(Transform3::default());
    gtrsf.set_vectorial_part(Matrix3::new([[1., 0., 0.], [0., 2., 0.], [0., 0., 3.]]));
    let mut maker = BRepBuilderAPIGTransform::new(gtrsf);
    maker.perform(shape, false).unwrap();
    if !maker.is_done() {
        log::result("Error: Result is not done\n");
        return None;
    }
    match maker.modified_shape(shape) {
        Ok(modified) => Some(modified),
        Err(_) => {
            log::result("Error: Exception is thrown\n");
            None
        }
    }
}

fn update_box(bounds: [f64; 6]) -> BoundingBox3 {
    let [x1, y1, z1, x2, y2, z2] = bounds;
    let mut bnd = BoundingBox3::default();
    bnd.update_bounds([x1, y1, z1], [x2, y2, z2]);
    bnd
}

/// `BUC60852 edge x1 y1 z1 x2 y2 z2`: `Bnd_Box::IsOut(gp_Lin)` of the
/// edge's line against the box, logged.
pub fn buc60852(edge: &Shape, bounds: [f64; 6]) -> String {
    let edge = topods::edge(edge).expect("BUC60852: an edge");
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let line = curve.line().unwrap();
    let text = if update_box(bounds).is_out_line(line) {
        "Line that lies on edge does not intersect the box\n"
    } else {
        "Line that lies on edge intersects the box\n"
    };
    log::result(text);
    text.to_string()
}

/// `BUC60889 p1 p2 edge x1 y1 z1 x2 y2 z2`: `Bnd_Box::IsOut(P1, P2, D)` of
/// the band between the points along the edge's line direction, logged.
pub fn buc60889(p1: Point3, p2: Point3, edge: &Shape, bounds: [f64; 6]) -> String {
    let edge = topods::edge(edge).expect("BUC60889: an edge");
    let curve = BRepAdaptorCurve::with_edge(&edge).unwrap();
    let direction = curve.line().unwrap().direction();
    let text = if update_box(bounds).is_out_segment(p1, p2, direction) {
        "The band lies out of the box\n"
    } else {
        "The band intersects the box\n"
    };
    log::result(text);
    text.to_string()
}

/// `QANullifyShape name`: the variable is reset to the nullified shape.
pub fn qa_nullify_shape(mut shape: Shape) -> Shape {
    shape.nullify();
    shape
}
