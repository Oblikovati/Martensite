//! The geometry creation commands of the tests/mkface scripts, on the Rust API
//! (GeomliteTest_CurveCommands.cxx `anacurve`/`polecurve`, GeomliteTest_SurfaceCommands.cxx
//! `anasurface`/`polesurface`/`sweepsurface`/`offseting`/`trimming` at
//! 3d097a0328e71b826377d4814ab05ec3c3d23871). Each takes the numbers of its script line.
use martensite_tkernel::ncollection::Array2;
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkg2d::geom2d::arrays;
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomBezierSurface, GeomCircle, GeomConicalSurface,
    GeomCurve, GeomCylindricalSurface, GeomEllipse, GeomHyperbola, GeomLine, GeomOffsetSurface,
    GeomParabola, GeomPlane, GeomRectangularTrimmedSurface, GeomSphericalSurface, GeomSurface,
    GeomSurfaceOfLinearExtrusion, GeomSurfaceOfRevolution, GeomToroidalSurface,
};
use martensite_tkmath::gp::{Axis1, Axis2, Axis3, Direction3, Point3};

fn curve<T: GeomCurve + RuntimeTyped + 'static>(value: T) -> Handle<dyn GeomCurve> {
    Handle::new(value).cast()
}

fn surface<T: GeomSurface + RuntimeTyped + 'static>(value: T) -> Handle<dyn GeomSurface> {
    Handle::new(value).cast()
}

fn point(v: &[f64]) -> Point3 {
    Point3::new(v[0], v[1], v[2])
}

fn direction(v: &[f64]) -> Direction3 {
    Direction3::new(v[0], v[1], v[2]).expect("a non-null direction")
}

/// The `gp_Ax2` of the 3D conic forms: `x y z` with Z, `x y z nx ny nz`, or with the X
/// direction too; the remaining words are the conic's parameters.
fn conic_frame(words: &[f64], parameters: usize) -> (Axis2, &[f64]) {
    let (frame, rest) = match words.len() - parameters {
        3 => (
            Axis2::new(point(words), Direction3::Z).unwrap(),
            &words[3..],
        ),
        6 => (
            Axis2::new(point(words), direction(&words[3..])).unwrap(),
            &words[6..],
        ),
        9 => (
            Axis2::with_x_direction(point(words), direction(&words[3..]), direction(&words[6..]))
                .expect("a valid frame"),
            &words[9..],
        ),
        other => panic!("conic: {other} placement words; expected 3, 6 or 9"),
    };
    (frame, rest)
}

/// `line name x y z dx dy dz`.
pub fn line(words: [f64; 6]) -> Handle<dyn GeomCurve> {
    curve(GeomLine::from_point(point(&words), direction(&words[3..])))
}

/// `circle name x y z [nx ny nz [xx xy xz]] r`.
pub fn circle(words: &[f64]) -> Handle<dyn GeomCurve> {
    let (frame, r) = conic_frame(words, 1);
    curve(GeomCircle::new(frame, r[0]).unwrap())
}

/// `ellipse name x y z [nx ny nz [xx xy xz]] major minor`.
pub fn ellipse(words: &[f64]) -> Handle<dyn GeomCurve> {
    let (frame, r) = conic_frame(words, 2);
    curve(GeomEllipse::new(frame, r[0], r[1]).unwrap())
}

/// `hyperbola name x y z [nx ny nz [xx xy xz]] major minor`.
pub fn hyperbola(words: &[f64]) -> Handle<dyn GeomCurve> {
    let (frame, r) = conic_frame(words, 2);
    curve(GeomHyperbola::new(frame, r[0], r[1]).unwrap())
}

/// `parabola name x y z [nx ny nz [xx xy xz]] focal`.
pub fn parabola(words: &[f64]) -> Handle<dyn GeomCurve> {
    let (frame, f) = conic_frame(words, 1);
    curve(GeomParabola::new(frame, f[0]).unwrap())
}

/// `beziercurve name nbpoles x y z [w] ...` (weights when four words per pole).
pub fn beziercurve(words: &[f64]) -> Handle<dyn GeomCurve> {
    let np = words[0] as usize;
    let per_pole = (words.len() - 1) / np;
    assert!(
        (3..=4).contains(&per_pole),
        "beziercurve: {per_pole} words per pole"
    );
    let mut poles = arrays::one_based(np as i32).unwrap();
    let mut weights = arrays::one_based(np as i32).unwrap();
    for i in 0..np {
        let at = 1 + i * per_pole;
        arrays::put(&mut poles, i as i32 + 1, point(&words[at..])).unwrap();
        if per_pole == 4 {
            arrays::put(&mut weights, i as i32 + 1, words[at + 3]).unwrap();
        }
    }
    if per_pole == 4 {
        curve(GeomBezierCurve::with_weights(&poles, &weights).unwrap())
    } else {
        curve(GeomBezierCurve::new(&poles).unwrap())
    }
}

/// `bsplinecurve name deg nbknots k1 m1 ... x1 y1 z1 w1 ...`, or `pbsplinecurve` for the
/// periodic form (as many poles as the multiplicities without the last one).
pub fn bsplinecurve(words: &[f64], periodic: bool) -> Handle<dyn GeomCurve> {
    let degree = words[0] as i32;
    let nb_knots = words[1] as usize;
    let mut knots = arrays::one_based(nb_knots as i32).unwrap();
    let mut mults = arrays::one_based(nb_knots as i32).unwrap();
    let mut sigma = 0;
    for i in 0..nb_knots {
        arrays::put(&mut knots, i as i32 + 1, words[2 + 2 * i]).unwrap();
        let mult = words[3 + 2 * i] as i32;
        arrays::put(&mut mults, i as i32 + 1, mult).unwrap();
        sigma += mult;
    }
    let last_mult = words[1 + 2 * nb_knots] as i32;
    let np = if periodic {
        sigma - last_mult
    } else {
        sigma - degree - 1
    } as usize;
    let mut poles = arrays::one_based(np as i32).unwrap();
    let mut weights = arrays::one_based(np as i32).unwrap();
    let start = 2 + 2 * nb_knots;
    for i in 0..np {
        let at = start + 4 * i;
        arrays::put(&mut poles, i as i32 + 1, point(&words[at..])).unwrap();
        arrays::put(&mut weights, i as i32 + 1, words[at + 3]).unwrap();
    }
    let bspline =
        GeomBSplineCurve::with_weights(&poles, &weights, &knots, &mults, degree, periodic);
    curve(bspline.unwrap())
}

/// The `gp_Ax3` of `anasurface` and the index of the first parameter word.
fn surface_frame(words: &[f64]) -> (Axis3, usize) {
    let n = words.len() + 2;
    if n < 5 {
        (Axis3::default(), 0)
    } else if n < 8 {
        let frame = Axis3::new(point(words), Direction3::Z, Direction3::X);
        (frame.unwrap(), 3)
    } else if n < 11 {
        let frame = Axis2::new(point(words), direction(&words[3..])).unwrap();
        (frame.axis3(), 6)
    } else {
        let frame = Axis3::new(point(words), direction(&words[3..]), direction(&words[6..]));
        (frame.unwrap(), 9)
    }
}

/// `plane name [x y z [dx dy dz [ux uy uz]]]`.
pub fn plane(words: &[f64]) -> Handle<dyn GeomSurface> {
    surface(GeomPlane::new(surface_frame(words).0))
}

/// `cylinder name [placement] r`.
pub fn cylinder(words: &[f64]) -> Handle<dyn GeomSurface> {
    let (frame, i) = surface_frame(words);
    surface(GeomCylindricalSurface::new(frame, words[i]).unwrap())
}

/// `sphere name [placement] r`.
pub fn sphere(words: &[f64]) -> Handle<dyn GeomSurface> {
    let (frame, i) = surface_frame(words);
    surface(GeomSphericalSurface::new(frame, words[i]).unwrap())
}

/// `cone name [placement] semi-angle-degrees r`.
pub fn cone(words: &[f64]) -> Handle<dyn GeomSurface> {
    let (frame, i) = surface_frame(words);
    let angle = words[i] * (std::f64::consts::PI / 180.0);
    surface(GeomConicalSurface::new(frame, angle, words[i + 1]).unwrap())
}

/// `torus name [placement] major minor`.
pub fn torus(words: &[f64]) -> Handle<dyn GeomSurface> {
    let (frame, i) = surface_frame(words);
    surface(GeomToroidalSurface::new(frame, words[i], words[i + 1]).unwrap())
}

/// `beziersurf name nu nv x y z [w] ...` (poles in V-major order of the script).
pub fn beziersurf(words: &[f64]) -> Handle<dyn GeomSurface> {
    let (nu, nv) = (words[0] as usize, words[1] as usize);
    let per_pole = (words.len() - 2) / (nu * nv);
    assert!(
        (3..=4).contains(&per_pole),
        "beziersurf: {per_pole} words per pole"
    );
    let (nu_i, nv_i) = (nu as i32, nv as i32);
    let mut poles: Array2<'static, Point3> = Array2::with_bounds(1, nu_i, 1, nv_i).unwrap();
    let mut weights: Array2<'static, f64> = Array2::with_bounds(1, nu_i, 1, nv_i).unwrap();
    let mut at = 2;
    for j in 1..=nv as i32 {
        for i in 1..=nu as i32 {
            poles.set_value(i, j, point(&words[at..])).unwrap();
            at += 3;
            if per_pole == 4 {
                weights.set_value(i, j, words[at]).unwrap();
                at += 1;
            }
        }
    }
    if per_pole == 4 {
        surface(GeomBezierSurface::with_weights(&poles, &weights).unwrap())
    } else {
        surface(GeomBezierSurface::new(&poles).unwrap())
    }
}

/// `extsurf name curve dx dy dz`.
pub fn extsurf(basis: &Handle<dyn GeomCurve>, d: [f64; 3]) -> Handle<dyn GeomSurface> {
    surface(GeomSurfaceOfLinearExtrusion::new(basis, direction(&d)).unwrap())
}

/// `revsurf name curve x y z dx dy dz`.
pub fn revsurf(basis: &Handle<dyn GeomCurve>, words: [f64; 6]) -> Handle<dyn GeomSurface> {
    let axis = Axis1::new(point(&words), direction(&words[3..]));
    surface(GeomSurfaceOfRevolution::new(basis, axis).unwrap())
}

/// `offset name surface distance`: a Geom_OffsetSurface.
pub fn offset(basis: &Handle<dyn GeomSurface>, distance: f64) -> Handle<dyn GeomSurface> {
    surface(GeomOffsetSurface::new(basis, distance, false).unwrap())
}

/// BRepTest `mkface name surface umin umax vmin vmax` inside a Tcl `catch`: the face, or
/// the StdFail_NotDone its conversion raises when BRepBuilderAPI_MakeFace is not done.
pub fn mkface_bounds_checked(
    surface: &Handle<dyn GeomSurface>,
    [u_min, u_max, v_min, v_max]: [f64; 4],
) -> Result<martensite_tkbrep::topods::Shape, String> {
    use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
    let bounds = (u_min, u_max, v_min, v_max);
    let confusion = martensite_tkernel::precision::confusion();
    let mut maker = BRepBuilderAPIMakeFace::from_surface_bounds(surface, bounds, confusion)
        .map_err(|failure| format!("{failure:?}"))?;
    maker
        .shape()
        .cloned()
        .map_err(|failure| format!("{failure:?}"))
}

/// `trim name curve u1 u2`: a Geom_TrimmedCurve, sense true.
pub fn trim_curve(basis: &Handle<dyn GeomCurve>, u1: f64, u2: f64) -> Handle<dyn GeomCurve> {
    use martensite_tkg3d::geom::GeomTrimmedCurve;
    curve(GeomTrimmedCurve::new(basis, u1, u2, true, true).unwrap())
}

/// `trim name surface u1 u2 v1 v2`: a Geom_RectangularTrimmedSurface, both senses true.
pub fn trim(basis: &Handle<dyn GeomSurface>, uv: [f64; 4]) -> Handle<dyn GeomSurface> {
    let [u1, u2, v1, v2] = uv;
    surface(GeomRectangularTrimmedSurface::new(basis, (u1, u2), (v1, v2), (true, true)).unwrap())
}
