//! The hole helpers: the barycentre of the edges of a part, the extent of a shape along
//! the axis, and the cylinder between two points of the axis lengthened to clear the
//! inclined faces it crosses.
use crate::errors::null_object;
use crate::loc_ope::PntFace;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape, Shell};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::cslib::{self, NormalStatus};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{Axis1, Axis2, Point3, Xyz};
use martensite_tkprim::brep_prim::{BRepPrimCylinder, BRepPrimOneAxis};
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use std::f64::consts::PI;

/// The 3D curve of an edge moved by its location (`C->Transformed(L)`).
fn located_curve(e: &topods::Edge) -> Result<(Handle<dyn GeomCurve>, f64, f64), Failure> {
    let ec = BRepTool::curve_with_location(e)?;
    let c = ec
        .curve
        .get()
        .ok_or_else(|| null_object("Baryc", "edge without 3D curve"))?;
    Ok((
        c.transformed(ec.location.transformation())?.cast(),
        ec.first,
        ec.last,
    ))
}

/// The mean of 11 points evenly spread on each edge not degenerated of
/// `s` (NaN without such edge).
pub(super) fn baryc(s: &Shape) -> Result<Point3, Failure> {
    let mut bar = Xyz::new(0., 0., 0.);
    let mut nbp = 0;
    for e in Explorer::new(s, ShapeEnum::Edge, ShapeEnum::Shape)? {
        let e = topods::edge(&e)?;
        // Calculate points by non-degenerated edges
        if BRepTool::degenerated(&e)? {
            continue;
        }
        let (c, first, last) = located_curve(&e)?;
        let c = c
            .get()
            .ok_or_else(|| null_object("Baryc", "edge without 3D curve"))?;
        for i in 1..=11 {
            let prm = (((11 - i) as f64) * first + ((i - 1) as f64) * last) / 10.;
            bar += c.value(prm)?.xyz();
            nbp += 1;
        }
    }
    bar /= nbp as f64;
    Ok(Point3::from(bar))
}

/// The extreme parameters along the axis of
/// the corners of the bounding box of `s`.
pub(super) fn box_parameters(s: &Shape, axis: &Axis1) -> Result<(f64, f64), Failure> {
    let mut b = BoundingBox3::default();
    BRepBndLib::add(s, &mut b, true)?;
    let [xmin, ymin, zmin, xmax, ymax, zmax] = b.get()?;
    let (mut parmin, mut parmax) = (f64::MAX, f64::MIN);
    for x in [xmin, xmax] {
        for y in [ymin, ymax] {
            for z in [zmin, zmax] {
                let param = elclib::line_parameter(*axis, Point3::new(x, y, z));
                parmin = param.min(parmin);
                parmax = param.max(parmax);
            }
        }
    }
    Ok((parmin, parmax))
}

/// How far the cylinder must go past the
/// point to clear the face it crosses (`radius |tan a|`, `a` the angle of the axis with
/// the normal), None for an undefined normal or a face along the axis.
fn get_offset(pnt_info: &PntFace, radius: f64, axis: &Axis1) -> Result<Option<f64>, Failure> {
    let ffa = BRepAdaptorSurface::with_face(pnt_info.face(), true)?;
    let d1 = ffa.d1(pnt_info.u_parameter(), pnt_info.v_parameter())?;
    let res = cslib::normal_with_magnitude(d1.d1u, d1.d1v, precision::angular())?;
    let (NormalStatus::Defined, Some(norm_f)) = (res.status, res.normal) else {
        return Ok(None);
    };
    let angle = axis.direction().angle(&norm_f);
    if (PI / 2. - angle).abs() < precision::angular() {
        return Ok(None);
    }
    Ok(Some(radius * martensite_cmath::tan(angle).abs()))
}

/// The cylinder of the hole and its top and bottom faces.
pub(super) struct HoleCylinder {
    pub(super) shell: Shell,
    pub(super) top: Face,
    pub(super) bottom: Face,
}

/// The cylinder of `radius` on the axis direction from `orig`, of `height`.
pub(super) fn cylinder(
    orig: Xyz,
    axis: &Axis1,
    radius: f64,
    height: f64,
) -> Result<HoleCylinder, Failure> {
    let a2 = Axis2::new(Point3::from(orig), axis.direction())?;
    let mut the_cylinder = BRepPrimCylinder::new(a2, radius, height);
    Ok(HoleCylinder {
        shell: the_cylinder.shell()?,
        top: the_cylinder.top_face()?,
        bottom: the_cylinder.bottom_face()?,
    })
}

/// The cylinder along the
/// axis from `First - offF` to `Last + offL` (the offsets the radius when undefined).
pub(super) fn create_cyl(
    first: &PntFace,
    last: &PntFace,
    radius: f64,
    axis: &Axis1,
) -> Result<HoleCylinder, Failure> {
    let heigth = last.parameter() - first.parameter();
    let off_f = get_offset(first, radius, axis)?.unwrap_or(radius);
    let off_l = get_offset(last, radius, axis)?.unwrap_or(radius);
    // create cylinder along the axis (axis); from 'First - offF' to 'Last + offL' params
    let orig = first.pnt().xyz() - axis.direction().xyz() * off_f;
    cylinder(orig, axis, radius, heigth + off_f + off_l)
}
