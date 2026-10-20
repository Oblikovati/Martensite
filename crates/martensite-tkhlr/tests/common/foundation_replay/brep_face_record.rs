//! Whole face record, wire-edge flag and face-iterator report.
//! Source: `tkhlr_probe_hlrbrep_records`::hb_facedata_case.
use super::brep_face_fixture::face;
use super::common::{Sink, attempt, end, flag, int, real};
use super::geometry_records::common::Reader;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkhlr::hlr_brep::{FaceData, FaceIterator};

fn read_w_edges(r: &mut Reader, fd: &mut FaceData) -> Result<(), Failure> {
    let mut wi = r.integer();
    while wi > 0 {
        let (ewi, ei, eo) = (r.integer(), r.integer(), r.integer());
        let (a, b, c, dd) = (r.flag(), r.flag(), r.flag(), r.flag());
        fd.set_w_edge(wi, ewi, ei, Orientation::ALL[eo as usize], a, b, c, dd)?;
        wi = r.integer();
    }
    Ok(())
}

fn set_face_flags(fd: &mut FaceData, o: i32, cl: bool) {
    fd.set_back(true);
    fd.set_side(o == 1);
    fd.set_hiding(cl);
    fd.set_simple(true);
    fd.set_cut(false);
    fd.set_with_out_l(cl);
    fd.set_plane(true);
    fd.set_cylinder(o == 2);
    fd.set_cone(false);
    fd.set_sphere(true);
    fd.set_torus(cl);
    fd.set_size(2.5);
    fd.set_selected(false);
}

fn walk(tokens: &mut Sink, fd: &FaceData) -> Result<(), Failure> {
    let mut it = FaceIterator::new();
    it.init_edge(fd)?;
    while it.more_edge() {
        int(tokens, it.edge()?);
        int(tokens, it.orientation()? as i32);
        for v in [it.out_line()?, it.internal()?, it.double()?, it.iso_line()?] {
            flag(tokens, v);
        }
        flag(tokens, it.beginning_of_wire());
        flag(tokens, it.end_of_wire());
        int(tokens, it.wire()?.get().map_or(0, |w| w.nb_edges()));
        it.next_edge()?;
    }
    Ok(())
}

pub(crate) fn face_boolean_flags(tokens: &mut Sink, fd: &FaceData) {
    for v in [
        fd.selected(),
        fd.back(),
        fd.side(),
        fd.closed(),
        fd.hiding(),
        fd.simple(),
        fd.cut(),
        fd.with_out_l(),
        fd.plane(),
        fd.cylinder(),
        fd.cone(),
        fd.sphere(),
        fd.torus(),
    ] {
        flag(tokens, v);
    }
}

fn face_state_report(tokens: &mut Sink, fd: &FaceData) {
    face_boolean_flags(tokens, fd);
    real(tokens, fd.size());
    int(tokens, fd.orientation() as i32);
    real(tokens, f64::from(fd.tolerance()));
    int(tokens, fd.geometry().get_type() as i32);
}

fn facedata_body(tokens: &mut Sink, r: &mut Reader) -> Result<(), Failure> {
    let f = face(r)?;
    let (o, cl, nw) = (r.integer(), r.flag(), r.integer());
    let mut fd = FaceData::new();
    flag(tokens, fd.selected());
    real(tokens, fd.size());
    fd.set(&f, Orientation::ALL[o as usize], cl, nw)?;
    for w in 1..=nw {
        fd.set_wire(w, r.integer())?;
    }
    read_w_edges(r, &mut fd)?;
    set_face_flags(&mut fd, o, cl);
    face_state_report(tokens, &fd);
    walk(tokens, &fd)
}

/// `face or closed nw (ne)* (wi ewi ei or outl inte dble isol)* -1`: see `hb_facedata_case`.
pub fn facedata_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(tokens, |tokens| facedata_body(tokens, &mut r));
    end(tokens);
}
