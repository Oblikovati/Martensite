//! Offset surface evaluation and mutation requests.
use super::common::{
    Reader, attempt, built_surface, dump, end, flag, gtrsf2d, int, isos, name, real, show,
    surface_summary, trsf,
};
use super::output::Output;
use super::shapes;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomGeometry, GeomOffsetSurface, GeomRectangularTrimmedSurface, GeomSurface,
};

/// Surface kinds 9 (two-direction trim), 10 (one-direction trim) and 11 (offset).
pub fn spec(
    output: &Output,
    kind: i32,
    r: &mut Reader,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    match kind {
        9 => {
            let (u1, u2, v1, v2) = (r.real(), r.real(), r.real(), r.real());
            let (us, vs) = (r.flag(), r.flag());
            let basis = shapes::surface(output, r)?;
            let trimmed = GeomRectangularTrimmedSurface::new(&basis, (u1, u2), (v1, v2), (us, vs))?;
            Ok(Handle::new(trimmed).cast())
        }
        10 => {
            let (p1, p2) = (r.real(), r.real());
            let (u_trim, sense) = (r.flag(), r.flag());
            let basis = shapes::surface(output, r)?;
            let trimmed =
                GeomRectangularTrimmedSurface::with_one_direction(&basis, p1, p2, u_trim, sense)?;
            Ok(Handle::new(trimmed).cast())
        }
        _ => {
            let (d, no_check) = (r.real(), r.flag());
            let basis = shapes::surface(output, r)?;
            Ok(Handle::new(GeomOffsetSurface::new(&basis, d, no_check)?).cast())
        }
    }
}

fn trim_state(output: &Output, t: &GeomRectangularTrimmedSurface, uv: &[f64]) {
    name(output, &t.basis_surface());
    surface_summary(output, t, uv);
    dump(output, t, -1);
}

pub fn rect_trim_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let (uv, iso, params) = (r.reals(), r.reals(), r.reals());
    let (u1, u2, v1, v2) = (r.real(), r.real(), r.real(), r.real());
    let (us, vs) = (r.flag(), r.flag());
    let (p1, p2) = (r.real(), r.real());
    let (u_trim, sense) = (r.flag(), r.flag());
    let m = trsf(&mut r);
    let t: Handle<GeomRectangularTrimmedSurface> = s.cast();
    trim_state(output, &t, &uv);
    show(output, t.copy(), |c| {
        trim_state(output, &c.cast::<GeomRectangularTrimmedSurface>(), &uv)
    });
    isos(output, &*t, &iso, &params);
    attempt(output, || t.set_trim(u1, u2, v1, v2, us, vs));
    trim_state(output, &t, &uv);
    attempt(output, || t.set_trim_direction(p1, p2, u_trim, sense));
    trim_state(output, &t, &uv);
    attempt(output, || t.u_reverse());
    trim_state(output, &t, &uv);
    attempt(output, || t.v_reverse());
    trim_state(output, &t, &uv);
    attempt(output, || t.transform(m));
    trim_state(output, &t, &uv);
    isos(output, &*t, &iso, &params);
    end(output);
}

fn offset_state(output: &Output, o: &GeomOffsetSurface, uv: &[f64]) {
    real(output, o.offset());
    int(output, o.basis_surf_continuity() as i32);
    name(output, &o.basis_surface());
    flag(output, o.has_eval_representation());
    show(output, o.surface(), |e| {
        name(output, &e);
        if let Some(e) = e.get() {
            surface_summary(output, e, uv);
        }
    });
    surface_summary(output, o, uv);
    dump(output, o, -1);
}

fn osculating(output: &Output, o: &GeomOffsetSurface, uv: &[f64]) {
    for pair in uv.chunks(2).filter(|pair| pair.len() == 2) {
        let (u, v) = (pair[0], pair[1]);
        for along_u in [true, false] {
            let mut opposite = false;
            let mut surface: Handle<GeomBSplineSurface> = Handle::default();
            let found = if along_u {
                o.u_osculating_surface(u, v, &mut opposite, &mut surface)
            } else {
                o.v_osculating_surface(u, v, &mut opposite, &mut surface)
            };
            show(output, found, |found| {
                flag(output, found);
                flag(output, opposite);
                name(output, &surface);
                if let Some(s) = surface.get() {
                    surface_summary(output, s, uv);
                }
            });
        }
    }
}

pub fn offset_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let (uv, iso, params) = (r.reals(), r.reals(), r.reals());
    let (d, no_check) = (r.real(), r.flag());
    let m = trsf(&mut r);
    let basis: Option<Handle<dyn GeomSurface>> = match shapes::surface(output, &mut r) {
        Ok(basis) => Some(basis),
        Err(error) => {
            replay_print!(output, "{} ", error.exception_type());
            None
        }
    };
    let o: Handle<GeomOffsetSurface> = s.cast();
    offset_state(output, &o, &uv);
    osculating(output, &o, &uv);
    isos(output, &*o, &iso, &params);
    show(output, o.copy(), |c| {
        offset_state(output, &c.cast::<GeomOffsetSurface>(), &uv)
    });
    attempt(output, || o.set_offset_value(d));
    offset_state(output, &o, &uv);
    attempt(output, || o.u_reverse());
    offset_state(output, &o, &uv);
    attempt(output, || o.v_reverse());
    offset_state(output, &o, &uv);
    attempt(output, || o.transform(m));
    offset_state(output, &o, &uv);
    isos(output, &*o, &iso, &params);
    for pair in uv.chunks(2).filter(|pair| pair.len() == 2) {
        let (u, v) = o.transform_parameters(pair[0], pair[1], m);
        real(output, u);
        real(output, v);
    }
    show(output, o.parametric_transformation(m), |value| {
        gtrsf2d(output, value)
    });
    if let Some(basis) = basis {
        attempt(output, || o.set_basis_surface(&basis, no_check));
        offset_state(output, &o, &uv);
        osculating(output, &o, &uv);
    }
    attempt(output, || o.set_basis_surface(&o.cast(), no_check));
    offset_state(output, &o, &uv);
    end(output);
}
