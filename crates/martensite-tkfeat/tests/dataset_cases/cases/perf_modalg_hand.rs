//! `perf/modalg/bug26929`: every edge of `aE` splits `aF`, under the script's own CPU budget.
use crate::support::{
    checkshape::checkshape_named, commands::explode, feat_script::FeatScript, restore::restore,
    splitshape::splitshape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::time::Instant;

pub fn bug26929(s: &mut FeatScript<'_>) {
    println!("============");
    println!("26929");
    println!("============");
    println!();
    println!("CPU and elapsed time limit set to 100 seconds");
    restore(s, "OCC26629-face.brep", "aF");
    restore(s, "OCC26629-edge.brep", "aE");
    let started = Instant::now();
    let edges = explode(s, "aE", Some(ShapeEnum::Edge));
    let mut args: Vec<&str> = Vec::with_capacity(edges.len() * 2);
    for edge in &edges {
        args.extend(["aF", edge.as_str()]);
    }
    splitshape(s, "result", "aF", &args, &[]);
    println!("COUNTER splitshape: {}", started.elapsed().as_secs_f64());
    checkshape_named(s, "result");
}
