//! All four authored checks retain their exact percentage formulas and literal thresholds.
use crate::property_queries::{Kind, properties};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
pub(super) fn report(shapes: &[Shape; 3]) -> Result<[bool; 4], Failure> {
    let areas = [area(&shapes[0])?, area(&shapes[1])?, area(&shapes[2])?];
    let bounds = [
        bounds(&shapes[0])?,
        bounds(&shapes[1])?,
        bounds(&shapes[2])?,
    ];
    evaluate(areas, bounds)
}
fn evaluate(areas: [f64; 3], bounds: [[f64; 6]; 3]) -> Result<[bool; 4], Failure> {
    let area_gb = (areas[2] - areas[0]).abs() / areas[0] * 100.;
    let area_br = areas[0] / areas[1];
    let green = differences(&bounds[2], &bounds[0]);
    let _red = differences(&bounds[1], &bounds[0]);
    let height = height_difference(bounds);
    let cases = [
        area_gb <= 0.01,
        (2.9..=3.).contains(&area_br),
        green.iter().all(|v| *v <= 0.01),
        height <= 10.,
    ];
    diagnostics(cases, area_gb, area_br, green, height);
    Ok(cases)
}
fn height_difference(bounds: [[f64; 6]; 3]) -> f64 {
    let height_blue = bounds[0][5] - bounds[0][2];
    let height_red = bounds[1][5] - bounds[1][2];
    (height_red - height_blue).abs() / height_blue * 100.
}
fn area(shape: &Shape) -> Result<f64, Failure> {
    Ok(properties(shape, Kind::Area, false)?.mass())
}
fn bounds(shape: &Shape) -> Result<[f64; 6], Failure> {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(shape, &mut bounds, true)?;
    bounds.get()
}
fn differences(first: &[f64; 6], base: &[f64; 6]) -> [f64; 6] {
    std::array::from_fn(|i| (first[i] - base[i]).abs() / base[i] * 100.)
}
fn diagnostics(cases: [bool; 4], area_gb: f64, area_br: f64, green: [f64; 6], height: f64) {
    if !cases[0] {
        println!("delta_area_GB = {area_gb}\nmaxdelta = 0.01");
    }
    result(cases[0], 1);
    if !cases[1] {
        println!("delta_area_BR = {area_br}\ndelta_area_BR = {area_br}");
    }
    result(cases[1], 2);
    boundary_diagnostics(cases, green, height);
}
fn boundary_diagnostics(cases: [bool; 4], green: [f64; 6], height: f64) {
    if !cases[2] {
        coordinate_diagnostics(green);
    }
    result(cases[2], 3);
    if !cases[3] {
        println!("delta_H_RB = {height}\nmaxdelta_H = 10");
    }
    result(cases[3], 4);
}
fn coordinate_diagnostics(green: [f64; 6]) {
    for (axis, value) in ["xmin", "ymin", "zmin", "xmax", "ymax", "zmax"]
        .into_iter()
        .zip(green)
    {
        println!("delta_{axis}_GB = {value}");
    }
    println!("maxdelta = 0.01");
}
fn result(passed: bool, case: usize) {
    println!(
        "BUC60857: {} (case {case})",
        if passed { "OK" } else { "ERROR" }
    );
}
