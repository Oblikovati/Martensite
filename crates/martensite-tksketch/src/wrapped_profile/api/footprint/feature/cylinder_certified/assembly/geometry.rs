//! Exact native cylinder datum and literal continuous unit-domain pcurve readers.
use super::*;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkg3d::geom::GeomCylindricalSurface;
use martensite_tkmath::gp::Transform3;

#[derive(Clone, Copy)]
pub(in super::super) struct CylinderDatum {
    pub origin: [f64; 3],
    pub radius: f64,
    pub height_sign: f64,
}

impl CylinderDatum {
    pub(in super::super) fn read(attempt: Attempt, face: &Face) -> Checked<Self> {
        let (surface, location) = attempt.native(
            ASSEMBLY,
            "native cylinder carrier",
            BRepTool::surface_with_location(face),
        )?;
        let cylinder = surface.downcast::<GeomCylindricalSurface>();
        require(
            attempt,
            ASSEMBLY,
            !cylinder.is_null(),
            "analytic cylinder carrier",
        )?;
        let geometry =
            attempt.native(ASSEMBLY, "read actual cylinder datum", cylinder.cylinder())?;
        let frame = geometry.position();
        let axis = *frame.direction().coordinates();
        require(
            attempt,
            ASSEMBLY,
            *frame.x_direction().coordinates() == [1., 0., 0.]
                && *frame.y_direction().coordinates() == [0., 1., 0.]
                && [1., -1.].contains(&axis[2])
                && axis[0] == 0.
                && axis[1] == 0.
                && geometry.radius().is_finite()
                && geometry.radius() > 0.,
            "literal cylindrical basis and positive radius",
        )?;
        let origin = exact_translation(
            attempt,
            *frame.location().coordinates(),
            location.transformation(),
        )?;
        Ok(Self {
            origin,
            radius: geometry.radius(),
            height_sign: axis[2],
        })
    }
}

pub(in super::super) fn exact_translation(
    attempt: Attempt,
    origin: [f64; 3],
    transform: Transform3,
) -> Checked<[f64; 3]> {
    require(
        attempt,
        ASSEMBLY,
        transform.scale_factor() == 1.
            && transform.matrix().coefficients() == &[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        "only the admitted exact translation",
    )?;
    let moved = *transform.translation_part().coordinates();
    let result = std::array::from_fn(|i| origin[i] + moved[i]);
    let exact = origin
        .into_iter()
        .zip(moved)
        .zip(result)
        .all(|((a, b), sum)| {
            let (big, small) = if a.abs() >= b.abs() { (a, b) } else { (b, a) };
            sum.is_finite() && small - (sum - big) == 0.
        });
    require(
        attempt,
        ASSEMBLY,
        exact,
        "literal composed translation without discarded coordinate bits",
    )?;
    Ok(result)
}

pub(in super::super) fn polynomial_trace(
    attempt: Attempt,
    face: &Face,
    edge: &topods::Edge,
) -> Checked<(
    martensite_tkernel::standard::Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve>,
    Vec<[f64; 2]>,
)> {
    let trace = attempt.native(
        ASSEMBLY,
        "stored cap pcurve",
        BRepTool::curve_on_face(edge, face),
    )?;
    let bezier = trace.pcurve.downcast::<Geom2dBezierCurve>();
    require(
        attempt,
        ASSEMBLY,
        trace.is_stored
            && [trace.first, trace.last] == [0., 1.]
            && !bezier.is_null()
            && !bezier.is_rational()
            && !bezier.has_eval_representation()
            && (2..=4).contains(&bezier.nb_poles()),
        "literal polynomial unit-domain cap trim",
    )?;
    let words = (1..=bezier.nb_poles())
        .map(|index| {
            let point =
                attempt.native(ASSEMBLY, "native cap pcurve control", bezier.pole(index))?;
            Ok([point.x(), point.y()])
        })
        .collect::<Checked<Vec<_>>>()?;
    require(
        attempt,
        ASSEMBLY,
        words.iter().flatten().all(|v| v.is_finite()),
        "finite native spline trim controls",
    )?;
    Ok((trace.pcurve, words))
}
