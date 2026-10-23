//! Intersection, distance, bounds and projection helpers for geometric regression cases.
//! Surface pairs and curve/surface pairs use their respective intersection builders.
use super::printed::g17;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkernel::{precision, standard::Handle, storage::format_general};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkgeomalgo::geom_api::{IntCS, ProjectPointOnSurf};
use martensite_tkgeomalgo::geom_int::{GeomIntIntSS, GeomIntIntSSApprox};
use martensite_tkgeombase::extrema::{ExtAlgo, ExtFlag};
use martensite_tkmath::gp::Point3;

/// `intersect result s1 s2`: the intersection curves, `Err("No intersections found!")`
/// when the intersector is not done.
pub fn intersect_surfaces(
    s1: &Handle<dyn GeomSurface>,
    s2: &Handle<dyn GeomSurface>,
) -> Result<Vec<Handle<dyn GeomCurve>>, String> {
    let mut inters = GeomIntIntSS::new();
    inters
        .perform(
            s1,
            s2,
            precision::confusion(),
            GeomIntIntSSApprox::default(),
        )
        .map_err(|failure| format!("{failure:?}"))?;
    if !inters.is_done() {
        return Err("No intersections found!\n".into());
    }
    (1..=inters.nb_lines())
        .map(|i| inters.line(i).map_err(|failure| format!("{failure:?}")))
        .collect()
}

/// `intersect result c s`: the intersection points of the curve and the surface (the
/// segments, which none of the cases gets, are refused).
pub fn intersect_curve_surface(
    c: &Handle<dyn GeomCurve>,
    s: &Handle<dyn GeomSurface>,
) -> Result<Vec<Point3>, String> {
    let inters = IntCS::with_curve(c, s).map_err(|failure| format!("{failure:?}"))?;
    if !inters.is_done() {
        return Err("No intersections found!\n".into());
    }
    let segments = inters.nb_segments().unwrap();
    assert_eq!(
        segments, 0,
        "intersect: {segments} segments; expected only points"
    );
    (1..=inters.nb_points().unwrap())
        .map(|i| inters.point(i).map_err(|failure| format!("{failure:?}")))
        .collect()
}

/// `bounds c u1 u2`: the first and last parameters of the curve.
pub fn bounds(c: &Handle<dyn GeomCurve>) -> (f64, f64) {
    let curve = c.get().expect("bounds: a curve");
    (curve.first_parameter(), curve.last_parameter())
}

/// `xdistcs curve surface t1 t2 nbpoints tol`: the distance from `nbpoints` curve points
/// to the surface (ProjectPointOnSurf, Grad), `Error in name:` before each one
/// above `tol`; returns the text and the largest distance.
pub fn xdistcs(
    name: &str,
    c: &Handle<dyn GeomCurve>,
    s: &Handle<dyn GeomSurface>,
    (t1, t2): (f64, f64),
    count: usize,
    tolerance: f64,
) -> (String, f64) {
    let curve = c.get().expect("xdistcs: a curve");
    let mut projection = ProjectPointOnSurf::new();
    let (mut text, mut max_distance, mut max_param) = (String::new(), 0., 0.);
    let step = (t2 - t1) / (count as f64 - 1.);
    for i in 0..count {
        let t = if i == count - 1 {
            t2
        } else {
            t1 + i as f64 * step
        };
        let p = curve.d0(t).unwrap();
        projection
            .init_tolerance(p, s, 1.0e-7, ExtAlgo::Grad)
            .unwrap();
        if !projection.is_done() {
            text.push_str("Error: GeomAPI_ProjectPointOnSurf failed\n");
            return (text, max_distance);
        }
        let distance = projection.lower_distance().unwrap();
        if distance > tolerance {
            text.push_str(&format!("Error in {name}:"));
        }
        let g = |v| format_general(v, 6);
        text.push_str(&format!(" T={}\tD={}\n", g(t), g(distance)));
        if distance > max_distance {
            (max_param, max_distance) = (t, distance);
        }
    }
    text.push_str(&format!(
        "Max distance = {}\nParam = {}\n",
        g17(max_distance),
        g17(max_param)
    ));
    (text, max_distance)
}

/// `projponf face point`: the point projected on the face's surface bounds (Grad,
/// MINMAX); the command's printout.
pub fn projponf(face: &Shape, point: Point3) -> String {
    let face = topods::face(face).expect("projponf: a face");
    let (surface, location) = BRepTool::surface_with_location(&face).unwrap();
    let local = point.transformed(location.inverted().unwrap().transformation());
    let bounds = surface.get().unwrap().bounds();
    let mut projection = ProjectPointOnSurf::new();
    projection
        .init_surface(&surface, bounds, ExtAlgo::Grad)
        .unwrap();
    projection.set_extrema_algo(ExtAlgo::Grad);
    projection.set_extrema_flag(ExtFlag::MinMax);
    projection.perform(local).unwrap();
    if projection.nb_points().unwrap() == 0 {
        return if projection.is_done() {
            "no projection found\n".into()
        } else {
            "projection failed\n".into()
        };
    }
    let (u, v) = projection.lower_distance_parameters().unwrap();
    let nearest = projection
        .nearest_point()
        .unwrap()
        .transformed(location.transformation());
    format!(
        "proj dist = {}\nuvproj = {} {}\npproj = {} {} {}\n",
        g17(projection.lower_distance().unwrap()),
        g17(u),
        g17(v),
        g17(nearest.x()),
        g17(nearest.y()),
        g17(nearest.z())
    )
}
