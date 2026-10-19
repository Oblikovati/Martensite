//! Small ordinary geometry fixtures shared by the exception contract matrices.
use martensite_tkernel::{
    ncollection::{Array1, Array2},
    standard::Handle,
};
use martensite_tkg3d::{
    geom::{GeomCurve, GeomSurface},
    geom_eval::*,
};
use martensite_tkmath::gp::{Axis2, Axis3, Point3, Transform3, Vector3};

pub(super) type CurveFixture = (&'static str, Handle<dyn GeomCurve>);
pub(super) type SurfaceFixture = (&'static str, Handle<dyn GeomSurface>);

pub(super) fn curve_poles() -> Array1<'static, Point3> {
    let mut poles = Array1::with_bounds(1, 3).unwrap();
    poles.set_value(1, Point3::new(1., 0., 0.)).unwrap();
    poles.set_value(2, Point3::new(0., 1., 0.)).unwrap();
    poles.set_value(3, Point3::new(0., 0., 1.)).unwrap();
    poles
}

pub(super) fn surface_poles() -> Array2<'static, Point3> {
    let mut poles = Array2::with_bounds(1, 3, 1, 3).unwrap();
    for i in 1..=3 {
        for j in 1..=3 {
            poles
                .set_value(
                    i,
                    j,
                    Point3::new(f64::from(i), f64::from(j), f64::from(i * j)),
                )
                .unwrap();
        }
    }
    poles
}

pub(super) fn curve_fixtures() -> Vec<CurveFixture> {
    vec![
        ("GeomEval_CircularHelixCurve", exception_helix_fixture()),
        ("GeomEval_SineWaveCurve", exception_sine_fixture()),
        ("GeomEval_TBezierCurve", exception_tbezier_fixture()),
        ("GeomEval_AHTBezierCurve", exception_aht_fixture()),
    ]
}

pub(super) fn surface_fixtures() -> Vec<SurfaceFixture> {
    let mut fixtures = elementary_fixtures();
    fixtures.push(("GeomEval_TBezierSurface", exception_tbezier_patch()));
    fixtures.push(("GeomEval_AHTBezierSurface", exception_aht_patch()));
    fixtures
}

fn elementary_fixtures() -> Vec<SurfaceFixture> {
    vec![
        ("GeomEval_EllipsoidSurface", exception_ellipsoid_fixture()),
        (
            "GeomEval_HyperboloidSurface",
            exception_hyperboloid_fixture(),
        ),
        (
            "GeomEval_HypParaboloidSurface",
            exception_hyp_paraboloid_fixture(),
        ),
        ("GeomEval_ParaboloidSurface", exception_paraboloid_fixture()),
        (
            "GeomEval_CircularHelicoidSurface",
            exception_helicoid_fixture(),
        ),
    ]
}

fn exception_helix_fixture() -> Handle<dyn GeomCurve> {
    let geometry = GeomEvalCircularHelixCurve::new(Axis2::default(), 2., 3.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_sine_fixture() -> Handle<dyn GeomCurve> {
    let geometry = GeomEvalSineWaveCurve::new(Axis2::default(), 2., 3., 0.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_tbezier_fixture() -> Handle<dyn GeomCurve> {
    let geometry = GeomEvalTBezierCurve::new(&curve_poles(), 1.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_aht_fixture() -> Handle<dyn GeomCurve> {
    let geometry = GeomEvalAHTBezierCurve::new(&curve_poles(), 2, 0., 0.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_ellipsoid_fixture() -> Handle<dyn GeomSurface> {
    let geometry = GeomEvalEllipsoidSurface::new(Axis3::default(), 2., 3., 4.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_hyperboloid_fixture() -> Handle<dyn GeomSurface> {
    let geometry =
        GeomEvalHyperboloidSurface::new(Axis3::default(), 2., 3., HyperboloidSheetMode::OneSheet)
            .unwrap();
    Handle::new(geometry).cast()
}

fn exception_hyp_paraboloid_fixture() -> Handle<dyn GeomSurface> {
    let geometry = GeomEvalHypParaboloidSurface::new(Axis3::default(), 2., 3.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_paraboloid_fixture() -> Handle<dyn GeomSurface> {
    let geometry = GeomEvalParaboloidSurface::new(Axis3::default(), 2.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_helicoid_fixture() -> Handle<dyn GeomSurface> {
    let geometry = GeomEvalCircularHelicoidSurface::new(Axis3::default(), 2.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_tbezier_patch() -> Handle<dyn GeomSurface> {
    let geometry = GeomEvalTBezierSurface::new(&surface_poles(), 1., 1.).unwrap();
    Handle::new(geometry).cast()
}

fn exception_aht_patch() -> Handle<dyn GeomSurface> {
    let geometry =
        GeomEvalAHTBezierSurface::new(&surface_poles(), [2, 2], [0.; 2], [0.; 2]).unwrap();
    Handle::new(geometry).cast()
}

pub(super) fn transformations() -> [Transform3; 2] {
    let mut translation = Transform3::default();
    translation.set_translation(Vector3::new(1., 2., 3.));
    [Transform3::default(), translation]
}

pub(super) fn rational_trigonometric_patch() -> Handle<dyn GeomSurface> {
    let mut weights = Array2::with_bounds(1, 3, 1, 3).unwrap();
    for weight in weights.as_mut_slice() {
        *weight = 1.;
    }
    let geometry =
        GeomEvalTBezierSurface::with_weights(&surface_poles(), &weights, 1., 1.).unwrap();
    Handle::new(geometry).cast()
}
