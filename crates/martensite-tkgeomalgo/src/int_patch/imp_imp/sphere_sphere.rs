//! Two spheres.
use super::super::gline::IntPatchGLine;
use super::plane::{Lines, Points, push, tangent_point};
use super::transitions::{by_sign_tol, dot_cross};
use crate::int_surf::IntSurfQuadric;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::int_ana::{QuadQuadGeo, ResultType};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::Circle3;

/// `IntSpSp(Quad1, Quad2, Tol, Empty, Same, slin, spnt)`: the same sphere, a tangent
/// point or a circle.
pub(super) fn int_spsp(
    quads: (&IntSurfQuadric, &IntSurfQuadric),
    tol: f64,
    (empty, same): (&mut bool, &mut bool),
    slin: &mut Lines,
    spnt: &mut Points,
) -> Result<bool, Failure> {
    let mut inter = QuadQuadGeo::new();
    inter.perform_spheres(quads.0.sphere()?, quads.1.sphere()?, tol)?;
    if !inter.is_done() {
        return Ok(false);
    }
    let typint: ResultType = inter.type_inter()?;
    *empty = false;
    *same = false;
    match typint {
        ResultType::Empty => *empty = true,
        ResultType::Same => *same = true,
        ResultType::Point => spnt.push(tangent_point(inter.point(1)?, quads, tol, true)),
        ResultType::Circle => {
            let cirsol: Circle3 = inter.circle(1)?;
            let (ptref, tgt) = elclib::d1(0.0, &cirsol);
            let qwe: f64 = dot_cross(tgt, quads.0, quads.1, ptref)?;
            push(
                slin,
                IntPatchGLine::from_circle(&cirsol, false, by_sign_tol(qwe, 0.00000001)),
            );
        }
        _ => return Ok(false), // must not happen
    }
    Ok(true)
}
