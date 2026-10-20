//! HLRAlgo polyhedral classes:
//! PolyData with HideByPolyData, PolyShellData and PolyAlgo (Update, the hide and show
//! walks).
use super::common::{Reader, Sink, attempt, end, flag, int, real, xy, xyz};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkhlr::hlr_algo::{
    BiPoint, BiPointFace, BiPointIndices, BiPointPoints, EdgeStatus, PolyAlgo, PolyData,
    PolyHidingData, PolyShellData, PolyTriangle, TriangleData,
};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Xy, Xyz};

fn shared<T: Default + Clone + Send + Sync + 'static>(values: Vec<T>) -> Handle<HArray1<T>> {
    let array = HArray1::with_bounds(1, values.len() as i32).expect("probe array bounds");
    {
        let mut slots = array.change_array1();
        for (slot, value) in slots.iter_mut().zip(values) {
            *slot = value;
        }
    }
    Handle::new(array)
}

/// One face: see `hl_read_face`.
fn read_face(r: &mut Reader) -> Handle<PolyData> {
    let face = PolyData::new();
    let nodes: Vec<Xyz> = (0..r.integer()).map(|_| r.xyz()).collect();
    let triangles: Vec<TriangleData> = (0..r.integer())
        .map(|_| TriangleData {
            node1: r.integer(),
            node2: r.integer(),
            node3: r.integer(),
            flags: r.integer(),
        })
        .collect();
    let nh = r.integer();
    if nh >= 0 {
        let hiding: Vec<PolyHidingData> = (0..nh)
            .map(|_| {
                let (a, b, c) = (r.integer(), r.integer(), r.integer());
                let (x, y, z) = (r.real(), r.real(), r.real());
                let mut data = PolyHidingData::default();
                data.set(a, b, c, x, y, z, r.real());
                data
            })
            .collect();
        face.set_h_ph_dat(shared(hiding));
    }
    face.set_h_nodes(shared(nodes));
    face.set_h_t_data(shared(triangles));
    face.set_face_index(r.integer());
    face.indices().min = r.integer();
    face.indices().max = r.integer();
    Handle::new(face)
}

fn print_face(tokens: &mut Sink, face: &PolyData) -> Result<(), Failure> {
    int(tokens, face.face_index());
    let indices = *face.indices();
    int(tokens, indices.min);
    int(tokens, indices.max);
    flag(tokens, face.hiding());
    let triangles = face.t_data()?;
    triangles
        .get()
        .into_iter()
        .for_each(|t| t.array1().iter().for_each(|t| int(tokens, t.flags)));
    if face.hiding() {
        let hiding = face.ph_dat()?;
        for h in hiding
            .get()
            .into_iter()
            .flat_map(|h| h.array1().iter().copied().collect::<Vec<_>>())
        {
            let (i, p) = (h.indices(), h.plane());
            [i.index, i.min, i.max]
                .into_iter()
                .for_each(|value| int(tokens, value));
            xyz(tokens, p.normal);
            real(tokens, p.d);
        }
    }
    Ok(())
}

fn print_hide_status(tokens: &mut Sink, st: &EdgeStatus) {
    flag(tokens, st.all_hidden());
    flag(tokens, st.all_visible());
    let nb = st.nb_visible_part();
    int(tokens, nb);
    for i in 1..=nb {
        if let Ok((s, ts, e, te)) = st.visible_part(i) {
            [s, f64::from(ts), e, f64::from(te)]
                .into_iter()
                .for_each(|value| real(tokens, value));
        }
    }
}

fn print_triangle(tokens: &mut Sink, t: &PolyTriangle) {
    [t.v1, t.v2, t.v3]
        .into_iter()
        .for_each(|value| xy(tokens, value));
    [t.param, t.tol_param, t.tol_ang, t.tolerance]
        .into_iter()
        .for_each(|value| real(tokens, value));
}

fn read_indices(r: &mut Reader) -> BiPointIndices {
    let mut f = [0; 10];
    f.iter_mut().for_each(|v| *v = r.integer());
    BiPointIndices {
        shape_index: f[0],
        face_conex1: f[1],
        face1_pt1: f[2],
        face1_pt2: f[3],
        face_conex2: f[4],
        face2_pt1: f[5],
        face2_pt2: f[6],
        min_seg: f[7],
        max_seg: f[8],
        seg_flags: f[9],
    }
}

fn read_xy(r: &mut Reader) -> Xy {
    let x = r.real();
    Xy::new(x, r.real())
}

fn hide_segment(tokens: &mut Sink, r: &mut Reader, face: &PolyData) {
    let (p1, p2) = (r.xyz(), r.xyz());
    let points = BiPointPoints {
        pnt1: p1,
        pnt2: p2,
        pnt_p1: p1,
        pnt_p2: p2,
    };
    let indices = read_indices(r);
    let hiding_shell = r.flag();
    let (v1, v2, v3) = (read_xy(r), read_xy(r), read_xy(r));
    let mut triangle = PolyTriangle {
        v1,
        v2,
        v3,
        param: r.real(),
        tol_param: r.real(),
        tol_ang: r.real(),
        tolerance: r.real(),
    };
    let tol = triangle.tol_param as f32;
    let mut st = EdgeStatus::with_bounds(0.0, tol, 1.0, tol);
    attempt(tokens, |_| {
        face.hide_by_poly_data(&points, &mut triangle, &indices, hiding_shell, &mut st)
    });
    print_hide_status(tokens, &st);
    print_triangle(tokens, &triangle);
}

/// `face n segments`: see `hl_ha_polydata_case`.
pub fn polydata_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let handle = read_face(&mut r);
    let Some(face) = handle.get() else {
        return end(tokens);
    };
    attempt(tokens, |tokens| print_face(tokens, face));
    let mut bounds = BoundingBox3::default();
    attempt(tokens, |tokens| {
        face.update_global_min_max(&mut bounds)?;
        if bounds.is_void() {
            int(tokens, -1);
        } else {
            let l = bounds.limits();
            [l.x_min, l.x_max, l.y_min, l.y_max, l.z_min, l.z_max]
                .into_iter()
                .for_each(|value| real(tokens, value));
        }
        Ok(())
    });
    for _ in 0..r.integer() {
        hide_segment(tokens, &mut r, face);
    }
    end(tokens);
}

fn read_shell(r: &mut Reader) -> Result<Handle<PolyShellData>, Failure> {
    let nf = r.integer();
    let shell = PolyShellData::new(nf)?;
    for f in 1..=nf {
        let face = read_face(r);
        shell
            .poly_data()
            .set_value(f, face)
            .map_err(|e| e.into_failure("probe"))?;
    }
    for _ in 0..r.integer() {
        let mut c = [0.0; 12];
        c.iter_mut().for_each(|x| *x = r.real());
        let index = r.integer();
        let face1 = BiPointFace {
            conex: r.integer(),
            pt1: r.integer(),
            pt2: r.integer(),
        };
        let face2 = BiPointFace {
            conex: r.integer(),
            pt1: r.integer(),
            pt2: r.integer(),
        };
        let points = BiPointPoints::from_coordinates(c);
        shell.edges().append(BiPoint::with_faces_flags(
            points,
            index,
            [face1, face2],
            r.integer(),
        ));
    }
    Ok(Handle::new(shell))
}

fn print_shells(tokens: &mut Sink, algo: &PolyAlgo) -> Result<(), Failure> {
    let shells: Vec<Handle<PolyShellData>> = algo.poly_shell().iter().cloned().collect();
    for shell in shells.iter().filter_map(Handle::get) {
        let indices = *shell.indices();
        int(tokens, indices.min);
        int(tokens, indices.max);
        flag(tokens, shell.hiding());
        int(tokens, shell.hiding_poly_data().length());
        for bp in shell.edges().iter() {
            int(tokens, bp.indices().min_seg);
            int(tokens, bp.indices().max_seg);
        }
        let faces: Vec<Handle<PolyData>> = shell.poly_data().iter().cloned().collect();
        for face in faces.iter().filter_map(Handle::get) {
            print_face(tokens, face)?;
        }
        let hiding: Vec<Handle<PolyData>> = shell.hiding_poly_data().iter().cloned().collect();
        hiding
            .iter()
            .filter_map(Handle::get)
            .for_each(|face| int(tokens, face.face_index()));
    }
    Ok(())
}

fn walk_segments(tokens: &mut Sink, algo: &PolyAlgo) -> Result<(), Failure> {
    let mut st = EdgeStatus::new();
    algo.init_hide()?;
    while algo.more_hide() {
        let segment = algo.hide(&mut st)?;
        int(tokens, segment.index);
        [segment.reg1, segment.regn, segment.outl, segment.intl]
            .into_iter()
            .for_each(|value| flag(tokens, value));
        xyz(tokens, segment.points.pnt_p1);
        print_hide_status(tokens, &st);
        algo.next_hide()?;
    }
    int(tokens, -3);
    algo.init_show()?;
    while algo.more_show() {
        let segment = algo.show()?;
        int(tokens, segment.index);
        [segment.reg1, segment.regn, segment.outl, segment.intl]
            .into_iter()
            .for_each(|value| flag(tokens, value));
        xyz(tokens, segment.points.pnt2);
        algo.next_show()?;
    }
    Ok(())
}

/// `nbShells shells`: see `hl_ha_polyalgo_case`.
pub fn polyalgo_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let algo = PolyAlgo::new();
    let ns = r.integer();
    let built = (|| -> Result<(), Failure> {
        algo.init(ns)?;
        for s in 1..=ns {
            let shell = read_shell(&mut r)?;
            algo.change_poly_shell()
                .set_value(s, shell)
                .map_err(|e| e.into_failure("probe"))?;
        }
        Ok(())
    })();
    if built.is_err() {
        return end(tokens);
    }
    attempt(tokens, |tokens| {
        algo.update()?;
        print_shells(tokens, &algo)?;
        walk_segments(tokens, &algo)
    });
    end(tokens);
}
