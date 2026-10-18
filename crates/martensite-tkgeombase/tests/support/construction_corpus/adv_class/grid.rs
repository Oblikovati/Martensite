/// The one-patch grid of `InitGrid` with the cuts of the request applied.
struct Grid {
    net: AdvApp2VarNetwork,
    frame: AdvApp2VarFramework,
}

impl Grid {
    fn new(iu: i32, iv: i32, d: [f64; 4]) -> Result<Self, Failure> {
        let mut p: Sequence<Handle<AdvApp2VarPatch>> = Sequence::default();
        p.append(Handle::new(AdvApp2VarPatch::with_domain(d, iu, iv)));
        let (mut us, mut vs) = (Sequence::default(), Sequence::default());
        us.append(d[0]);
        us.append(d[1]);
        vs.append(d[2]);
        vs.append(d[3]);
        let net = AdvApp2VarNetwork::new(p, us, vs);
        let mut bag: Sequence<Handle<AdvApp2VarNode>> = Sequence::default();
        for (u, v) in [(d[0], d[2]), (d[1], d[2]), (d[0], d[3]), (d[1], d[3])] {
            bag.append(Handle::new(AdvApp2VarNode::with_coord(
                Xy::new(u, v),
                iu,
                iv,
            )?));
        }
        let iso = |t: IsoType, cte: f64, pos: i32| {
            Handle::new(AdvApp2VarIso::with_domain(t, cte, d, pos, iu, iv))
        };
        let (mut bu, mut bv) = (Sequence::default(), Sequence::default());
        bu.append(iso(IsoType::IsoV, d[2], 1));
        bu.append(iso(IsoType::IsoV, d[3], 2));
        bv.append(iso(IsoType::IsoU, d[0], 3));
        bv.append(iso(IsoType::IsoU, d[1], 4));
        let (mut su, mut sv) = (Sequence::default(), Sequence::default());
        su.append(bu);
        sv.append(bv);
        Ok(Self {
            net,
            frame: AdvApp2VarFramework::new(bag, su, sv),
        })
    }

    fn cut(&mut self, r: &mut Reader) -> Result<(), Failure> {
        for _ in 0..r.integer() {
            let (dir, x) = (r.integer(), r.real());
            if dir == 0 {
                self.net.update_in_u(x)?;
                self.frame.update_in_u(x)?;
            } else {
                self.net.update_in_v(x)?;
                self.frame.update_in_v(x)?;
            }
        }
        Ok(())
    }
}

fn print_iso(transcript: &Output, i: &AdvApp2VarIso) {
    int(transcript, iso_index(i.iso_type()));
    for v in [
        i.constante(),
        i.t0(),
        i.t1(),
        i.u0(),
        i.u1(),
        i.v0(),
        i.v1(),
    ] {
        real(transcript, v);
    }
    int(transcript, i.u_order());
    int(transcript, i.v_order());
    int(transcript, i.position());
    flag(transcript, i.is_approximated());
    flag(transcript, i.has_result());
}

fn grid_patches(
    transcript: &Output,
    n: &mut AdvApp2VarNetwork,
    iu: i32,
    iv: i32,
) -> Result<(), Failure> {
    int(transcript, n.nb_patch());
    int(transcript, n.nb_patch_in_u());
    int(transcript, n.nb_patch_in_v());
    for i in 1..=n.nb_patch_in_u() + 1 {
        real(transcript, n.u_parameter(i)?);
    }
    for j in 1..=n.nb_patch_in_v() + 1 {
        real(transcript, n.v_parameter(j)?);
    }
    for j in 1..=n.nb_patch_in_v() {
        for i in 1..=n.nb_patch_in_u() {
            let p = n.patch(i, j)?;
            [p.u0(), p.u1(), p.v0(), p.v1()]
                .into_iter()
                .for_each(|value| real(transcript, value));
            int(transcript, p.u_order());
            int(transcript, p.v_order());
        }
    }
    let first = n.first_not_approx();
    flag(transcript, first.is_some());
    int(transcript, first.unwrap_or(0));
    for i in 1..=n.nb_patch() {
        n.change_patch(i)?.change_nb_coeff(i + 2, (2 * i) % 7 + 1);
    }
    let (cu, cv) = n.same_degree(iu, iv);
    int(transcript, cu);
    int(transcript, cv);
    for i in 1..=n.nb_patch() {
        int(transcript, n.change_patch(i)?.nb_coeff_in_u());
        int(transcript, n.change_patch(i)?.nb_coeff_in_v());
    }
    Ok(())
}

fn grid_frame(
    transcript: &Output,
    n: &AdvApp2VarNetwork,
    f: &AdvApp2VarFramework,
) -> Result<(), Failure> {
    let nodes = (n.nb_patch_in_u() + 1) * (n.nb_patch_in_v() + 1);
    for i in 1..=nodes {
        xy(transcript, f.node(i)?.coord());
        int(transcript, f.node(i)?.u_order());
    }
    for j in 1..=n.nb_patch_in_v() + 1 {
        for i in 1..=n.nb_patch_in_u() {
            print_iso(
                transcript,
                f.iso_v(n.u_parameter(i)?, n.u_parameter(i + 1)?, n.v_parameter(j)?)?,
            );
            int(transcript, f.first_node(IsoType::IsoV, j, i));
            int(transcript, f.last_node(IsoType::IsoV, j, i));
        }
    }
    for i in 1..=n.nb_patch_in_u() + 1 {
        for j in 1..=n.nb_patch_in_v() {
            print_iso(
                transcript,
                f.iso_u(n.u_parameter(i)?, n.v_parameter(j)?, n.v_parameter(j + 1)?)?,
            );
            int(transcript, f.first_node(IsoType::IsoU, i, j));
            int(transcript, f.last_node(IsoType::IsoU, i, j));
        }
    }
    xy(
        transcript,
        f.node_at(n.u_parameter(1)?, n.v_parameter(n.nb_patch_in_v() + 1)?)?
            .coord(),
    );
    let first = f.first_not_approx();
    flag(transcript, first.is_some());
    let (i1, i2) = first.map_or((0, 0), |(_, a, b)| (a, b));
    int(transcript, i1);
    int(transcript, i2);
    Ok(())
}

fn grid_body(transcript: &Output, r: &mut Reader) -> Result<(), Failure> {
    let (iu, iv) = (r.integer(), r.integer());
    let d = [r.real(), r.real(), r.real(), r.real()];
    let mut g = Grid::new(iu, iv, d)?;
    g.cut(r)?;
    grid_patches(transcript, &mut g.net, iu, iv)?;
    grid_frame(transcript, &g.net, &g.frame)
}

/// aa_grid: iu iv u0 u1 v0 v1 cuts.
pub fn grid_case(transcript: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(transcript, || grid_body(transcript, &mut r));
    end(transcript);
}
