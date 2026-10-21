//! Original ellipse spacing regression, including its output-token assertion.
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomEllipse};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::gcpnts::{AbscissaPoint, UniformAbscissa};
use martensite_tkgeombase::geom_lprop::CLProps;
use martensite_tkmath::gp::Axis2;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

fn command_ellipse() -> Result<Handle<dyn GeomCurve>, Failure> {
    let curve: Handle<dyn GeomCurve> =
        Handle::new(GeomEllipse::new(Axis2::default(), 100., 50.)?).cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&curve)?.edge()?;
    assert!(!edge.is_null());
    Ok(curve)
}

fn command_abscissa_output() -> Result<String, Failure> {
    let curve = command_ellipse()?;
    let mut properties = CLProps::new(curve.clone(), 2, precision::intersection())?;
    properties.set_curve(curve.clone());
    let adaptor = GeomAdaptorCurve::with_curve(&curve)?;
    let length = command_curve_length(&adaptor)?;
    let spacing = length / (10 - 1) as f64;
    let algorithm = command_uniform_spacing(&curve, &adaptor, spacing)?;
    let prefix = format!(
        "Type Of curve: {}\nEllipse length = {length}\n CUR : Abscissa {spacing}\n",
        adaptor.get_type()? as i32
    );
    append_point_parameters(prefix, &algorithm)
}

fn command_curve_length(adaptor: &GeomAdaptorCurve) -> Result<f64, Failure> {
    AbscissaPoint::length_between_with_tolerance(
        adaptor,
        adaptor.first_parameter()?,
        adaptor.last_parameter()?,
        precision::confusion(),
    )
}

fn command_uniform_spacing(
    curve: &Handle<dyn GeomCurve>,
    adaptor: &GeomAdaptorCurve,
    spacing: f64,
) -> Result<UniformAbscissa, Failure> {
    UniformAbscissa::with_abscissa_range(
        adaptor,
        spacing,
        curve.first_parameter(),
        curve.last_parameter(),
        -1.,
    )
}

fn append_point_parameters(
    mut output: String,
    algorithm: &UniformAbscissa,
) -> Result<String, Failure> {
    if !algorithm.is_done() {
        return Ok(output);
    }
    let count = algorithm.nb_points()?;
    output.push_str(&format!(" CasCurve  - nbpoints {count}\n"));
    for index in 1..=count {
        output.push_str(&format!(
            "{index} points = {}\n",
            algorithm.parameter(index)?
        ));
    }
    Ok(output)
}

#[test]
fn bug18541_2_original_ellipse_spacing_output_reports_ten_points() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "ellipse_uniform_abscissa::bug18541_2_original_ellipse_spacing_output_reports_ten_points",
        || Ok(bug18541_2_original_ellipse_spacing_output_reports_ten_points_body()?),
    );
    Ok(())
}

fn bug18541_2_original_ellipse_spacing_output_reports_ten_points_body() -> Result<(), Failure> {
    let output = command_abscissa_output()?;
    let tokens: Vec<&str> = output.split_whitespace().collect();
    assert!(
        tokens.len() > 15,
        "Faulty OCC18541: Bad draw command: {output}"
    );
    assert_eq!(tokens[15].parse::<i32>().unwrap(), 10, "Faulty OCC18541");
    Ok(())
}
