//! `BRepTest_BasicCommands` `bounding` (`BoundBox`, with `ConvertBndToShape`) and
//! `isbbinterf` (`IsBoxesInterfered`), on BRepBndLib, Bnd_Box and Bnd_OBB.
use super::log;
use super::session::{Session, ViewStep};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    standard::dump::{DumpStream, format_json},
    storage::format_general,
};
use martensite_tkmath::{
    bnd::{BoundingBox3, OrientedBoundingBox},
    gp::{Axis2, Direction3, Point3, Xyz},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;

/// The input and options of one `bounding` call.
#[derive(Clone, Copy, Default)]
pub struct Bounding<'a> {
    /// The shape argument.
    pub shape: Option<&'a Shape>,
    /// `xmin ymin zmin xmax ymax zmax` instead of a shape.
    pub min_max: Option<[f64; 6]>,
    /// `-obb`.
    pub obb: bool,
    /// `-optimal`.
    pub optimal: bool,
    /// `-dump` / `-print`.
    pub print: bool,
    /// `-dumpJson`.
    pub dump_json: bool,
    /// `-save x1 y1 z1 x2 y2 z2`.
    pub save: bool,
    /// `-shape name`.
    pub result_shape: Option<&'a str>,
    /// `-nodraw`.
    pub no_draw: bool,
    /// `-finite`.
    pub finite: bool,
}

impl<'a> Bounding<'a> {
    /// `bounding shape` (no option).
    pub fn of(shape: &'a Shape) -> Self {
        Self {
            shape: Some(shape),
            ..Self::default()
        }
    }
}

/// What `bounding` computed, printed and saved.
#[derive(Clone, Default)]
pub struct BoundingOutput {
    /// The script result (logged).
    pub text: String,
    /// The `-save` values `xmin ymin zmin xmax ymax zmax` (as the command prints numbers).
    pub saved: Option<[f64; 6]>,
    /// The `-shape` solid (a null shape for a void box).
    pub shape: Option<Shape>,
}

/// `%.17g`, the format of a printed real.
fn g17(value: f64) -> String {
    format_general(value, 17)
}

fn xyz_line(label: &str, xyz: Xyz) -> String {
    format!(
        "{label}: {} {} {}\n",
        g17(xyz.x()),
        g17(xyz.y()),
        g17(xyz.z())
    )
}

/// `ConvertBndToShape`: the box solid of the oriented box (a null shape when void).
pub fn convert_bnd_to_shape(obb: &OrientedBoundingBox) -> Shape {
    if obb.is_void() {
        return Shape::default();
    }
    let center = obb.center();
    let (x, y, z) = (obb.x_direction(), obb.y_direction(), obb.z_direction());
    let (hx, hy, hz) = (obb.x_h_size(), obb.y_h_size(), obb.z_h_size());
    let dir = |v: Xyz| Direction3::new(v.x(), v.y(), v.z()).expect("OBB axis");
    let corner = center - x * hx - y * hy - z * hz;
    let axes = Axis2::with_x_direction(
        Point3::new(corner.x(), corner.y(), corner.z()),
        dir(z),
        dir(x),
    )
    .expect("ConvertBndToShape: gp_Ax2");
    BRepPrimAPIMakeBox::with_axes(axes, 2.0 * hx, 2.0 * hy, 2.0 * hz)
        .solid()
        .unwrap_or_else(|failure| panic!("ConvertBndToShape: {failure:?}"))
        .into_shape()
}

/// `bounding ...`: the box, its report, the saved values, the `-shape` variable (set in
/// `session`) and the orange box it displays.
pub fn bounding(session: &mut dyn Session, args: Bounding<'_>) -> BoundingOutput {
    let mut print = args.print;
    let old_syntax = !print && !args.dump_json && !args.save && args.result_shape.is_none();
    if old_syntax {
        print = true;
    }
    let mut out = BoundingOutput::default();
    let mut drawn: Option<OrientedBoundingBox> = None;
    if args.obb {
        let shape = args.shape.expect("bounding -obb of a shape");
        let mut obb = OrientedBoundingBox::default();
        BRepBndLib::add_obb(shape, &mut obb, true, args.optimal, false)
            .unwrap_or_else(|failure| panic!("bounding -obb: {failure:?}"));
        if obb.is_void() {
            out.text.push_str("Void box.\n");
        } else if print {
            out.text.push_str("Oriented bounding box\n");
            out.text.push_str(&xyz_line("Center", obb.center()));
            out.text.push_str(&xyz_line("X-axis", obb.x_direction()));
            out.text.push_str(&xyz_line("Y-axis", obb.y_direction()));
            out.text.push_str(&xyz_line("Z-axis", obb.z_direction()));
            out.text.push_str(&format!(
                "Half X: {}\nHalf Y: {}\nHalf Z: {}\n",
                g17(obb.x_h_size()),
                g17(obb.y_h_size()),
                g17(obb.z_h_size())
            ));
        }
        if args.dump_json {
            let mut stream = DumpStream::default();
            obb.dump_json(&mut stream, -1).unwrap();
            out.text.push_str("Oriented bounding box\n");
            out.text
                .push_str(&String::from_utf8_lossy(format_json(&stream, 3).as_bytes()));
        }
        if !args.no_draw && args.result_shape.is_none() && !obb.is_void() {
            drawn = Some(obb);
        }
        if let Some(name) = args.result_shape {
            let solid = convert_bnd_to_shape(&obb);
            session.set(name, &solid.clone().into());
            out.shape = Some(solid);
        }
    } else {
        let mut aabb = BoundingBox3::default();
        if let Some([x1, y1, z1, x2, y2, z2]) = args.min_max {
            aabb.add_point(Point3::new(x1, y1, z1));
            aabb.add_point(Point3::new(x2, y2, z2));
        }
        if let Some(shape) = args.shape {
            aabb.set_void();
            let done = if args.optimal {
                BRepBndLib::add_optimal(shape, &mut aabb, true, false)
            } else {
                BRepBndLib::add(shape, &mut aabb, true)
            };
            done.unwrap_or_else(|failure| panic!("bounding: {failure:?}"));
        }
        if aabb.is_void() {
            out.text.push_str("Void box.\n");
        } else {
            if args.finite && aabb.is_open() {
                aabb = aabb.finite_part();
            }
            let min = aabb.corner_min().unwrap();
            let max = aabb.corner_max().unwrap();
            if print {
                if old_syntax {
                    out.text.push_str(&format!(
                        "{} {} {} {} {} {}\n",
                        g17(min.x()),
                        g17(min.y()),
                        g17(min.z()),
                        g17(max.x()),
                        g17(max.y()),
                        g17(max.z())
                    ));
                } else {
                    out.text.push_str("Axes-aligned bounding box\n");
                    out.text.push_str(&ranges(min, max));
                    if aabb.is_open() && aabb.has_finite_part() {
                        let finite = aabb.finite_part();
                        out.text.push_str("Finite part\n");
                        out.text.push_str(&ranges(
                            finite.corner_min().unwrap(),
                            finite.corner_max().unwrap(),
                        ));
                    }
                }
            }
            if args.dump_json {
                let mut stream = DumpStream::default();
                aabb.dump_json(&mut stream).unwrap();
                out.text.push_str("Bounding box\n");
                out.text
                    .push_str(&String::from_utf8_lossy(format_json(&stream, 3).as_bytes()));
            }
            if args.save {
                out.saved = Some([min.x(), min.y(), min.z(), max.x(), max.y(), max.z()]);
            }
            if !args.no_draw && args.result_shape.is_none() {
                drawn = Some(OrientedBoundingBox::from_box(&aabb));
            }
        }
        if let Some(name) = args.result_shape {
            let solid = convert_bnd_to_shape(&OrientedBoundingBox::from_box(&aabb));
            session.set(name, &solid.clone().into());
            out.shape = Some(solid);
        }
    }
    log::result(&out.text);
    if let Some(obb) = drawn {
        session.step(ViewStep::BoundBox(&obb));
    }
    out
}

fn ranges(min: Point3, max: Point3) -> String {
    format!(
        "X-range: {} {}\nY-range: {} {}\nZ-range: {} {}\n",
        g17(min.x()),
        g17(max.x()),
        g17(min.y()),
        g17(max.y()),
        g17(min.z()),
        g17(max.z())
    )
}

/// `isbbinterf shape1 shape2 [-o]`: whether the boxes of the two shapes are disjoint
/// (`IsOut`), with the default `BRepBndLib::Add` / `AddOBB` arguments; logged.
pub fn isbbinterf(first: &Shape, second: &Shape, oriented: bool) -> String {
    let text = if oriented {
        let mut b1 = OrientedBoundingBox::default();
        let mut b2 = OrientedBoundingBox::default();
        BRepBndLib::add_obb(first, &mut b1, true, false, true).unwrap();
        BRepBndLib::add_obb(second, &mut b2, true, false, true).unwrap();
        if b1.is_out_box(&b2) {
            "The shapes are NOT interfered by OBB.\n"
        } else {
            "The shapes are interfered by OBB.\n"
        }
    } else {
        let mut b1 = BoundingBox3::default();
        let mut b2 = BoundingBox3::default();
        BRepBndLib::add(first, &mut b1, true).unwrap();
        BRepBndLib::add(second, &mut b2, true).unwrap();
        if b1.is_out_box(&b2) {
            "The shapes are NOT interfered by AABB.\n"
        } else {
            "The shapes are interfered by AABB.\n"
        }
    };
    log::result(text);
    text.to_string()
}
