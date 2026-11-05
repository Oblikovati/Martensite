//! BRepFillFilling probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::{failed, list};
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::{TpShapes, tp_shapes};
use martensite_tkbool::brep_fill::BRepFillFilling;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Face};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::Shape as Continuity;

/// The continuity of its enumeration value (C0..CN).
fn continuity_of(value: i32) -> Continuity {
    match value {
        1 => Continuity::G1,
        2 => Continuity::C1,
        3 => Continuity::G2,
        4 => Continuity::C2,
        5 => Continuity::C3,
        6 => Continuity::CN,
        _ => Continuity::C0,
    }
}

/// The face of a program index.
fn face_at(t: &TpShapes, index: i32) -> Result<Face, Failure> {
    topods::face(&at(t, index)?)
}

/// The edges of the request, each printing its index.
fn add_edges(
    output: &BufferedTokens,
    r: &mut Reader<'_>,
    t: &TpShapes,
    f: &mut BRepFillFilling,
) -> Result<(), Failure> {
    for _ in 0..r.integer() {
        let e = topods::edge(&at(t, r.integer())?)?;
        let face = r.integer();
        let (order, bound) = (continuity_of(r.integer()), r.flag());
        if face < 0 {
            output.int(f.add_edge(&e, order, bound));
        } else {
            output.int(f.add_edge_on_face(&e, &face_at(t, face)?, order, bound));
        }
    }
    Ok(())
}

/// The free faces, the points and the initial face of the request.
fn add_others(
    output: &BufferedTokens,
    r: &mut Reader<'_>,
    t: &TpShapes,
    f: &mut BRepFillFilling,
) -> Result<(), Failure> {
    for _ in 0..r.integer() {
        let face = face_at(t, r.integer())?;
        output.int(f.add_face(&face, continuity_of(r.integer())));
    }
    for _ in 0..r.integer() {
        if r.integer() == 0 {
            output.int(f.add_point(r.pnt())?);
            continue;
        }
        let (u, v) = (r.real(), r.real());
        let face = face_at(t, r.integer())?;
        output.int(f.add_point_on_face(u, v, &face, continuity_of(r.integer()))?);
    }
    let init = r.integer();
    if init >= 0 {
        f.load_init_surface(&face_at(t, init)?);
    }
    Ok(())
}

/// The filling of the request's parameters.
fn filling_of(r: &mut Reader<'_>) -> BRepFillFilling {
    let (degree, nbpts, nbiter, aniso) = (r.integer(), r.integer(), r.integer(), r.flag());
    let (tol2d, tol3d, tolang, tolcurv) = (r.real(), r.real(), r.real(), r.real());
    let (maxdeg, maxseg) = (r.integer(), r.integer());
    BRepFillFilling::new(
        degree, nbpts, nbiter, aniso, tol2d, tol3d, tolang, tolcurv, maxdeg, maxseg,
    )
}

/// The errors and the generated shapes of a done filling.
fn print_done(output: &BufferedTokens, t: &TpShapes, f: &mut BRepFillFilling) {
    output.attempt(|| {
        output.real(f.g0_error()?);
        output.real(f.g1_error()?);
        output.real(f.g2_error()?);
        Ok(())
    });
    let face = f.face();
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes(&face, &mut map, true, true);
    for g in &t.shapes {
        output.attempt(|| list(output, &map, f.generated(g)));
    }
}

/// `bf_filling program degree nbpts nbiter aniso tol2d tol3d tolang tolcurv maxdeg
/// maxseg n (edge face order bound)... m (face order)... k (point)... init`.
pub fn filling_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let mut f = filling_of(&mut r);
    let built = add_edges(output, &mut r, &t, &mut f)
        .and_then(|()| add_others(output, &mut r, &t, &mut f))
        .and_then(|()| f.build());
    if let Err(error) = built {
        return failed(output, error);
    }
    output.flag(f.is_done());
    output.attempt(|| shape(output, f.face().as_ref()));
    if f.is_done() {
        print_done(output, &t, &mut f);
    }
    output.end();
}
