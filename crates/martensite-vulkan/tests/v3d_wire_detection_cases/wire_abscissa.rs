//! Preserve RegressionCommands OCC105 connected-edge orientation and all sampled point output.
use super::*;
use martensite_tkbrep::topods::{Edge, Vertex, Wire};
use martensite_tkbrep::{brep::BRepTool, brep_tools::WireExplorer, topexp, topods};
use martensite_tkernel::{standard::Handle, storage::format_general};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve, geom::GeomCurve, geom_adaptor::GeomAdaptorCurve,
};
use martensite_tkgeombase::gcpnts::UniformAbscissa;

fn first_wire_vertex(wire: &Wire, edge: &Edge) -> Vertex {
    let (first, last) = topexp::wire_vertices(wire);
    let (start, end) = topexp::vertices(edge, false);
    if first.is_same(&start) || first.is_same(&end) {
        first
    } else {
        last
    }
}

pub(super) fn oriented_curve(
    edge: &Edge,
    previous: &mut Vertex,
) -> Result<(Handle<dyn GeomCurve>, f64, f64), Failure> {
    let (start, end) = topexp::vertices(edge, false);
    let (curve, first, last) = BRepTool::curve(edge)?;
    if start.is_same(previous) {
        *previous = end;
        return Ok((curve, first, last));
    }
    *previous = start;
    let first = curve.reversed_parameter(first)?;
    let last = curve.reversed_parameter(last)?;
    Ok((curve.reversed()?, first, last))
}

fn report_samples(curve: &GeomAdaptorCurve, samples: &UniformAbscissa) -> Result<(), Failure> {
    if !samples.is_done() {
        println!("Not Done!!!");
    }
    for index in 1..=samples.nb_points()? {
        let parameter = samples.parameter(index)?;
        let point = curve.value(parameter)?;
        println!("Parameter t = {}", format_general(parameter, 17));
        println!(
            "Value Pnt = {} {} {}",
            format_general(point.x(), 17),
            format_general(point.y(), 17),
            format_general(point.z(), 17)
        );
    }
    Ok(())
}

pub(super) fn sample_wire(shape: &Shape) -> Result<(), Failure> {
    let wire = topods::wire(shape)?;
    let mut explorer = WireExplorer::with_wire(&wire)?;
    let mut previous = first_wire_vertex(&wire, explorer.current());
    while explorer.more() {
        println!("\n\n New Edge \n\n");
        let (curve, first, last) = oriented_curve(explorer.current(), &mut previous)?;
        let adaptor = GeomAdaptorCurve::with_curve(&curve)?;
        let samples = UniformAbscissa::with_abscissa_range(&adaptor, 0.5, first, last, -1.)?;
        report_samples(&adaptor, &samples)?;
        explorer.next()?;
    }
    Ok(())
}

const HEADING: &[&str] = &[
    "================",
    "BUC61026",
    "OCC105",
    "================",
    "",
];

/// Keep actual wire traversal and the catch branch before the standard view check.
/// Example: `wire_abscissa_regression()?`.
pub(crate) fn wire_abscissa_regression() -> Outcome {
    banner(HEADING);
    let result = read_dataset_shape("OCC105.brep")?;
    crate::shape_report_checks::check_shape_structure(&result)?;
    match sample_wire(&result) {
        Ok(()) => println!("BUC61032: OK"),
        Err(_) => println!("Error : BUC61032"),
    }
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.dump_and_finish("bug105.png", false)
}
