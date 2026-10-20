//! `bugs_modalg_5` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    commands::{dump_surface, explode, length2d, mkcurve, mksurface, offset_surface, project},
    printed::g17,
    restore::restore_as,
    session::{Session, ViewStep, set, set_exploded},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkgeombase::geom_convert::ApproxSurface;
use martensite_tkmath::geom_abs::Shape as Continuity;

/// 100 threads approximate the same surface with
/// GeomConvert_ApproxSurface(S, 1e-4, C1, C1, 9, 9, 100, 1); each must give
/// `NbUPoles` poles in U.
pub fn bug23952_1(s: &mut dyn Session) {
    let a = restore_as(s, "bug23952_trim-torus.draw", "a").surface();
    let nb_u_poles = 18;
    std::thread::scope(|scope| {
        let threads: Vec<_> = (0..100)
            .map(|_| scope.spawn(|| approx_u_poles(&a)))
            .collect();
        for (i, thread) in threads.into_iter().enumerate() {
            let poles = thread.join().expect("Error: Failed waiting for thread");
            assert_eq!(
                poles, nb_u_poles,
                "Error: wrong number of poles in thread {i}!"
            );
        }
    });
}

fn approx_u_poles(surface: &Handle<dyn GeomSurface>) -> i32 {
    let continuities = (Continuity::C1, Continuity::C1);
    let approx = ApproxSurface::from_surface(surface, 1e-4, continuities, (9, 9), 100, 1);
    let approx = approx.expect("GeomConvert_ApproxSurface");
    assert!(approx.is_done(), "Error: ApproxSurface is not done!");
    let surface = approx.surface();
    let surface = surface
        .get()
        .expect("Error: BSplineSurface is not created!");
    surface.nb_u_poles()
}

/// `offset o a 20` of the C0 surface must be built (no continuity check
/// rejects it) and its dump after the 39th character must differ from the basis dump;
/// then `donly a o`, `fit` and the screenshot.
fn offset_differs(s: &mut dyn Session, file: &str, names: (&str, &str), image: &str) {
    s.step(ViewStep::SmallView("AXON"));
    let a = restore_as(s, file, names.0).surface();
    let o = set(
        s,
        names.1,
        offset_surface(&a, 20.).expect("offset of the C0 surface"),
    );
    let tail = |text: String| text.chars().skip(39).collect::<String>();
    let bug_info_1 = tail(dump_surface(names.0, &a));
    let bug_info_2 = tail(dump_surface(names.1, &o));
    assert_ne!(bug_info_1, bug_info_2, "ERROR: OCC25124 is reproduced");
    s.step(ViewStep::Donly(&[names.0, names.1]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd(image));
}

pub fn bug25124_6(s: &mut dyn Session) {
    offset_differs(s, "OCC25124_a_2999.draw", ("a1", "o1"), "bug25124_6");
}

pub fn bug25124_7(s: &mut dyn Session) {
    offset_differs(s, "OCC25124_a_15592.draw", ("a2", "o2"), "bug25124_7");
}

/// The projections of edges 3 and 5 on the face surface have the length
/// whose first seven printed characters are `1.57079`.
pub fn bug25886(s: &mut dyn Session) {
    s.step(ViewStep::SmallView("-2D-"));
    let cx = restore_as(s, "OCC25886_cx.brep", "cx").shape();
    let cx = set_exploded(s, "cx", explode(&cx));
    let f = set(s, "f", cx[0].clone());
    let surface = set(s, "s", mksurface(&f));
    let c3 = set(s, "c3", mkcurve(&cx[2]));
    let c5 = set(s, "c5", mkcurve(&cx[4]));
    for (name, curve) in [("c3x", c3), ("c5x", c5)] {
        let projected = project(&curve, &surface, precision::confusion());
        let projected = set(s, name, projected.expect("Projection Failed"));
        let printed = g17(length2d(&projected, precision::confusion()));
        let length: String = printed.chars().take(7).collect();
        assert_eq!(length, "1.57079", "ERROR: OCC25866 is reproduced");
    }
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug25886"));
}

/// `catch {project x c s}` must not fail.
pub fn bug25980(s: &mut dyn Session) {
    let c = restore_as(s, "c2t.draw", "c").curve();
    let surface = restore_as(s, "s1t.draw", "s").surface();
    let x = project(&c, &surface, precision::confusion());
    set(s, "x", x.expect("ERROR: OCC25980 is reproduced."));
}
