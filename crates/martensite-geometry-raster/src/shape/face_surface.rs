//! Surface-domain restriction and continuity sampling categories for face drawing.
use super::IsoSurface;
use martensite_tkbrep::{brep::BRepTool, brep_tools::uv_bounds, topods::Face};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkmath::geom_abs::{CurveType, IsoType, SurfaceType};
/// The parameter span below which a range counts as empty in the restriction test.
const ZERO_SPAN: f64 = 1.0e-9;
/// The parameter span above which a face range counts as infinite.
const HUGE_SPAN: f64 = 4.1e100;

/// How the points inside one continuity interval are generated.
pub(super) enum IntervalSteps {
    None,
    Uniform,
    Adaptive,
}
pub(super) fn surface_steps(surface: &IsoSurface, kind: IsoType) -> Result<IntervalSteps, Failure> {
    Ok(match surface.kind {
        SurfaceType::Plane => IntervalSteps::None,
        SurfaceType::Cylinder | SurfaceType::Cone => circular_surface_steps(kind),
        SurfaceType::Sphere
        | SurfaceType::Torus
        | SurfaceType::OffsetSurface
        | SurfaceType::OtherSurface => IntervalSteps::Uniform,
        SurfaceType::BezierSurface | SurfaceType::BSplineSurface => IntervalSteps::Adaptive,
        SurfaceType::SurfaceOfExtrusion | SurfaceType::SurfaceOfRevolution => {
            swept_steps(surface, kind)?
        }
    })
}
fn circular_surface_steps(kind: IsoType) -> IntervalSteps {
    if kind == IsoType::IsoV {
        return IntervalSteps::Uniform;
    }
    IntervalSteps::None
}
/// The swept surfaces: along the sweep (V isos of a revolution) by uniform steps,
/// nothing along an extrusion direction, else by the type of the basis curve.
fn swept_steps(surface: &IsoSurface, kind: IsoType) -> Result<IntervalSteps, Failure> {
    let revolution = surface.kind == SurfaceType::SurfaceOfRevolution;
    let along_sweep =
        (kind == IsoType::IsoV && revolution) || (kind == IsoType::IsoU && !revolution);
    if along_sweep {
        return Ok(if revolution {
            IntervalSteps::Uniform
        } else {
            IntervalSteps::None
        });
    }
    basis_steps(surface)
}

/// The restriction flag of the face adaptor: a face of a periodic surface whose UV
/// bounds leave the surface bounds is restricted to its wires, unless one of the
/// ranges is empty, reversed or huge.
pub(super) fn restriction(face: &Face) -> Result<bool, Failure> {
    let (surface, _) = BRepTool::surface_with_location(face)?;
    let Some(surface) = surface.get() else {
        return Ok(false);
    };
    let (u_periodic, v_periodic) = (surface.is_u_periodic(), surface.is_v_periodic());
    if !u_periodic && !v_periodic {
        return Ok(false);
    }
    let surface_bounds = surface.bounds();
    let face_bounds = uv_bounds(face)?;
    Ok(restricted_ranges(
        surface_bounds,
        face_bounds,
        (u_periodic, v_periodic),
    ))
}

fn basis_steps(surface: &IsoSurface) -> Result<IntervalSteps, Failure> {
    let basis = surface.adaptor.basis_curve()?;
    let curve_type = basis
        .get()
        .ok_or_else(|| Failure::with_message("Standard_NullObject", "basis curve"))?
        .get_type()?;
    Ok(match curve_type {
        CurveType::Line => IntervalSteps::None,
        CurveType::Circle | CurveType::Ellipse => IntervalSteps::Uniform,
        _ => IntervalSteps::Adaptive,
    })
}

type SurfaceBounds = (f64, f64, f64, f64);
fn restricted_ranges(
    (su1, su2, sv1, sv2): SurfaceBounds,
    (fu1, fu2, fv1, fv2): SurfaceBounds,
    (u_periodic, v_periodic): (bool, bool),
) -> bool {
    let outside = |f: f64, s1: f64, s2: f64| f < s1 || f > s2;
    let mut restricted = u_periodic && (outside(fu1, su1, su2) || outside(fu2, su1, su2));
    if !restricted && v_periodic {
        restricted = outside(fv1, sv1, sv2) || outside(fv2, sv1, sv2);
    }
    let zero_surface = (su2 - su1).abs() <= ZERO_SPAN || (sv2 - sv1).abs() <= ZERO_SPAN;
    let zero_face = (fu2 - fu1).abs() <= ZERO_SPAN || (fv2 - fv1).abs() <= ZERO_SPAN;
    let reversed = fu1 >= fu2 || fv1 >= fv2;
    let huge = (fu2 - fu1).abs() > HUGE_SPAN || (fv2 - fv1).abs() > HUGE_SPAN;
    restricted && !(zero_surface || zero_face) && !reversed && !huge
}
