//! `StepToTopoDSGeometricTool`: pcurves of surface curves, seams, and 3D parameters of edges.
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomLine, StepGeomPcurve, StepGeomSeamCurve, StepGeomSurface, StepGeomSurfaceCurve,
};
use crate::step_shape::{StepShapeEdge, StepShapeEdgeLoop};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomBoundedCurve, GeomCurve};
use martensite_tkmath::elclib;

/// `StepToTopoDSGeometricTool`: static members.
pub struct StepToTopoDSGeometricTool;

/// The first pcurve of a surface curve at a rank, null when it is a surface.
fn associated_pcurve(
    curve: &StepGeomSurfaceCurve,
    rank: i32,
) -> Result<Handle<StepGeomPcurve>, Failure> {
    Ok(curve.associated_geometry_value(rank)?.pcurve())
}

/// The two pcurves of a surface curve when both lie on `surface`.
fn both_on_surface(
    curve: &StepGeomSurfaceCurve,
    surface: &Handle<StepGeomSurface>,
) -> Result<Option<(Handle<StepGeomPcurve>, Handle<StepGeomPcurve>)>, Failure> {
    let first = associated_pcurve(curve, 1)?;
    let second = associated_pcurve(curve, 2)?;
    let on_surface = |pcurve: &StepGeomPcurve| pcurve.basis_surface() == *surface;
    match (first.get(), second.get()) {
        (Some(a), Some(b)) if on_surface(a) && on_surface(b) => Ok(Some((first, second))),
        _ => Ok(None),
    }
}

/// How many oriented edges of a loop reference `edge`.
fn occurrences_in_loop(
    edge: &Handle<StepShapeEdge>,
    edge_loop: &Handle<StepShapeEdgeLoop>,
) -> Result<i32, Failure> {
    let live = step_live(edge_loop, "StepToTopoDSGeometricTool")?;
    let mut count = 0;
    for rank in 1..=live.nb_edge_list() {
        let oriented = live.edge_list_value(rank)?;
        if *edge == step_live(&oriented, "StepToTopoDSGeometricTool")?.edge_element() {
            count += 1;
        }
    }
    Ok(count)
}

/// The line that a pcurve references, null when it is another curve.
fn pcurve_line(pcurve: &Handle<StepGeomPcurve>) -> Result<Handle<StepGeomLine>, Failure> {
    let live = step_live(pcurve, "StepToTopoDSGeometricTool::IsLikeSeam")?;
    let representation = live.reference_to_curve();
    Ok(
        step_live(&representation, "StepToTopoDSGeometricTool::IsLikeSeam")?
            .items_value(1)?
            .cast(),
    )
}

/// Same origin in X or Y and same vector, within the parametric tolerance.
fn lines_look_like_seam(line1: &StepGeomLine, line2: &StepGeomLine) -> Result<bool, Failure> {
    let (point1, point2) = (line1.pnt(), line2.pnt());
    let (origin1, origin2) = (
        step_live(&point1, "IsLikeSeam")?,
        step_live(&point2, "IsLikeSeam")?,
    );
    let (vector1, vector2) = (line1.dir(), line2.dir());
    let direction1 = step_live(&vector1, "IsLikeSeam")?.orientation();
    let direction2 = step_live(&vector2, "IsLikeSeam")?.orientation();
    let (ratios1, ratios2) = (
        step_live(&direction1, "IsLikeSeam")?,
        step_live(&direction2, "IsLikeSeam")?,
    );
    let delta_x = (origin1.coordinates_value(1)? - origin2.coordinates_value(1)?).abs();
    let delta_y = (origin1.coordinates_value(2)? - origin2.coordinates_value(2)?).abs();
    let delta_dir_x =
        (ratios1.direction_ratios_value(1)? - ratios2.direction_ratios_value(1)?).abs();
    let delta_dir_y =
        (ratios1.direction_ratios_value(2)? - ratios2.direction_ratios_value(2)?).abs();
    let preci2d = precision::p_confusion(); //:S4136: Parametric(BRepAPI::precision(),10);
    if delta_x < preci2d || delta_y < preci2d {
        return Ok(delta_dir_x < preci2d && delta_dir_y < preci2d);
    }
    Ok(false)
}

/// A parameter brought back between the natural bounds.
fn clamp_parameter(value: &mut f64, first: f64, last: f64) {
    if *value < first {
        *value = first;
    } else if *value > last {
        *value = last;
    }
}

/// A closed curve, non periodic: one projected point is on the origin of the parameters.
fn update_closed(
    curve: &dyn GeomCurve,
    w1: &mut f64,
    w2: &mut f64,
    preci: f64,
) -> Result<(), Failure> {
    let (cf, cl) = (curve.first_parameter(), curve.last_parameter());
    if (*w2 - cf).abs() < precision::p_confusion() {
        *w2 = cl;
        return Ok(());
    }
    if (*w1 - cl).abs() < precision::p_confusion() {
        *w1 = cf;
        return Ok(());
    }
    // The origin is crossed, illegal on a closed non periodic curve: the parameters are inverted anyway.
    //:S4136 abv 20 Apr 99: r0701_ug.stp #6230: add check in 3d
    if curve.value(*w1)?.distance(&curve.value(cf)?) < preci {
        *w1 = cf;
    }
    if curve.value(*w2)?.distance(&curve.value(cl)?) < preci {
        *w2 = cl;
    }
    if (*w2 - *w1).abs() < precision::p_confusion() {
        *w1 = cf;
        *w2 = cl;
    } else if *w1 > *w2 {
        std::mem::swap(w1, w2);
    }
    Ok(())
}

/// abv 15.03.00 #72 bm1_pe_t4 protection of exceptions in draw: the curve itself is reversed.
fn reverse_with_parameters(
    curve: &dyn GeomCurve,
    w1: &mut f64,
    w2: &mut f64,
) -> Result<(), Failure> {
    if *w1 > *w2 {
        *w1 = curve.reversed_parameter(*w1)?;
        *w2 = curve.reversed_parameter(*w2)?;
        curve.reverse()?;
    }
    Ok(())
}

/// A spline: closed within the 3D tolerance it behaves as closed; returns the result.
fn update_bspline(
    spline: &GeomBSplineCurve,
    w1: &mut f64,
    w2: &mut f64,
    preci: f64,
) -> Result<bool, Failure> {
    let (cf, cl) = (spline.first_parameter(), spline.last_parameter());
    if spline.start_point()?.distance(&spline.end_point()?) <= preci {
        if (*w2 - cf).abs() < precision::p_confusion() {
            *w2 = cl;
        } else if (*w1 - cl).abs() < precision::p_confusion() {
            *w1 = cf;
        } else {
            std::mem::swap(w1, w2);
        }
    } else {
        reverse_with_parameters(spline, w1, w2)?;
    }
    //:j9 abv 11 Dec 98: PRO7747 #4875; gka 10.07.1998 file PRO7656 entity 33334
    if *w1 == *w2 {
        *w1 = cf;
        *w2 = cl;
        return Ok(false);
    }
    Ok(true)
}

impl StepToTopoDSGeometricTool {
    /// `PCurve(SurfCurve, BasisSurf, thePCurve, last)`: the rank after `last` of a pcurve lying on
    /// the surface and that pcurve, 0 and a null handle without one.
    pub fn pcurve(
        surf_curve: &Handle<StepGeomSurfaceCurve>,
        basis_surf: &Handle<StepGeomSurface>,
        last: i32,
    ) -> Result<(i32, Handle<StepGeomPcurve>), Failure> {
        let live = step_live(surf_curve, "StepToTopoDSGeometricTool::PCurve")?;
        for rank in (last + 1)..=live.nb_associated_geometry()? {
            let pcurve = associated_pcurve(live, rank)?;
            if pcurve
                .get()
                .is_some_and(|curve| curve.basis_surface() == *basis_surf)
            {
                return Ok((rank, pcurve));
            }
        }
        Ok((0, Handle::default()))
    }
    /// `is_seam_curve`: two oriented edges of the same wire reference the same edge.
    pub fn is_seam_curve(
        surf_curve: &Handle<StepGeomSurfaceCurve>,
        surf: &Handle<StepGeomSurface>,
        step_edge: &Handle<StepShapeEdge>,
        edge_loop: &Handle<StepShapeEdgeLoop>,
    ) -> Result<bool, Failure> {
        if surf_curve.cast::<StepGeomSeamCurve>().is_some() {
            return Ok(true);
        }
        let live = step_live(surf_curve, "StepToTopoDSGeometricTool::IsSeamCurve")?;
        if live.nb_associated_geometry()? != 2 || both_on_surface(live, surf)?.is_none() {
            return Ok(false);
        }
        Ok(occurrences_in_loop(step_edge, edge_loop)? == 2)
    }
    /// `is_like_seam`: the two pcurves lie on the same surface but on different wires, the typical
    /// CATIA B-rep of a cylinder coded as two faces on one closed B-spline surface.
    pub fn is_like_seam(
        surf_curve: &Handle<StepGeomSurfaceCurve>,
        surf: &Handle<StepGeomSurface>,
        step_edge: &Handle<StepShapeEdge>,
        edge_loop: &Handle<StepShapeEdgeLoop>,
    ) -> Result<bool, Failure> {
        let live = step_live(surf_curve, "StepToTopoDSGeometricTool::IsLikeSeam")?;
        if live.nb_associated_geometry()? != 2 {
            return Ok(false);
        }
        let Some((first, second)) = both_on_surface(live, surf)? else {
            return Ok(false);
        };
        if occurrences_in_loop(step_edge, edge_loop)? != 1 {
            return Ok(false);
        }
        // check if the two pcurves are not identical ?
        let (line1, line2) = (pcurve_line(&first)?, pcurve_line(&second)?);
        match (line1.get(), line2.get()) {
            (Some(a), Some(b)) => lines_look_like_seam(a, b),
            _ => Ok(false),
        }
    }
    /// `UpdateParam3d(theCurve, w1, w2, preci)`: when an edge crosses the parametric origin,
    /// `w1 > w2`. False when the parameters could only be forced. The curve may be reversed.
    pub fn update_param_3d(
        curve: &Handle<dyn GeomCurve>,
        w1: &mut f64,
        w2: &mut f64,
        preci: f64,
    ) -> Result<bool, Failure> {
        let live = step_live(curve, "StepToTopoDSGeometricTool::UpdateParam3d")?;
        let (cf, cl) = (live.first_parameter(), live.last_parameter());
        // w1 et/ou w2 peuvent etre en dehors des bornes naturelles de la courbe.
        // On donnera alors la valeur en bout a w1 et/ou w2
        if curve.cast::<dyn GeomBoundedCurve>().is_some() && !live.is_closed()? {
            clamp_parameter(w1, cf, cl);
            clamp_parameter(w2, cf, cl);
        }
        if *w1 < *w2 {
            return Ok(true);
        }
        if live.is_periodic() {
            //:a7 abv 11 Feb 98: preci -> PConfusion
            (*w1, *w2) = elclib::adjust_periodic(cf, cl, precision::p_confusion(), *w1, *w2);
            return Ok(true);
        }
        if live.is_closed()? {
            update_closed(live, w1, w2, preci)?;
            return Ok(true);
        }
        if let Some(spline) = curve.cast::<GeomBSplineCurve>().get() {
            return update_bspline(spline, w1, w2, preci);
        }
        reverse_with_parameters(live, w1, w2)?;
        // pdn 11.01.99 #144 bm1_pe_t4 protection of exceptions in draw
        if *w1 == *w2 {
            *w1 -= precision::p_confusion();
            *w2 += precision::p_confusion();
        }
        Ok(false)
    }
}
