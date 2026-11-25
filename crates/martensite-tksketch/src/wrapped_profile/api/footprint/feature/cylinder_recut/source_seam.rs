//! Bind the native seam branches to the exact closed rim anchors.
use super::*;

pub(super) struct Seam {
    pub(super) edge: Edge,
    pub(super) tolerance: f64,
    pub(super) order: [usize; 2],
    pub(super) base: usize,
}

impl Seam {
    pub(super) fn capture(
        attempt: Attempt,
        side: &Face,
        edges: &[Edge],
        carrier: &Carrier,
        ends: &[Rim; 2],
    ) -> FeatureChecked<Self> {
        let uses: Vec<_> = edges
            .iter()
            .filter(|edge| ends.iter().all(|rim| !edge.is_same(&rim.edge)))
            .collect();
        let [a, b] = uses.as_slice() else {
            return Err(refuse(attempt, "two uses of the single paired seam"));
        };
        if !a.is_same(b)
            || !opposite(a.orientation(), b.orientation())
            || !attempt.native(
                STAGE,
                "paired seam storage",
                BRepTool::is_closed_on_face(a, side),
            )?
        {
            return Err(refuse(
                attempt,
                "one original seam with opposite directed uses",
            ));
        }
        let forward = attempt.native(
            STAGE,
            "forward generator",
            topods::edge(&a.oriented(Orientation::Forward)),
        )?;
        let order = seam_order(attempt, side, &forward, carrier.height)?;
        let stored = curve(attempt, &forward)?;
        let base = seam_ends(attempt, &forward, ends)?;
        check_generator(attempt, &stored, carrier)?;
        let tolerance = attempt
            .native(
                STAGE,
                "original seam tolerance",
                edge_tshape(&forward, "cylinder recut"),
            )?
            .tolerance();
        Ok(Self {
            edge: forward,
            tolerance,
            order,
            base,
        })
    }
}

fn seam_order(
    attempt: Attempt,
    side: &Face,
    edge: &Edge,
    height: f64,
) -> FeatureChecked<[usize; 2]> {
    let native = attempt.native(
        STAGE,
        "forward native band query",
        topods::face(&side.oriented(Orientation::Forward)),
    )?;
    let back = attempt.native(
        STAGE,
        "reverse generator coedge",
        topods::edge(&edge.reversed()),
    )?;
    let trims = [
        trim(attempt, edge, &native)?,
        trim(attempt, &back, &native)?,
    ];
    if trims.iter().any(|t| [t.first, t.last] != [0., height]) {
        return Err(refuse(
            attempt,
            "same 3D and UV range for the original paired generator",
        ));
    }
    match [
        line(attempt, &trims[0], [0., -1.])?,
        line(attempt, &trims[1], [0., -1.])?,
    ] {
        [a, b] if a == [0., 0.] && b == [TAU, 0.] => Ok([0, 1]),
        [a, b] if a == [TAU, 0.] && b == [0., 0.] => Ok([1, 0]),
        _ => Err(refuse(
            attempt,
            "opposite native angular lifts at zero and one turn",
        )),
    }
}

fn seam_ends(attempt: Attempt, seam: &Edge, ends: &[Rim; 2]) -> FeatureChecked<usize> {
    let first = topexp::first_vertex(seam, false);
    let last = topexp::last_vertex(seam, false);
    let base = ends
        .iter()
        .position(|rim| rim.anchor.is_same(&first))
        .ok_or_else(|| refuse(attempt, "generator's original start vertex owns a rim"))?;
    if ends[base].v != 0. || !ends[1 - base].anchor.is_same(&last) || first.is_same(&last) {
        return Err(refuse(
            attempt,
            "generator runs between the exact original rim anchors",
        ));
    }
    Ok(base)
}

fn check_generator(
    attempt: Attempt,
    stored: &EdgeCurve3D,
    carrier: &Carrier,
) -> FeatureChecked<()> {
    let line = stored
        .curve
        .downcast_ref::<GeomLine>()
        .ok_or_else(|| refuse(attempt, "spatial generator is a stored straight line"))?;
    let line = line.lin();
    let [ox, oy, oz] = carrier.origin;
    let (x, _) = sum_with_residual(ox, carrier.radius);
    if [stored.first, stored.last] != [0., carrier.height]
        || *line.direction().coordinates() != UP
        || world_point(attempt, *line.location().coordinates(), &stored.location)? != [x, oy, oz]
    {
        return Err(refuse(
            attempt,
            "stock generator matches the original band's zero-angle line",
        ));
    }
    Ok(())
}
