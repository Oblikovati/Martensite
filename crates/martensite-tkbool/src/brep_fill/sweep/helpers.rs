//! Small helpers of BRepFillSweep: pole counts, curve representations, table columns
//! and section boxes.
use crate::brep_fill::shape_table::{ShapeTable, table_error};
use martensite_tkbrep::brep::access::edge_tshape;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topods::{self, Edge, Shape, ShapeIterator, Wire};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg2d::geom2d::arrays::one_based;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkgeomalgo::geom_fill::GeomFillSectionLaw;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::geom_abs::{CurveType, Shape as Continuity};
use martensite_tkmath::gp::Point3;
use martensite_tkmath::toploc::Location;

/// `NumberOfPoles(W)`: the poles of the B-spline and Bezier edges, 2 per segment, 4
/// per conic and 15 plus the C3 interval count for another curve (degenerated edges
/// add nothing).
pub(super) fn number_of_poles(w: &Wire) -> Result<i32, Failure> {
    let mut nb_points: i32 = 0;
    let mut iter = ShapeIterator::new(w, true, true);
    while iter.more() {
        let c = BRepAdaptorCurve::with_edge(&topods::edge(iter.value()?)?)?;
        iter.next();
        if martensite_tkernel::standard::real::is_equal(c.first_parameter()?, c.last_parameter()?) {
            // Degenerate.
            continue;
        }
        nb_points += match c.get_type()? {
            CurveType::BezierCurve => {
                poles_at_least_two(c.bezier()?.get().map_or(0, |b| b.nb_poles()))
            }
            CurveType::BSplineCurve => {
                poles_at_least_two(c.bspline()?.get().map_or(0, |b| b.nb_poles()))
            }
            CurveType::Line => 2,
            CurveType::Circle | CurveType::Ellipse | CurveType::Hyperbola | CurveType::Parabola => {
                4
            }
            _ => 15 + c.nb_intervals(Continuity::C3)?,
        };
    }
    Ok(nb_points)
}

/// The pole count when there are at least two poles, else 0.
fn poles_at_least_two(nb: i32) -> i32 {
    if nb >= 2 { nb } else { 0 }
}

/// `HasPCurves(E)`: the edge has a curve on surface representation.
pub(super) fn has_pcurves(e: &Edge) -> Result<bool, Failure> {
    let curves = edge_tshape(e, "BRepFillSweep HasPCurves")?.curves();
    Ok(curves
        .iter()
        .any(|cr| cr.get().is_some_and(|c| c.is_curve_on_surface())))
}

/// `Translate(ArrayIn, In, ArrayOut, Out)`: column `in_col` of `input` copied into
/// column `out_col` of `output` (over the rows of `output`).
pub(super) fn translate(
    input: &ShapeTable,
    in_col: i32,
    output: &mut ShapeTable,
    out_col: i32,
) -> Result<(), Failure> {
    for ii in 1..=output.upper_row() {
        let value: Shape = input.value(ii, in_col).map_err(table_error)?.clone();
        *output.change_value(ii, out_col).map_err(table_error)? = value;
    }
    Ok(())
}

/// `Box(Sec, U, Box)`: the box of the poles of the section law at `u`.
pub(super) fn section_box(
    sec: &Handle<dyn GeomFillSectionLaw>,
    u: f64,
) -> Result<BoundingBox3, Failure> {
    let mut bounds = BoundingBox3::default();
    let Some(law) = sec.get() else {
        return Err(Failure::with_type(
            "Standard_NullObject",
            "BRepFillSweep Box: null section law; expected a law",
            "BRepFillSweep",
        ));
    };
    let (nb_poles, _, _) = law.section_shape()?;
    let mut poles = one_based::<Point3>(nb_poles)?;
    let mut weights = one_based::<f64>(nb_poles)?;
    law.d0(u, &mut poles, &mut weights)?;
    for p in poles.iter() {
        bounds.add_point(*p);
    }
    Ok(bounds)
}

/// `Couture(E, S, L)`: the p-curve of the edge on the located surface (the second one
/// of a seam for a REVERSED edge), null when there is none.
pub(super) fn couture(
    e: &Edge,
    s: &Handle<dyn GeomSurface>,
    l: &Location,
) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let local: Location = l.predivided(e.location())?;
    let reversed: bool = e.orientation() == Orientation::Reversed;
    for cr in edge_tshape(e, "BRepFillSweep Couture")?.curves() {
        let Some(rep) = cr.get() else { continue };
        if rep.is_curve_on_surface_at(s, &local) {
            if rep.is_curve_on_closed_surface() && reversed {
                return rep.pcurve2();
            }
            return rep.pcurve();
        }
    }
    Ok(Handle::default())
}
