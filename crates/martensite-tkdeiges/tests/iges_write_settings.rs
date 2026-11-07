//! Shapes and geometry written under the settings of the format and read back.
#[path = "support/setting_results.rs"]
mod setting_results;

use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkdeiges::iges_control::{IgesControlReader, IgesControlWriter};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkg3d::geom::{
    GeomCircle, GeomConicalSurface, GeomCylindricalSurface, GeomEllipse, GeomHyperbola, GeomLine,
    GeomParabola, GeomPlane, GeomSphericalSurface, GeomToroidalSurface, GeomTrimmedCurve,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, Point3, Vector3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakePrism,
    BRepPrimAPIMakeRevol, BRepPrimAPIMakeSphere, BRepPrimAPIMakeTorus,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxsbase::interface::InterfaceStatic;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use setting_results::SETTING_RESULTS;
use std::f64::consts::PI;
use std::sync::{Mutex, MutexGuard, PoisonError};

/// How far a size may be, relatively, from the one of the table.
const SIZE_TOLERANCE: f64 = 1.0e-5;

/// The settings are process-wide.
fn locked() -> MutexGuard<'static, ()> {
    static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
    ONE_AT_A_TIME.lock().unwrap_or_else(PoisonError::into_inner)
}

/// A setting for the time of a scope.
struct Setting {
    key: &'static str,
    old: i32,
}

impl Setting {
    fn new(key: &'static str, value: i32) -> Self {
        let _reader = IgesControlReader::new().expect("the settings exist with the norm");
        let old = InterfaceStatic::i_val(key);
        InterfaceStatic::set_i_val(key, value).expect("the setting");
        Self { key, old }
    }
}

impl Drop for Setting {
    fn drop(&mut self) {
        let _ = InterfaceStatic::set_i_val(self.key, self.old);
    }
}

type Row = (
    &'static str,
    i32,
    &'static str,
    [i32; 3],
    [f64; 3],
    [i32; 3],
    [f64; 3],
);

fn read_text(text: &[u8]) -> Shape {
    let mut reader = IgesControlReader::new().expect("the norm");
    let mut bytes = text;
    reader
        .read_stream("written.igs", &mut bytes)
        .expect("the reading");
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("the transfer");
    reader.xs_reader().one_shape()
}

fn written(shape: &Shape, mode: i32) -> Vec<u8> {
    let mut writer = IgesControlWriter::with_unit(b"MM", mode).expect("the writer");
    assert!(
        writer
            .add_shape(shape, &ProgressRange::default())
            .expect("the translation")
    );
    let mut text = Vec::new();
    assert!(writer.write_stream(&mut text, false).expect("the writing"));
    text
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).expect("an explorer");
    let mut found = 0;
    while explorer.more() {
        found += 1;
        explorer.next().expect("the next one");
    }
    found
}

fn area(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::surface_properties(shape, &mut properties, false, false).expect("the area");
    properties.mass()
}

fn length(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::linear_properties(shape, &mut properties, false, false).expect("the length");
    properties.mass()
}

fn primitives() -> Vec<(&'static str, Shape)> {
    let mut boxed = BRepPrimAPIMakeBox::from_sizes(1.0, 2.0, 3.0).expect("box");
    let mut cylinder = BRepPrimAPIMakeCylinder::new(2.0, 5.0);
    let mut cone = BRepPrimAPIMakeCone::new(3.0, 1.0, 4.0).expect("cone");
    let mut sphere = BRepPrimAPIMakeSphere::new(3.0).expect("sphere");
    let mut torus = BRepPrimAPIMakeTorus::new(5.0, 1.0).expect("torus");
    vec![
        ("box", boxed.shape().expect("box").clone()),
        ("cylinder", cylinder.shape().expect("cylinder").clone()),
        ("cone", cone.shape().expect("cone").clone()),
        ("sphere", sphere.shape().expect("sphere").clone()),
        ("torus", torus.shape().expect("torus").clone()),
    ]
}

fn swept() -> Vec<(&'static str, Shape)> {
    let line =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 0.0, 1.0))
            .expect("edge")
            .shape()
            .expect("edge")
            .clone();
    let mut prism =
        BRepPrimAPIMakePrism::new(&line, Vector3::new(0.0, 4.0, 0.0), false, true).expect("prism");
    let axis = Axis1::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).expect("z"),
    );
    let mut revolution =
        BRepPrimAPIMakeRevol::new(&line, axis, 2.0 * PI, false).expect("revolution");
    vec![
        ("prism", prism.shape().expect("prism").clone()),
        (
            "revolution",
            revolution.shape().expect("revolution").clone(),
        ),
    ]
}

/// The faces, area, edges and length of a shape; nothing for a null one.
fn sizes(shape: &Shape) -> (i32, f64, i32, f64) {
    if shape.is_null() {
        return (0, 0.0, 0, 0.0);
    }
    let (faces, edges) = (count(shape, ShapeEnum::Face), count(shape, ShapeEnum::Edge));
    (faces as i32, area(shape), edges as i32, length(shape))
}

fn check_sizes(found: (i32, f64, i32, f64), row: &Row, step: usize) -> Result<(), String> {
    let close = |a: f64, b: f64| (a - b).abs() <= SIZE_TOLERANCE * b.abs().max(1.0);
    let good = found.0 == row.3[step]
        && close(found.1, row.4[step])
        && found.2 == row.5[step]
        && close(found.3, row.6[step]);
    if good {
        return Ok(());
    }
    Err(format!(
        "{} {}={} step {step}: {found:?}",
        row.2, row.0, row.1
    ))
}

fn check_row(row: &Row, text: &[u8]) -> Result<(), String> {
    let shape = read_text(text);
    check_sizes(sizes(&shape), row, 0)?;
    for mode in [0, 1] {
        check_sizes(
            sizes(&read_text(&written(&shape, mode))),
            row,
            mode as usize + 1,
        )?;
    }
    Ok(())
}

#[test]
fn every_setting_gives_the_sizes_of_the_table() {
    let _guard = locked();
    let shapes: Vec<(&str, Vec<u8>)> = primitives()
        .iter()
        .chain(swept().iter())
        .map(|(name, shape)| (*name, written(shape, 0)))
        .collect();
    let mut problems = Vec::new();
    for row in SETTING_RESULTS {
        let _setting = Setting::new(row.0, row.1);
        let text = &shapes
            .iter()
            .find(|(name, _)| *name == row.2)
            .expect("the shape")
            .1;
        if let Err(problem) = check_row(row, text) {
            problems.push(problem);
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

fn geometry_of(handle: Handle<StandardTransient>) -> Shape {
    let mut writer = IgesControlWriter::with_unit(b"MM", 0).expect("the writer");
    assert!(writer.add_geom(&handle).expect("the translation"));
    let mut text = Vec::new();
    assert!(writer.write_stream(&mut text, false).expect("the writing"));
    read_text(&text)
}

#[test]
fn curves_come_back_at_their_length() {
    let _guard = locked();
    let frame = Axis2::new(
        Point3::new(1.0, 2.0, 3.0),
        Direction3::new(0.0, 0.0, 1.0).expect("z"),
    )
    .expect("frame");
    let trim = |curve: Handle<StandardTransient>,
                first: f64,
                last: f64|
     -> Handle<StandardTransient> {
        Handle::new(GeomTrimmedCurve::new(&curve.cast(), first, last, true, true).expect("trim"))
            .cast()
    };
    let line: Handle<StandardTransient> = Handle::new(GeomLine::new(Axis1::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).expect("x"),
    )))
    .cast();
    let circle: Handle<StandardTransient> =
        Handle::new(GeomCircle::new(frame, 2.0).expect("circle")).cast();
    let ellipse: Handle<StandardTransient> =
        Handle::new(GeomEllipse::new(frame, 3.0, 1.0).expect("ellipse")).cast();
    let hyperbola: Handle<StandardTransient> =
        Handle::new(GeomHyperbola::new(frame, 3.0, 1.0).expect("hyperbola")).cast();
    let parabola: Handle<StandardTransient> =
        Handle::new(GeomParabola::new(frame, 1.0).expect("parabola")).cast();
    assert!((length(&geometry_of(trim(line, 0.0, 5.0))) - 5.0).abs() < 1.0e-3);
    assert!((length(&geometry_of(circle.clone())) - 4.0 * PI).abs() < 1.0e-3);
    assert!((length(&geometry_of(trim(circle, 0.0, PI / 2.0))) - PI).abs() < 1.0e-3);
    assert!((length(&geometry_of(ellipse.clone())) - 13.3649).abs() < 1.0e-2);
    assert!(length(&geometry_of(trim(ellipse, 0.0, 1.0))) > 0.0);
    assert!(length(&geometry_of(trim(hyperbola, -1.0, 1.0))) > 0.0);
    assert!(length(&geometry_of(trim(parabola, -1.0, 1.0))) > 0.0);
}

#[test]
fn surfaces_are_written() {
    let _guard = locked();
    let frame = Axis3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).expect("z"),
        Direction3::new(1.0, 0.0, 0.0).expect("x"),
    )
    .expect("frame");
    let surfaces: Vec<Handle<StandardTransient>> = vec![
        Handle::new(GeomPlane::new(frame)).cast(),
        Handle::new(GeomCylindricalSurface::new(frame, 2.0).expect("cylinder")).cast(),
        Handle::new(GeomConicalSurface::new(frame, 0.3, 2.0).expect("cone")).cast(),
        Handle::new(GeomSphericalSurface::new(frame, 2.0).expect("sphere")).cast(),
        Handle::new(GeomToroidalSurface::new(frame, 4.0, 1.0).expect("torus")).cast(),
    ];
    for surface in surfaces {
        let mut writer = IgesControlWriter::with_unit(b"MM", 0).expect("the writer");
        assert!(writer.add_geom(&surface).expect("the translation"));
        let mut text = Vec::new();
        assert!(writer.write_stream(&mut text, false).expect("the writing"));
        let _ = read_text(&text);
    }
}
