//! HLRAlgo_Projector:
//! the five constructors, Set and Scaled, and every projection.
use super::common::{Reader, Sink, attempt, end, flag, pnt, pnt2d, real, xy, xyz};
use martensite_tkernel::standard::Failure;
use martensite_tkhlr::hlr_algo::Projector;
use martensite_tkmath::gp::{Axis2, Axis3, Point2, Transform3, Vector2, Vector3};

/// `P N X k V`: see `hl_read_trsf`.
fn read_trsf(r: &mut Reader) -> Result<Transform3, Failure> {
    let (p, n, x, k, v) = (r.pnt(), r.dir(), r.dir(), r.real(), r.xyz());
    let mut t = Transform3::default();
    t.set_transformation_to(Axis3::from_axis2(Axis2::with_x_direction(p, n, x)?));
    t.set_scale_factor(k)?;
    t.set_translation_part(Vector3::new(v.x(), v.y(), v.z()));
    Ok(t)
}

fn print_trsf(tokens: &mut Sink, t: &Transform3) -> Result<(), Failure> {
    for i in 1..=3 {
        for j in 1..=4 {
            real(tokens, t.value(i, j)?);
        }
    }
    Ok(())
}

fn print_projector(tokens: &mut Sink, pr: &Projector) -> Result<(), Failure> {
    flag(tokens, pr.perspective());
    print_trsf(tokens, &pr.transformation())?;
    print_trsf(tokens, &pr.inverted_transformation())?;
    print_trsf(tokens, &pr.full_transformation())?;
    let (d1, d2, d3) = pr.directions();
    [d1, d2, d3].into_iter().for_each(|d| xy(tokens, d.xy()));
    attempt(tokens, |tokens| {
        real(tokens, pr.focus()?);
        Ok(())
    });
    Ok(())
}

fn build_projector(r: &mut Reader, kind: i32) -> Result<Projector, Failure> {
    match kind {
        1 => {
            let (p, n, x) = (r.pnt(), r.dir(), r.dir());
            Projector::with_frame(Axis2::with_x_direction(p, n, x)?)
        }
        2 => {
            let (p, n, x) = (r.pnt(), r.dir(), r.dir());
            let frame = Axis2::with_x_direction(p, n, x)?;
            Projector::with_frame_focus(frame, r.real())
        }
        3 => {
            let t = read_trsf(r)?;
            let persp = r.flag();
            Projector::with_transformation(t, persp, r.real())
        }
        4 => {
            let t = read_trsf(r)?;
            let (persp, focus) = (r.flag(), r.real());
            let (a, b, c) = (r.pnt2d(), r.pnt2d(), r.pnt2d());
            let dirs = [a, b, c].map(|p| Vector2::new(p.x(), p.y()));
            Projector::with_directions(t, persp, focus, dirs)
        }
        _ => Projector::new(),
    }
}

fn project_point(tokens: &mut Sink, r: &mut Reader, pr: &Projector) {
    let (p, d, x, y) = (r.pnt(), r.xyz(), r.real(), r.real());
    let mut out = Point2::new(7.0, 8.0);
    pr.project(p, &mut out);
    pnt2d(tokens, out);
    let (px, py, pz) = pr.project_xyz(p);
    [px, py, pz]
        .into_iter()
        .for_each(|value| real(tokens, value));
    let d = Vector3::new(d.x(), d.y(), d.z());
    let (o2, v2) = pr.project_d1(p, d);
    pnt2d(tokens, o2);
    xy(tokens, v2.xy());
    attempt(tokens, |tokens| {
        let line = pr.shoot(x, y)?;
        pnt(tokens, line.location());
        super::common::dir(tokens, line.direction());
        Ok(())
    });
    let mut tp = p;
    pr.transform_point(&mut tp);
    pnt(tokens, tp);
    let mut tv = d;
    pr.transform_vector(&mut tv);
    xyz(tokens, *tv.xyz());
}

fn projector_body(tokens: &mut Sink, r: &mut Reader) -> Result<(), Failure> {
    let kind = r.integer();
    let mut pr = build_projector(r, kind)?;
    if r.flag() {
        let t = read_trsf(r)?;
        let persp = r.flag();
        pr.set(t, persp, r.real())?;
    }
    let scaled = r.integer();
    if scaled >= 0 {
        pr.scaled(scaled != 0)?;
    }
    print_projector(tokens, &pr)?;
    for _ in 0..r.integer() {
        project_point(tokens, r, &pr);
    }
    Ok(())
}

/// `kind ... n (P D x y)*n`: see `hl_ha_projector_case`.
pub fn projector_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(tokens, |tokens| projector_body(tokens, &mut r));
    end(tokens);
}
