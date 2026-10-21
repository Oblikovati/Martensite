//! `bugs_moddata_1` case bodies.
use crate::cases::script::{banner, captured};
use crate::cases::script_words::words;
use crate::support::commands::checks::{checklength, checkreal};
use crate::support::commands::creation::{bsplinecurve2d, circle2d, ellipse2d, offset2d};
use crate::support::commands::extrema::extrema;
use crate::support::commands::geometry::to3d;
use crate::support::commands::planar::extrema2d;
use crate::support::commands::qa::buc60843;
use crate::support::commands::views::checkview_display;
use crate::support::numbers::script_real;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

/// The iterative tangent line is infinite (`checklength` of 4e100).
fn infinite_tangent(d: &mut Script, curves: [&str; 2], start: &[f64]) -> ScriptStep {
    buc60843(d, "result", curves, start)?;
    checklength(d, "result", "4.0000000000000001e+100", None)
}

pub fn buc60843_1(d: &mut Script) -> ScriptStep {
    banner(d, "========", "BUC60843");
    circle2d(d, "test_circle", [0., 0., 50.])?;
    ellipse2d(d, "test_ellipse", [460., 0., 150., 100.])?;
    infinite_tangent(d, ["test_circle", "test_ellipse"], &[0.1])
}

pub fn buc60843_2(d: &mut Script) -> ScriptStep {
    banner(d, "========", "BUC60843");
    ellipse2d(d, "test_ellipse_1", [0., 0., 50., 50.])?;
    ellipse2d(d, "test_ellipse_2", [460., 0., 150., 100.])?;
    infinite_tangent(d, ["test_ellipse_1", "test_ellipse_2"], &[1.0, 1.0])
}

pub fn buc60890(d: &mut Script) -> ScriptStep {
    banner(d, "========", "BUC60890");
    // `eval 2dbsplinecurve curve_1 1 [llength $knots] [join $knots] [join $poles]`.
    let poles = [
        [0., 0., 1.],
        [1., 0., 1.],
        [1.3, 1., 1.],
        [2., 1., 1.],
        [2., 0., 1.],
        [3., 0., 1.],
        [3.5, 1., 1.],
        [4., 1., 1.],
        [4., 0., 1.],
        [5., 0., 1.],
        [5., 1., 1.],
        [6., 1., 1.],
        [6., 0., 1.],
        [7., 0., 1.],
        [7., 1., 1.],
        [8., 1., 1.],
        [8., 0., 1.],
        [9., 0., 1.],
    ];
    let knots: Vec<[f64; 2]> = (0..18)
        .map(|k| [k as f64, if k == 0 || k == 17 { 2. } else { 1. }])
        .collect();
    let mut words_in = vec![1., knots.len() as f64];
    words_in.extend(knots.iter().flatten());
    words_in.extend(poles.iter().flatten());
    bsplinecurve2d(d, "curve_1", &words_in)?;
    circle2d(d, "curve_2", [5.3, -0.5, 2.])?;
    if words(&extrema2d(d, "curve_1", "curve_2")?).len() == 8 {
        d.puts("BUC60890 OK: Function 2dextrema works properly.");
    } else {
        d.puts("Faulty BUC60890 : Function 2dextrema works wrongly.");
    }
    Ok(())
}

pub fn bug103(d: &mut Script) -> ScriptStep {
    d.puts("TODO OCC12345 ALL: An exception was caught");
    d.puts("");
    d.puts("================");
    d.puts("BUC60961");
    d.puts("OCC103");
    d.puts("================");
    d.puts("");
    circle2d(d, "c", [0., 0., 10.])?;
    to3d(d, "c1", "c")?;
    offset2d(d, "o", "c", -2.)?;
    // `catch {to3d result o}`: the caught failure is still logged.
    let _ = to3d(d, "result", "o");
    checkview_display(d, "result", "bug103")
}

pub fn buc60967(d: &mut Script) -> ScriptStep {
    banner(d, "========", "BUC60967");
    restore(d, "buc60967.draw", "bs2")?;
    to3d(d, "result", "bs2")?;
    checkview_display(d, "result", "buc60967")
}

pub fn buc60968(d: &mut Script) -> ScriptStep {
    d.puts("========================");
    d.puts("BUC60968");
    d.puts("========================");
    restore(d, "buc60968-c.draw", "c")?;
    restore(d, "buc60968-s1.draw", "s1")?;
    let che = extrema(d, "c", "s1")?;
    if words(&che).is_empty() {
        d.puts("Error : command extrema does NOT work properly");
    }
    Ok(())
}

pub fn bug22241(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC22241");
    restore(d, "bug22241_c1.draw", "c1")?;
    restore(d, "bug22241_c2.draw", "c2")?;
    let result = extrema(d, "c1", "c2")?;
    if words(&result).len() < 8 {
        d.puts("OCC22241: Faulty");
        return Ok(());
    }
    // regexp {Extrema 1 is point : +([-0-9.+eE]+) +([-0-9.+eE]+) +([-0-9.+eE]+)}
    let point = result.split("Extrema 1 is point : ").nth(1);
    let mut coordinates = point.unwrap_or("").split_whitespace();
    let x = captured(coordinates.next(), "X")?;
    let y = captured(coordinates.next(), "Y")?;
    let z = captured(coordinates.next(), "Z")?;
    checkreal(d, "X", script_real(x), 1., 0., 0.001);
    checkreal(d, "Y", script_real(y), 3.0891916307971289e-33, 0., 0.001);
    checkreal(d, "Z", script_real(z), 0.059999999999999998, 0., 0.001);
    Ok(())
}
