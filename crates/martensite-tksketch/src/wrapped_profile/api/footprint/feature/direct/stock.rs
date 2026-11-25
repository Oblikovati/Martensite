//! Test-shaped stock certificate for the private direct frustum route.
use martensite_tkbrep::{
    brep::{BRepTEdge, BRepTFace, BRepTVertex, BRepTool},
    topods::{self, Edge, Face, Shape, ShapeIterator, Vertex},
};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dLine};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomConicalSurface, GeomLine, GeomPlane},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::predicates::roots::metric_bounds::ConicalTrimCarrier;
use std::f64::consts::TAU;
mod metric;
mod native;
mod refusal;

use ConicalStockFailure as StockRefusal;
use Orientation::{Forward, Reversed};
pub use refusal::ConicalStockFailure;
type Checked<T> = Result<T, StockRefusal>;

/// An immutable proof of the owned stock subgraph and a bound against a closed frustum.
pub(crate) struct StockFrustumReceipt {
    source: StockWords,
    spatial_upper: f64,
}

/// Actual original stock boundary returned only after its full source replay.
pub(in super::super) struct OriginalBoundary {
    pub(in super::super) band: Face,
    pub(in super::super) wire: Shape,
    pub(in super::super) rims: [Edge; 2],
    pub(in super::super) generator: Edge,
    pub(in super::super) anchors: [Vertex; 2],
}

struct CapturedStock {
    words: StockWords,
    boundary: OriginalBoundary,
}

#[derive(Clone, Copy)]
pub(crate) struct Faces<'a> {
    pub(crate) band: &'a Face,
    pub(crate) base: &'a Face,
    pub(crate) lid: &'a Face,
}

/// By-value descriptor borrowed from the existing stock Carrier capture.
#[derive(Clone, Copy)]
pub(crate) struct StockChart {
    pub(crate) datum: ConicalTrimCarrier,
    pub(crate) sheet: [[f64; 2]; 2],
    pub(crate) sew: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Word(u64);

impl Word {
    fn from(x: f64) -> Checked<Self> {
        if !x.is_finite() {
            return Err(StockRefusal::NonFinite(x));
        }
        Ok(Self(x.to_bits()))
    }
    fn get(self) -> f64 {
        f64::from_bits(self.0)
    }
}

fn words<const N: usize>(input: [f64; N]) -> Checked<[Word; N]> {
    let mut result = [Word(0); N];
    for (out, value) in result.iter_mut().zip(input) {
        *out = Word::from(value)?;
    }
    Ok(result)
}

type Frame = [[Word; 3]; 4];

fn frame(origin: [f64; 3], x: [f64; 3], y: [f64; 3], z: [f64; 3]) -> Checked<Frame> {
    if x != [1., 0., 0.] || y != [0., 1., 0.] || z != [0., 0., 1.] || origin[..2] != [0., 0.] {
        return Err(StockRefusal::Carrier);
    }
    Ok([words(origin)?, words(x)?, words(y)?, words(z)?])
}

#[derive(Debug, PartialEq, Eq)]
struct ConeWords {
    frame: Frame,
    radius: Word,
    angle: Word,
    tolerance: Word,
}

#[derive(Debug, PartialEq, Eq)]
struct RingWords {
    plane: Frame,
    plane_tol: Word,
    circle: Frame,
    radius: Word,
    edge_tol: Word,
    vertex: [Word; 3],
    vertex_tol: Word,
    range: [Word; 2],
    cone_uv: [Word; 6],
    plane_uv: [Word; 9],
}

#[derive(Debug, PartialEq, Eq)]
struct GeneratorWords {
    source: [[Word; 3]; 2],
    range: [Word; 2],
    edge_tol: Word,
    uv: [[Word; 6]; 2],
}

#[derive(Debug, PartialEq, Eq)]
struct StockWords {
    cone: ConeWords,
    ends: [RingWords; 2],
    seam: GeneratorWords,
}

impl StockFrustumReceipt {
    pub(crate) fn issue(body: &Shape, band: &Face, chart: StockChart) -> Checked<Self> {
        let faces = native::stock_faces(body, band)?;
        let source = native::capture(body, faces.refs(), false)?.words;
        let carrier = [
            Word::from(chart.datum.radius)?,
            Word::from(chart.datum.axial_origin)?,
            Word::from(chart.datum.semi_angle)?,
        ];
        if carrier
            != [
                source.cone.radius,
                source.cone.frame[0][2],
                source.cone.angle,
            ]
            || chart.sheet != [[0., TAU], [0., source.seam.range[1].get()]]
        {
            return Err(StockRefusal::Carrier);
        }
        let spatial_upper = metric::prove(&source, Word::from(chart.sew)?)?;
        Ok(Self {
            source,
            spatial_upper,
        })
    }

    pub(crate) fn spatial_upper(&self) -> f64 {
        self.spatial_upper
    }

    /// Recheck exact stored geometry and within-result owner incidence using archive-verified images.
    pub(in super::super) fn rebind(
        &self,
        decoded_body: &Shape,
        decoded_stock: Faces<'_>,
    ) -> Checked<OriginalBoundary> {
        let decoded = native::capture(decoded_body, decoded_stock, true)?;
        if decoded.words != self.source {
            return Err(StockRefusal::SourceChanged);
        }
        Ok(decoded.boundary)
    }
}
