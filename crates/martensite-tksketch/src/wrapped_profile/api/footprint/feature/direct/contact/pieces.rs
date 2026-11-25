//! Associate each cubic support piece with its actual decoded contour and corners.
use super::{
    collar::Receipt,
    interval::{I, smooth_trig},
    *,
};
use std::collections::BTreeSet;

pub(in super::super) struct Piece {
    pub(super) edge: CylinderBodyRef,
    pub(super) span: u16,
    pub(super) contour: (CylinderBodyRef, u32),
    pub(super) endpoints: [Option<CylinderBodyRef>; 2],
    pub(super) uv: [[f64; 2]; 4],
}

pub(in super::super) struct WindowsInput {
    pub(in super::super) pieces: Vec<Piece>,
    pub(super) sin: [f64; 2],
    pub(super) radius: f64,
    pub(super) slant: [f64; 2],
    pub(super) rim: f64,
    pub(super) periodic: f64,
    pub(in super::super) window_threshold: f64,
}

impl SourceGraph {
    pub(in super::super) fn window_source(
        &self,
        attempt: Attempt,
        topology: &topology::Topology,
        chart: stock::StockChart,
        stock: &stock_binding::BoundStock,
        collars: &BTreeMap<CylinderBodyRef, Receipt>,
    ) -> FeatureChecked<WindowsInput> {
        let pieces = self.pieces(attempt, topology, stock.band)?;
        let max_of = |f: fn(&Receipt) -> f64| collars.values().map(f).fold(0.0_f64, f64::max);
        let collar = max_of(|r| r.all().envelope_upper);
        let travel = max_of(|r| r.all().from_support_curve_upper);
        let window_threshold = I::p(collar).add(I::p(travel)).hi;
        let [[_, angular_end], slant] = chart.sheet;
        let [cos, sin] = smooth_trig(chart.datum.semi_angle)
            .ok_or_else(|| refused(attempt, "finite support angle for contact-domain clearance"))?;
        let span = [control_range(&pieces, 0), control_range(&pieces, 1)];
        let minor_radius = I::p(chart.datum.radius).add(I::p(span[1][0]).mul(sin));
        let pad = I::p(travel).add(I::p(stock.spatial_upper));
        let rim = I::p(span[1][0])
            .sub(I::p(slant[0]))
            .mul(cos)
            .lo
            .min(I::p(slant[1]).sub(I::p(span[1][1])).mul(cos).lo);
        let [low, high] = span[0];
        if angular_end != std::f64::consts::TAU || !(0. < low && low <= high && high < 3.) {
            return Err(refused(
                attempt,
                "one open minor angular sheet for periodic clearance",
            ));
        }
        let lo_sin = |x| {
            smooth_trig(x)
                .map(|v| v[1].lo)
                .ok_or_else(|| refused(attempt, "actual periodic angular gap"))
        };
        let y_gap = minor_radius.mul(I::p(lo_sin(low)?.min(lo_sin(high)?))).lo;
        let [rim, periodic] = [rim, y_gap].map(|bound| I::p(bound).sub(pad).lo);
        if rim <= collar || periodic <= collar || window_threshold <= collar {
            return Err(refused(
                attempt,
                "separated stock rims and original periodic seam",
            ));
        }
        Ok(WindowsInput {
            pieces,
            sin: [sin.lo, sin.hi],
            radius: chart.datum.radius,
            slant,
            rim,
            periodic,
            window_threshold,
        })
    }

    fn pieces(
        &self,
        attempt: Attempt,
        topology: &topology::Topology,
        band: CylinderBodyRef,
    ) -> FeatureChecked<Vec<Piece>> {
        let mut pieces = Vec::new();
        let mut visited = BTreeSet::new();
        for &support in self.owner_tolerances.keys() {
            let wires = topology
                .wires(support)
                .ok_or_else(|| refused(attempt, "actual support-boundary wires"))?;
            let mut original = 0;
            for (wire_number, wire) in wires.iter().enumerate() {
                if wire.iter().all(|use_| !self.rails.contains_key(&use_.edge)) && support == band {
                    original += 1;
                    continue;
                }
                let contour = (
                    support,
                    u32::try_from(wire_number)
                        .map_err(|_| refused(attempt, "finite native support cycle ordinal"))?,
                );
                for edge in wire {
                    let rail = self
                        .rails
                        .get(&edge.edge)
                        .ok_or_else(|| refused(attempt, "authored support contour rail"))?;
                    if !visited.insert(edge.edge)
                        || rail.support != support
                        || rail.direction != edge.sense
                    {
                        return Err(refused(
                            attempt,
                            "unique native rail use on the bound stock contour",
                        ));
                    }
                    pieces.extend(cubics(attempt, edge.edge, rail, contour)?);
                }
            }
            if original != usize::from(support == band) {
                return Err(refused(
                    attempt,
                    "exactly one retained natural stock boundary",
                ));
            }
        }
        if visited.len() != self.rails.len() {
            return Err(refused(
                attempt,
                "all live rails belong to exact support contour",
            ));
        }
        Ok(pieces)
    }
}

fn cubics(
    attempt: Attempt,
    edge: CylinderBodyRef,
    rail: &Rail,
    contour: (CylinderBodyRef, u32),
) -> FeatureChecked<Vec<Piece>> {
    let spans = rail.knots.len() - 1;
    (0..spans)
        .map(|i| {
            let start = (i == 0).then_some(rail.corners[0]);
            let end = (i == spans - 1).then_some(rail.corners[1]);
            let span = u16::try_from(i)
                .map_err(|_| refused(attempt, "bounded proof rail polynomial count"))?;
            let uv = rail.poles[3 * i..3 * i + 4]
                .try_into()
                .map_err(|_| refused(attempt, "four stored cubic UV coefficients"))?;
            Ok(Piece {
                edge,
                span,
                contour,
                endpoints: [start, end],
                uv,
            })
        })
        .collect()
}

fn control_range(pieces: &[Piece], axis: usize) -> [f64; 2] {
    let values = pieces
        .iter()
        .flat_map(|p| p.uv.iter().map(move |xy| xy[axis]));
    [
        values.clone().fold(f64::INFINITY, f64::min),
        values.fold(f64::NEG_INFINITY, f64::max),
    ]
}
