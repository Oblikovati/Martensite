//! The TKTopAlgo-level scenario topology commands the cases call, on the Rust API. Each
//! function names the command and the the retained behavior source it follows.
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools::{self, BRepToolsReShape, ReShape, WireExplorer},
    topexp::Explorer,
    topods::{self, Compound, Shape},
    toptools::MapOfShape,
};
use martensite_tkernel::{precision, standard::Handle, storage::IoStream};
use martensite_tkg3d::{
    geom::{GeomCurve, GeomSurface},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{
    Axis1, Axis2, Direction3, GeneralTransform3, Matrix3, Point3, Transform3, Vector3,
};
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPICopy, BRepBuilderAPIFaceError, BRepBuilderAPIGTransform,
        BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
        BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire, BRepBuilderAPINurbsConvert,
        BRepBuilderAPITransform,
    },
    brep_lib,
};
use std::io::{Read, Seek, SeekFrom};

/// `setflags name locked`: `Locked(true)` on the shape and on every explored
/// vertex, edge and face (the flags live on the shared TShapes).
pub fn setflags_locked(shape: &Shape) {
    shape.tshape().get().unwrap().set_locked(true);
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
        while explorer.more() {
            explorer.current().tshape().get().unwrap().set_locked(true);
            explorer.next().unwrap();
        }
    }
}

/// `sameparameter shape [tol]` (`fsameparameter` when
/// `forced`): `BRepLib::SameParameter(S, tol, force)` in place; `tol` 1e-7 when absent.
pub fn sameparameter_in_place(shape: &Shape, tolerance: f64, forced: bool) {
    brep_lib::same_parameter_shape(shape, tolerance, forced).unwrap();
}

/// The default `sameparameter` tolerance.
pub const SAMEPARAMETER_TOLERANCE: f64 = 1.0e-7;

/// `sameparameter result shape tol`: `BRepLib::SameParameter`
/// with a `BRepTools_ReShape`, the result being the re-shaped input.
pub fn sameparameter_reshaped(shape: &Shape, tolerance: f64, forced: bool) -> Shape {
    let reshaper = BRepToolsReShape::new();
    brep_lib::same_parameter_shape_with_reshape(shape, &reshaper, tolerance, forced).unwrap();
    reshaper.apply(shape, ShapeEnum::Shape).unwrap()
}

/// `updatetolerance result shape param`: with a fourth word
/// that names no shape, `BRepLib::UpdateTolerances(S, ReShape, true)` (face tolerances
/// verified), the result being the re-shaped input.
pub fn updatetolerance_reshaped(shape: &Shape) -> Shape {
    let reshaper = BRepToolsReShape::new();
    brep_lib::update_tolerances_with_reshape(shape, &reshaper, true).unwrap();
    reshaper.apply(shape, ShapeEnum::Shape).unwrap()
}

/// `vertex name x y z`: `BRepBuilderAPI_MakeVertex`.
pub fn vertex(point: Point3) -> Shape {
    BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}

/// `mkpoint p vertex`: the vertex's point.
pub fn mkpoint(vertex: &Shape) -> Point3 {
    BRepTool::pnt(&topods::vertex(vertex).expect("mkpoint: a vertex")).unwrap()
}

/// `mkedge name curve`: `BRepBuilderAPI_MakeEdge(C)`.
pub fn mkedge(curve: &Handle<dyn GeomCurve>) -> Shape {
    BRepBuilderAPIMakeEdge::from_curve(curve)
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}

/// `compound s1 s2 ... name`: a compound of the shapes, in order.
pub fn compound(shapes: &[&Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut shape = compound.into_shape();
    for part in shapes {
        builder.add(&mut shape, part).unwrap();
    }
    shape
}

/// `wire name e1/w1 e2/w2 ...`: `BRepBuilderAPI_MakeWire`
/// adding each edge or wire; `None` with the error when it is not done.
pub fn wire(parts: &[&Shape]) -> Result<Shape, String> {
    let mut maker = BRepBuilderAPIMakeWire::new();
    for part in parts {
        match part.shape_type().unwrap() {
            ShapeEnum::Edge => maker.add_edge(&topods::edge(part).unwrap()).unwrap(),
            ShapeEnum::Wire => maker.add_wire(&topods::wire(part).unwrap()).unwrap(),
            _ => {}
        }
    }
    match maker.shape() {
        Ok(shape) => Ok(shape.clone()),
        Err(_) => Err(format!("Wire not done with an error: {:?}", maker.error())),
    }
}

/// `mkplane name wire`: `BRepBuilderAPI_MakeFace(W,
/// OnlyPlane = false)`; `Err` with the command's `Error. mkplane ...` line otherwise.
pub fn mkplane(wire: &Shape) -> Result<Shape, String> {
    let wire = topods::wire(wire).map_err(|_| "mkplane: not a wire".to_string())?;
    let maker = BRepBuilderAPIMakeFace::from_wire(&wire, false).unwrap();
    match maker.error() {
        BRepBuilderAPIFaceError::FaceDone => Ok(maker.face().unwrap().into_shape()),
        error => Err(format!(
            "Error. mkplane has been finished with {error:?} status."
        )),
    }
}

/// `mkface name surface`: `BRepBuilderAPI_MakeFace(S,
/// Precision::Confusion())`.
pub fn mkface(surface: &Handle<dyn GeomSurface>) -> Shape {
    let mut maker = BRepBuilderAPIMakeFace::from_surface(surface, precision::confusion()).unwrap();
    maker.shape().unwrap().clone()
}

/// `mkface name surface umin umax vmin vmax`.
pub fn mkface_bounds(surface: &Handle<dyn GeomSurface>, bounds: [f64; 4]) -> Shape {
    let [u_min, u_max, v_min, v_max] = bounds;
    let bounds = (u_min, u_max, v_min, v_max);
    let mut maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(surface, bounds, precision::confusion())
            .unwrap();
    maker.shape().unwrap().clone()
}

/// `tcopy shape copy`: `BRepBuilderAPI_Copy(S, copyGeom =
/// true, copyMesh = false)`.
pub fn tcopy(shape: &Shape) -> Shape {
    let mut copy = BRepBuilderAPICopy::with_shape(shape, true, false).unwrap();
    copy.shape().unwrap().clone()
}

/// `transform` (`ttranslate`, `trotate`, `tmirror`, `tscale`)
/// of one shape without `-copy`: `S.Moved(L)` for an identity, else
/// `BRepBuilderAPI_Transform(T)` with `-copymesh` as given.
pub fn transform(shape: &Shape, trsf: Transform3, copy_mesh: bool) -> Shape {
    if trsf.form() == martensite_tkmath::gp::TransformForm::Identity {
        return shape
            .moved(&martensite_tkmath::toploc::Location::new(trsf), true)
            .unwrap();
    }
    let mut maker = BRepBuilderAPITransform::with_shape(shape, trsf, false, copy_mesh).unwrap();
    maker.shape().unwrap().clone()
}

/// `tmirror name x y z dx dy dz`: the mirror about the plane through the point.
pub fn tmirror(shape: &Shape, point: [f64; 3], normal: [f64; 3]) -> Shape {
    let mut trsf = Transform3::default();
    let axis = Axis2::new(
        Point3::new(point[0], point[1], point[2]),
        Direction3::new(normal[0], normal[1], normal[2]).unwrap(),
    )
    .unwrap();
    trsf.set_mirror_plane(axis);
    transform(shape, trsf, false)
}

/// `tscale name x y z scale [-copymesh]`.
pub fn tscale(shape: &Shape, center: [f64; 3], scale: f64, copy_mesh: bool) -> Shape {
    let mut trsf = Transform3::default();
    trsf.set_scale(Point3::new(center[0], center[1], center[2]), scale)
        .unwrap();
    transform(shape, trsf, copy_mesh)
}

/// `trotate name x y z dx dy dz angle_degrees [-copymesh]`.
pub fn trotate(
    shape: &Shape,
    point: [f64; 3],
    axis: [f64; 3],
    degrees: f64,
    copy_mesh: bool,
) -> Shape {
    let mut trsf = Transform3::default();
    let axis = Axis1::new(
        Point3::new(point[0], point[1], point[2]),
        Direction3::new(axis[0], axis[1], axis[2]).unwrap(),
    );
    trsf.set_rotation(axis, degrees * (std::f64::consts::PI / 180.0))
        .unwrap();
    transform(shape, trsf, copy_mesh)
}

/// `ttranslate name dx dy dz [-copymesh]`.
pub fn ttranslate(shape: &Shape, vector: [f64; 3], copy_mesh: bool) -> Shape {
    let mut trsf = Transform3::default();
    trsf.set_translation(Vector3::new(vector[0], vector[1], vector[2]));
    transform(shape, trsf, copy_mesh)
}

/// `deform new name cx cy cz`: `BRepBuilderAPI_GTransform`
/// with the diagonal vectorial part.
pub fn deform(shape: &Shape, coefficients: [f64; 3]) -> Shape {
    let mut gtrsf = GeneralTransform3::from(Transform3::default());
    let [x, y, z] = coefficients;
    gtrsf.set_vectorial_part(Matrix3::new([[x, 0., 0.], [0., y, 0.], [0., 0., z]]));
    let mut maker = BRepBuilderAPIGTransform::new(gtrsf);
    maker.perform(shape, false).unwrap();
    maker.shape().unwrap().clone()
}

/// `nurbsconvert result shape`: `BRepBuilderAPI_NurbsConvert`.
pub fn nurbsconvert(shape: &Shape) -> Shape {
    let mut maker = BRepBuilderAPINurbsConvert::new();
    maker.perform(shape, false).unwrap();
    maker.shape().unwrap().clone()
}

/// `wexplo wire [face]`: the `WEDGE_i` edges of
/// `BRepTools_WireExplorer`, in order.
pub fn wexplo(wire: &Shape, face: Option<&Shape>) -> Vec<Shape> {
    let wire = topods::wire(wire).expect("wexplo: a wire");
    let mut explorer = match face {
        Some(face) => WireExplorer::with_wire_on_face(&wire, &topods::face(face).unwrap()),
        None => WireExplorer::with_wire(&wire),
    }
    .unwrap();
    let mut edges = Vec::new();
    while explorer.more() {
        edges.push(explorer.current().clone().into_shape());
        explorer.next().unwrap();
    }
    edges
}

/// `maxtolerance shape`: the minimum and maximum tolerance
/// of the explored faces, edges and vertices (`None` for a type with none).
pub fn maxtolerance(shape: &Shape) -> [Option<(f64, f64)>; 3] {
    let range = |kind: ShapeEnum, tolerance: &dyn Fn(&Shape) -> f64| {
        let mut extremes: Option<(f64, f64)> = None;
        let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
        while explorer.more() {
            let t = tolerance(explorer.current());
            extremes = Some(extremes.map_or((t, t), |(lo, hi)| (lo.min(t), hi.max(t))));
            explorer.next().unwrap();
        }
        extremes
    };
    [
        range(ShapeEnum::Face, &|s| {
            BRepTool::face_tolerance(&topods::face(s).unwrap()).unwrap()
        }),
        range(ShapeEnum::Edge, &|s| {
            BRepTool::edge_tolerance(&topods::edge(s).unwrap()).unwrap()
        }),
        range(ShapeEnum::Vertex, &|s| {
            BRepTool::vertex_tolerance(&topods::vertex(s).unwrap()).unwrap()
        }),
    ]
}

/// `validrange edge`: `BRepLib::FindValidRange` printed as
/// `u1 u2` (`%.17g` each), or `edge has no valid range`.
pub fn validrange(edge: &Shape) -> Option<(f64, f64)> {
    let range = brep_lib::find_valid_range_of_edge(&topods::edge(edge).unwrap()).unwrap();
    range
        .is_valid
        .then(|| (range.first.unwrap(), range.last.unwrap()))
}

/// `checksection shape`: the number of vertices met an odd
/// number of times (` nb alone Vertices : N`).
pub fn checksection(shape: &Shape) -> usize {
    let mut vertices = MapOfShape::default();
    let mut explorer = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let current = explorer.current().clone();
        if !vertices.add(current.clone()) {
            vertices.remove(&current);
        }
        explorer.next().unwrap();
    }
    let alone = vertices.extent();
    println!(" nb alone Vertices : {alone}");
    alone
}

/// `whatis name` of a shape: `name is a shape TYPE ...`
/// (`DBRep_DrawableShape::Whatis`); the type word the cases read.
pub fn shape_type_word(shape: &Shape) -> &'static str {
    match shape.shape_type().unwrap() {
        ShapeEnum::Compound => "COMPOUND",
        ShapeEnum::CompSolid => "COMPSOLID",
        ShapeEnum::Solid => "SOLID",
        ShapeEnum::Shell => "SHELL",
        ShapeEnum::Face => "FACE",
        ShapeEnum::Wire => "WIRE",
        ShapeEnum::Edge => "EDGE",
        ShapeEnum::Vertex => "VERTEX",
        ShapeEnum::Shape => "SHAPE",
    }
}

/// `dump name` of a shape: the banner, then
/// `DBRep_DrawableShape::Dump` (`BRepTools::Dump`) at precision 15.
pub fn dump_shape(name: &str, shape: &Shape) -> String {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    brep_tools::dump(shape, &mut stream).unwrap();
    let mut device = stream.into_device().expect("in-memory dump stream");
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut body = String::new();
    device.read_to_string(&mut body).expect("dump text");
    format!("\n\n*********** Dump of {name} *************\n{body}")
}

/// C++ `std::scientific` with `precision(p)`: `1.00000e-07`.
pub fn scientific(value: f64, precision: usize) -> String {
    let text = format!("{value:.precision$e}");
    let (mantissa, exponent) = text.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{mantissa}e{sign}{:02}", exponent.abs())
}

/// `maxtolerance name`: the printed report (the counts of
/// distinct faces, edges and vertices, then `Min`/`Max` of each kind in `%.5e`), logged.
pub fn maxtolerance_text(name: &str, shape: &Shape) -> String {
    let count = |kind: ShapeEnum| {
        let mut map = MapOfShape::default();
        let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
        while explorer.more() {
            map.add(explorer.current().clone());
            explorer.next().unwrap();
        }
        map.extent()
    };
    let mut text = format!(
        "\n## Tolerances on the shape {name}  (nbFaces:{}  nbEdges:{} nbVtx:{})\n",
        count(ShapeEnum::Face),
        count(ShapeEnum::Edge),
        count(ShapeEnum::Vertex)
    );
    let labels = ["Face  ", "Edge  ", "Vertex"];
    for (label, range) in labels.iter().zip(maxtolerance(shape)) {
        if let Some((min, max)) = range {
            text.push_str(&format!(
                "\n    {label} : Min {:>8}    Max  {:>8} \n ",
                scientific(min, 5),
                scientific(max, 5)
            ));
        }
    }
    super::log::result(&text);
    text
}
