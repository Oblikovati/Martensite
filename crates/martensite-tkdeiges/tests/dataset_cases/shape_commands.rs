//! The modelling commands of the scripts, each on the library's own API, with what it writes.
use crate::support::{script_error::script_error, session::Session};
use crate::xs_session::XsSession;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topexp::Explorer,
    topods::{self, Compound, Shape},
};
use martensite_tkernel::{message::ProgressRange, precision, standard::Handle, storage::IoStream};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCurve, GeomPlane, GeomSurface, SurfaceBounds},
    top_abs::ShapeEnum,
};
use martensite_tkgeombase::gc::MakeArcOfCircle;
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, Point3, Transform3, Vector3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire, BRepBuilderAPINurbsConvert,
    BRepBuilderAPISewing, BRepBuilderAPITransform,
};
use std::io::{Read as _, Seek as _, SeekFrom};

fn real(word: &str) -> f64 {
    word.parse()
        .unwrap_or_else(|_| panic!("{word}: expected a real"))
}

fn point_of(words: &[&str]) -> Point3 {
    Point3::new(real(words[0]), real(words[1]), real(words[2]))
}

fn direction_of(words: &[&str]) -> Direction3 {
    Direction3::new(real(words[0]), real(words[1]), real(words[2])).expect("a direction")
}

/// A command of the modelling scripts by name.
/// Example: `run_shape_command(&mut session, "box", &["b", "1", "1", "1"])`.
pub fn run_shape_command(session: &mut XsSession, name: &str, words: &[&str]) {
    println!("{name} {} ", words.join(" "));
    match name {
        "restore" => {
            let file = words[0].strip_prefix("@data/").expect("a data file");
            crate::support::restore::restore(&mut session.raster, file, words[1]);
        }
        "box" => make_box(session, words),
        "psphere" => {
            let mut maker = BRepPrimAPIMakeSphere::new(real(words[1])).expect("a sphere");
            let made = maker.shape().expect("a sphere").clone();
            session.raster.bind(words[0], &made);
        }
        "pcylinder" => {
            let mut maker = BRepPrimAPIMakeCylinder::new(real(words[1]), real(words[2]));
            let made = maker.shape().expect("a cylinder").clone();
            session.raster.bind(words[0], &made);
        }
        "pcone" => {
            let mut maker =
                BRepPrimAPIMakeCone::new(real(words[1]), real(words[2]), real(words[3]))
                    .expect("a cone");
            let made = maker.shape().expect("a cone").clone();
            session.raster.bind(words[0], &made);
        }
        "compound" => compound(session, words),
        "explode" => {
            println!("{} ", explode(session, words).join(" "));
        }
        "renamevar" => renamevar(session, words[0], words[1]),
        "point" => {
            session
                .points
                .insert(words[0].to_owned(), point_of(&words[1..]));
        }
        "gcarc" => arc_through_three_points(session, words),
        "ttranslate" => translate(session, words),
        "trotate" => rotate(session, words),
        "vertex" => {
            let mut maker = BRepBuilderAPIMakeVertex::new(point_of(&words[1..])).expect("a vertex");
            let made = maker.shape().expect("a vertex").clone();
            session.raster.bind(words[0], &made);
        }
        "edge" => edge(session, words),
        "wire" => wire(session, words),
        "circle" => circle(session, words),
        "plane" => plane(session, words),
        "mkedge" => mkedge(session, words),
        "mkface" => mkface(session, words),
        "mksurface" => mksurface(session, words),
        "nurbsconvert" => nurbsconvert(session, words),
        "setflags" => {}
        "treverse" => treverse(session, words[0]),
        "normals" => normals(session, words),
        "tclean" => {
            brep_tools::clean(&session.bound_shape(words[0]), false).expect("a clean");
        }
        "sewing" => sewing(session, words),
        "shape_divide" => shape_divide(session, words),
        "tcopy" => tcopy(session, words),
        "dump" => {
            dump_shape(session, words[0]);
        }
        "smallview" | "donly" | "display" | "don" | "fit" | "xwd" | "clear" | "isos"
        | "triangles" | "dall" => crate::steps::view_command(session, name, words),
        other => panic!("{}: command {other} is not hosted", session.source),
    }
}

fn make_box(session: &mut XsSession, words: &[&str]) {
    let mut maker = match words.len() {
        4 => BRepPrimAPIMakeBox::from_sizes(real(words[1]), real(words[2]), real(words[3]))
            .expect("a box"),
        7 => BRepPrimAPIMakeBox::at_point(
            point_of(&words[1..]),
            real(words[4]),
            real(words[5]),
            real(words[6]),
        )
        .expect("a box"),
        count => panic!("box: {count} words; expected a name and 3 or 6 reals"),
    };
    let made = maker.shape().expect("a box").clone();
    session.raster.bind(words[0], &made);
}

/// `compound a b ... result`.
fn compound(session: &mut XsSession, words: &[&str]) {
    let (result, parts) = words.split_last().expect("a result");
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut whole = compound.into_shape();
    for part in parts {
        builder
            .add(&mut whole, &session.bound_shape(part))
            .expect("a part");
    }
    session.raster.bind(result, &whole);
}

fn renamevar(session: &mut XsSession, from: &str, to: &str) {
    let shape = session.bound_shape(from);
    session.raster.unbind(from);
    session.raster.bind(to, &shape);
}

/// `explode name [kind]`: the sub-shapes of a kind, or the direct children, as `name_N`.
pub fn explode(session: &mut XsSession, words: &[&str]) -> Vec<String> {
    let shape = session.bound_shape(words[0]);
    let parts = match words.get(1) {
        Some(kind) => parts_of(&shape, kind),
        None => children_of(&shape),
    };
    let names: Vec<String> = (1..=parts.len())
        .map(|at| format!("{}_{at}", words[0]))
        .collect();
    for (name, part) in names.iter().zip(&parts) {
        session.raster.bind(name, part);
    }
    names
}

fn parts_of(shape: &Shape, kind: &str) -> Vec<Shape> {
    let kind = match kind.to_ascii_lowercase().as_str() {
        "f" => ShapeEnum::Face,
        "e" => ShapeEnum::Edge,
        "v" => ShapeEnum::Vertex,
        "w" => ShapeEnum::Wire,
        "sh" => ShapeEnum::Shell,
        "so" => ShapeEnum::Solid,
        other => panic!("explode: {other}; expected f e v w sh so"),
    };
    // `explode` lists a sub-shape once, in the order the explorer meets it.
    let mut parts: Vec<Shape> = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).expect("an explorer");
    while explorer.more() {
        let part = explorer.current().clone();
        if !parts.iter().any(|known| known.is_same(&part)) {
            parts.push(part);
        }
        explorer.next().expect("an explorer step");
    }
    parts
}

fn children_of(shape: &Shape) -> Vec<Shape> {
    let mut parts = Vec::new();
    let mut children = martensite_tkbrep::topods::ShapeIterator::new(shape, true, true);
    while children.more() {
        parts.push(children.value().expect("a child").clone());
        children.next();
    }
    parts
}

fn moved(shape: &Shape, by: Transform3) -> Shape {
    let mut maker = BRepBuilderAPITransform::with_shape(shape, by, false, false).expect("a move");
    maker.shape().expect("a moved shape").clone()
}

fn translate(session: &mut XsSession, words: &[&str]) {
    let mut by = Transform3::default();
    by.set_translation(Vector3::new(real(words[1]), real(words[2]), real(words[3])));
    let shape = moved(&session.bound_shape(words[0]), by);
    session.raster.bind(words[0], &shape);
}

/// `trotate name x y z dx dy dz degrees`.
fn rotate(session: &mut XsSession, words: &[&str]) {
    let axis = Axis1::new(point_of(&words[1..]), direction_of(&words[4..]));
    let mut by = Transform3::default();
    by.set_rotation(axis, real(words[7]).to_radians())
        .expect("a rotation");
    let shape = moved(&session.bound_shape(words[0]), by);
    session.raster.bind(words[0], &shape);
}

fn edge(session: &mut XsSession, words: &[&str]) {
    let first = topods::vertex(&session.bound_shape(words[1])).expect("a vertex");
    let last = topods::vertex(&session.bound_shape(words[2])).expect("a vertex");
    let mut maker = BRepBuilderAPIMakeEdge::from_vertices(&first, &last).expect("an edge");
    let made = maker.shape().expect("an edge").clone();
    session.raster.bind(words[0], &made);
}

fn wire(session: &mut XsSession, words: &[&str]) {
    let mut maker = BRepBuilderAPIMakeWire::default();
    for name in &words[1..] {
        maker
            .add_edge(&topods::edge(&session.bound_shape(name)).expect("an edge"))
            .expect("a wire");
    }
    let made = maker.shape().expect("a wire").clone();
    session.raster.bind(words[0], &made);
}

/// `circle name x y z dx dy dz radius`, kept for `mkedge`.
fn circle(session: &mut XsSession, words: &[&str]) {
    let frame = Axis2::new(point_of(&words[1..]), direction_of(&words[4..])).expect("a frame");
    let curve: Handle<dyn GeomCurve> =
        Handle::new(GeomCircle::new(frame, real(words[7])).expect("a circle")).cast();
    session.curves.insert(words[0].to_owned(), curve);
}

/// `gcarc name cir p1 p2 p3`: the arc through three points, kept for `mkedge`.
fn arc_through_three_points(session: &mut XsSession, words: &[&str]) {
    let at = |word: &str| session.points[word];
    let arc =
        MakeArcOfCircle::through_points(at(words[2]), at(words[3]), at(words[4])).expect("an arc");
    let curve: Handle<dyn GeomCurve> = arc.value().expect("an arc").cast();
    session.curves.insert(words[0].to_owned(), curve);
}

/// `plane name x y z nx ny nz vx vy vz`, kept for `mkface`.
fn plane(session: &mut XsSession, words: &[&str]) {
    let frame = Axis3::new(
        point_of(&words[1..]),
        direction_of(&words[4..]),
        direction_of(&words[7..]),
    )
    .expect("a frame");
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    session.surfaces.insert(words[0].to_owned(), surface);
}

fn mkedge(session: &mut XsSession, words: &[&str]) {
    let curve = session.curves[words[1]].clone();
    let mut maker = BRepBuilderAPIMakeEdge::from_curve(&curve).expect("an edge");
    let made = maker.shape().expect("an edge").clone();
    session.raster.bind(words[0], &made);
}

/// `mkface name surface u1 u2 v1 v2`.
fn mkface(session: &mut XsSession, words: &[&str]) {
    let surface = session.surfaces[words[1]].clone();
    let bounds: SurfaceBounds = (
        real(words[2]),
        real(words[3]),
        real(words[4]),
        real(words[5]),
    );
    let mut maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(&surface, bounds, precision::confusion())
            .expect("a face");
    let made = maker.shape().expect("a face").clone();
    session.raster.bind(words[0], &made);
}

/// `mksurface name face`: the surface of the face, kept for `mkface`.
fn mksurface(session: &mut XsSession, words: &[&str]) {
    let face = first_face(&session.bound_shape(words[1]));
    let surface = martensite_tkbrep::brep::BRepTool::surface(&face).expect("a surface");
    session.surfaces.insert(words[0].to_owned(), surface);
}

fn first_face(shape: &Shape) -> topods::Face {
    let explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).expect("an explorer");
    topods::face(explorer.current()).expect("a face")
}

fn nurbsconvert(session: &mut XsSession, words: &[&str]) {
    let mut maker = BRepBuilderAPINurbsConvert::with_shape(&session.bound_shape(words[1]), false)
        .expect("a conversion");
    let made = maker.shape().expect("a conversion").clone();
    session.raster.bind(words[0], &made);
}

fn treverse(session: &mut XsSession, name: &str) {
    let reversed = session.bound_shape(name).reversed();
    session.raster.bind(name, &reversed);
}

/// `normals name length`: the legend it writes. The arrows belong to the view and the case checks
/// nothing of them.
fn normals(session: &XsSession, words: &[&str]) {
    session.bound_shape(words[0]);
    println!("rouge  FORWARD\nbleu   REVERSED\nrose   EXTERNAL\norange INTERNAL");
}

/// `sewing result [tolerance] shapes...`; the dump goes to the stream, as the command writes it.
fn sewing(session: &mut XsSession, words: &[&str]) {
    let (tolerance, shapes) = match words[1].parse::<f64>() {
        Ok(tolerance) => (tolerance, &words[2..]),
        Err(_) => (1.0e-6, &words[1..]),
    };
    let working = tolerance.max(precision::confusion());
    let mut sewer = BRepBuilderAPISewing::default();
    sewer.init(working, true, true, true, false);
    sewer.set_same_parameter_mode(true);
    sewer.set_floating_edges_mode(false);
    sewer.set_face_mode(true);
    sewer.set_min_tolerance((tolerance * 1e-4).max(precision::confusion()));
    sewer.set_max_tolerance(precision::infinite());
    for name in shapes {
        sewer
            .add(&session.bound_shape(name))
            .expect("a shape to sew");
    }
    sewer
        .perform(&ProgressRange::default())
        .unwrap_or_else(|failure| script_error(&format!("{failure:?}")));
    let mut dump = Vec::new();
    sewer.dump(&mut dump).expect("a memory stream");
    print!("{}", String::from_utf8_lossy(&dump));
    if !sewer.sewed_shape().is_null() {
        session.raster.bind(words[0], sewer.sewed_shape());
    }
}

/// `shape_divide result shape`: the division, then the command's same-parameter fix of what it made.
fn shape_divide(session: &mut XsSession, words: &[&str]) {
    use martensite_tkshhealing::shape_extend::ShapeExtendStatus as Status;
    use martensite_tkshhealing::shape_fix::ShapeFix;
    use martensite_tkshhealing::shape_upgrade::ShapeUpgradeShapeDivideContinuity;
    let tool = ShapeUpgradeShapeDivideContinuity::new(session.bound_shape(words[1]));
    tool.perform(true).expect("a division");
    let divided = tool.result();
    for (flag, text) in [
        (Status::Done1, "DONE1"),
        (Status::Done2, "DONE2"),
        (Status::Done3, "DONE3"),
        (Status::Done4, "DONE4"),
        (Status::Done5, "DONE5"),
        (Status::Done6, "DONE6"),
        (Status::Done7, "DONE7"),
        (Status::Done8, "DONE8"),
        (Status::Fail1, "FAIL1"),
        (Status::Fail2, "FAIL2"),
        (Status::Fail3, "FAIL3"),
        (Status::Fail4, "FAIL4"),
        (Status::Fail5, "FAIL5"),
        (Status::Fail6, "FAIL6"),
        (Status::Fail7, "FAIL7"),
        (Status::Fail8, "FAIL8"),
    ] {
        if tool.status_flag(flag) {
            println!("Status: {text}");
        }
    }
    ShapeFix::same_parameter(
        &divided,
        false,
        0.0,
        &ProgressRange::default(),
        &Handle::default(),
    )
    .expect("a same parameter");
    session.raster.bind(words[0], &divided);
}

fn tcopy(session: &mut XsSession, words: &[&str]) {
    let mut maker = BRepBuilderAPICopy::with_shape(&session.bound_shape(words[0]), true, false)
        .expect("a copy");
    let made = maker.shape().expect("a copy").clone();
    session.raster.bind(words[1], &made);
}

/// `dump name`: the shape's text, which a script may read back.
pub fn dump_shape(session: &XsSession, name: &str) -> String {
    println!("\n\n*********** Dump of {name} *************\n");
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    brep_tools::dump(&session.bound_shape(name), &mut stream).expect("a dump");
    let mut device = stream.into_device().expect("a memory device");
    device.seek(SeekFrom::Start(0)).expect("a memory device");
    let mut text = String::new();
    device.read_to_string(&mut text).expect("a memory device");
    println!("{text}");
    text
}
