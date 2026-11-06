//! TopOpeBRepDS interference processing probe, the Rust twin of
//! the recorded capture (the records are documented there).
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::top_ope_brep_tool_tool::solid;
use martensite_tkbool::top_ope_brep_ds::{
    ListOfInterference, TopOpeBRepDSConfig, TopOpeBRepDSCurvePointInterference,
    TopOpeBRepDSEdgeVertexInterference, TopOpeBRepDSFaceEdgeInterference,
    TopOpeBRepDSHDataStructure, TopOpeBRepDSInterference, TopOpeBRepDSKind, TopOpeBRepDSPoint,
    TopOpeBRepDSShapeShapeInterference, TopOpeBRepDSTransition, fdscnx_close,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::top_abs::{ShapeEnum, State};

#[path = "top_ope_brep_ds_ops_run.rs"]
mod run;

/// The kinds in their native order.
pub(crate) const KINDS: [TopOpeBRepDSKind; 12] = [
    TopOpeBRepDSKind::Point,
    TopOpeBRepDSKind::Curve,
    TopOpeBRepDSKind::Surface,
    TopOpeBRepDSKind::Vertex,
    TopOpeBRepDSKind::Edge,
    TopOpeBRepDSKind::Wire,
    TopOpeBRepDSKind::Face,
    TopOpeBRepDSKind::Shell,
    TopOpeBRepDSKind::Solid,
    TopOpeBRepDSKind::CompSolid,
    TopOpeBRepDSKind::Compound,
    TopOpeBRepDSKind::Unknown,
];

/// The configurations in their native order.
const CONFIGS: [TopOpeBRepDSConfig; 3] = [
    TopOpeBRepDSConfig::UnshGeometry,
    TopOpeBRepDSConfig::SameOriented,
    TopOpeBRepDSConfig::DiffOriented,
];

type H = Handle<TopOpeBRepDSInterference>;

/// `BlDs`.
pub(crate) struct Ds {
    pub(crate) hds: Handle<TopOpeBRepDSHDataStructure>,
    pub(crate) idx: [Vec<i32>; 4],
    pub(crate) np: i32,
}

/// `bl_ds_ref`.
pub(crate) fn ds_ref(d: &Ds, r: i32) -> i32 {
    let (k, n) = (r / 1000, r % 1000);
    if k == 4 {
        return if d.np == 0 { 0 } else { n % d.np + 1 };
    }
    let v = &d.idx[k as usize];
    if v.is_empty() {
        0
    } else {
        v[n as usize % v.len()]
    }
}

/// `bl_ds_type`.
fn ds_type(i: &H) -> i32 {
    let names = [
        "TopOpeBRepDSCurvePointInterference",
        "TopOpeBRepDSEdgeVertexInterference",
        "TopOpeBRepDSFaceEdgeInterference",
        "TopOpeBRepDSShapeShapeInterference",
        "TopOpeBRepDSSurfaceCurveInterference",
        "TopOpeBRepDSSolidSurfaceInterference",
    ];
    names
        .iter()
        .position(|n| i.is_instance_name(n))
        .unwrap_or(6) as i32
}

/// `bl_ds_i`.
pub(crate) fn ds_i(output: &BufferedTokens, i: &H) {
    let Some(ii) = i.get() else {
        output.int(-1);
        return;
    };
    let ty = ds_type(i);
    output.int(ty);
    let t = ii.transition();
    output.int(t.before() as i32);
    output.int(t.after() as i32);
    output.int(t.shape_before() as i32);
    output.int(t.shape_after() as i32);
    output.int(t.index_before());
    output.int(t.index_after());
    output.int(ii.support_type() as i32);
    output.int(ii.support());
    output.int(ii.geometry_type() as i32);
    output.int(ii.geometry());
    if ty == 0 {
        let c: Handle<TopOpeBRepDSCurvePointInterference> = i.cast();
        output.real(c.get().map_or(0., |c| c.parameter()));
    }
    if ty == 1 {
        let e: Handle<TopOpeBRepDSEdgeVertexInterference> = i.cast();
        output.real(e.get().map_or(0., |e| e.parameter()));
    }
    if (1..=3).contains(&ty) {
        let s: Handle<TopOpeBRepDSShapeShapeInterference> = i.cast();
        if let Some(s) = s.get() {
            output.flag(s.gbound());
            output.int(s.config() as i32);
        }
    }
}

/// `bl_ds_loi`.
pub(crate) fn ds_loi(output: &BufferedTokens, l: &ListOfInterference) {
    output.int(l.extent());
    for i in l.iter() {
        ds_i(output, i);
    }
}

/// `bl_ds_dump`.
fn ds_dump(output: &BufferedTokens, d: &Ds) -> Result<(), Failure> {
    let h = d.hds.get().ok_or_else(|| Failure::new("null"))?;
    let ds = h.ds();
    output.int(ds.nb_shapes());
    for i in 1..=ds.nb_shapes() {
        let l = ds.shape_interferences_index(i, true)?;
        if l.is_empty() {
            continue;
        }
        output.int(i);
        ds_loi(output, l);
    }
    let mut se = Vec::new();
    for i in 1..=ds.nb_section_edges() {
        se.push(ds.shape_index(&ds.section_edge(i, true)?.into_shape(), true));
    }
    se.sort_unstable();
    output.int(ds.nb_section_edges());
    se.into_iter().for_each(|value| output.int(value));
    for i in 1..=ds.nb_points() {
        output.flag(ds.keep_point(i)?);
    }
    Ok(())
}

/// `bl_ds_make`.
fn ds_make(d: &Ds, r: &mut Reader<'_>) -> Result<(i32, H), Failure> {
    let h = d.hds.get().ok_or_else(|| Failure::new("null"))?;
    let target = ds_ref(d, r.integer());
    let ty = r.integer();
    let (sb, sa) = (
        State::ALL[r.integer() as usize],
        State::ALL[r.integer() as usize],
    );
    let (shb, sha) = (
        ShapeEnum::ALL[r.integer() as usize],
        ShapeEnum::ALL[r.integer() as usize],
    );
    let (ib, ia) = (ds_ref(d, r.integer()), ds_ref(d, r.integer()));
    let sk = KINDS[r.integer() as usize];
    let s = ds_ref(d, r.integer());
    let gk = KINDS[r.integer() as usize];
    let g = ds_ref(d, r.integer());
    let t = r.real();
    let gb = r.integer() != 0;
    let cf = CONFIGS[r.integer() as usize];
    let ts = h.ds().shape(target, true).clone();
    let mut p = t;
    if !ts.is_null() && ts.shape_type()? == ShapeEnum::Edge {
        let (f, l) = BRepTool::range(&topods::edge(&ts)?)?;
        p = f + t * (l - f);
    }
    let mut tr = TopOpeBRepDSTransition::new(sb, sa, shb, sha);
    tr.set_index_before(ib);
    tr.set_index_after(ia);
    let i: H = match ty {
        0 => Handle::new(TopOpeBRepDSCurvePointInterference::new(
            &tr, sk, s, gk, g, p,
        ))
        .cast(),
        1 => Handle::new(TopOpeBRepDSEdgeVertexInterference::new(
            &tr, sk, s, g, gb, cf, p,
        ))
        .cast(),
        2 => Handle::new(TopOpeBRepDSFaceEdgeInterference::new(&tr, s, g, gb, cf)).cast(),
        _ => Handle::new(TopOpeBRepDSShapeShapeInterference::new(
            &tr, sk, s, gk, g, gb, cf,
        ))
        .cast(),
    };
    Ok((target, i))
}

/// The solids, faces, edges and vertices of the two solids added with their ranks.
fn add_shapes(d: &mut Ds, s: &[Shape; 2]) -> Result<(), Failure> {
    let h = d.hds.get().ok_or_else(|| Failure::new("null"))?;
    let mut ds = h.change_ds();
    for (k, sk) in s.iter().enumerate() {
        d.idx[0].push(ds.add_shape_with_rank(sk, k as i32 + 1));
    }
    for (k, sk) in s.iter().enumerate() {
        for (j, ty) in [ShapeEnum::Face, ShapeEnum::Edge, ShapeEnum::Vertex]
            .into_iter()
            .enumerate()
        {
            for x in Explorer::new(sk, ty, ShapeEnum::Shape)? {
                let had = ds.has_shape(&x, true);
                let i = ds.add_shape_with_rank(&x, k as i32 + 1);
                if !had {
                    d.idx[j + 1].push(i);
                }
            }
        }
    }
    Ok(())
}

/// `bl_ds_build`.
fn ds_build(r: &mut Reader<'_>) -> Result<Ds, Failure> {
    let mut d = Ds {
        hds: Handle::new(TopOpeBRepDSHDataStructure::new()),
        idx: Default::default(),
        np: 0,
    };
    let s = [solid(r)?, solid(r)?];
    add_shapes(&mut d, &s)?;
    let h = d.hds.clone();
    let h = h.get().ok_or_else(|| Failure::new("null"))?;
    let np = r.integer();
    for _ in 0..np {
        let e = ds_ref(&d, r.integer());
        let (t, tol) = (r.real(), r.real());
        let mut ds = h.change_ds();
        let bc = BRepAdaptorCurve::with_edge(&topods::edge(ds.shape(e, true))?)?;
        let (f, l) = (bc.first_parameter()?, bc.last_parameter()?);
        ds.add_point(&TopOpeBRepDSPoint::new(bc.value(f + t * (l - f))?, tol));
    }
    d.np = np;
    for _ in 0..r.integer() {
        let (a, b) = (ds_ref(&d, r.integer()), ds_ref(&d, r.integer()));
        let mut ds = h.change_ds();
        let (sa, sb) = (ds.shape(a, true).clone(), ds.shape(b, true).clone());
        ds.fill_shapes_same_domain(&sa, &sb, true)?;
    }
    for _ in 0..r.integer() {
        let e = ds_ref(&d, r.integer());
        let mut ds = h.change_ds();
        let se = topods::edge(ds.shape(e, true))?;
        ds.add_section_edge(&se);
    }
    for _ in 0..r.integer() {
        let (target, i) = ds_make(&d, r)?;
        let mut ds = h.change_ds();
        let ts = ds.shape(target, true).clone();
        ds.change_shape_interferences(&ts).append(i);
    }
    Ok(d)
}

/// `dsx solids... op target arg`.
pub fn dsx_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let d = match ds_build(&mut r) {
        Ok(d) => d,
        Err(e) => {
            output.emit(format_args!("{} ", e.exception_type()));
            output.end();
            return;
        }
    };
    let (op, target, arg) = (r.integer(), ds_ref(&d, r.integer()), r.integer());
    output.attempt(|| run::dsx_run(output, &d, op, target, arg));
    output.attempt(|| ds_dump(output, &d));
    fdscnx_close();
    output.end();
}
