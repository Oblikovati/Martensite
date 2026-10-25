//! Complete derivative bitstreams retain exact independent line checks without timing assertions.
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, geom_adaptor::GeomAdaptorCurve};
use std::{
    error::Error,
    io::{self, Write},
};

/// The exact derivatives of P(u)=(u,0,0); `closed_values(0.5)`.
pub(super) fn closed_values(parameter: f64) -> [f64; 30] {
    std::array::from_fn(|index| match index {
        0 | 3 | 9 | 18 => parameter,
        6 | 12 | 21 => 1.,
        _ => 0.,
    })
}

/// Evaluate all four derivative orders through the existing adaptor; `cached_values(&adaptor, 0.5)`.
pub(super) fn cached_values(
    adaptor: &GeomAdaptorCurve,
    parameter: f64,
) -> Result<[f64; 30], Box<dyn Error>> {
    let p0 = adaptor.d0(parameter)?;
    let (p1, v1) = adaptor.d1(parameter)?;
    let (p2, v21, v22) = adaptor.d2(parameter)?;
    let (p3, v31, v32, v33) = adaptor.d3(parameter)?;
    let triples = [
        p0.xyz(),
        p1.xyz(),
        *v1.xyz(),
        p2.xyz(),
        *v21.xyz(),
        *v22.xyz(),
        p3.xyz(),
        *v31.xyz(),
        *v32.xyz(),
        *v33.xyz(),
    ];
    let coordinates = triples.map(|xyz| [xyz.x(), xyz.y(), xyz.z()]);
    Ok(std::array::from_fn(|index| {
        coordinates[index / 3][index % 3]
    }))
}

/// Refuse any incorrect derivative and retain every raw bit; `checked_record(output, u, values)`.
pub(super) fn checked_record(
    output: &mut impl Write,
    parameter: f64,
    actual: [f64; 30],
) -> Result<(), Box<dyn Error>> {
    let expected = closed_values(parameter);
    if actual != expected {
        return Err(io::Error::other(format!(
            "derivatives at {parameter}: got {actual:?}, expected {expected:?}"
        ))
        .into());
    }
    output.write_all(&parameter.to_le_bytes())?;
    for value in actual {
        output.write_all(&value.to_le_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_derivative_refuses_the_complete_record_before_writing() {
        let mut actual = closed_values(0.5);
        actual[6] = 2.;
        let mut output = Vec::new();
        let failure = checked_record(&mut output, 0.5, actual).unwrap_err();
        assert!(failure.to_string().contains("derivatives at 0.5"));
        assert!(failure.to_string().contains("expected"));
        assert!(output.is_empty());
    }
}
