//! The planar interface supplies its own actual values without changing the sampling sequence.
use martensite_tkg2d::{adaptor2d::Adaptor2dCurve2d, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkmath::gp::Xy;

/// Collect every planar derivative coordinate; `values(&adaptor,0.25)`.
pub(super) fn values(adaptor: &Geom2dAdaptorCurve, parameter: f64) -> [f64; 20] {
    let first = adaptor.d0(parameter).unwrap().xy();
    let mut rows = [first; 10];
    rows[1..].copy_from_slice(&derivative_rows(adaptor, parameter));
    let coordinates = rows.map(|xy| [xy.x(), xy.y()]);
    std::array::from_fn(|index| coordinates[index / 2][index % 2])
}

fn derivative_rows(adaptor: &Geom2dAdaptorCurve, parameter: f64) -> [Xy; 9] {
    let (p1, v1) = adaptor.d1(parameter).unwrap();
    let (p2, v21, v22) = adaptor.d2(parameter).unwrap();
    let (p3, v31, v32, v33) = adaptor.d3(parameter).unwrap();
    [
        p1.xy(),
        v1.xy(),
        p2.xy(),
        v21.xy(),
        v22.xy(),
        p3.xy(),
        v31.xy(),
        v32.xy(),
        v33.xy(),
    ]
}
