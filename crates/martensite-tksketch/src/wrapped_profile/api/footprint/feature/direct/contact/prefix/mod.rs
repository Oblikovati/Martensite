//! Conditional closed-trim contact composition for actual stock/ruled-side sources.
mod metric;
mod numeric;
use numeric::*;
mod scalar;
use scalar::I;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Face(pub u32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Edge(pub u32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex(pub u32);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PieceId(pub Edge, pub u16);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Loop(pub Face, pub u32);

#[derive(Clone, Copy)]
pub struct Piece {
    pub id: PieceId,
    pub contour: Loop,
    pub endpoints: [Option<Vertex>; 2],
    pub uv: [[f64; 2]; 4],
}

#[derive(Clone, Copy)]
pub struct Side {
    pub edge: Edge,
    pub face: Face,
    pub face_tolerance: f64,
    pub rail_tolerance: f64,
    pub ruling_tolerance: [f64; 2],
    pub vertex_tolerance: [f64; 2],
    pub vertex_corner_upper: [f64; 2],
    pub depth_upper: f64,
    pub signed_depth_rate_lower: f64,
    pub speed_upper: f64,
    pub uv_anchor_pad_upper: [f64; 2],
    pub xyz_anchor_pad_upper: f64,
    pub sew_upper: f64,
    pub own_ratio_upper: f64,
    pub own_margin_lower: f64,
}

#[derive(Clone, Copy)]
pub struct Cone {
    pub radius: f64,
    pub sin: [f64; 2],
    // The original support sheet excludes the apex and spans this slant interval.
    pub slant: [f64; 2],
    pub chart_angle: [f64; 2],
}

#[derive(Clone, Copy)]
pub struct StockFace {
    pub id: Face,
    pub tolerance: f64,
}

#[derive(Clone, Copy)]
pub struct Separation {
    pub all_nonincident_lower: f64,
    pub all_cross_contour_lower: f64,
    pub all_foreign_face_lower: f64,
    pub native_rim_lower: f64,
    pub native_periodic_lower: f64,
    pub excluded_loop_pairs: usize,
    pub endpoint_window_threshold: f64,
    pub weighted_angle_scale: f64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Incidence(pub [PieceId; 2], pub Vertex);

#[derive(Clone, Copy)]
pub struct Window {
    pub pair: Incidence,
    // Oriented toward the actual shared endpoint; fraction of this cubic piece.
    pub fraction: f64,
}

// One radial retraction proof's arithmetic inputs, independently derived from
// restricted source and target curves, with chart carrier evaluation bounds.
#[derive(Clone, Copy)]
pub struct ArmProof {
    pub pair: Incidence,
    pub source_end: usize,
    pub window_fraction: f64,
    pub linear_min_lower: f64,
    pub linear_sum_upper: f64,
    pub quadratic_upper: f64,
    pub derivative_remainder_upper: f64,
    pub anchor_upper: f64,
}

pub struct CensusInput<'a> {
    pub cone: Cone,
    pub pieces: &'a [Piece],
    // Outward upper threshold used for every contact and endpoint outside-window exclusion.
    pub endpoint_window_threshold: f64,
    pub native_rim_lower: f64,
    pub native_periodic_lower: f64,
}

pub struct Census {
    pub separation: Separation,
    pub windows: Vec<Window>,
    pub pairs: usize,
    pub split_visits: usize,
}

pub fn census(input: CensusInput<'_>) -> Result<Census, Failure> {
    let charts = piece_index(input.pieces)?;
    let incident = full_incidence(&charts)?;
    let (ranges, loops) = chart_ranges(&charts);
    check_box(ranges, input.cone, [I::p(0.); 2])?;
    metric::census(input, &charts, &incident, ranges, loops)
}

pub struct Input<'a> {
    pub cone: Cone,
    pub stock: &'a [StockFace],
    pub sides: &'a [Side],
    pub pieces: &'a [Piece],
    pub windows: &'a [Window],
    pub arms: &'a [ArmProof],
    pub separation: Separation,
    pub fuzzy: f64,
    // Existing numerical proof domain, built from actual max entity tolerance.
    pub collar_threshold: f64,
    pub continuation_floor: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    InvalidScalar,
    Identity,
    Census,
    Pad,
    NonConvex,
    Separation,
    Core,
}

#[derive(Clone, Copy, Debug)]
pub struct Summary {
    pub pieces: usize,
    pub sides: usize,
    pub faces: usize,
    pub incident_cases: usize,
    pub closed_piece_pairs: usize,
    pub interval_split_visits: usize,
    pub threshold_min: [f64; 4],
    pub threshold_max: [f64; 4],
    pub prefix_hit_upper: f64,
    pub foreign_cross_upper: f64,
    pub normal_half_hessian_lower: f64,
    pub vertex_half_hessian_lower: f64,
    pub own_core_upper: f64,
    pub incident_core_upper: f64,
}

#[derive(Clone, Copy)]
struct Ranges {
    lo: [f64; 2],
    hi: [f64; 2],
}

struct Geometry {
    side_faces: BTreeMap<Edge, (Face, [Vertex; 2])>,
    charts: BTreeMap<PieceId, Piece>,
    incident: BTreeSet<Incidence>,
    ranges: Ranges,
    loops: usize,
}

impl Geometry {
    fn capture(input: &Input<'_>) -> Result<Self, Failure> {
        let charts = piece_index(input.pieces)?;
        let side_faces = side_index(input, &charts)?;
        let incident = full_incidence(&charts)?;
        let (ranges, loops) = chart_ranges(&charts);
        Ok(Self {
            side_faces,
            charts,
            incident,
            ranges,
            loops,
        })
    }
}

fn chart_ranges(charts: &BTreeMap<PieceId, Piece>) -> (Ranges, usize) {
    let mut ranges = Ranges {
        lo: [f64::INFINITY; 2],
        hi: [f64::NEG_INFINITY; 2],
    };
    let mut loops = BTreeSet::new();
    for piece in charts.values() {
        loops.insert(piece.contour);
        for uv in piece.uv {
            for (axis, &coordinate) in uv.iter().enumerate() {
                ranges.lo[axis] = ranges.lo[axis].min(coordinate);
                ranges.hi[axis] = ranges.hi[axis].max(coordinate);
            }
        }
    }
    (ranges, loops.len())
}

fn piece_index(pieces: &[Piece]) -> Result<BTreeMap<PieceId, Piece>, Failure> {
    let mut map = BTreeMap::new();
    for &piece in pieces {
        if piece.uv.iter().flatten().any(|n| !n.is_finite()) {
            return Err(Failure::InvalidScalar);
        }
        if map.insert(piece.id, piece).is_some() {
            return Err(Failure::Identity);
        }
    }
    if map.is_empty() {
        return Err(Failure::Census);
    }
    Ok(map)
}

fn side_index(
    input: &Input<'_>,
    charts: &BTreeMap<PieceId, Piece>,
) -> Result<BTreeMap<Edge, (Face, [Vertex; 2])>, Failure> {
    let mut side_faces = BTreeMap::new();
    let valid_faces: BTreeSet<_> = input.stock.iter().map(|face| face.id).collect();
    if valid_faces.len() != input.stock.len() {
        return Err(Failure::Identity);
    }
    for side in input.sides {
        if !valid_faces.contains(&side.face) {
            return Err(Failure::Identity);
        }
        let mut native_pieces = charts.range(PieceId(side.edge, 0)..=PieceId(side.edge, u16::MAX));
        let (_, first) = native_pieces.next().ok_or(Failure::Identity)?;
        let mut previous = first;
        let contour = first.contour;
        if first.id.1 != 0 || contour.0 != side.face {
            return Err(Failure::Identity);
        }
        for (_, this) in native_pieces {
            if this.id.1 != previous.id.1 + 1
                || this.contour != contour
                || previous.endpoints[1].is_some()
                || this.endpoints[0].is_some()
                || previous.uv[3].map(f64::to_bits) != this.uv[0].map(f64::to_bits)
            {
                return Err(Failure::Identity);
            }
            previous = this;
        }
        let a = first.endpoints[0].ok_or(Failure::Identity)?;
        let b = previous.endpoints[1].ok_or(Failure::Identity)?;
        if a == b || side_faces.insert(side.edge, (side.face, [a, b])).is_some() {
            return Err(Failure::Identity);
        }
    }
    if charts
        .keys()
        .any(|piece| !side_faces.contains_key(&piece.0))
        || side_faces
            .values()
            .map(|(face, _)| *face)
            .collect::<BTreeSet<_>>()
            != valid_faces
    {
        return Err(Failure::Identity);
    }
    Ok(side_faces)
}

fn full_incidence(charts: &BTreeMap<PieceId, Piece>) -> Result<BTreeSet<Incidence>, Failure> {
    let mut tips = BTreeMap::<Vertex, Vec<&Piece>>::new();
    for p in charts.values() {
        for &id in p.endpoints.iter().flatten() {
            tips.entry(id).or_default().push(p);
        }
    }
    tips.into_iter()
        .map(|(v, arms)| {
            let [a, b] = arms.as_slice() else {
                return Err(Failure::Identity);
            };
            if a.id.0 == b.id.0 || a.contour != b.contour {
                return Err(Failure::Identity);
            }
            let uv_a = if a.endpoints[0] == Some(v) {
                a.uv[0]
            } else {
                a.uv[3]
            };
            let uv_b = if b.endpoints[0] == Some(v) {
                b.uv[0]
            } else {
                b.uv[3]
            };
            if uv_a.map(f64::to_bits) != uv_b.map(f64::to_bits) {
                return Err(Failure::Identity);
            }
            let mut ids = [a.id, b.id];
            ids.sort();
            Ok(Incidence(ids, v))
        })
        .collect()
}

pub fn certify(input: Input<'_>) -> Result<Summary, Failure> {
    let geo = Geometry::capture(&input)?;
    let tolerances = thresholds(&input)?;
    let bounds = convexity(&input, &geo)?;
    let guarded = vertex_guard(&input);
    let hit_upper = (I::p(tolerances.max()) + I::p(max_wall(&input)) + I::p(guarded)).1;
    let metric = metric::prove(&input, &geo)?;
    verify_separations(&input, &geo, bounds, metric, hit_upper, tolerances.max())?;
    if !positive(input.continuation_floor) || input.continuation_floor > tolerances.min() {
        return Err(Failure::InvalidScalar);
    }
    let own = own_cores(&input)?;
    let incident = incident_cores(&input, &geo)?;
    if [own, incident]
        .into_iter()
        .any(|core| core >= input.continuation_floor)
    {
        return Err(Failure::Core);
    }
    Ok(Summary {
        pieces: geo.charts.len(),
        sides: geo.side_faces.len(),
        faces: input.stock.len(),
        incident_cases: 2 * geo.incident.len(),
        closed_piece_pairs: metric.pairs,
        interval_split_visits: metric.split_visits,
        threshold_min: tolerances.lo,
        threshold_max: tolerances.hi,
        prefix_hit_upper: hit_upper,
        foreign_cross_upper: foreign_upper(
            metric.angle_scale,
            bounds.angular_metric_lower,
            hit_upper,
        ),
        normal_half_hessian_lower: bounds.moving_hessian,
        vertex_half_hessian_lower: bounds.vertex_hessian,
        own_core_upper: own,
        incident_core_upper: incident,
    })
}
