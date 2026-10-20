//! The primitive commands the cases call, on the Rust API: the primitives (`box`,
//! `wedge`, `pcylinder`, `psphere`, `ptorus`), the sweeps (`prism`, `revol`) with the
//! history they record (`savehistory`, `dump`), and `whatis` of a shape. Each function
//! names the command it follows.
use martensite_tkbrep::{
    brep_tools::{History, HistoryAlgorithm},
    topods::Shape,
};
use martensite_tkernel::{standard::Failure, storage::IoStream};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Point3, Vector3};
use martensite_tkprim::{
    brep_preview_api::BRepPreviewAPIMakeBox,
    brep_prim_api::{
        BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis, BRepPrimAPIMakePrism,
        BRepPrimAPIMakeRevol, BRepPrimAPIMakeSphere, BRepPrimAPIMakeTorus, BRepPrimAPIMakeWedge,
    },
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::f64::consts::PI;
use std::io::{Read, Seek, SeekFrom};

/// `Script::Atof(a) * (M_PI / 180.0)`.
fn radians(degrees: f64) -> f64 {
    degrees * (PI / 180.0)
}

/// `box name dx dy dz`: `BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), dx, dy, dz)`, as a solid.
pub fn box_sizes(dx: f64, dy: f64, dz: f64) -> Shape {
    box_at([0.0; 3], [dx, dy, dz])
}

/// `box name x y z dx dy dz`: `BRepPrimAPI_MakeBox(gp_Pnt(x, y, z), dx, dy, dz)`.
pub fn box_at(origin: [f64; 3], sizes: [f64; 3]) -> Shape {
    let [x, y, z] = origin;
    let [dx, dy, dz] = sizes;
    BRepPrimAPIMakeBox::at_point(Point3::new(x, y, z), dx, dy, dz)
        .and_then(|mut maker| maker.solid())
        .unwrap_or_else(|failure| panic!("box: {failure:?}"))
        .into_shape()
}

/// `box name -min x y z -max X Y Z`: `BRepPrimAPI_MakeBox(gp_Pnt(min), gp_Pnt(max))`.
pub fn box_min_max(min: [f64; 3], max: [f64; 3]) -> Shape {
    BRepPrimAPIMakeBox::from_points(point(min), point(max))
        .and_then(|mut maker| maker.solid())
        .unwrap_or_else(|failure| panic!("box: {failure:?}"))
        .into_shape()
}

/// `box name x y z dx dy dz -preview`: `BRepPreviewAPI_MakeBox::Init(gp_Pnt, dx, dy, dz)`
/// and its shape (a vertex, an edge, a rectangle or the box).
pub fn box_preview_at(origin: [f64; 3], sizes: [f64; 3]) -> Shape {
    let [dx, dy, dz] = sizes;
    let mut preview = BRepPreviewAPIMakeBox::new();
    preview.init_at_point(point(origin), dx, dy, dz).unwrap();
    preview.shape().unwrap().clone()
}

/// `box name -min x y z -max X Y Z -preview`: `BRepPreviewAPI_MakeBox::Init(min, max)`.
pub fn box_preview_min_max(min: [f64; 3], max: [f64; 3]) -> Shape {
    let mut preview = BRepPreviewAPIMakeBox::new();
    preview.init_points(point(min), point(max)).unwrap();
    preview.shape().unwrap().clone()
}

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

/// `wedge name Ox Oy Oz Zx Zy Zz Xx Xy Xz dx dy dz xmin zmin xmax zmax` (18 words):
/// `BRepPrimAPI_MakeWedge(gp_Ax2(P, N, Vx), dx, dy, dz, xmin, zmin, xmax, zmax)`.
pub fn wedge_axes(
    origin: [f64; 3],
    normal: [f64; 3],
    x_dir: [f64; 3],
    sizes: [f64; 3],
    limits: [f64; 4],
) -> Shape {
    let axes = Axis2::with_x_direction(point(origin), direction(normal), direction(x_dir))
        .expect("wedge: gp_Ax2");
    let [dx, dy, dz] = sizes;
    BRepPrimAPIMakeWedge::with_axes_limits(axes, dx, dy, dz, limits)
        .solid()
        .unwrap_or_else(|failure| panic!("wedge: {failure:?}"))
        .into_shape()
}

/// `wedge name dx dy dz xmin zmin xmax zmax` (9 words).
pub fn wedge_limits(sizes: [f64; 3], limits: [f64; 4]) -> Shape {
    let [dx, dy, dz] = sizes;
    BRepPrimAPIMakeWedge::with_limits(dx, dy, dz, limits)
        .and_then(|mut maker| maker.solid())
        .unwrap_or_else(|failure| panic!("wedge: {failure:?}"))
        .into_shape()
}

fn direction([x, y, z]: [f64; 3]) -> Direction3 {
    Direction3::new(x, y, z).expect("a gp_Dir")
}

/// `pcylinder name R H`: `BRepPrimAPI_MakeCylinder(R, H)`.
pub fn pcylinder(r: f64, h: f64) -> Shape {
    BRepPrimAPIMakeCylinder::new(r, h)
        .solid()
        .unwrap_or_else(|failure| panic!("pcylinder: {failure:?}"))
        .into_shape()
}

/// `psphere name R`: `BRepPrimAPI_MakeSphere(R)`.
pub fn psphere(r: f64) -> Shape {
    one_axis_solid(BRepPrimAPIMakeSphere::new(r), "psphere")
}

/// `psphere name R angle` (4 words, no plane): `BRepPrimAPI_MakeSphere(R, angle)`.
pub fn psphere_angle(r: f64, angle_degrees: f64) -> Shape {
    one_axis_solid(
        BRepPrimAPIMakeSphere::with_angle(r, radians(angle_degrees)),
        "psphere",
    )
}

/// `psphere name R angle1 angle2` (5 words, no plane): `BRepPrimAPI_MakeSphere(R, a1, a2)`.
pub fn psphere_latitudes(r: f64, angle1_degrees: f64, angle2_degrees: f64) -> Shape {
    one_axis_solid(
        BRepPrimAPIMakeSphere::with_latitudes(r, radians(angle1_degrees), radians(angle2_degrees)),
        "psphere",
    )
}

/// `ptorus name R1 R2`: `BRepPrimAPI_MakeTorus(R1, R2)`.
pub fn ptorus(r1: f64, r2: f64) -> Shape {
    one_axis_solid(BRepPrimAPIMakeTorus::new(r1, r2), "ptorus")
}

/// `ptorus name R1 R2 angle1 angle2 angle` (7 words, no plane):
/// `BRepPrimAPI_MakeTorus(R1, R2, angle1, angle2, angle)`.
pub fn ptorus_pipe_angle(r1: f64, r2: f64, angles_degrees: [f64; 3]) -> Shape {
    let [a1, a2, angle] = angles_degrees.map(radians);
    one_axis_solid(
        BRepPrimAPIMakeTorus::with_pipe_angle(r1, r2, a1, a2, angle),
        "ptorus",
    )
}

/// `TopoDS_Solid S = BRepPrimAPI_Make...(...)`: the maker's `Solid()`.
fn one_axis_solid<T: BRepPrimAPIMakeOneAxis>(maker: Result<T, Failure>, command: &str) -> Shape {
    maker
        .and_then(|mut maker| maker.solid())
        .unwrap_or_else(|failure| panic!("{command}: {failure:?}"))
        .into_shape()
}

/// The algorithm queries `BRepTools_History(theArguments, theAlgo)` reads.
struct Relations<'a>(&'a mut dyn BRepBuilderAPIMakeShape);

impl HistoryAlgorithm for Relations<'_> {
    fn is_deleted(&mut self, shape: &Shape) -> Result<bool, Failure> {
        self.0.is_deleted(shape)
    }
    fn modified(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.modified(shape)?.iter().cloned().collect())
    }
    fn generated(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.generated(shape)?.iter().cloned().collect())
    }
}

/// `BRepTest_Objects::SetHistory(anArgs, algo)`: the history of the operation on `base`.
fn history_of(base: &Shape, maker: &mut dyn BRepBuilderAPIMakeShape) -> History {
    History::from_algorithm(std::slice::from_ref(base), &mut Relations(maker))
        .unwrap_or_else(|failure| panic!("BRepTools_History: {failure:?}"))
}

/// A sweep result and the history the command records for `savehistory`.
pub struct Swept {
    pub shape: Shape,
    pub history: History,
}

/// The fourth word of `prism name base dx dy dz [flag]`, by its first letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrismFlag {
    /// No flag (or an unknown one): `BRepPrimAPI_MakePrism(S, V, Copy = false)`.
    None,
    /// `c`: `BRepPrimAPI_MakePrism(S, V, Copy = true)`.
    Copy,
    /// `i`: the infinite prism `BRepPrimAPI_MakePrism(S, gp_Dir(V), Inf = true)`.
    Infinite,
    /// `s` (`SemiInf`): the semi-infinite prism, `Inf = false`.
    SemiInfinite,
}

/// `prism name base dx dy dz [c|i|s]`: `Prism->Shape()` and its history; `Err` with the
/// exception the command lets through to script.
pub fn prism(base: &Shape, vector: [f64; 3], flag: PrismFlag) -> Result<Swept, Failure> {
    let [x, y, z] = vector;
    let v = Vector3::new(x, y, z);
    let mut maker = match flag {
        PrismFlag::Infinite | PrismFlag::SemiInfinite => {
            let direction = Direction3::from_vector(v)?;
            BRepPrimAPIMakePrism::infinite(
                base,
                direction,
                flag == PrismFlag::Infinite,
                false,
                true,
            )?
        }
        PrismFlag::Copy => BRepPrimAPIMakePrism::new(base, v, true, true)?,
        PrismFlag::None => BRepPrimAPIMakePrism::new(base, v, false, true)?,
    };
    let shape = maker.shape()?.clone();
    let history = history_of(base, &mut maker);
    Ok(Swept { shape, history })
}

/// `revol name base px py pz dx dy dz angle [copy]`: `BRepPrimAPI_MakeRevol(S, gp_Ax1(P,
/// D), angle, Copy = (an eleventh word))`; `None` after logging `Revol not done ` when the
/// maker is not done (the variable is then left unset).
pub fn revol(
    base: &Shape,
    origin: [f64; 3],
    axis: [f64; 3],
    angle_degrees: f64,
    copy: bool,
) -> Option<Swept> {
    let axis = Axis1::new(point(origin), direction(axis));
    let mut maker = BRepPrimAPIMakeRevol::new(base, axis, radians(angle_degrees), copy)
        .unwrap_or_else(|failure| panic!("revol: {failure:?}"));
    if !maker.is_done() {
        super::log::result("Revol not done \n");
        return None;
    }
    let shape = maker.shape().unwrap().clone();
    let history = history_of(base, &mut maker);
    Some(Swept { shape, history })
}

/// `savehistory h; dump h`: `dump` of the
/// BRepTest_DrawableHistory (`BRepTools_History::Dump`), logged.
pub fn dump_history(name: &str, history: &History) -> String {
    let mut stream = IoStream::from_bytes(Vec::new());
    history.dump(&mut stream);
    let mut device = stream.into_device().expect("in-memory history dump");
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut body = String::new();
    device.read_to_string(&mut body).unwrap();
    let text = format!("\n\n*********** Dump of {name} *************\n{body}");
    super::log::result(&text);
    text
}

/// `whatis name` of a shape variable: `name is a ` and
/// the shape `whatis` text (`shape TYPE ORIENTATION` and the set flags), logged.
pub fn whatis(name: &str, shape: &Shape) -> String {
    let mut text = format!(
        "{name} is a shape {} {}",
        shape_type_text(shape.shape_type().unwrap()),
        orientation_text(shape.orientation())
    );
    let flags = [
        (shape.free(), " Free"),
        (shape.modified(), " Modified"),
        (shape.orientable(), " Orientable"),
        (shape.closed(), " Closed"),
        (shape.infinite(), " Infinite"),
        (shape.convex(), " Convex"),
    ];
    for (flag, word) in flags {
        if flag.unwrap() {
            text.push_str(word);
        }
    }
    text.push('\n');
    super::log::result(&text);
    text
}

/// `TopAbs::ShapeTypeToString`.
pub fn shape_type_text(kind: ShapeEnum) -> &'static str {
    match kind {
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

/// `TopAbs::ShapeOrientationToString`.
fn orientation_text(orientation: Orientation) -> &'static str {
    match orientation {
        Orientation::Forward => "FORWARD",
        Orientation::Reversed => "REVERSED",
        Orientation::Internal => "INTERNAL",
        Orientation::External => "EXTERNAL",
    }
}
