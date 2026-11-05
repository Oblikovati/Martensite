//! The 3d curve of a section edge in the global frame, oriented as the edge.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::Edge;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::errors::null_object;
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkg3d::top_abs::Orientation;

/// A located, oriented copy of the 3d curve of an edge, its adaptor and range.
#[derive(Default)]
pub(crate) struct KPartEdge {
    /// The adaptor loaded over its own bounds on the transformed (and, for a REVERSED
    /// edge, reversed) copy of the curve.
    pub(crate) adaptor: GeomAdaptorCurve,
    /// The edge range on `curve`.
    pub(crate) first: f64,
    /// The edge range on `curve`.
    pub(crate) last: f64,
    /// The range of the edge on its stored curve, before the reversal.
    pub(crate) raw: (f64, f64),
}

/// The curve of `edge` transformed by its location (`Curve(E, L, f, l)` then
/// `Transformed(L)`, always a copy) and, for a REVERSED edge, reversed with the range
/// mapped by `ReversedParameter`; `None` without a 3d curve.
/// `let section = oriented_curve(&edge)?;`
pub(crate) fn oriented_curve(edge: &Edge) -> Result<Option<KPartEdge>, Failure> {
    let found = BRepTool::curve_with_location(edge)?;
    let Some(stored) = found.curve.get() else {
        return Ok(None);
    };
    let curve: Handle<dyn GeomCurve> = stored
        .transformed(found.location.transformation())?
        .cast::<dyn GeomCurve>();
    let (ff, ll) = (found.first, found.last);
    let (mut first, mut last) = (ff, ll);
    if edge.orientation() == Orientation::Reversed {
        let Some(copy) = curve.get() else {
            return Err(null_object(
                "BRepFillGenerator: the transformed copy of an edge curve is null; expected a curve",
            ));
        };
        copy.reverse()?;
        first = copy.reversed_parameter(ll)?;
        last = copy.reversed_parameter(ff)?;
    }
    let adaptor = GeomAdaptorCurve::with_curve(&curve)?;
    Ok(Some(KPartEdge {
        adaptor,
        first,
        last,
        raw: (ff, ll),
    }))
}
