//! Shared geometry record decoding and output encoding.
use super::output::Output;
pub use super::print::{Reader, attempt, end, flag, int, real, show};
use super::shapes;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::geom::{GeomCurve, GeomGeometry, GeomSurface};
use martensite_tkmath::gp::{
    Axis1, Axis2, Axis3, Direction3, GeneralTransform2, Point3, Transform3, Vector3, Xyz,
};

pub fn xyz(output: &Output, value: Xyz) {
    real(output, value.x());
    real(output, value.y());
    real(output, value.z());
}
pub fn pnt(output: &Output, value: Point3) {
    xyz(output, value.xyz());
}
pub fn vec(output: &Output, value: Vector3) {
    xyz(output, *value.xyz());
}
pub fn dir(output: &Output, value: Direction3) {
    xyz(output, value.xyz());
}
pub fn ax1(output: &Output, value: Axis1) {
    pnt(output, value.location());
    dir(output, value.direction());
}
pub fn ax2(output: &Output, value: Axis2) {
    pnt(output, value.location());
    dir(output, value.direction());
    dir(output, value.x_direction());
    dir(output, value.y_direction());
}
pub fn ax3(output: &Output, value: Axis3) {
    pnt(output, value.location());
    dir(output, value.direction());
    dir(output, value.x_direction());
    dir(output, value.y_direction());
}
/// The class name of a handle, `null` for a null one (g3_name).
pub fn name<T: ?Sized + RuntimeTyped>(output: &Output, handle: &Handle<T>) {
    match handle.dynamic_type() {
        Some(kind) => replay_print!(output, "{} ", kind.name()),
        None => replay_print!(output, "null "),
    }
}
pub fn dump(output: &Output, geometry: &dyn GeomGeometry, depth: i32) {
    let mut stream = DumpStream::default();
    geometry.dump_json(&mut stream, depth).unwrap();
    replay_print!(output, "{{{}}} ", stream.text());
}
pub fn gtrsf2d(output: &Output, value: GeneralTransform2) {
    for i in 1..=2 {
        for j in 1..=3 {
            real(output, value.value(i, j).unwrap());
        }
    }
    int(output, value.form() as i32);
}

impl Reader<'_> {
    pub fn pnt(&mut self) -> Point3 {
        let (x, y) = (self.real(), self.real());
        Point3::new(x, y, self.real())
    }
    pub fn vec(&mut self) -> Vector3 {
        let (x, y) = (self.real(), self.real());
        Vector3::new(x, y, self.real())
    }
    pub fn dir(&mut self) -> Direction3 {
        let (x, y) = (self.real(), self.real());
        Direction3::new(x, y, self.real()).expect("probe direction")
    }
    pub fn ax1(&mut self) -> Axis1 {
        let p = self.pnt();
        Axis1::new(p, self.dir())
    }
    pub fn ax2(&mut self) -> Axis2 {
        let (p, n) = (self.pnt(), self.dir());
        Axis2::with_x_direction(p, n, self.dir()).expect("probe frame")
    }
    pub fn ax3(&mut self) -> Axis3 {
        let (p, n) = (self.pnt(), self.dir());
        Axis3::new(p, n, self.dir()).expect("probe frame")
    }
}

/// One transformation, as g3_trsf reads it.
pub fn trsf(r: &mut Reader) -> Transform3 {
    let kind = r.integer();
    let p: Vec<f64> = (0..9).map(|_| r.real()).collect();
    let mut t = Transform3::default();
    let a = Point3::new(p[0], p[1], p[2]);
    let d = || Direction3::new(p[3], p[4], p[5]).unwrap();
    match kind {
        0 => t.set_mirror_point(a),
        1 => t.set_mirror_axis(Axis1::new(a, d())),
        2 => {
            let x = Direction3::new(p[6], p[7], p[8]).unwrap();
            t.set_mirror_plane(Axis2::with_x_direction(a, d(), x).unwrap());
        }
        3 => t.set_rotation(Axis1::new(a, d()), p[6]).unwrap(),
        4 => t.set_scale(a, p[3]).unwrap(),
        5 => t.set_translation(Vector3::new(p[0], p[1], p[2])),
        6 => t.set_translation_between(a, Point3::new(p[3], p[4], p[5])),
        7 => {
            let mut s = Transform3::default();
            s.set_scale(a, p[7]).unwrap();
            t.set_rotation(Axis1::new(a, d()), p[6]).unwrap();
            t = t.multiplied(s);
        }
        _ => {}
    }
    t
}

/// Reads a curve spec, reporting a construction failure and ending the line (g3_built_curve).
pub fn built_curve(output: &Output, r: &mut Reader) -> Option<Handle<dyn GeomCurve>> {
    report(output, shapes::curve(output, r))
}

/// Reads a surface spec, reporting a construction failure and ending the line (g3_built_surface).
pub fn built_surface(output: &Output, r: &mut Reader) -> Option<Handle<dyn GeomSurface>> {
    report(output, shapes::surface(output, r))
}

fn report<T>(output: &Output, built: Result<T, Failure>) -> Option<T> {
    match built {
        Ok(value) => Some(value),
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            None
        }
    }
}

/// The curve summary of g3_curve_summary.
pub fn curve_summary(output: &Output, c: &dyn GeomCurve, params: &[f64]) {
    real(output, c.first_parameter());
    real(output, c.last_parameter());
    show(output, c.is_closed(), |value| flag(output, value));
    flag(output, c.is_periodic());
    show(output, c.period(), |value| real(output, value));
    int(output, c.continuity() as i32);
    for n in -1..=4 {
        show(output, c.is_cn(n), |value| flag(output, value));
    }
    for &u in params {
        curve_evaluations(output, c, u);
    }
}

fn curve_evaluations(output: &Output, c: &dyn GeomCurve, u: f64) {
    show(output, c.eval_d0(u), |value| pnt(output, value));
    show(output, c.eval_d1(u), |r| {
        pnt(output, r.point);
        vec(output, r.d1);
    });
    show(output, c.eval_d2(u), |r| {
        pnt(output, r.point);
        [r.d1, r.d2]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, c.eval_d3(u), |r| {
        pnt(output, r.point);
        [r.d1, r.d2, r.d3]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    for n in 0..=5 {
        show(output, c.eval_dn(u, n), |value| vec(output, value));
    }
    show(output, c.value(u), |value| pnt(output, value));
    show(output, c.reversed_parameter(u), |value| real(output, value));
}

/// The surface summary of g3_surface_summary.
pub fn surface_summary(output: &Output, s: &dyn GeomSurface, uv: &[f64]) {
    let (u1, u2, v1, v2) = s.bounds();
    [u1, u2, v1, v2]
        .into_iter()
        .for_each(|value| real(output, value));
    show(output, s.is_u_closed(), |value| flag(output, value));
    show(output, s.is_v_closed(), |value| flag(output, value));
    flag(output, s.is_u_periodic());
    flag(output, s.is_v_periodic());
    show(output, s.u_period(), |value| real(output, value));
    show(output, s.v_period(), |value| real(output, value));
    int(output, s.continuity() as i32);
    for n in -1..=4 {
        show(output, s.is_cn_u(n), |value| flag(output, value));
        show(output, s.is_cn_v(n), |value| flag(output, value));
    }
    for pair in uv.as_chunks::<2>().0 {
        surface_evaluations(output, s, pair[0], pair[1]);
    }
}

fn surface_evaluations(output: &Output, s: &dyn GeomSurface, u: f64, v: f64) {
    show(output, s.eval_d0(u, v), |value| pnt(output, value));
    show(output, s.eval_d1(u, v), |r| {
        pnt(output, r.point);
        [r.d1u, r.d1v]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, s.eval_d2(u, v), |r| {
        pnt(output, r.point);
        [r.d1u, r.d1v, r.d2u, r.d2v, r.d2uv]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, s.eval_d3(u, v), |r| {
        pnt(output, r.point);
        let d = [
            r.d1u, r.d1v, r.d2u, r.d2v, r.d2uv, r.d3u, r.d3v, r.d3uuv, r.d3uvv,
        ];
        d.into_iter().for_each(|value| vec(output, value));
    });
    for nu in 0..=3 {
        for nv in 0..=3 {
            show(output, s.eval_dn(u, v, nu, nv), |value| vec(output, value));
        }
    }
    show(output, s.eval_dn(u, v, -1, 2), |value| vec(output, value));
    show(output, s.value(u, v), |value| pnt(output, value));
    show(output, s.u_reversed_parameter(u), |value| {
        real(output, value)
    });
    show(output, s.v_reversed_parameter(v), |value| {
        real(output, value)
    });
}

/// The iso curves of g3_isos.
pub fn isos(output: &Output, s: &dyn GeomSurface, isos: &[f64], params: &[f64]) {
    for &p in isos {
        show(output, s.u_iso(p), |c| {
            name(output, &c);
            curve_summary(output, &*c, params);
        });
        show(output, s.v_iso(p), |c| {
            name(output, &c);
            curve_summary(output, &*c, params);
        });
    }
}
