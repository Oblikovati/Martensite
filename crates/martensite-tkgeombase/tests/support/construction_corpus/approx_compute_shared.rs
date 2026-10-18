//! Shared existing Approx constructors and serializers consumed by AppDef and Approx probes.
use super::{
    common as p,
    fixture_input::GeometryFixtureInput,
    multi_curve_tokens::curve,
    print::{Output, attempt},
};
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkgeombase::app_par_curves::{Constraint, LineTool, MultiBSpCurve, MultiCurveEval};
use martensite_tkgeombase::approx::{ComputeLine, ComputeLineOptions, ParametrizationType};
use martensite_tkmath::math::Vector;

pub fn constraint(code: i32) -> Constraint {
    Constraint::from_code(code).expect("probe constraint")
}

/// `gb_ax_array`.
pub fn array(output: &Output, a: &Array1<'_, f64>) {
    p::int(output, a.lower());
    p::int(output, a.upper());
    a.iter().for_each(|&v| p::real(output, v));
}

/// `gb_ax_bspline`.
pub fn bspline(output: &Output, c: &MultiBSpCurve) -> Result<(), Failure> {
    curve(output, c)?;
    p::int(output, MultiCurveEval::degree(c)?);
    if c.nb_poles() == 0 {
        return Ok(());
    }
    array(output, &c.knots()?);
    c.multiplicities()?.iter().for_each(|&m| p::int(output, m));
    Ok(())
}

/// `gb_ax_read_options`.
pub fn read_options(r: &mut impl GeometryFixtureInput) -> ComputeLineOptions {
    ComputeLineOptions {
        degree_min: r.integer(),
        degree_max: r.integer(),
        tolerance_3d: r.real(),
        tolerance_2d: r.real(),
        nb_iterations: r.integer(),
        cutting: r.flag(),
        parametrization: ParametrizationType::from_code(r.integer()).expect("probe type"),
        squares: r.flag(),
    }
}

/// The ComputeLine of a variant (also Compute for `ad_compute`).
pub fn build_compute<L: LineTool + Clone>(
    line: &L,
    variant: i32,
    o: ComputeLineOptions,
    u: &Vector<'static>,
) -> Result<ComputeLine, Failure> {
    Ok(match variant {
        0 => ComputeLine::with_line(line, o)?,
        1 => ComputeLine::with_line_and_parameters(line, u, o)?,
        2 => ComputeLine::new(o),
        _ => ComputeLine::from_parameters(u, o)?,
    })
}

/// Also printed for Compute (`ad_compute`).
pub fn compute_results(output: &Output, c: &mut ComputeLine) -> Result<(), Failure> {
    p::flag(output, c.is_all_approximated());
    p::flag(output, c.is_tolerance_reached());
    p::int(output, c.parametrization().code());
    let n: i32 = c.nb_multi_curves();
    p::int(output, n);
    for i in 1..=n {
        curve(output, c.value(i)?)?;
        let (t3, t2) = c.error(i)?;
        p::real(output, t3);
        p::real(output, t2);
        array(output, c.parameters(i)?);
    }
    attempt(output, || bspline(output, c.spline_value()?));
    attempt(output, || curve(output, c.value(n + 1)?));
    Ok(())
}
