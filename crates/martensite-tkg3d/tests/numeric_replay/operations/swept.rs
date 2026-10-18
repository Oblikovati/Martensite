//! Extruded and revolved surface request cases.
use super::common::{
    Reader, ax1, ax2, built_surface, curve_summary, dir, dump, end, flag, int, name, pnt, show,
    surface_summary, trsf,
};
use super::output::Output;
use super::shapes;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomOffsetSurface, GeomSurface, GeomSurfaceOfLinearExtrusion,
    GeomSurfaceOfRevolution, GeomSweptSurface,
};
use martensite_tkg3d::geom_eval::{
    Domain2d, Map2d, RepSurfaceDesc, RepSurfaceDescDerivBounded, RepSurfaceDescFull,
    RepSurfaceDescMapped,
};

fn swept_state(output: &Output, s: &Handle<dyn GeomSweptSurface>, uv: &[f64], params: &[f64]) {
    dir(output, s.direction());
    let basis = s.basis_curve();
    name(output, &basis);
    curve_summary(output, &*basis, params);
    int(output, s.continuity() as i32);
    if let Some(v) = s.cast::<GeomSurfaceOfRevolution>().get() {
        ax1(output, v.axis());
        pnt(output, v.location());
        show(output, v.reference_plane(), |value| ax2(output, value));
    }
    surface_summary(output, &**s, uv);
    dump(output, &**s, -1);
}

pub fn swept_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(surface) = built_surface(output, &mut r) else {
        return;
    };
    let (uv, params) = (r.reals(), r.reals());
    let (d, a, p) = (r.dir(), r.ax1(), r.pnt());
    let c = shapes::curve(output, &mut r).expect("probe basis curve");
    let s: Handle<dyn GeomSweptSurface> = surface.cast();
    swept_state(output, &s, &uv, &params);
    if let Some(e) = s.cast::<GeomSurfaceOfLinearExtrusion>().get() {
        e.set_direction(d);
        swept_state(output, &s, &uv, &params);
        e.set_basis_curve(&c).unwrap();
        swept_state(output, &s, &uv, &params);
    }
    if let Some(v) = s.cast::<GeomSurfaceOfRevolution>().get() {
        v.set_direction(d);
        swept_state(output, &s, &uv, &params);
        v.set_location(p);
        swept_state(output, &s, &uv, &params);
        v.set_axis(a);
        swept_state(output, &s, &uv, &params);
        v.set_basis_curve(&c).unwrap();
        swept_state(output, &s, &uv, &params);
    }
    end(output);
}

// One descriptor spec, as g3_surface_desc reads it.
pub fn surface_desc(r: &mut Reader, rep: Handle<dyn GeomSurface>) -> Handle<dyn RepSurfaceDesc> {
    let kind = r.integer();
    if kind == 0 {
        return Handle::new(RepSurfaceDescFull::new(rep)).cast();
    }
    let order = r.integer() as usize;
    if kind == 1 {
        return Handle::new(RepSurfaceDescDerivBounded::new(rep, order)).cast();
    }
    let has_domain = r.flag();
    let domain = Domain2d {
        u_first: r.real(),
        u_last: r.real(),
        v_first: r.real(),
        v_last: r.real(),
    };
    let map = Map2d {
        scale_u: r.real(),
        offset_u: r.real(),
        scale_v: r.real(),
        offset_v: r.real(),
        swap_uv: r.flag(),
    };
    let domain = has_domain.then_some(domain);
    Handle::new(RepSurfaceDescMapped::new(rep, order, domain, map)).cast()
}

fn has_rep(s: &Handle<dyn GeomSurface>) -> bool {
    if let Some(o) = s.cast::<GeomOffsetSurface>().get() {
        return o.has_eval_representation();
    }
    if let Some(b) = s.cast::<GeomBSplineSurface>().get() {
        return b.has_eval_representation();
    }
    if let Some(e) = s.cast::<GeomSurfaceOfLinearExtrusion>().get() {
        return e.has_eval_representation();
    }
    s.cast::<GeomSurfaceOfRevolution>()
        .get()
        .is_some_and(|v| v.has_eval_representation())
}

fn set_rep(s: &Handle<dyn GeomSurface>, d: Handle<dyn RepSurfaceDesc>) -> Result<(), Failure> {
    if let Some(o) = s.cast::<GeomOffsetSurface>().get() {
        return o.set_eval_representation(d);
    }
    if let Some(b) = s.cast::<GeomBSplineSurface>().get() {
        return b.set_eval_representation(d);
    }
    if let Some(e) = s.cast::<GeomSurfaceOfLinearExtrusion>().get() {
        return e.set_eval_representation(d);
    }
    let revolution = s.cast::<GeomSurfaceOfRevolution>();
    revolution
        .get()
        .map_or(Ok(()), |v| v.set_eval_representation(d))
}

fn clear_rep(s: &Handle<dyn GeomSurface>) {
    if let Some(o) = s.cast::<GeomOffsetSurface>().get() {
        o.clear_eval_representation();
    }
    if let Some(b) = s.cast::<GeomBSplineSurface>().get() {
        b.clear_eval_representation();
    }
    if let Some(e) = s.cast::<GeomSurfaceOfLinearExtrusion>().get() {
        e.clear_eval_representation();
    }
    if let Some(v) = s.cast::<GeomSurfaceOfRevolution>().get() {
        v.clear_eval_representation();
    }
}

pub fn surface_evalrep_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let uv = r.reals();
    let rep = shapes::surface(output, &mut r).expect("probe representation surface");
    let d = surface_desc(&mut r, rep);
    let t = trsf(&mut r);
    flag(output, has_rep(&s));
    show(output, set_rep(&s, d.clone()), |()| ());
    flag(output, has_rep(&s));
    surface_summary(output, &*s, &uv);
    let c: Handle<dyn GeomSurface> = s.copy().unwrap().cast();
    flag(output, has_rep(&c));
    surface_summary(output, &*c, &uv);
    let own: Handle<dyn RepSurfaceDesc> = Handle::new(RepSurfaceDescFull::new(s.clone())).cast();
    show(output, set_rep(&s, own), |()| ());
    flag(output, has_rep(&s));
    let none: Handle<dyn RepSurfaceDesc> =
        Handle::new(RepSurfaceDescFull::new(Handle::default())).cast();
    show(output, set_rep(&s, none), |()| ());
    clear_rep(&s);
    flag(output, has_rep(&s));
    show(output, set_rep(&s, d), |()| ());
    s.transform(t).unwrap();
    flag(output, has_rep(&s));
    surface_summary(output, &*s, &uv);
    end(output);
}
