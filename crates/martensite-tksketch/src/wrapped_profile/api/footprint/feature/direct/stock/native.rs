use super::*;

pub(super) fn stock_faces<'a>(body: &Shape, band: &'a Face) -> Checked<OwnedFaces<'a>> {
    let all = body_faces(body)?;
    if all.len() != 3 || all.iter().filter(|s| s.is_equal(band)).count() != 1 {
        return Err(StockRefusal::Body);
    }
    let mut caps = all.iter().filter(|s| !s.is_same(band));
    let a = topods::face(caps.next().ok_or(StockRefusal::Body)?)?;
    let b = topods::face(caps.next().ok_or(StockRefusal::Body)?)?;
    let (base, lid) = if a.orientation() == Reversed {
        (a, b)
    } else {
        (b, a)
    };
    Ok(OwnedFaces { band, base, lid })
}

fn body_faces(body: &Shape) -> Checked<Vec<Shape>> {
    if body.shape_type()? != ShapeEnum::Solid
        || body.orientation() != Forward
        || !body.location().is_identity()
    {
        return Err(StockRefusal::Body);
    }
    let [shell]: [_; 1] = children(body)?.try_into().map_err(|_| StockRefusal::Body)?;
    if shell.shape_type()? != ShapeEnum::Shell
        || shell.orientation() != Forward
        || !shell.location().is_identity()
        || !shell.closed()?
    {
        return Err(StockRefusal::Body);
    }
    children(&shell)
}

pub(super) struct OwnedFaces<'a> {
    band: &'a Face,
    base: Face,
    lid: Face,
}
impl OwnedFaces<'_> {
    pub(super) fn refs(&self) -> Faces<'_> {
        Faces {
            band: self.band,
            base: &self.base,
            lid: &self.lid,
        }
    }
}

fn children(shape: &Shape) -> Checked<Vec<Shape>> {
    let mut it = ShapeIterator::new(shape, true, true);
    let mut result = vec![];
    while it.more() {
        result.push(it.value()?.clone());
        it.next();
    }
    Ok(result)
}

fn wire_edges(wire: &Shape) -> Checked<Vec<Shape>> {
    if wire.shape_type()? != ShapeEnum::Wire
        || wire.orientation() != Forward
        || !wire.location().is_identity()
        || !wire.closed()?
    {
        return Err(StockRefusal::Band);
    }
    children(wire)
}

fn cap_edge(face: &Face, want: Orientation) -> Checked<Edge> {
    if face.orientation() != want || !face.location().is_identity() {
        return Err(StockRefusal::Disk);
    }
    let [wire]: [_; 1] = children(face)?.try_into().map_err(|_| StockRefusal::Disk)?;
    let [edge]: [_; 1] = wire_edges(&wire)?
        .try_into()
        .map_err(|_| StockRefusal::Disk)?;
    if edge.orientation() != want {
        return Err(StockRefusal::Incidence);
    }
    Ok(topods::edge(&edge)?)
}

fn cone_words(face: &Face) -> Checked<ConeWords> {
    let (surface, location) = BRepTool::surface_with_location(face)?;
    if face.orientation() != Forward || !face.location().is_identity() || !location.is_identity() {
        return Err(StockRefusal::Carrier);
    }
    let handle = surface.downcast::<GeomConicalSurface>();
    let cone = handle.get().ok_or(StockRefusal::Carrier)?.cone()?;
    let p = cone.position();
    Ok(ConeWords {
        frame: frame(
            *p.location().coordinates(),
            *p.x_direction().coordinates(),
            *p.y_direction().coordinates(),
            *p.direction().coordinates(),
        )?,
        radius: Word::from(cone.ref_radius())?,
        angle: Word::from(cone.semi_angle())?,
        tolerance: face_tol(face)?,
    })
}

fn face_tol(face: &Face) -> Checked<Word> {
    Word::from(
        face.tshape()
            .downcast_ref::<BRepTFace>()
            .ok_or(StockRefusal::Carrier)?
            .tolerance(),
    )
}

fn edge_curve(edge: &Edge) -> Checked<(martensite_tkbrep::brep::EdgeCurve3D, Word)> {
    let c = BRepTool::curve_with_location(edge)?;
    if !edge.location().is_identity()
        || !c.location.is_identity()
        || !BRepTool::same_parameter(edge)?
        || !BRepTool::same_range(edge)?
        || BRepTool::degenerated(edge)?
        || c.first != 0.
    {
        return Err(StockRefusal::Curve);
    }
    let tol = edge
        .tshape()
        .downcast_ref::<BRepTEdge>()
        .ok_or(StockRefusal::Curve)?
        .tolerance();
    Ok((c, Word::from(tol)?))
}

fn curve_vertices(edge: &Edge) -> Checked<[Vertex; 2]> {
    let [a, b]: [_; 2] = children(edge)?
        .try_into()
        .map_err(|_| StockRefusal::Incidence)?;
    let (a, b) = if a.orientation() == Forward {
        (a, b)
    } else {
        (b, a)
    };
    if a.orientation() != Forward
        || b.orientation() != Reversed
        || !a.location().is_identity()
        || !b.location().is_identity()
    {
        return Err(StockRefusal::Incidence);
    }
    Ok([topods::vertex(&a)?, topods::vertex(&b)?])
}

fn vertex_words(vertex: &Vertex) -> Checked<([Word; 3], Word)> {
    let tolerance = vertex
        .tshape()
        .downcast_ref::<BRepTVertex>()
        .ok_or(StockRefusal::Incidence)?
        .tolerance();
    Ok((
        words(*BRepTool::pnt(vertex)?.coordinates())?,
        Word::from(tolerance)?,
    ))
}

fn line_uv(face: &Face, edge: &Edge, closed: bool, expected: [f64; 6]) -> Checked<[Word; 6]> {
    let p = BRepTool::curve_on_face(edge, face)?;
    let line = p.pcurve.downcast::<Geom2dLine>();
    let line = line.get().ok_or(StockRefusal::Pcurve)?;
    let values = [
        p.first,
        p.last,
        line.location().x(),
        line.location().y(),
        line.direction().x(),
        line.direction().y(),
    ];
    if !p.is_stored || BRepTool::is_closed_on_face(edge, face)? != closed || values != expected {
        return Err(StockRefusal::Pcurve);
    }
    words(values)
}

fn circle_uv(face: &Face, edge: &Edge, radius: f64) -> Checked<[Word; 9]> {
    let p = BRepTool::curve_on_face(edge, face)?;
    let c = p.pcurve.downcast::<Geom2dCircle>();
    let c = c.get().ok_or(StockRefusal::Pcurve)?.circ2d()?;
    let values = [
        p.first,
        p.last,
        c.radius(),
        c.location().x(),
        c.location().y(),
        c.x_axis().direction().x(),
        c.x_axis().direction().y(),
        c.y_axis().direction().x(),
        c.y_axis().direction().y(),
    ];
    if !p.is_stored
        || BRepTool::is_closed_on_face(edge, face)?
        || values != [0., TAU, radius, 0., 0., 1., 0., 0., 1.]
    {
        return Err(StockRefusal::Pcurve);
    }
    words(values)
}

fn rim(
    band: &Face,
    cone_edge: &Edge,
    cap: &Face,
    cap_edge: &Edge,
    slant: f64,
) -> Checked<(RingWords, Vertex)> {
    let (surface, location) = BRepTool::surface_with_location(cap)?;
    let plane = surface.downcast::<GeomPlane>();
    let p = plane.get().ok_or(StockRefusal::Disk)?.pln().position();
    if !location.is_identity() {
        return Err(StockRefusal::Disk);
    }
    let plane = frame(
        *p.location().coordinates(),
        *p.x_direction().coordinates(),
        *p.y_direction().coordinates(),
        *p.direction().coordinates(),
    )?;
    let (source, edge_tol) = edge_curve(cap_edge)?;
    let circle = source.curve.downcast::<GeomCircle>();
    let c = circle.get().ok_or(StockRefusal::Curve)?.circ()?;
    let p = c.position();
    let circle = frame(
        *p.location().coordinates(),
        *p.x_direction().coordinates(),
        *p.y_direction().coordinates(),
        *p.direction().coordinates(),
    )?;
    if source.last != TAU
        || circle[0] != plane[0]
        || c.radius() <= 0.
        || !cap_edge.is_same(cone_edge)
        || cap_edge.orientation() == cone_edge.orientation()
    {
        return Err(StockRefusal::Disk);
    }
    let [v, w] = curve_vertices(cap_edge)?;
    if !v.is_same(&w) {
        return Err(StockRefusal::Incidence);
    }
    let [a, b] = curve_vertices(cone_edge)?;
    if !v.is_same(&a) || !v.is_same(&b) {
        return Err(StockRefusal::Incidence);
    }
    let (vertex, vertex_tol) = vertex_words(&v)?;
    Ok((
        RingWords {
            plane,
            plane_tol: face_tol(cap)?,
            circle,
            radius: Word::from(c.radius())?,
            edge_tol,
            vertex,
            vertex_tol,
            range: words([source.first, source.last])?,
            cone_uv: line_uv(band, cone_edge, false, [0., TAU, 0., slant, 1., 0.])?,
            plane_uv: circle_uv(cap, cap_edge, c.radius())?,
        },
        v,
    ))
}

fn bound_band(band: &Face, caps: [&Edge; 2], allow_extra: bool) -> Checked<(Shape, [Shape; 4])> {
    let mut wires = children(band)?;
    if !allow_extra && wires.len() != 1 {
        return Err(StockRefusal::Band);
    }
    let mut bound = None;
    for wire in wires.drain(..) {
        let edges = children(&wire)?;
        if !edges.iter().any(|e| caps.iter().any(|rim| e.is_same(rim))) {
            continue;
        }
        if bound.is_some() {
            return Err(StockRefusal::Incidence);
        }
        let boundary = wire_edges(&wire)?
            .try_into()
            .map_err(|_| StockRefusal::Band)?;
        bound = Some((wire, boundary));
    }
    bound.ok_or(StockRefusal::Band)
}

fn split_band(edges: [Shape; 4], caps: [&Edge; 2]) -> Checked<([Edge; 2], [Edge; 2])> {
    let mut matched: [Option<Edge>; 2] = [None, None];
    let mut seam: [Option<Edge>; 2] = [None, None];
    for shape in &edges {
        let edge = topods::edge(shape)?;
        let direction = match edge.orientation() {
            Forward => 0,
            Reversed => 1,
            _ => return Err(StockRefusal::Incidence),
        };
        let slot = match caps.iter().position(|cap| cap.is_same(&edge)) {
            Some(rim) => &mut matched[rim],
            None => &mut seam[direction],
        };
        if slot.replace(edge).is_some() {
            return Err(StockRefusal::Incidence);
        }
    }
    let [Some(low), Some(high)] = matched else {
        return Err(StockRefusal::Incidence);
    };
    let [Some(fwd), Some(rev)] = seam else {
        return Err(StockRefusal::Incidence);
    };
    if !fwd.is_same(&rev) {
        return Err(StockRefusal::Incidence);
    }
    Ok(([low, high], [fwd, rev]))
}

fn generator(face: &Face, pair: [&Edge; 2], ends: [&Vertex; 2], h: f64) -> Checked<GeneratorWords> {
    let (curve, edge_tol) = edge_curve(pair[0])?;
    let line = curve.curve.downcast::<GeomLine>();
    let p = line.get().ok_or(StockRefusal::Curve)?.position();
    let [a, b] = curve_vertices(pair[0])?;
    let [c, d] = curve_vertices(pair[1])?;
    if ends[0].is_same(ends[1])
        || !a.is_same(ends[0])
        || !b.is_same(ends[1])
        || !c.is_same(ends[1])
        || !d.is_same(ends[0])
    {
        return Err(StockRefusal::Incidence);
    }
    Ok(GeneratorWords {
        source: [
            words(*p.location().coordinates())?,
            words(*p.direction().coordinates())?,
        ],
        range: words([curve.first, curve.last])?,
        edge_tol,
        uv: [
            line_uv(face, pair[0], true, [0., h, TAU, 0., 0., 1.])?,
            line_uv(face, pair[1], true, [0., h, 0., 0., 0., 1.])?,
        ],
    })
}

fn same_stock_incidence(faces: &[Shape], stock: [&Edge; 3], anchors: [&Vertex; 2]) -> Checked<()> {
    let mut count = [0; 3];
    for face in faces {
        for wire in children(face)? {
            for edge in children(&wire)? {
                if let Some(i) = stock.iter().position(|s| s.is_same(&edge)) {
                    count[i] += 1;
                } else if children(&edge)?
                    .iter()
                    .any(|v| anchors.iter().any(|s| s.is_same(v)))
                {
                    return Err(StockRefusal::Incidence);
                }
            }
        }
    }
    if count != [2; 3] {
        return Err(StockRefusal::Incidence);
    }
    Ok(())
}

pub(super) fn capture(body: &Shape, f: Faces<'_>, allow_extra: bool) -> Checked<CapturedStock> {
    let all = body_faces(body)?;
    if [f.band, f.base, f.lid]
        .iter()
        .any(|face| all.iter().filter(|s| s.is_equal(face)).count() != 1)
    {
        return Err(StockRefusal::Incidence);
    }
    let base = cap_edge(f.base, Reversed)?;
    let lid = cap_edge(f.lid, Forward)?;
    if base.is_same(&lid) {
        return Err(StockRefusal::Incidence);
    }
    let (wire, band) = bound_band(f.band, [&base, &lid], allow_extra)?;
    let (ends, seam) = split_band(band, [&base, &lid])?;
    let h = edge_curve(&seam[0])?.0.last;
    if !h.is_finite() || h <= 0. {
        return Err(StockRefusal::Band);
    }
    let (low, low_v) = rim(f.band, &ends[0], f.base, &base, 0.)?;
    let (high, high_v) = rim(f.band, &ends[1], f.lid, &lid, h)?;
    let generator = generator(f.band, [&seam[0], &seam[1]], [&low_v, &high_v], h)?;
    same_stock_incidence(&all, [&base, &lid, &seam[0]], [&low_v, &high_v])?;
    let words = StockWords {
        cone: cone_words(f.band)?,
        ends: [low, high],
        seam: generator,
    };
    Ok(CapturedStock {
        words,
        boundary: OriginalBoundary {
            band: f.band.clone(),
            wire,
            rims: [base, lid],
            generator: seam[0].clone(),
            anchors: [low_v, high_v],
        },
    })
}
