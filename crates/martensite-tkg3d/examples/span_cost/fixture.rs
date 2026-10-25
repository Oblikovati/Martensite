//! Unit-spaced poles give an exact X-axis line across arbitrarily many distinct spans.
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg3d::{
    geom::{GeomBSplineCurve, GeomCurve},
    geom_adaptor::GeomAdaptorCurve,
};
use martensite_tkmath::gp::Point3;
use std::error::Error;

type AxisArrays = (
    Array1<'static, Point3>,
    Array1<'static, f64>,
    Array1<'static, i32>,
);

fn arrays(spans: i32) -> Result<AxisArrays, Box<dyn Error>> {
    let mut arrays = (
        Array1::with_bounds(1, spans + 1)?,
        Array1::with_bounds(1, spans + 1)?,
        Array1::with_bounds(1, spans + 1)?,
    );
    fill_axis_arrays(&mut arrays)?;
    Ok(arrays)
}

fn fill_axis_arrays(arrays: &mut AxisArrays) -> Result<(), Box<dyn Error>> {
    let last = arrays.0.upper();
    for index in 1..=last {
        let parameter = f64::from(index - 1);
        arrays.0.set_value(index, Point3::new(parameter, 0., 0.))?;
        arrays.1.set_value(index, parameter)?;
        arrays
            .2
            .set_value(index, if index == 1 || index == last { 2 } else { 1 })?;
    }
    Ok(())
}

/// Construct the same adaptor in both cost modes; `axis_adaptor(512)`.
pub(super) fn axis_adaptor(spans: i32) -> Result<GeomAdaptorCurve, Box<dyn Error>> {
    let (poles, knots, multiplicities) = arrays(spans)?;
    let curve = GeomBSplineCurve::new(&poles, &knots, &multiplicities, 1, false)?;
    let curve: Handle<dyn GeomCurve> = Handle::new(curve).cast();
    Ok(GeomAdaptorCurve::with_curve(&curve)?)
}
