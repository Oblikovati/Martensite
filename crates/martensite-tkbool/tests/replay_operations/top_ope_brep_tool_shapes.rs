//! TopOpeBRepToolShapeTool, PROJECT, GEOMETRY, FC2D, FTOL, HBoxTool and TOOL::UVISO
//! probe operations, the Rust twin of the recorded capture
//! (the records are documented there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::brep_fill_multi_line::curve2d;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shapes::tp_shapes;
use martensite_tkbool::top_ope_brep_tool::{
    PCurveOnFace, TopOpeBRepToolHBoxTool, TopOpeBRepToolShapeTool, fc2d_curve_on_surface,
    fc2d_curve_on_surface_on_edge, fc2d_editable_curve_on_surface, fc2d_has_c3d,
    fc2d_has_new_curve_on_surface, fc2d_has_old_curve_on_surface, fc2d_prepare,
    ftol_face_tolerances3d, fun_tool_bounds, fun_tool_closed, fun_tool_dir_c, fun_tool_line,
    fun_tool_ng_s, fun_tool_onapex, fun_tool_proj_ponbounded_f, fun_tool_proj_ponc2d,
    fun_tool_proj_pone, fun_tool_proj_ponf, fun_tool_quad, fun_tool_quad_surface, uviso_edge,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve2d;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::extrema::{ExtAlgo, ExtFlag};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Direction2, Direction3, Point2};

/// The shapes of a request.
fn shapes_of(r: &mut Reader<'_>, n: usize) -> Result<Vec<Shape>, Failure> {
    let t = tp_shapes(r)?;
    (0..n).map(|_| at(&t, r.integer())).collect()
}

/// The six coordinates of a box.
fn bounds(output: &BufferedTokens, b: &BoundingBox3) -> Result<(), Failure> {
    b.get()?.into_iter().for_each(|value| output.real(value));
    Ok(())
}

/// A p-curve with its range and tolerance.
fn pcurve_on_face(output: &BufferedTokens, pc: &PCurveOnFace) -> Result<(), Failure> {
    curve2d(output, &pc.0)?;
    output.real(pc.1);
    output.real(pc.2);
    output.real(pc.3);
    Ok(())
}

/// The UV bounds and periodic adjustment of `tt_shape_tool`.
fn uv_members(output: &BufferedTokens, f: &Shape, u: f64, v: f64) {
    output.attempt(|| {
        let mut out = (false, false, -1., -1., -1., -1.);
        TopOpeBRepToolShapeTool::uv_bounds_of_face(&topods::face(f)?, &mut out)?;
        output.flag(out.0);
        output.flag(out.1);
        [out.2, out.3, out.4, out.5]
            .into_iter()
            .for_each(|value| output.real(value));
        Ok(())
    });
    output.attempt(|| {
        let (mut uu, mut vv) = (u, v);
        TopOpeBRepToolShapeTool::adjust_on_periodic(f, &mut uu, &mut vv)?;
        output.real(uu);
        output.real(vv);
        Ok(())
    });
}

/// `tt_shape_tool program s1 s2 e f par u v`.
pub fn shape_tool_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let s = match shapes_of(&mut r, 4) {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    let (s1, s2, e, f) = (&s[0], &s[1], &s[2], &s[3]);
    let (par, u, v) = (r.real(), r.real(), r.real());
    output.attempt(|| TopOpeBRepToolShapeTool::tolerance(s1).map(|value| output.real(value)));
    output.attempt(|| TopOpeBRepToolShapeTool::pnt(s1).map(|value| output.pnt(value)));
    output.attempt(|| {
        TopOpeBRepToolShapeTool::shapes_same_oriented(s1, s2).map(|value| output.flag(value))
    });
    output.attempt(|| {
        let bs = TopOpeBRepToolShapeTool::basis_surface_of_face(&topods::face(f)?)?;
        output.text(bs.dynamic_type().map_or("", |t| t.name()).as_bytes());
        Ok(())
    });
    uv_members(output, f, u, v);
    output.attempt(|| TopOpeBRepToolShapeTool::closed(e, f).map(|value| output.flag(value)));
    output.attempt(|| {
        TopOpeBRepToolShapeTool::periodize_parameter(par, e, f).map(|value| output.real(value))
    });
    output.attempt(|| {
        let (mut t, mut n, mut c) = (Direction3::Z, Direction3::Z, -1.);
        output.real(TopOpeBRepToolShapeTool::edge_data_of_edge(
            e, par, &mut t, &mut n, &mut c,
        )?);
        output.dir(t);
        output.dir(n);
        output.real(c);
        Ok(())
    });
    output.attempt(|| {
        TopOpeBRepToolShapeTool::resolution3d_of_face(&topods::face(f)?, 1.0e-9)
            .map(|value| output.real(value))
    });
    output.end();
}

/// `tt_project program e f x y z`.
pub fn project_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let s = match shapes_of(&mut r, 2) {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    let p = r.pnt();
    let ef = (|| Ok::<_, Failure>((topods::edge(&s[0])?, topods::face(&s[1])?)))();
    let (e, f) = match ef {
        Ok(v) => v,
        Err(error) => return failed(output, error),
    };
    output.attempt(|| {
        let (a, b) = fun_tool_bounds(&e)?;
        output.real(a);
        output.real(b);
        Ok(())
    });
    output.attempt(|| {
        let (mut pa, mut d) = (-1., -1.);
        output.flag(fun_tool_proj_pone(p, &e, &mut pa, &mut d)?);
        output.real(pa);
        output.real(d);
        Ok(())
    });
    output.attempt(|| {
        let (mut uv, mut d) = (Point2::new(-1., -1.), -1.);
        output.flag(fun_tool_proj_ponf(
            p,
            &f,
            &mut uv,
            &mut d,
            ExtFlag::MinMax,
            ExtAlgo::Grad,
        )?);
        output.pnt2d(uv);
        output.real(d);
        Ok(())
    });
    output.attempt(|| {
        let (mut uv, mut d) = (Point2::new(-1., -1.), -1.);
        output.flag(fun_tool_proj_ponbounded_f(p, &f, &mut uv, &mut d)?);
        output.pnt2d(uv);
        output.real(d);
        Ok(())
    });
    output.attempt(|| {
        let bc = BRepAdaptorCurve2d::with_edge_on_face(&e, &f)?;
        let (mut pa, mut d) = (-1., -1.);
        output.flag(fun_tool_proj_ponc2d(p, &bc, &mut pa, &mut d)?);
        output.real(pa);
        output.real(d);
        Ok(())
    });
    output.end();
}

/// `tt_geometry program e f u v`.
pub fn geometry_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let s = match shapes_of(&mut r, 2) {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    let uv = r.pnt2d();
    let geometry = (|| {
        let (c, a, b) = BRepTool::curve(&topods::edge(&s[0])?)?;
        Ok::<_, Failure>((c, 0.5 * (a + b), BRepTool::surface(&topods::face(&s[1])?)?))
    })();
    let (c, mid, surface) = match geometry {
        Ok(v) => v,
        Err(error) => return failed(output, error),
    };
    output.attempt(|| fun_tool_dir_c(mid, &c).map(|value| output.dir(value)));
    output.attempt(|| fun_tool_line(&c).map(|value| output.flag(value)));
    output.attempt(|| fun_tool_quad(&c).map(|value| output.flag(value)));
    output.attempt(|| fun_tool_ng_s(uv, &surface).map(|value| output.dir(value)));
    output.attempt(|| fun_tool_onapex(uv, &surface).map(|value| output.flag(value)));
    output.attempt(|| fun_tool_quad_surface(&surface).map(|value| output.flag(value)));
    output.attempt(|| {
        let (uc, up, vc, vp) = fun_tool_closed(&surface)?.unwrap_or((false, 0., false, 0.));
        output.flag(uc || vc);
        output.flag(uc);
        output.real(up);
        output.flag(vc);
        output.real(vp);
        Ok(())
    });
    output.end();
}

/// `tt_fc2d program s1 s2 e f ef`.
pub fn fc2d_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let s = match shapes_of(&mut r, 5) {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    let shapes = (|| {
        Ok::<_, Failure>((
            topods::edge(&s[2])?,
            topods::face(&s[3])?,
            topods::edge(&s[4])?,
        ))
    })();
    let (e, f, ef) = match shapes {
        Ok(v) => v,
        Err(error) => return failed(output, error),
    };
    fc2d_prepare(&s[0], &s[1]);
    output.attempt(|| fc2d_has_c3d(&e).map(|value| output.flag(value)));
    output.attempt(|| fc2d_has_old_curve_on_surface(&e, &f).map(|o| output.flag(o.is_some())));
    output.attempt(|| pcurve_on_face(output, &fc2d_curve_on_surface(&e, &f, false)?));
    output.flag(fc2d_has_new_curve_on_surface(&e, &f).is_some());
    output.attempt(|| pcurve_on_face(output, &fc2d_editable_curve_on_surface(&e, &f, false)?));
    output.attempt(|| {
        let (mut d, mut o) = (Direction2::X, Point2::new(0., 0.));
        let (iso, iu, iv) = uviso_edge(&e, &f, &mut d, &mut o)?;
        [iso, iu, iv]
            .into_iter()
            .for_each(|value| output.flag(value));
        output.xy(d.xy());
        output.pnt2d(o);
        Ok(())
    });
    output.attempt(|| pcurve_on_face(output, &fc2d_curve_on_surface_on_edge(&e, &f, &ef, false)?));
    output.attempt(|| ftol_face_tolerances3d(&f, &f).map(|value| output.real(value)));
    output.end();
}

/// `tt_hbox program s ts`.
pub fn hbox_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let s = match shapes_of(&mut r, 1) {
        Ok(s) => s,
        Err(error) => return failed(output, error),
    };
    let ts = ShapeEnum::ALL[r.integer() as usize];
    let hbt = TopOpeBRepToolHBoxTool::new();
    output.attempt(|| hbt.add_boxes(&s[0], ts, ShapeEnum::Shape));
    output.int(hbt.extent());
    for i in 1..=hbt.extent() {
        output.attempt(|| bounds(output, &hbt.box_of_index(i)?));
        output.int(hbt.shape(i).map_or(0, |sh| hbt.index(&sh)));
    }
    output.attempt(|| hbt.box_of_index(hbt.extent() + 1).map(|_| ()));
    output.attempt(|| {
        let mut b = BoundingBox3::default();
        TopOpeBRepToolHBoxTool::compute_box_on_vertices(&s[0], &mut b)?;
        bounds(output, &b)
    });
    output.end();
}
