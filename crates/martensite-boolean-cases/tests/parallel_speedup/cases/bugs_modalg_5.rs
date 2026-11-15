//! The unchanged 75-edge grid and 20% parallel-speedup requirement (#480).
use crate::measurement::{filler_seconds, meets_requirement};
use crate::support::{
    bugs_commands::options_bop::ScriptBop,
    commands::{compound, ttranslate},
    construction::{edge, nurbsconvert, tcopy, vertex},
    session::Session,
};

fn edge_grid(
    s: &mut dyn Session,
    axis: char,
    end: [f64; 3],
    offset: impl Fn(f64, f64) -> [f64; 3],
) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", end);
    let prototype = format!("e{axis}");
    edge(s, &prototype, "v1", "v2");
    let names = translated_grid(s, axis, &prototype, offset);
    let borrowed: Vec<&str> = names.iter().map(String::as_str).collect();
    compound(s, &borrowed, &format!("b{axis}"));
}

fn translated_grid(
    s: &mut dyn Session,
    axis: char,
    prototype: &str,
    offset: impl Fn(f64, f64) -> [f64; 3],
) -> Vec<String> {
    let mut names = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            let at = offset(i as f64 + 1., j as f64 + 1.);
            let name = format!("e{axis}_{i}_{{{j}}}");
            tcopy(s, prototype, &name);
            ttranslate(s, &name, at);
            names.push(name);
        }
    }
    names
}

fn grid_arguments(s: &mut dyn Session) -> ScriptBop {
    let side = 5. + 1.;
    edge_grid(s, 'z', [0., 0., side], |i, j| [i, j, 0.]);
    edge_grid(s, 'x', [side, 0., 0.], |i, j| [0., i, j]);
    edge_grid(s, 'y', [0., side, 0.], |i, j| [i, 0., j]);
    for name in ["bx", "by", "bz"] {
        nurbsconvert(s, name, name);
    }
    let mut bop = ScriptBop::default();
    for name in ["bx", "by", "bz"] {
        bop.add_compound(s, name);
    }
    bop
}

/// Requires the parallel filler to finish within 80% of the sequential elapsed time.
pub fn bug24639(s: &mut dyn Session) {
    println!("=========\nOCC24639\n=========\n");
    let mut bop = grid_arguments(s);
    bop.options.run_parallel = false;
    let sequential = filler_seconds(&mut bop);
    bop.options.run_parallel = true;
    let parallel = filler_seconds(&mut bop);
    if !meets_requirement(sequential, parallel) {
        println!("Error: Parallel execution does not give expected performance speed up");
    }
}
