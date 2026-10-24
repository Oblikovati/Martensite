//! The heal cases on surfaces, p-curves and their 2D views.
use super::banner;
use crate::support::{
    checkshape::CheckOptions,
    checkshape_structural::checkshape_structural,
    commands::{explode_type, mksurface, pcurve},
    log,
    logged_checks::checknbshapes_some,
    props::{Prop, props_logged},
    restore::{Restored, binrestore, restore_as},
    session::{Session, ViewStep, set, set_exploded},
    swdraw::{fixshape, same_parameter_fix},
    topology::mkface,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkshhealing::shape_custom::{
    ShapeCustom, ShapeCustomRestrictionParameters, ShapeCustomRestrictionRequest,
};

/// `bsplres result shape tol3d tol2d maxdeg maxseg C1 C2 0 1`:
/// ShapeCustom::BSplineRestriction, then ShapeFix::same_parameter of a modified result.
fn bsplres_c1_c2(shape: &martensite_tkbrep::topods::Shape) -> martensite_tkbrep::topods::Shape {
    let request = ShapeCustomRestrictionRequest {
        tol_3d: 1e-4,
        tol_2d: 1e-9,
        max_degree: 15,
        max_nb_segment: 100,
        continuity_3d: Continuity::C1,
        continuity_2d: Continuity::C2,
        degree: false,
        rational: true,
        parameters: Handle::new(ShapeCustomRestrictionParameters::new()),
    };
    let result = ShapeCustom::bspline_restriction(shape, &request)
        .unwrap_or_else(|failure| panic!("bsplres: {failure:?}"));
    assert!(!result.is_null(), "bsplres: NO RESULT");
    if result == *shape {
        log::result("NO MODIFICATIONS\n");
        return result;
    }
    same_parameter_fix(&result);
    result
}

/// GeometryTest `getsurfcontinuity name`: `name has <C0..CN> continuity.`.
fn getsurfcontinuity(name: &str, continuity: Continuity) -> String {
    let word = match continuity {
        Continuity::C0 => "C0",
        Continuity::G1 => "G1",
        Continuity::C1 => "C1",
        Continuity::G2 => "G2",
        Continuity::C2 => "C2",
        Continuity::C3 => "C3",
        Continuity::CN => "CN",
    };
    format!("{name} has {word} continuity.\n")
}

/// heal — bug23944.
/// The B-spline restriction of the face keeps its surface C0 (the restriction
/// parameters are read, not typo-swapped).
pub fn bug23944(s: &mut dyn Session) {
    banner(&["============", "OCC23944", "============", ""]);
    let surface = restore_as(s, "bug23944_s.draw", "s").surface();
    let f = set(s, "f", mkface(&surface));
    let result = set(s, "result", bsplres_c1_c2(&f));
    let rsurf = mksurface(&result);
    s.set("rsurf", &Restored::Surface(rsurf.clone()));
    let info = getsurfcontinuity("rsurf", rsurf.get().unwrap().continuity());
    log::result(&info);
    if !info.contains("rsurf has C0 continuity.") {
        log::puts("Faulty OCC23944");
    }
}

/// heal — bug25967 (`TODO OCC25967 ALL: Error :  is WRONG because number of EDGE
/// entities in shape`): the healed face's p-curves shown in a 2D view.
pub fn bug25967(s: &mut dyn Session) {
    log::todo("Error :  is WRONG because number of EDGE entities in shape");
    banner(&["========", "OCC25967", "========", ""]);
    let f = restore_as(s, "bug25967_face-no-sh-fwd.brep", "f").shape();
    let result = set(s, "result", fixshape(&f, &[]));
    for (index, curve) in pcurve(&result).into_iter().enumerate() {
        s.set(&format!("result_{}", index + 1), &Restored::Curve2d(curve));
    }
    let mut edges = [None; 9];
    edges[1] = Some(4);
    checknbshapes_some(&result, edges);
    s.step(ViewStep::V2d);
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug25967"));
}

/// heal — bug28523 (`TODO OCC28523 ALL: Error: Bad area of face`).
pub fn bug28523(s: &mut dyn Session) {
    log::todo("Error: Bad area of face");
    banner(&["========", "OCC28523", "========", ""]);
    let f2 = restore_as(s, "bug28377_problemface.brep", "f2").shape();
    let newface = set(s, "newface", fixshape(&f2, &[]));
    checkshape_structural(&newface, "newface", "ff1", CheckOptions::default());
    let area = props_logged(Prop::Area, &newface, None).read_mass();
    if area <= 0. {
        log::puts("Error: Bad area of face");
    }
}

/// heal — bug28768: the healed edges shown before and after, two screenshots.
pub fn bug28768(s: &mut dyn Session) {
    banner(&["========", "OCC28768", "========", ""]);
    let f12g = set(s, "f12g", binrestore("bug28768_f12g.brep"));
    let r12g = set(s, "r12g", fixshape(&f12g, &[]));
    set_exploded(s, "r12g", explode_type(&r12g, ShapeEnum::Edge));
    s.step(ViewStep::SmallView("+X+Y"));
    s.step(ViewStep::Clear);
    for name in ["r12g_4", "r12g_6"] {
        s.step(ViewStep::Display(&[name]));
    }
    s.step(ViewStep::Fit);
    s.step(ViewStep::Display(&["r12g_5"]));
    s.step(ViewStep::Xwd("bug28768_should_be"));
    s.step(ViewStep::Clear);
    for name in ["r12g_4", "r12g_6"] {
        s.step(ViewStep::Display(&[name]));
    }
    s.step(ViewStep::Fit);
    for name in ["r12g_7", "r12g_3"] {
        s.step(ViewStep::Display(&[name]));
    }
    s.step(ViewStep::Xwd("bug28768_is"));
}
