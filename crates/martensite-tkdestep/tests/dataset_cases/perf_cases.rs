//! `perf/de/bug29830_N`: assemblies of 2000 and 4000 located faces with a colour on each, written and read back.
//! The times are measured and printed, nothing compares them; `XShow` and its screenshot belong to the real
//! renderer and are left out.
use crate::reports::g17;
use crate::script_values::label_of;
use crate::support::nbshapes::{Counting, NbCount, checknbshapes};
use crate::support::script_error::script_error;
use crate::table::Place;
use crate::xs_session::XsSession;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkernel::{
    precision, quantity::ColorRgba, standard::Handle, tcollection::AsciiString,
};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface, SurfaceBounds};
use martensite_tklcaf::tdf::{TdfLabel, TdfTool};
use martensite_tkmath::{
    gp::{Direction3, Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
};
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorType, XCAFDocDocumentTool};
use std::time::Instant;

const STEPS: [f64; 3] = [2.0, 4.0, 6.0];
const COLUMNS: usize = 10;

/// `plane a 0 0 0 1 0 0`, `mkface a a 0 3 0 2`.
fn base_face() -> Shape {
    let plane = GeomPlane::from_point(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).expect("a direction"),
    )
    .expect("a plane");
    let surface: Handle<dyn GeomSurface> = Handle::new(plane).cast();
    let bounds: SurfaceBounds = (0.0, 3.0, 0.0, 2.0);
    BRepBuilderAPIMakeFace::from_surface_bounds(&surface, bounds, precision::confusion())
        .expect("a face")
        .shape()
        .expect("a face")
        .clone()
}

/// The compound `co`: `copy` shares the face, `tcopy` copies it, `ttranslate` moves the copy, `add` adds it.
fn assembly_of(sharing: bool, nx: usize) -> Compound {
    let base = base_face();
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for i in 0..nx {
        for j in 0..COLUMNS {
            for k in 0..COLUMNS {
                let copy = if sharing {
                    base.clone()
                } else {
                    BRepBuilderAPICopy::with_shape(&base, true, false)
                        .expect("a copy")
                        .shape()
                        .expect("a copy")
                        .clone()
                };
                let move_by = Transform3::translation_vector(Vector3::new(
                    STEPS[0] * i as f64,
                    STEPS[1] * j as f64,
                    STEPS[2] * k as f64,
                ));
                let moved = copy
                    .moved(&Location::new(move_by), true)
                    .expect("a moved face");
                builder.add(&mut compound, &moved).expect("a compound");
            }
        }
    }
    compound
}

/// `XFindComponent D co_1`'s first label, with its entry's last tag stepped per component, each given a colour.
fn assign_colors(session: &XsSession, doc: &str, first_part: &Shape, parts: usize) {
    let main = session.documents[doc]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let shapes = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    let mut labels: Vec<TdfLabel> = Vec::new();
    shapes
        .get()
        .expect("a shape tool")
        .find_component(first_part, &mut labels)
        .expect("components");
    let mut entry = AsciiString::default();
    TdfTool::entry(
        labels
            .first()
            .unwrap_or_else(|| script_error("XFindComponent: no component")),
        &mut entry,
    );
    let entry = String::from_utf8_lossy(entry.as_bytes()).into_owned();
    let (prefix, last) = entry.rsplit_once(':').expect("an entry with a tag");
    let first_tag: usize = last.parse().expect("a tag");
    let colors = XCAFDocDocumentTool::color_tool(&main).expect("a color tool");
    let colors = colors.get().expect("a color tool");
    let (mut i, mut j, mut k) = (0_usize, 0_usize, 0_usize);
    for part in 0..parts {
        let color = ColorRgba::new(
            (i % 3) as f32 / 3.0,
            (j % 3) as f32 / 3.0,
            (k % 3) as f32 / 3.0,
            1.0,
        )
        .expect("a colour");
        let label = label_of(session, doc, &format!("{prefix}:{}", first_tag + part));
        colors
            .set_color(&label, &color, XCAFDocColorType::ColorGen)
            .expect("a colour on a component");
        k += 1;
        if k % 3 == 0 {
            j += 1;
            if j % 3 == 0 {
                i += 1;
            }
        }
    }
}

/// One pass of the loop: build, colour, write, read, count. Returns the seconds of the write and of the read.
fn pass(session: &mut XsSession, sharing: bool, nx: usize, file: &str) -> (f64, f64) {
    let faces = nx * COLUMNS * COLUMNS;
    println!("Creating assembly of {nx}*{COLUMNS}*{COLUMNS} boxes");
    let compound = assembly_of(sharing, nx).into_shape();
    session.new_document("D");
    let main = session.documents["D"]
        .get()
        .expect("a document")
        .main()
        .expect("a main label");
    let shapes = XCAFDocDocumentTool::shape_tool(&main).expect("a shape tool");
    shapes
        .get()
        .expect("a shape tool")
        .add_shape(&compound, true, true)
        .expect("a shape label");
    println!("Assigning colors to components");
    let mut children = martensite_tkbrep::topods::ShapeIterator::new(&compound, true, true);
    let first_part = children.value().expect("a child").clone();
    children.next();
    assign_colors(session, "D", &first_part, faces);
    println!("Writing STEP model");
    let started = Instant::now();
    if session.write_step("D", file).is_err() {
        script_error("WriteStep D");
    }
    let write_seconds = started.elapsed().as_secs_f64();
    session.close_document("D");
    println!("Reading STEP model");
    let started = Instant::now();
    if session.read_step("D1", file).is_err() {
        script_error("ReadStep D1");
    }
    let read_seconds = started.elapsed().as_secs_f64();
    println!("XGetOneShape result D1 ");
    if session.get_one_shape("result", "D1").is_err() {
        script_error("XGetOneShape result D1");
    }
    if sharing {
        checknbshapes(
            &session.raster,
            "result",
            Counting::Located,
            &[NbCount::Faces(1)],
        );
    } else {
        checknbshapes(
            &session.raster,
            "result",
            Counting::Located,
            &[NbCount::Faces(faces)],
        );
    }
    checknbshapes(
        &session.raster,
        "result",
        Counting::Typed,
        &[NbCount::Faces(faces)],
    );
    session.close_document("D1");
    let _ = std::fs::remove_file(file);
    (write_seconds, read_seconds)
}

fn run(session: &mut XsSession, sharing: bool) {
    let file = session.path_of(Place::Image("bug29830_tmp.stp"));
    let (write_1, read_1) = pass(session, sharing, 20, &file);
    let (write_2, read_2) = pass(session, sharing, 40, &file);
    println!("time_write_1={}", g17(write_1));
    println!("time_write_2={}", g17(write_2));
    println!("time_read_1={}", g17(read_1));
    println!("time_read_2={}", g17(read_2));
    println!("time_raise_write={}", g17(write_2 / write_1));
    println!("time_raise_read={}", g17(read_2 / read_1));
}

macro_rules! partly {
    ($name:ident, $source:literal, $body:expr) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            crate::runner::run_partly(&format!("{module}::{}", stringify!($name)), $source, $body);
        }
    };
}

partly!(perf_de_bug29830_1, "tests/perf/de/bug29830_1", |s| run(
    s, true
));
partly!(perf_de_bug29830_2, "tests/perf/de/bug29830_2", |s| run(
    s, false
));
