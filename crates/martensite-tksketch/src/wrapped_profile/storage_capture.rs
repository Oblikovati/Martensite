//! Capture actual retained native data, including unit weights after rational demotion.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomBezierCurve, GeomBezierSurface, GeomCylindricalSurface, GeomElementarySurface,
};

pub(super) fn curve(
    key: BindingKey,
    bindings: &Bindings<'_>,
) -> Result<OwnedCurve, StorageFailure> {
    let edge = native(key, topods::edge(lookup(key, bindings)?))?;
    let captured = native(key, BRepTool::curve_with_location(&edge))?;
    curve_domain(
        key,
        [captured.first, captured.last],
        captured.location.is_identity(),
    )?;
    let curve: Handle<GeomBezierCurve> = captured.curve.cast();
    let Some(curve) = curve.get() else {
        return Err(mismatch(key, "stored Bezier class", vec![0], vec![1]));
    };
    curve_coefficients(key, curve)
}

fn curve_domain(key: BindingKey, range: [f64; 2], located: bool) -> Result<(), StorageFailure> {
    if !located || range != [0., 1.] {
        return Err(mismatch(
            key,
            "Bezier chart and identity location",
            vec![range[0].to_bits(), range[1].to_bits(), u64::from(located)],
            vec![0, 1f64.to_bits(), 1],
        ));
    }
    Ok(())
}

fn curve_coefficients(
    key: BindingKey,
    curve: &GeomBezierCurve,
) -> Result<OwnedCurve, StorageFailure> {
    let poles = curve
        .poles()
        .iter()
        .map(|p| [p.x(), p.y(), p.z()])
        .collect();
    let weights = (1..=curve.nb_poles())
        .map(|i| native(key, curve.weight(i)))
        .collect::<Result<_, _>>()?;
    Ok(OwnedCurve { poles, weights })
}

pub(super) fn caps(request: &Request, bindings: &Bindings<'_>) -> Result<(), StorageFailure> {
    let roles = [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace];
    for (depth, role) in roles.into_iter().enumerate() {
        let key = BindingKey {
            source: Source::Region(request.region),
            role,
        };
        cap(key, request.radius + request.depths[depth], bindings)?;
    }
    Ok(())
}

fn cap(key: BindingKey, radius: f64, bindings: &Bindings<'_>) -> Result<(), StorageFailure> {
    let face = native(key, topods::face(lookup(key, bindings)?))?;
    let (surface, location) = native(key, BRepTool::surface_with_location(&face))?;
    let cylinder: Handle<GeomCylindricalSurface> = surface.cast();
    let Some(cylinder) = cylinder.get() else {
        return Err(mismatch(key, "analytic cylinder class", vec![0], vec![1]));
    };
    cap_placement(key, cylinder, radius, location.is_identity())?;
    cap_frame(key, cylinder)
}

fn cap_placement(
    key: BindingKey,
    cylinder: &GeomCylindricalSurface,
    radius: f64,
    located: bool,
) -> Result<(), StorageFailure> {
    if !located || cylinder.radius().to_bits() != radius.to_bits() {
        return Err(mismatch(
            key,
            "cap radius and location",
            vec![cylinder.radius().to_bits(), u64::from(located)],
            vec![radius.to_bits(), 1],
        ));
    }
    Ok(())
}

fn cap_frame(key: BindingKey, cylinder: &GeomCylindricalSurface) -> Result<(), StorageFailure> {
    let frame = cylinder.position();
    let p = frame.location();
    let axes = [frame.x_direction(), frame.y_direction(), frame.direction()];
    let mut actual = vec![p.x(), p.y(), p.z()];
    for d in axes {
        actual.extend([d.x(), d.y(), d.z()]);
    }
    let expected = [0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 1.];
    // Signed zero is geometrically identical here; the frozen codec retains its bits separately.
    if actual != expected {
        return Err(mismatch(
            key,
            "local +Z cylinder frame",
            actual.into_iter().map(f64::to_bits).collect(),
            expected.into_iter().map(f64::to_bits).collect(),
        ));
    }
    Ok(())
}

pub(super) fn side(
    key: BindingKey,
    curves: [&OwnedCurve; 2],
    bindings: &Bindings<'_>,
) -> Result<(), StorageFailure> {
    let face = native(key, topods::face(lookup(key, bindings)?))?;
    let (surface, location) = native(key, BRepTool::surface_with_location(&face))?;
    let surface: Handle<GeomBezierSurface> = surface.cast();
    let Some(surface) = surface.get() else {
        return Err(mismatch(
            key,
            "stored Bezier surface class",
            vec![0],
            vec![1],
        ));
    };
    let expected = vec![curves[0].poles.len() as u64, 2, 1];
    let actual = vec![
        surface.nb_u_poles() as u64,
        surface.nb_v_poles() as u64,
        u64::from(location.is_identity()),
    ];
    if actual != expected {
        return Err(mismatch(key, "radial side dimensions", actual, expected));
    }
    if curves[0].weights != curves[1].weights {
        return Err(mismatch(
            key,
            "common radial weights",
            curves[1].weights.iter().map(|w| w.to_bits()).collect(),
            curves[0].weights.iter().map(|w| w.to_bits()).collect(),
        ));
    }
    side_rows(key, surface, curves)
}

fn side_rows(
    key: BindingKey,
    surface: &GeomBezierSurface,
    curves: [&OwnedCurve; 2],
) -> Result<(), StorageFailure> {
    for (depth, curve) in curves.into_iter().enumerate() {
        for (i, point) in curve.poles.iter().enumerate() {
            side_pole(
                key,
                surface,
                (i as i32 + 1, depth as i32 + 1),
                *point,
                curve.weights[i],
            )?;
        }
    }
    Ok(())
}

fn side_pole(
    key: BindingKey,
    surface: &GeomBezierSurface,
    index: (i32, i32),
    expected: [f64; 3],
    weight: f64,
) -> Result<(), StorageFailure> {
    let point = native(key, surface.pole(index.0, index.1))?;
    let w = native(key, surface.weight(index.0, index.1))?;
    let actual = [point.x(), point.y(), point.z(), w].map(f64::to_bits);
    let expected = [expected[0], expected[1], expected[2], weight].map(f64::to_bits);
    if actual != expected {
        return Err(mismatch(
            key,
            "ruled control-net row",
            actual.to_vec(),
            expected.to_vec(),
        ));
    }
    Ok(())
}
