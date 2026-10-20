//! The diagnostic commands the cases call, on the Rust API; each names its command.
use super::log;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface},
    brep_tools,
    topexp::{self, Explorer},
    topods::{self, Shape},
};
use martensite_tkernel::{ncollection::List, precision, standard::Handle};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    geom::{GeomBSplineCurve, GeomCurve, GeomSurface, GeomTrimmedCurve},
    geom_adaptor::GeomAdaptorCurve,
    top_abs::{Orientation, ShapeEnum, State},
};
use martensite_tkgeomalgo::{
    geom_fill::GeomFillPipe, geom2d_api::InterCurveCurve, int_curve_surface::HInter,
};
use martensite_tkgeombase::{
    gcpnts::{UniformAbscissa, UniformDeflection},
    geom_proj_lib,
};
use martensite_tkmath::{
    geom_abs::Shape as GeomAbsShape,
    gp::{Point2, Point3},
};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire},
    brep_check::BRepCheckAnalyzer,
    brep_class::BRepClassFaceClassifier,
    brep_top_adaptor::BRepTopAdaptorFClass2d,
};

/// `OCC26525 r edge face`: `IntCurveSurface_HInter` of the edge's and the
/// face's adaptors; prints the number of points, then `point r_i x y z` (`%lg`).
pub fn occ26525(r: &str, edge: &Shape, face: &Shape) -> String {
    let edge = topods::edge(edge).expect("OCC26525: an edge");
    let face = topods::face(face).expect("OCC26525: a face");
    let curve: Handle<dyn Adaptor3dCurve> =
        Handle::new(BRepAdaptorCurve::with_edge(&edge).unwrap()).cast();
    let surface: Handle<dyn Adaptor3dSurface> =
        Handle::new(BRepAdaptorSurface::with_face(&face, true).unwrap()).cast();
    let mut inter = HInter::new();
    inter.perform(&curve, &surface).unwrap();
    if !inter.is_done() {
        let text = " intersection is not done\n".to_string();
        log::result(&text);
        return text;
    }
    let nb_points = inter.nb_points().unwrap();
    let mut text = format!(" Number of intersection points found: {nb_points}\n");
    for i in 1..=nb_points {
        let p = inter.point(i).unwrap().pnt();
        text.push_str(&format!(
            "point {r}_{i} {} {} {}  \n",
            lg(p.x()),
            lg(p.y()),
            lg(p.z())
        ));
    }
    log::result(&text);
    text
}

/// C `%lg` (6 significant digits).
fn lg(value: f64) -> String {
    martensite_tkernel::storage::format_general(value, 6)
}

/// `BUC60652 face`: `BRepTools::OuterWire` of the face, set as `w`.
pub fn buc60652(face: &Shape) -> Shape {
    let face = topods::face(face).expect("BUC60652: a face");
    brep_tools::outer_wire(&face).unwrap().into_shape()
}

/// `OCC32744 edge`: for a valid edge, `GCPnts_UniformDeflection` of its 3D
/// curve with deflection 0.001 (the case only needs it to finish within the cpulimit).
pub fn occ32744(edge: &Shape) {
    let edge = topods::edge(edge).expect("OCC32744: an edge");
    let analyzer = BRepCheckAnalyzer::new(&edge.clone().into_shape(), true, false, false).unwrap();
    if analyzer.is_valid().unwrap() {
        let found = BRepTool::curve_with_location(&edge).unwrap();
        let adaptor = GeomAdaptorCurve::with_range(&found.curve, found.first, found.last).unwrap();
        let adaptor: &dyn Adaptor3dCurve = &adaptor;
        let _ = UniformDeflection::with_range(adaptor, 0.001, found.first, found.last, true);
    }
}

/// `OCC27884 curve number_points check`: the polygon (or the curve pieces)
/// through the curve's points made a face; the CPU time of 100 classifications of a
/// point outside its UV box with and without the boxes; prints
/// `Improving time: <percent> %` and returns the percent.
pub fn occ27884(curve: &Handle<dyn GeomCurve>, number_points: f64, check: f64) -> f64 {
    let c = curve.get().unwrap();
    let is_bspline = curve.cast::<GeomBSplineCurve>().get().is_some();
    let a_f = c.first_parameter();
    let mut a_l = c.last_parameter();
    let a_sig = (a_l - a_f) / (number_points - 1.0);
    let edge_points = |a: Point3, b: Point3| {
        BRepBuilderAPIMakeEdge::from_points(a, b)
            .unwrap()
            .edge()
            .unwrap()
            .into_shape()
    };
    let piece = |a_l: f64| {
        if is_bspline {
            let copy = c.copy().unwrap();
            let bspline = copy.cast::<GeomBSplineCurve>();
            bspline
                .get()
                .unwrap()
                .segment(a_l - a_sig, a_l, 1.0e-9)
                .unwrap();
            let copy: Handle<dyn GeomCurve> = bspline.cast();
            BRepBuilderAPIMakeEdge::from_curve(&copy)
                .unwrap()
                .edge()
                .unwrap()
                .into_shape()
        } else {
            let trimmed = GeomTrimmedCurve::new(curve, a_l - a_sig, a_l, true, true).unwrap();
            let trimmed: Handle<dyn GeomCurve> = Handle::new(trimmed).cast();
            BRepBuilderAPIMakeEdge::from_curve(&trimmed)
                .unwrap()
                .edge()
                .unwrap()
                .into_shape()
        }
    };
    let mut edges = List::default();
    let a_p = c.value(a_f).unwrap();
    let mut a_pf = a_p;
    let mut a_pl = a_p;
    let mut i = 1;
    while (i as f64) < number_points {
        a_l = a_f + (i as f64 * a_sig);
        a_pl = c.value(a_l).unwrap();
        let edge = if check == 2.0 {
            let edge = if i % 2 == 1 {
                edge_points(a_pf, a_pl)
            } else {
                piece(a_l)
            };
            a_pf = a_pl;
            edge
        } else if check == 0.0 {
            let edge = edge_points(a_pf, a_pl);
            a_pf = a_pl;
            edge
        } else {
            piece(a_l)
        };
        edges.append(edge);
        i += 1;
    }
    if !c.is_closed().unwrap() {
        edges.append(edge_points(a_pl, a_p));
    }
    let mut wire = BRepBuilderAPIMakeWire::new();
    wire.add_list(&edges).unwrap();
    let face = BRepBuilderAPIMakeFace::from_wire(&wire.wire().unwrap(), false)
        .unwrap()
        .face()
        .unwrap();
    let (u_min, u_max, v_min, v_max) = brep_tools::uv_bounds(&face).unwrap();
    let point = Point2::new(u_min - (u_max + u_min) / 2.0, v_min - (v_max + v_min) / 2.0);
    let tolerance = BRepTool::face_tolerance(&face).unwrap();
    let mut classifier = BRepClassFaceClassifier::new();
    let timed = |classifier: &mut BRepClassFaceClassifier, use_box: bool| {
        let start = cpu_time();
        for _ in 0..100 {
            classifier
                .perform(&face, &point, tolerance, use_box, 0.1)
                .unwrap();
        }
        cpu_time() - start
    };
    let timer1 = timed(&mut classifier, true);
    let timer2 = timed(&mut classifier, false);
    let percent = (timer2 - timer1) / timer2 * 100.0;
    log::result(&format!("Improving time: {} %\n", lg(percent)));
    percent
}

/// `OSD_Timer::UserTimeCPU`: the user CPU time of the process, in seconds.
fn cpu_time() -> f64 {
    martensite_tkernel::osd::Chronometer::get_process_cpu().0
}

/// `OCC299 solid point [tol]`: the BRepClass3d_SolidClassifier state text,
/// as `bclassify` prints it.
pub fn occ299(solid: &Shape, point: Point3) -> String {
    let state = super::distance::bclassify_default(solid, point);
    super::distance::state_text(state).to_string()
}

/// `delta_percent(a, b)`.
fn delta_percent(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        ((a - b) / b).abs() * 100.0
    } else if a != 0.0 {
        ((a - b) / a).abs() * 100.0
    } else {
        0.0
    }
}

/// `OCC367 wire step goodX goodY goodZ percent`: GCPnts_UniformAbscissa
/// with the step along each edge of the wire in order (reversed edges reversed); the
/// last point of the first edge must be within `percent` of the expected point.
pub fn occ367(wire: &Shape, step: f64, good: [f64; 3], percent: f64) -> String {
    let wire = topods::wire(wire).expect("OCC367: a wire");
    let mut explorer = brep_tools::WireExplorer::with_wire(&wire).unwrap();
    let (vw1, vw2) = topexp::wire_vertices(&wire);
    let (ve1, ve2) = topexp::vertices(explorer.current(), false);
    let mut vlast = if vw1.is_same(&ve1) || vw1.is_same(&ve2) {
        vw1
    } else {
        assert!(
            vw2.is_same(&ve1) || vw2.is_same(&ve2),
            "Disconnected vertices"
        );
        vw2
    };
    let mut text = String::new();
    let (mut first_edge, mut delta) = ([0.0; 3], [0.0; 3]);
    let mut status = false;
    let mut edge_index = 0;
    while explorer.more() {
        edge_index += 1;
        text.push_str("\n\n New Edge \n\n");
        let edge = explorer.current().clone();
        let (ve1, ve2) = topexp::vertices(&edge, false);
        let found = BRepTool::curve_with_location(&edge).unwrap();
        let curve = found
            .curve
            .get()
            .unwrap()
            .transformed(found.location.transformation());
        let curve: Handle<dyn GeomCurve> = curve.unwrap().cast();
        let (curve, u1, u2) = if ve1.is_same(&vlast) {
            vlast = ve2;
            (curve, found.first, found.last)
        } else {
            assert!(ve2.is_same(&vlast), "Not the same vertex");
            let c = curve.get().unwrap();
            let (u1, u2) = (
                c.reversed_parameter(found.first).unwrap(),
                c.reversed_parameter(found.last).unwrap(),
            );
            vlast = ve1;
            (c.reversed().unwrap(), u1, u2)
        };
        let adaptor = GeomAdaptorCurve::with_curve(&curve).unwrap();
        let algo = UniformAbscissa::with_abscissa_range(&adaptor, step, u1, u2, -1.0).unwrap();
        if !algo.is_done() {
            text.push_str("Not Done!!!\n");
        }
        let max_index = algo.nb_points().unwrap_or(0);
        for index in 1..=max_index {
            let t = algo.parameter(index).unwrap();
            let p = adaptor.value(t).unwrap();
            text.push_str(&format!("Parameter t = {}\n", lg(t)));
            text.push_str(&format!(
                "Value Pnt = {} {} {}\n",
                lg(p.x()),
                lg(p.y()),
                lg(p.z())
            ));
            if edge_index == 1 && index == max_index {
                first_edge = [p.x(), p.y(), p.z()];
                delta = [0, 1, 2].map(|k| delta_percent(first_edge[k], good[k]));
                status = delta.iter().all(|d| *d <= percent);
            }
        }
        explorer.next().unwrap();
    }
    text.push_str(&format!(
        "\n\nFirstEdge = {} {} {}\ndeltaX = {} deltaY = {} deltaZ = {}\n",
        lg(first_edge[0]),
        lg(first_edge[1]),
        lg(first_edge[2]),
        lg(delta[0]),
        lg(delta[1]),
        lg(delta[2])
    ));
    text.push_str(if status {
        "OCC367 : OK\n"
    } else {
        "OCC367 : ERROR\n"
    });
    log::result(&text);
    text
}

/// `OCC8169 edge1 edge2 plane`: the edge curves projected on the face's
/// plane (GeomProjLib::Curve2d) and intersected (Geom2dAPI_InterCurveCurve, Confusion);
/// a segment whose first point differs on the two curves by more than Confusion prints
/// `OCC8169 Faulty`.
pub fn occ8169(e1: &Shape, e2: &Shape, face: &Shape) -> String {
    let face = topods::face(face).expect("OCC8169: a face");
    let plane = BRepTool::surface(&face).unwrap();
    let confusion = precision::confusion();
    let pcurve = |edge: &Shape| {
        let edge = topods::edge(edge).expect("OCC8169: an edge");
        let found = BRepTool::curve_with_location(&edge).unwrap();
        let curve = found
            .curve
            .get()
            .unwrap()
            .transformed(found.location.transformation());
        let curve: Handle<dyn GeomCurve> = curve.unwrap().cast();
        geom_proj_lib::curve2d_range(&curve, (found.first, found.last), &plane).unwrap()
    };
    let (c1, c2) = (pcurve(e1), pcurve(e2));
    let inter = InterCurveCurve::between(&c1, &c2, confusion).unwrap();
    let nb_points = inter.nb_points();
    let mut text = format!("NbPoints = {nb_points}\n");
    for i in 1..=nb_points {
        let p = inter.point(i).unwrap();
        text.push_str(&format!(
            "Point.X({i}) = {}   Point.Y({i}) = {}\n",
            lg(p.x()),
            lg(p.y())
        ));
    }
    let nb_segments = inter.nb_segments();
    text.push_str(&format!("\nNbSegments = {nb_segments}\n"));
    if nb_segments > 0 {
        let segment = inter.intersector().segment(1).unwrap();
        let first = segment.first_point().unwrap();
        let p1 = c1.get().unwrap().value(first.param_on_first()).unwrap();
        let p2 = c2.get().unwrap().value(first.param_on_second()).unwrap();
        let dist = p1.distance(p2);
        text.push_str(&format!(
            "aP1.X() = {}   aP1.Y() = {}\naP2.X() = {}   aP2.Y() = {}\nDistance = {}\nConfusion = {}\n",
            lg(p1.x()),
            lg(p1.y()),
            lg(p2.x()),
            lg(p2.y()),
            lg(dist),
            lg(confusion)
        ));
        text.push_str(if dist > confusion {
            "\nOCC8169 Faulty\n"
        } else {
            "\nOCC8169 OK\n"
        });
    } else {
        text.push_str("\nOCC8169 OK\n");
    }
    log::result(&text);
    text
}

/// `OCC9 result path cur1 cur2 radius [tolerance]`: GeomFill_Pipe of the
/// path and the two rails with the radius, `Perform(tol, true)` with a tolerance, else
/// `Perform(true)`; `Err` is the command's `GeomFill_Pipe cannot make a surface`.
pub fn pipe_occ9(
    path: &Handle<dyn GeomCurve>,
    rails: [&Handle<dyn GeomCurve>; 2],
    radius: f64,
    tolerance: Option<f64>,
) -> Result<Handle<dyn GeomSurface>, String> {
    let mut pipe = GeomFillPipe::with_rails(path, rails[0], rails[1], radius).unwrap();
    match tolerance {
        Some(tol) => pipe.perform(tol, true, GeomAbsShape::C1, 11, 30).unwrap(),
        None => pipe.perform_with_parameters(true, false).unwrap(),
    }
    if !pipe.is_done() {
        log::result("GeomFill_Pipe cannot make a surface\n");
        return Err("GeomFill_Pipe cannot make a surface".into());
    }
    Ok(pipe.surface())
}

/// `OCC377 file x y precuv`: for each wire of the face (FORWARD), the
/// BRepTopAdaptor_FClass2d states of the infinite point and of the point on a face made
/// of the wire and of the reversed wire; the point states must agree.
pub fn occ377(shape: &Shape, point: Point2, precuv: f64) -> String {
    let mut text = String::new();
    let state_word = |state: State| match state {
        State::In => "IN",
        State::Out => "OUT",
        State::On => "ON",
        _ => "UNKNOWN",
    };
    let forward = shape.oriented(Orientation::Forward);
    let builder = BRepBuilder::default();
    let mut explorer = Explorer::new(&forward, ShapeEnum::Wire, ShapeEnum::Shape).unwrap();
    let mut i = 1;
    let classify = |wire: &Shape| {
        let mut new_face = shape.empty_copied().unwrap();
        new_face.set_orientation(Orientation::Forward);
        builder.add(&mut new_face, wire).unwrap();
        let classifier =
            BRepTopAdaptorFClass2d::new(&topods::face(&new_face).unwrap(), precuv).unwrap();
        let infinite = classifier.perform_infinite_point().unwrap();
        let at = classifier.perform(point, true).unwrap();
        (infinite, at)
    };
    while explorer.more() {
        let wire = explorer.current().clone();
        let (inf1, stat1) = classify(&wire);
        text.push_str(&format!(
            "Wire {i}: Infinite point is {}\n",
            state_word(inf1)
        ));
        text.push_str(&format!(
            "Wire {i}: point ( {}, {} ) is {}\n",
            lg(point.x()),
            lg(point.y()),
            state_word(stat1)
        ));
        let reversed = wire.oriented(martensite_tkg3d::top_abs::reverse(wire.orientation()));
        let (inf2, stat2) = classify(&reversed);
        text.push_str(&format!(
            "Reversed Wire {i}: Infinite point is {}\n",
            state_word(inf2)
        ));
        text.push_str(&format!(
            "Reversed Wire {i}: point ( {}, {} ) is {}\n",
            lg(point.x()),
            lg(point.y()),
            state_word(stat2)
        ));
        if stat1 == stat2 {
            text.push_str("OCC377 OK\n");
        } else {
            text.push_str("OCC377 FAULTY\n");
            break;
        }
        explorer.next().unwrap();
        i += 1;
    }
    log::result(&text);
    text
}
