//! The projection cases of `bugs_moddata_3`: `bug25504`, `bug25660`, `bug25662`,
//! `bug25892_*`, `bug30272` and `bug5145`.
use crate::support::{
    analysis::{dump_nb_poles, dump_pole, gproject_errors},
    checks::{checklength2d, checkreal},
    commands::{dump_curve2d, mksurface, project},
    restore::restore_as,
    session::{Session, ViewStep, set},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomTrimmedCurve};
use martensite_tkmath::geom_abs::Shape as Continuity;
use std::f64::consts::PI;

/// The projection, outside any catch, must succeed.
pub fn bug25504(s: &mut dyn Session) {
    let surface = restore_as(s, "bug25504_surf.draw", "s").surface();
    let c = restore_as(s, "bug25504_curve.draw", "c").curve();
    let c2d = project(&c, &surface, precision::confusion());
    set(s, "c2d", c2d.expect("project c2d c s"));
}

/// The V coordinates of the projected curve's poles, read from
/// its dump: first and last by `checkreal`, the sequence increasing (25660) or not
/// decreasing by more than `tol_abs` (25662).
fn projected_poles(
    s: &mut dyn Session,
    files: (&str, &str),
    expected_v: (f64, f64),
    tol_abs: f64,
    strictly_increasing: bool,
) {
    let c3d = restore_as(s, files.0, "c3d").curve();
    let surf = restore_as(s, files.1, "surf").surface();
    let c2d = project(&c3d, &surf, precision::confusion()).expect("project c2d c3d surf");
    let log = dump_curve2d("c2d", &set(s, "c2d", c2d));
    let poles = dump_nb_poles(&log);
    let mut previous_v = 0.0;
    for i in 1..=poles {
        let (_, v) = dump_pole(&log, i);
        if i == 1 {
            checkreal("V_first", v, expected_v.0, tol_abs, 0.01);
        }
        if i == poles {
            checkreal("V_last", v, expected_v.1, tol_abs, 0.01);
        }
        let bad = if strictly_increasing {
            previous_v >= v
        } else {
            previous_v - v > tol_abs
        };
        assert!(i == 1 || !bad, "Error: Bad value of V in string: {i}");
        previous_v = v;
    }
}

pub fn bug25660(s: &mut dyn Session) {
    let files = ("bug25660_c3d.draw", "bug25660_surf.draw");
    projected_poles(s, files, (0.0, 6.96771797765), 1.0e-7, true);
}

pub fn bug25662(s: &mut dyn Session) {
    let files = ("bug25662_c3d.draw", "bug25662_surf.draw");
    projected_poles(
        s,
        files,
        (36.9997986855584, 55.0007653583147),
        1.0e-4,
        false,
    );
}

/// The `bug25892_*` view: `view 1 -2D- 728 20 400 400`, `2dfit` and the screenshot.
fn view_2d_capture(s: &mut dyn Session, image: &str) {
    s.step(ViewStep::View2d([728, 20, 400, 400]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd(image));
}

/// `bug25892_01` and `_02`: `REQUIRED All: Projection Failed`: the curve is too far from the
/// face surface to project.
fn projection_fails(s: &mut dyn Session, curve: &str, face: &str, image: &str) {
    let c = restore_as(s, curve, "c").curve();
    let f = restore_as(s, face, "f").shape();
    let surface = set(s, "s", mksurface(&f));
    let cx = project(&c, &surface, precision::confusion());
    assert!(cx.is_none(), "projection must fail");
    view_2d_capture(s, image);
}

pub fn bug25892_01(s: &mut dyn Session) {
    projection_fails(s, "bug25892_c001", "bug25892_f001", "bug25892_01");
}

pub fn bug25892_02(s: &mut dyn Session) {
    projection_fails(s, "bug25892_c002", "bug25892_f002", "bug25892_02");
}

/// `bug25892_03` to `_14`: `project cx c s` (no catch) must succeed.
pub fn projection_succeeds(s: &mut dyn Session, curve: &str, surface: &str, image: &str) {
    let c = restore_as(s, curve, "c").curve();
    let surf = restore_as(s, surface, "s").surface();
    let cx = project(&c, &surf, precision::confusion());
    set(s, "cx", cx.expect("project cx c s"));
    view_2d_capture(s, image);
}

macro_rules! bug25892_projections {
    ($($name:ident: $curve:literal on $surface:literal;)*) => {$(
        pub fn $name(s: &mut dyn Session) {
            projection_succeeds(s, $curve, $surface, stringify!($name));
        }
    )*};
}

bug25892_projections! {
    bug25892_03: "bug25892_c003" on "bug25892_s003";
    bug25892_04: "bug25892_c004" on "bug25892_s003";
    bug25892_05: "bug25892_c005" on "bug25892_s003";
    bug25892_06: "bug25892_c006" on "bug25892_s003";
    bug25892_07: "bug25892_c007" on "bug25892_s003";
    bug25892_08: "bug25892_c008" on "bug25892_s008";
    bug25892_09: "bug25892_c009" on "bug25892_s008";
    bug25892_10: "bug25892_c010" on "bug25892_s008";
    bug25892_11: "bug25892_c011" on "bug25892_s008";
    bug25892_12: "bug25892_c012" on "bug25892_s008";
    bug25892_13: "bug25892_c013" on "bug25892_s013";
    bug25892_14: "bug25892_c014" on "bug25892_s014";
}

/// 0030272: gproject with C0, C1 and C2 continuity reaches the original 2D and 3D
/// approximation errors (`checkreal`, 1e-4 absolute, 1e-4 relative).
pub fn bug30272(s: &mut dyn Session) {
    let c = restore_as(s, "bug30272_cur.brep", "c").curve();
    let surface = restore_as(s, "bug30272_sur.brep", "s").surface();
    let expected = [
        (
            Continuity::C0,
            "C0",
            [
                8.5166415968648575e-06,
                1.9383641349197776e-07,
                0.00039481100189762405,
            ],
        ),
        (
            Continuity::C1,
            "C1",
            [
                1.1207913216250422e-05,
                1.9546900926719333e-07,
                0.00029305148512110709,
            ],
        ),
        (
            Continuity::C2,
            "C2",
            [
                1.1368572775344132e-05,
                1.8525211887318316e-07,
                0.00024049581776618967,
            ],
        ),
    ];
    for (continuity, label, [u, v, e3d]) in expected {
        let errors = gproject_errors(&c, &surface, 1.0e-3, continuity);
        checkreal(&format!("2dUError_{label}"), errors.u2d, u, 1.0e-4, 0.0001);
        checkreal(&format!("2dVError_{label}"), errors.v2d, v, 1.0e-4, 0.0001);
        checkreal(&format!("3dError_{label}"), errors.e3d, e3d, 1.0e-4, 0.0001);
    }
}

/// The two half-circle projections on the sphere start and end at the original
/// parameters (the case's sign-agnostic `abs(x) - X > tol` checks) and keep the length;
/// then `v2d`, `2dfit` and the screenshot.
#[allow(clippy::approx_constant)] // The case writes these decimals, not the constants.
pub fn bug5145(s: &mut dyn Session) {
    let f1 = restore_as(s, "bug5145_f1.brep", "f1").shape();
    let c = restore_as(s, "bug5145_c.draw", "c").curve();
    let surface = set(s, "s", mksurface(&f1));
    let trim = |u1, u2| -> Handle<dyn GeomCurve> {
        Handle::new(GeomTrimmedCurve::new(&c, u1, u2, true, true).unwrap()).cast()
    };
    let c1 = set(s, "c1", trim(0., PI));
    let c2 = set(s, "c2", trim(PI, 2. * PI));
    let c2d1 = project(&c1, &surface, precision::confusion()).expect("project c2d1");
    let c2d2 = project(&c2, &surface, precision::confusion()).expect("project c2d2");
    let (c2d1, c2d2) = (set(s, "c2d1", c2d1), set(s, "c2d2", c2d2));
    let value = |curve: &Handle<dyn Geom2dCurve>, u: f64| curve.get().unwrap().d0(u).unwrap();
    let (p1, p2) = (value(&c2d1, 0.), value(&c2d1, PI));
    let (p3, p4) = (value(&c2d2, PI), value(&c2d2, 2. * PI));
    let tol = 1.0e-9;
    let checks = [
        p1.x().abs() - 4.7123885383825099,
        p1.y().abs() - 1.5707963267948966,
        p2.x().abs() - 6.2831853071795862,
        (-1.2217304763960311 - p2.y()).abs(),
        p3.x().abs(),
        (-1.2217304763960311 - p3.y()).abs(),
        p4.x().abs() - 1.5707967687970734,
        p4.y().abs() - 1.5707963267948948,
    ];
    checks
        .into_iter()
        .for_each(|value| script_not_above(value, tol));
    checklength2d(&c2d1, 3.68009);
    checklength2d(&c2d2, 3.68009);
    s.step(ViewStep::V2d);
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug5145"));
}

/// `if {$value > $tol} { puts "Error: Wrong projection in 2D" }`.
fn script_not_above(value: f64, tol: f64) {
    let above = value.partial_cmp(&tol) == Some(std::cmp::Ordering::Greater);
    assert!(!above, "Error: Wrong projection in 2D ({value})");
}
