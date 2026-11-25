//! Identity of a full parameterized edge with one literal Bézier patch boundary.
use martensite_tkbrep::brep::BRepCurveRepresentation;
use martensite_tkernel::ncollection::Array2;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomBezierSurface, GeomSurface};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkmath::gp::{Point3, Transform3, resolution};

#[derive(Clone, Copy)]
enum Boundary {
    AlongU(bool),
    AlongV(bool),
}

impl Boundary {
    fn from_trace(trace: &Geom2dBezierCurve) -> Option<Self> {
        // One snapshot keeps the controls and the effective evaluation mode together.
        let trace = Geom2dBezierCurve::copy_of(trace).ok()?;
        if trace.is_rational() || trace.has_eval_representation() {
            return None;
        }
        let poles = trace.poles();
        let [a, b] = poles.as_slice() else {
            return None;
        };
        match ([a.x(), a.y()], [b.x(), b.y()]) {
            ([0., 0.], [1., 0.]) => Some(Self::AlongU(false)),
            ([0., 1.], [1., 1.]) => Some(Self::AlongU(true)),
            ([0., 0.], [0., 1.]) => Some(Self::AlongV(false)),
            ([1., 0.], [1., 1.]) => Some(Self::AlongV(true)),
            _ => None,
        }
    }

    fn length(self, net: &Array2<'_, Point3>) -> usize {
        match self {
            Self::AlongU(_) => net.nb_rows(),
            Self::AlongV(_) => net.nb_columns(),
        }
    }

    fn index(self, net: &Array2<'_, Point3>, varying: i32) -> (i32, i32) {
        match self {
            Self::AlongU(false) => (varying, net.lower_col()),
            Self::AlongU(true) => (varying, net.upper_col()),
            Self::AlongV(false) => (net.lower_row(), varying),
            Self::AlongV(true) => (net.upper_row(), varying),
        }
    }
}

pub(super) fn identical_boundary(
    reference: &Handle<dyn Adaptor3dCurve>,
    surface: &Handle<dyn GeomSurface>,
    trace: &Handle<dyn Geom2dCurve>,
) -> bool {
    let trace = trace.downcast::<Geom2dBezierCurve>();
    let Some(boundary) = trace.get().and_then(Boundary::from_trace) else {
        return false;
    };
    matches_loaded_boundary(reference, surface, boundary).unwrap_or(false)
}

fn matches_loaded_boundary(
    reference: &Handle<dyn Adaptor3dCurve>,
    surface: &Handle<dyn GeomSurface>,
    boundary: Boundary,
) -> Option<bool> {
    let surface = surface.downcast::<GeomBezierSurface>();
    let reference = reference.downcast::<GeomAdaptorCurve>();
    let reference = reference.get()?.curve().downcast::<GeomBezierCurve>();
    matches_boundary(surface.get()?, boundary, reference.get()?)
}

pub(super) fn identical_located_boundary(
    reference: &Handle<dyn Adaptor3dCurve>,
    surface: &Handle<dyn GeomSurface>,
    trace: &Handle<dyn Geom2dCurve>,
    raw_reference: &dyn BRepCurveRepresentation,
    raw_surface: &Handle<dyn GeomSurface>,
    placement: Transform3,
) -> bool {
    let trace = trace.downcast::<Geom2dBezierCurve>();
    let Some(boundary) = trace.get().and_then(Boundary::from_trace) else {
        return false;
    };
    if !literal_translation(placement)
        || matches_loaded_boundary(reference, surface, boundary) != Some(true)
    {
        return false;
    }
    // Translating binary coordinates can round distinct poles to the same point.
    // Check the actual local coefficients as well as the loaded geometry.
    matches_raw_boundary(raw_reference, raw_surface, boundary).unwrap_or(false)
}

fn literal_translation(placement: Transform3) -> bool {
    placement.scale_factor() == 1.
        && placement.matrix().coefficients() == &[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]
        && placement
            .translation_part()
            .coordinates()
            .iter()
            .all(|v| v.is_finite())
}

fn matches_raw_boundary(
    reference: &dyn BRepCurveRepresentation,
    surface: &Handle<dyn GeomSurface>,
    boundary: Boundary,
) -> Option<bool> {
    let reference = reference.curve3d().ok()?.downcast::<GeomBezierCurve>();
    let surface = surface.downcast::<GeomBezierSurface>();
    // The native checker works from copies. Take coherent source snapshots here too.
    let reference = GeomBezierCurve::copy_of(reference.get()?).ok()?;
    let surface = GeomBezierSurface::copy_of(surface.get()?).ok()?;
    matches_boundary(&surface, boundary, &reference)
}

fn matches_boundary(
    surface: &GeomBezierSurface,
    boundary: Boundary,
    reference: &GeomBezierCurve,
) -> Option<bool> {
    if surface.has_eval_representation() || reference.has_eval_representation() {
        return None;
    }
    let net = surface.poles();
    let weights = surface.weights();
    let (poles, edge_weights) = reference.poles_weights_snapshot();
    if !defined_net(&net, weights.as_ref()) || boundary.length(&net) != poles.len() {
        return None;
    }
    Some(matching_controls(
        boundary,
        &net,
        weights.as_ref(),
        poles.as_slice(),
        edge_weights.as_ref().map(|w| w.as_slice()),
    ))
}

fn defined_net(net: &Array2<'_, Point3>, weights: Option<&Array2<'_, f64>>) -> bool {
    let sizes = [net.nb_rows(), net.nb_columns()];
    let degrees = 2..=GeomBezierCurve::max_degree() as usize + 1;
    sizes.iter().all(|n| degrees.contains(n))
        && net
            .iter()
            .all(|p| p.coordinates().iter().all(|x| x.is_finite()))
        && weights.is_none_or(|w| {
            [w.nb_rows(), w.nb_columns()] == sizes
                && w.iter().all(|v| v.is_finite() && *v > resolution())
        })
}

fn matching_controls(
    boundary: Boundary,
    net: &Array2<'_, Point3>,
    weights: Option<&Array2<'_, f64>>,
    edge: &[Point3],
    edge_weights: Option<&[f64]>,
) -> bool {
    if edge_weights.is_some_and(|w| w.len() != edge.len()) {
        return false;
    }
    // At a unit-square boundary, the other Bernstein basis is one at the
    // selected row or column and zero everywhere else.
    edge.iter().enumerate().all(|(i, pole)| {
        let (u, v) = boundary.index(net, i as i32 + 1);
        let surface_weight = weights.map_or(Some(1.), |w| w.value(u, v).ok().copied());
        let edge_weight = edge_weights.map_or(1., |w| w[i]);
        surface_weight == Some(edge_weight)
            && net
                .value(u, v)
                .is_ok_and(|p| p.coordinates() == pole.coordinates())
    })
}

#[cfg(test)]
mod tests;
