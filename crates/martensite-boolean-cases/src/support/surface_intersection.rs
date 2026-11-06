//! The commands of the `lowalgos/intss` scripts: `intersect`, `xdistcs` and the overlap check.
use super::session::Session;
use martensite_tkbrep::topods::{Edge, Shape};
use martensite_tkernel::{
    precision,
    standard::Handle,
    storage::{IoStream, format_general},
};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkgeomalgo::{
    geom_api::ProjectPointOnSurf,
    geom_int::{GeomIntIntSS, GeomIntIntSSApprox},
};
use martensite_tkgeombase::{extrema::ExtAlgo, geom_tools};
use martensite_tkshhealing::shape_analysis::ShapeAnalysisEdge;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

/// A surface and a curve are not shapes, so a script's surfaces live beside the session.
pub type Surface = Handle<dyn GeomSurface>;
/// A curve that `intersect` returned.
pub type Curve = Handle<dyn GeomCurve>;

/// `restore file name` of a `GeometrySurface` file; `read_draw_surface(bytes)`.
pub fn read_draw_surface(bytes: &[u8]) -> Surface {
    let end = bytes.iter().position(u8::is_ascii_whitespace).unwrap();
    let header = String::from_utf8_lossy(&bytes[..end]);
    assert_eq!(
        header, "DrawTrSurf_Surface",
        "restore: expected a surface file"
    );
    let surface = geom_tools::read_surface(&mut IoStream::from_bytes(bytes[end..].to_vec()));
    assert!(!surface.is_null(), "restore: expected a surface");
    surface
}

/// `intersect res s1 s2` at its defaults; `None` is the script's caught failure.
/// Example: `intersect(&first, &second).unwrap().len()`.
pub fn intersect(first: &Surface, second: &Surface) -> Option<Vec<Curve>> {
    let tolerance = precision::confusion();
    let approximation = GeomIntIntSSApprox::default();
    let operation = GeomIntIntSS::with_surfaces(first, second, tolerance, approximation).ok()?;
    if !operation.is_done() {
        return None;
    }
    Some(
        (1..=operation.nb_lines())
            .map(|index| operation.line(index).unwrap())
            .collect(),
    )
}

fn sample_distance(
    projection: &mut ProjectPointOnSurf,
    surface: &Surface,
    curve: &Curve,
    at: f64,
) -> f64 {
    let point = curve.get().unwrap().d0(at).unwrap();
    projection
        .init_tolerance(point, surface, 1.0e-7, ExtAlgo::Grad)
        .unwrap();
    assert!(projection.is_done(), "xdistcs: projection must finish");
    projection.lower_distance().unwrap()
}

/// `xdistcs curve surface first last samples error_limit`: one line per sample, flagged above
/// the limit, then the largest distance and its parameter.
/// Example: `xdistcs("res_1", &curve, &surface, [0., 1.], 100, 2.0e-7)`.
pub fn xdistcs(
    name: &str,
    curve: &Curve,
    surface: &Surface,
    range: [f64; 2],
    samples: usize,
    limit: f64,
) {
    let mut projection = ProjectPointOnSurf::default();
    let step = (range[1] - range[0]) / (samples - 1) as f64;
    let (mut largest, mut at_largest) = (0.0, 0.0);
    for index in 0..samples {
        let at = if index == samples - 1 {
            range[1]
        } else {
            range[0] + index as f64 * step
        };
        let distance = sample_distance(&mut projection, surface, curve, at);
        let flag = if distance > limit {
            format!("Error in {name}:")
        } else {
            String::new()
        };
        println!(
            "{flag} T={}\tD={}",
            format_general(at, 6),
            format_general(distance, 6)
        );
        if distance > largest {
            (largest, at_largest) = (distance, at);
        }
    }
    println!("Max distance = {}", format_general(largest, 17));
    println!("Param = {}", format_general(at_largest, 17));
}

/// `mkedge name curve`, bound and returned; `edge_of(s, "ee", &curve)`.
pub fn edge_of(session: &mut dyn Session, name: &str, curve: &Curve) -> Edge {
    let edge = BRepBuilderAPIMakeEdge::from_curve(curve)
        .unwrap()
        .edge()
        .unwrap();
    session.bind(name, &edge.clone().into_shape());
    edge
}

/// `checkoverlapedges e1 e2 tolerance`: whether the command finds the edges overlapping.
/// Example: `edges_overlap(&first, &second, 5.0e-5)`.
pub fn edges_overlap(first: &Edge, second: &Edge, tolerance: f64) -> bool {
    let mut tolerance = tolerance;
    ShapeAnalysisEdge::new()
        .check_overlapping(first, second, &mut tolerance, 0.)
        .unwrap()
}

/// The shapes of `names` for a Boolean's arguments; `names_of(&["ee"])`.
pub fn named(session: &dyn Session, name: &str) -> Shape {
    session
        .shape(name)
        .unwrap_or_else(|| panic!("{name}: no shape"))
        .clone()
}
