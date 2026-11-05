//! The `tt_topology` operation of the recorded capture.
use super::brep_fill_generator::failed;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::top_ope_brep_tool_tool::{EdgeFace, edge_face, normal, other_edge};
use martensite_tkbool::top_ope_brep_tool::*;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Edge, Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::BoundingBox2;
use martensite_tkmath::gp::{Direction3, Point2, Point3, Vector2, Vector3};

/// Four reals.
fn reals4(output: &BufferedTokens, b: (f64, f64, f64, f64)) {
    [b.0, b.1, b.2, b.3]
        .into_iter()
        .for_each(|value| output.real(value));
}

/// The bounds members of `tt_topology`.
fn topology_bounds(output: &BufferedTokens, x: &EdgeFace) {
    let f = &x.f;
    output.attempt(|| {
        let (a, b) = fun_tool_tol_uv(f)?;
        output.real(a);
        output.real(b);
        Ok(())
    });
    output.attempt(|| {
        let mut d = false;
        output.flag(fun_tool_direct(f, &mut d)?);
        output.flag(d);
        Ok(())
    });
    output.attempt(|| {
        let mut b = (-1.0, -1.0, -1.0, -1.0);
        output.flag(fun_tool_bounds_face(
            f, &mut b.0, &mut b.1, &mut b.2, &mut b.3,
        )?);
        reals4(output, b);
        Ok(())
    });
    output.attempt(|| {
        let b = fun_tool_geombounds(f)?;
        output.flag(b.is_some());
        reals4(output, b.unwrap_or((-1.0, -1.0, -1.0, -1.0)));
        Ok(())
    });
    output.attempt(|| {
        let mut b = (-1.0, -1.0, -1.0, -1.0);
        output.flag(fun_tool_isobounds(f, &mut b)?);
        reals4(output, b);
        Ok(())
    });
    output.attempt(|| {
        let (mut b, mut o) = ((-1.0, -1.0, -1.0, -1.0), false);
        output.flag(fun_tool_outbounds(f, &mut b, &mut o)?);
        reals4(output, b);
        output.flag(o);
        Ok(())
    });
}

/// The point members of `tt_topology`.
fn topology_points(output: &BufferedTokens, x: &EdgeFace, e2: &Edge) {
    let (e, f, par, uv) = (&x.e, &x.f, x.par, x.uv);
    output.attempt(|| {
        let bc = BRepAdaptorCurve::with_edge(e)?;
        let p = bc.value(par)?;
        output.flag(fun_tool_pin_c(p, &bc, 1.0e-7)?);
        let (a, b) = (bc.first_parameter()?, bc.last_parameter()?);
        output.flag(fun_tool_pin_c_range(
            p.translated(Vector3::new(0.0, 0.0, 1.0e-3)),
            &bc,
            a,
            b,
            1.0e-2,
        )?);
        Ok(())
    });
    output.attempt(|| {
        let p = fun_tool_value_on_edge(par, e)?;
        output.flag(p.is_some());
        output.pnt(p.unwrap_or(Point3::new(-1.0, -1.0, -1.0)));
        output.pnt(fun_tool_value_on_face(uv, f)?);
        Ok(())
    });
    output.attempt(|| {
        let p = BRepAdaptorCurve::with_edge(e)?.value(par)?;
        output.int(fun_tool_sta_pin_e(p, e)? as i32);
        output.int(
            fun_tool_sta_pin_e_tol(p.translated(Vector3::new(0.0, 0.0, 0.5)), e, 1.0e-3)? as i32,
        );
        Ok(())
    });
    output.attempt(|| {
        output.int(fun_tool_orient_v_in_e(&topexp::first_vertex(e, false), e));
        Ok(())
    });
    output.attempt(|| {
        let (ok, o) = fun_tool_orient_e_in_f(e, f)?;
        output.flag(ok);
        output.int(o as i32);
        let (ok, o) = fun_tool_orient_e_in_f_forward(e, &topods::face(&f.reversed())?)?;
        output.flag(ok);
        output.int(o as i32);
        output.flag(fun_tool_ebound_f(e, f)?);
        Ok(())
    });
    output.attempt(|| fun_tool_nggeom_f(uv, f).map(|v| output.xyz(*v.xyz())));
    output.attempt(|| {
        let n = fun_tool_nggeom_f_on_edge(par, e, f)?;
        output.flag(n.is_some());
        output.xyz(*n.unwrap_or_default().xyz());
        let n2 = fun_tool_nggeom_f_on_edge_tol(par, e2, f, 1.0e-9)?;
        output.flag(n2.is_some());
        output.xyz(*n2.or(n).unwrap_or_default().xyz());
        Ok(())
    });
    output.attempt(|| {
        output.flag(fun_tool_etg_f(par, e, uv, f, 1.0e-6)?);
        output.flag(fun_tool_etg_ooe(par, e, par, e2, 1.0e-6)?);
        Ok(())
    });
    output.attempt(|| {
        let (mut a, mut b) = (-10.0, 10.0);
        output.flag(fun_nearest_iso(f, uv.x(), true, &mut a, &mut b)?);
        output.real(a);
        output.real(b);
        Ok(())
    });
    output.attempt(|| {
        let (_, n) = normal(uv, f)?;
        output.flag(fun_tool_eitangentto_fe(n, e, par)?);
        Ok(())
    });
}

/// The type and closure members of `tt_topology`.
fn topology_types(output: &BufferedTokens, x: &EdgeFace) {
    let (e, f) = (&x.e, &x.f);
    output.attempt(|| {
        output.int(fun_tool_typ(e)? as i32);
        output.flag(fun_tool_line_edge(e)?);
        output.flag(fun_tool_plane_face(f)?);
        output.flag(fun_tool_cylinder_face(f)?);
        Ok(())
    });
    output.attempt(|| {
        output.flag(fun_tool_closed_s_flag(f)?);
        let c = fun_tool_closed_s(f)?;
        let (uc, up, vc, vp) = c.unwrap_or((false, -1.0, false, -1.0));
        output.flag(c.is_some_and(|c| c.0 || c.2));
        output.flag(uc);
        output.real(up);
        output.flag(vc);
        output.real(vp);
        let (mut inu, mut xm, mut xp) = (false, -1.0, -1.0);
        output.flag(fun_tool_closed_s_in_u(f, &mut inu, &mut xm, &mut xp)?);
        output.flag(inu);
        output.real(xm);
        output.real(xp);
        Ok(())
    });
    output.attempt(|| {
        let ex = Explorer::new(f, ShapeEnum::Wire, ShapeEnum::Shape)?;
        let w = ex.current().clone();
        let mut b = BoundingBox2::default();
        fun_tool_mk_bnd2d(&w, f, &mut b)?;
        b.get()?.into_iter().for_each(|value| output.real(value));
        output.flag(fun_tool_closing_e(e, &topods::wire(&w)?, f)?);
        output.flag(fun_tool_closed_w(&topods::wire(&w)?)?);
        Ok(())
    });
    output.attempt(|| {
        output.flag(fun_tool_is_closing_e(e, f, f)?);
        output.flag(fun_tool_in_s(e, f)?);
        output.flag(fun_tool_in_s(e, &x.f2)?);
        Ok(())
    });
}

/// The parameter members of `tt_topology`.
fn topology_params(output: &BufferedTokens, x: &EdgeFace, e2: &Edge) {
    let (e, f, par) = (&x.e, &x.f, x.par);
    let v1 = topexp::first_vertex(e, false);
    output.attempt(|| {
        let es = fun_tool_eshared(&v1, f, &x.f2)?;
        output.flag(es.is_some());
        output.flag(es.is_some_and(|s| s.is_same(e)));
        Ok(())
    });
    output.attempt(|| {
        let p = fun_tool_par_v_on_e(&v1, e)?;
        output.flag(p.is_some());
        output.real(p.unwrap_or(-1.0));
        let mut q = -1.0;
        output.flag(fun_tool_par_e(e, par, e2, &mut q)?);
        output.real(q);
        Ok(())
    });
    output.attempt(|| {
        let mut a = Point2::new(-1.0, -1.0);
        output.flag(fun_tool_par_f(e, par, f, &mut a)?);
        output.pnt2d(a);
        output.flag(fun_tool_paron_ef(e, par, &x.f2, &mut a)?);
        output.pnt2d(a);
        Ok(())
    });
    output.attempt(|| {
        let bc = BRepAdaptorCurve::with_edge(e)?;
        output.dir(fun_tool_dir_c_on_curve(par, &bc)?);
        output.xyz(*fun_tool_tggeom_e_vec(par, e)?.xyz());
        output.flag(fun_tool_line_bac(&bc)?);
        output.flag(fun_tool_quad_edge(e)?);
        output.flag(fun_tool_quad_bac(&bc)?);
        output.flag(fun_tool_quad_face(f)?);
        Ok(())
    });
    output.attempt(|| {
        let (p, q) = fun_tool_find_pin_e(e)?;
        output.pnt(p);
        output.real(q);
        Ok(())
    });
}

/// The shape members of `tt_topology`.
fn topology_shapes(output: &BufferedTokens, x: &EdgeFace, e2: &Edge) {
    let (e, f, par) = (&x.e, &x.f, x.par);
    output.attempt(|| {
        output.real(fun_tool_maxtol(&x.s)?);
        let mut t = 0.0;
        output.flag(fun_tool_maxtol_of_type(f, ShapeEnum::Edge, &mut t)?);
        output.real(t);
        Ok(())
    });
    output.attempt(|| {
        output.int(fun_tool_nbshapes(&x.s, ShapeEnum::Edge)?);
        let mut l: List<Shape> = List::default();
        fun_tool_shapes(f, ShapeEnum::Edge, &mut l)?;
        output.int(l.extent());
        Ok(())
    });
    output.attempt(|| {
        output.int(fun_tool_comparebndkole(f, &x.s)?);
        output.int(fun_tool_comparebndkole(e, f)?);
        Ok(())
    });
    output.attempt(|| {
        output.flag(fun_tool_same_ori(e, e)?);
        output.flag(fun_tool_same_ori(e, &topods::edge(&e.reversed())?)?);
        output.flag(fun_tool_haspc(e, f)?);
        Ok(())
    });
    output.attempt(|| {
        let mut so = false;
        output.flag(fun_tool_curves_so(e, e2, &mut so)?);
        output.flag(so);
        so = false;
        output.flag(fun_tool_curves_so_at(e, par, e, &mut so)?);
        output.flag(so);
        Ok(())
    });
    output.attempt(|| {
        let mut l: List<Shape> = List::default();
        l.append(x.f2.clone().into_shape());
        l.append(f.clone().into_shape());
        let fa = fun_tool_find_ancestor(&l, e)?;
        output.flag(fa.is_some());
        output.flag(fa.is_some_and(|fa| fa.is_same(&x.f2)));
        Ok(())
    });
    output.attempt(|| {
        output.xyz(*fun_tool_getgeomxx(f, e, par)?.xyz());
        let d = fun_tool_getxx(f, e, par)?;
        output.flag(d.is_some());
        output.dir(d.unwrap_or(Direction3::Z));
        Ok(())
    });
    output.attempt(|| {
        let d = fun_tool_getdxx(f, e, par)?;
        output.flag(d.is_some());
        // Natively the step is reset to (0, 0) first.
        output.xy(d.unwrap_or(Vector2::new(0.0, 0.0)).xy());
        Ok(())
    });
}

/// `tt_topology solid ie k t`.
pub fn topology_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let x = match edge_face(&mut r) {
        Ok(x) => x,
        Err(error) => return failed(output, error),
    };
    let e2 = other_edge(&x.f, &x.e).expect("other edge");
    topology_bounds(output, &x);
    topology_points(output, &x, &e2);
    topology_types(output, &x);
    topology_params(output, &x, &e2);
    topology_shapes(output, &x, &e2);
    output.end();
}
