//! HLRBRep_Intersector.
use super::brep_curve_fixture::projector;
use super::brep_face_fixture::face;
use super::brep_record_fixture::{projected_record as set_edge, vertex_edge as edge_v};
use super::brep_surface_print::ics_point;
use super::common::{Sink, attempt, end, flag, int};
use super::geometry_records::common::Reader;
use super::geometry_records::intersection_report::{point as ipoint, segment as iseg};
use martensite_tkernel::standard::Failure;
use martensite_tkhlr::hlr_algo::Projector;
use martensite_tkhlr::hlr_brep::{EdgeData, Intersector, Surface};
use std::sync::Arc;

fn inter_print(tokens: &mut Sink, i: &Intersector) -> Result<(), Failure> {
    flag(tokens, i.is_done()?);
    let n = i.nb_points()?;
    int(tokens, n);
    for k in 1..=n {
        attempt(tokens, |tokens| ipoint(tokens, &i.point(k)?));
    }
    let ns = i.nb_segments()?;
    int(tokens, ns);
    for k in 1..=ns {
        attempt(tokens, |tokens| iseg(tokens, &i.segment(k)?));
    }
    Ok(())
}

fn cs_print(tokens: &mut Sink, i: &Intersector) -> Result<(), Failure> {
    flag(tokens, i.is_done()?);
    let n = i.nb_points()?;
    int(tokens, n);
    for k in 1..=n {
        attempt(tokens, |tokens| {
            ics_point(tokens, &i.cs_point(k)?);
            Ok(())
        });
    }
    int(tokens, i.nb_segments()?);
    Ok(())
}

fn edge_modes(
    tokens: &mut Sink,
    i: &mut Intersector,
    (d1, d2): (&EdgeData, &EdgeData),
    [da1, db1, da2, db2]: [f64; 4],
    (enbout, u, v): (bool, f64, f64),
) {
    attempt(tokens, |tokens| {
        i.perform_self(d1, da1, db1)?;
        inter_print(tokens, i)
    });
    attempt(tokens, |tokens| {
        i.perform_edges(1, d1, da1, db1, 2, d2, da2, db2, enbout)?;
        inter_print(tokens, i)
    });
    attempt(tokens, |tokens| {
        i.simulate_one_point(d1, u, d2, v)?;
        inter_print(tokens, i)
    });
}

fn line_modes(
    tokens: &mut Sink,
    i: &mut Intersector,
    r: &mut Reader,
    p: &Arc<Projector>,
    f: &martensite_tkbrep::topods::Face,
) -> Result<(), Failure> {
    let mut s = Surface::new();
    s.set_projector(Some(p.clone()));
    s.set_surface(f)?;
    i.load();
    for _ in 0..3 {
        let (x, y, pp) = (r.real(), r.real(), r.real());
        attempt(tokens, |tokens| {
            i.perform_line(&s, &p.shoot(x, y)?, pp)?;
            cs_print(tokens, i)
        });
    }
    Ok(())
}

fn intersector_body(tokens: &mut Sink, r: &mut Reader, p: &Arc<Projector>) -> Result<(), Failure> {
    let (e1, e2) = (edge_v(r)?, edge_v(r)?);
    let (da1, db1, da2, db2) = (r.real(), r.real(), r.real(), r.real());
    let enbout = r.flag();
    let (u, v) = (r.real(), r.real());
    let f = face(r)?;
    let (d1, d2) = (set_edge(&e1, p)?, set_edge(&e2, p)?);
    let mut i = Intersector::new();
    edge_modes(
        tokens,
        &mut i,
        (&d1, &d2),
        [da1, db1, da2, db2],
        (enbout, u, v),
    );
    line_modes(tokens, &mut i, r, p, &f)?;
    Ok(())
}

/// `projector edge1 edge2 da1 db1 da2 db2 enbout u v face (x y P)*3`: see
/// `hb_intersector_case`.
pub fn intersector_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(tokens, |tokens| {
        let p = Arc::new(projector(&mut r)?);
        attempt(tokens, |tokens| intersector_body(tokens, &mut r, &p));
        Ok(())
    });
    end(tokens);
}
