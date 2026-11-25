//! Assemble one selected stock shell without a private Boolean or geometry mutation of inputs.
use super::material::{Admitted, Tool};
use super::*;
use martensite_tkbrep::{
    brep::{BRepBuilder, access::edge_tshape},
    brep_tools::outer_wire,
    topexp::Explorer,
    topods::{self, Builder, ShapeIterator},
};
mod geometry;
pub(super) use geometry::{CylinderDatum, exact_translation, polynomial_trace};

pub(super) const ASSEMBLY: CylinderFeatureStage = CylinderFeatureStage::Boolean;

pub(super) struct Candidate {
    pub bytes: Vec<u8>,
    pub history: CylinderFeatureHistory,
    pub native: DetachedCylinderFeature,
    pub original: proof::Trims,
    pub stock_sheet: [f64; 2],
    pub native_stock_error: f64,
    pub recut_clearance: Option<f64>,
    pub height_sign: f64,
}

pub(super) fn build(
    attempt: Attempt,
    input: &Admitted,
    tool: &Tool,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Candidate> {
    let original = proof::Trims::capture(attempt, input, tool, cancelled)?;
    let boundary = StockBoundary::capture(attempt, input, original.angular_hull())?;
    let (stock, band) = boundary.parts(input);
    let mut assembly = Shell::from_stock(attempt, stock, band, input.axial, boundary.sheet())?;
    for &region in &tool.regions {
        attempt.poll(cancelled, ASSEMBLY)?;
        let solid = named(attempt, &tool.native, region, HistoryRole::Solid)?;
        let near = named(attempt, &tool.native, region, HistoryRole::FirstDepthFace)?;
        assembly.attach(attempt, solid, near, cancelled)?;
    }
    freeze(
        attempt, input, tool, original, assembly, &boundary, cancelled,
    )
}

enum StockBoundary {
    Unchanged(cylinder_recut::UnchangedStock),
    Recut(cylinder_recut::RecutStock),
}

impl StockBoundary {
    const ORIGINAL_SHEET: [f64; 2] = [0., std::f64::consts::TAU];

    fn capture(attempt: Attempt, input: &Admitted, hull: [f64; 2]) -> Checked<Self> {
        let [min, max] = Self::ORIGINAL_SHEET;
        let sew = input.resolution.sew;
        if hull[0] > min && hull[1] < max {
            let stock =
                cylinder_recut::certify_unchanged(attempt, &input.solid, &input.stock.face, sew)?;
            Ok(Self::Unchanged(stock))
        } else {
            let stock =
                cylinder_recut::prepare(attempt, &input.stock.body, &input.stock.face, hull, sew)?;
            Ok(Self::Recut(stock))
        }
    }

    fn parts<'a>(&'a self, input: &'a Admitted) -> (&'a Shape, &'a Face) {
        match self {
            Self::Unchanged(_) => (&input.solid, &input.stock.face),
            Self::Recut(stock) => (&stock.solid, &stock.band),
        }
    }

    fn sheet(&self) -> [f64; 2] {
        match self {
            Self::Unchanged(_) => Self::ORIGINAL_SHEET,
            Self::Recut(stock) => stock.sheet,
        }
    }

    fn recut_clearance(&self) -> Option<f64> {
        match self {
            Self::Unchanged(_) => None,
            Self::Recut(stock) => Some(stock.clear_distance),
        }
    }

    fn error(&self) -> f64 {
        match self {
            Self::Unchanged(stock) => stock.spatial_upper(),
            Self::Recut(stock) => stock.spatial_upper,
        }
    }
}

fn freeze(
    attempt: Attempt,
    input: &Admitted,
    tool: &Tool,
    original: proof::Trims,
    assembly: Shell,
    boundary: &StockBoundary,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Candidate> {
    let band = assembly.band.clone();
    let height_sign = assembly.frame.height_sign;
    let (solid, replacement) = assembly.finish(attempt)?;
    let captured = if let StockBoundary::Recut(recut) = boundary {
        history::capture_recut_direct(
            attempt,
            &input.stock.face,
            &input.stock.body,
            &tool.native,
            history::RecutResult {
                solid,
                replacement: &replacement,
                recut,
            },
            cancelled,
        )?
    } else {
        history::capture_direct(
            attempt,
            &input.stock.face,
            &input.stock.body,
            &tool.native,
            solid,
            &replacement,
            cancelled,
        )?
    };
    let inheritance = match boundary {
        StockBoundary::Unchanged(stock) => stock.bind_final(attempt, &captured.index, &band)?,
        StockBoundary::Recut(stock) => stock.bind_final(attempt, &captured.index, &band)?,
    };
    let (bytes, native) = body::detach(attempt, &captured, cancelled)?;
    inheritance.rebind(attempt, &native)?;
    Ok(Candidate {
        bytes,
        history: captured.history,
        native,
        original,
        stock_sheet: boundary.sheet(),
        native_stock_error: boundary.error(),
        recut_clearance: boundary.recut_clearance(),
        height_sign,
    })
}

struct Shell {
    shell: topods::Shell,
    band: Face,
    images: Vec<Shape>,
    frame: CylinderDatum,
    axial: [f64; 2],
    sheet: [f64; 2],
}

impl Shell {
    fn from_stock(
        attempt: Attempt,
        stock: &Shape,
        band: &Face,
        axial: [f64; 2],
        sheet: [f64; 2],
    ) -> Checked<Self> {
        let all = faces(attempt, stock)?;
        let [wire]: [Shape; 1] = certify(
            attempt,
            ASSEMBLY,
            "one stock frame wire",
            children(attempt, band)?.try_into(),
        )?;
        let frame = CylinderDatum::read(attempt, band)?;
        let directed = chart_orientation(band.orientation(), frame.height_sign < 0.);
        require(
            attempt,
            ASSEMBLY,
            all.len() == 3 && directed == Orientation::Forward,
            "one forward physical cylinder band and two ends",
        )?;
        let mut shell = topods::Shell::default();
        Builder.make_shell(&mut shell);
        for face in all.into_iter().filter(|face| !face.is_same(band)) {
            attempt.native(
                ASSEMBLY,
                "retain exactly directed stock end",
                Builder.add(&mut shell, &face),
            )?;
        }
        let mut band = empty_face(attempt, band)?;
        attempt.native(
            ASSEMBLY,
            "retain stock frame incidence",
            Builder.add(&mut band, &wire),
        )?;
        Ok(Self {
            shell,
            band,
            images: Vec::new(),
            frame,
            axial,
            sheet,
        })
    }

    fn attach(
        &mut self,
        attempt: Attempt,
        solid: &Shape,
        near: &Shape,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let faces = faces(attempt, solid)?;
        let cap = directed_cap(attempt, &faces, near)?;
        let carrier = CylinderDatum::read(attempt, &cap)?;
        require(
            attempt,
            ASSEMBLY,
            carrier.origin == self.frame.origin
                && carrier.radius.to_bits() == self.frame.radius.to_bits()
                && carrier.height_sign == 1.,
            "identical stock and zero-depth space chart",
        )?;
        let outer = attempt.native(ASSEMBLY, "outer near cap wire", outer_wire(&cap))?;
        for wire in children(attempt, &cap)? {
            let directed = occurrence(attempt.kind, &wire);
            let chart = Transfer {
                frame: self.frame,
                axial: self.axial,
                sheet: self.sheet,
            };
            if wire.is_same(&outer) {
                chart.wire(attempt, &cap, &mut self.band, &directed, cancelled)?;
            } else {
                self.add_counter(attempt, chart, &cap, &directed, cancelled)?;
            }
        }
        for face in faces.into_iter().filter(|face| !face.is_same(near)) {
            attempt.native(
                ASSEMBLY,
                "retain directed tool side or far cap",
                Builder.add(&mut self.shell, &occurrence(attempt.kind, &face)),
            )?;
        }
        Ok(())
    }

    fn add_counter(
        &mut self,
        attempt: Attempt,
        chart: Transfer,
        cap: &Face,
        wire: &Shape,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let mut island = empty_face(attempt, &self.band)?;
        chart.wire(attempt, cap, &mut island, wire, cancelled)?;
        attempt.native(
            ASSEMBLY,
            "retain counter island",
            Builder.add(&mut self.shell, &island),
        )?;
        self.images.push(island.to_shape());
        Ok(())
    }

    fn finish(mut self, attempt: Attempt) -> Checked<(Shape, Vec<Shape>)> {
        attempt.native(
            ASSEMBLY,
            "retain stock remainder",
            Builder.add(&mut self.shell, &self.band),
        )?;
        attempt.native(
            ASSEMBLY,
            "declare candidate shell closed",
            self.shell.set_closed(true),
        )?;
        let mut solid = topods::Solid::default();
        Builder.make_solid(&mut solid);
        attempt.native(
            ASSEMBLY,
            "one attached candidate solid",
            Builder.add(&mut solid, &self.shell),
        )?;
        self.images.push(self.band.to_shape());
        Ok((solid.to_shape(), self.images))
    }
}

#[derive(Clone, Copy)]
struct Transfer {
    frame: CylinderDatum,
    axial: [f64; 2],
    sheet: [f64; 2],
}

impl Transfer {
    fn wire(
        self,
        attempt: Attempt,
        source: &Face,
        target: &mut Face,
        wire: &Shape,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        for use_ in children(attempt, wire)? {
            attempt.poll(cancelled, ASSEMBLY)?;
            let edge =
                attempt.native(ASSEMBLY, "actual source boundary edge", topods::edge(&use_))?;
            self.trace(attempt, source, target, &edge)?;
        }
        attempt.native(
            ASSEMBLY,
            "install complete directed source ring",
            Builder.add(target, wire),
        )
    }

    fn trace(
        self,
        attempt: Attempt,
        source: &Face,
        target: &Face,
        edge: &topods::Edge,
    ) -> Checked<()> {
        let (original, controls) = polynomial_trace(attempt, source, edge)?;
        let mapped: Vec<_> = controls
            .into_iter()
            .map(|[u, v]| [u, self.frame.height_sign * v])
            .collect();
        require(
            attempt,
            ASSEMBLY,
            mapped.iter().all(|&[u, v]| {
                u > self.sheet[0] && u < self.sheet[1] && v > self.axial[0] && v < self.axial[1]
            }),
            "whole transferred trim inside the native stock lift",
        )?;
        let installed = if self.frame.height_sign == 1. {
            original
        } else {
            attempt.native(
                ASSEMBLY,
                "exact reflected stock pcurve",
                crate::surface_sketch::wrapped_profile::boundaries::curve2(&mapped),
            )?
        };
        let tol = attempt
            .native(
                ASSEMBLY,
                "raw source edge tolerance",
                edge_tshape(edge, "cylinder attachment seam"),
            )?
            .tolerance();
        let builder = BRepBuilder::default();
        attempt.native(
            ASSEMBLY,
            "transfer same-parameter seam curve",
            builder.update_edge_pcurve_on_face(edge, &installed, target, tol),
        )?;
        attempt.native(
            ASSEMBLY,
            "preserve full native seam interval",
            builder.range_on_face(edge, target, 0., 1.),
        )?;
        let (retained, words) = polynomial_trace(attempt, target, edge)?;
        require(
            attempt,
            ASSEMBLY,
            retained == installed && words == mapped,
            "exact installed stock pcurve binding",
        )
    }
}

pub(super) fn directed_cap(attempt: Attempt, faces: &[Shape], named: &Shape) -> Checked<Face> {
    let mut matches = faces.iter().filter(|face| face.is_same(named));
    let candidate = matches
        .next()
        .ok_or_else(|| missing(attempt, ASSEMBLY, "one directed named cap occurrence"))?;
    require(
        attempt,
        ASSEMBLY,
        matches.next().is_none(),
        "unique directed named cap occurrence",
    )?;
    attempt.native(ASSEMBLY, "native cap face", topods::face(candidate))
}

fn empty_face(attempt: Attempt, face: &Face) -> Checked<Face> {
    let copy = attempt.native(
        ASSEMBLY,
        "copy stock carrier without old wire",
        face.empty_copied(),
    )?;
    attempt.native(ASSEMBLY, "fresh working stock face", topods::face(&copy))
}

pub(super) fn occurrence(kind: CylinderFeatureKind, shape: &Shape) -> Shape {
    match kind {
        CylinderFeatureKind::Boss => shape.clone(),
        CylinderFeatureKind::Pocket => shape.reversed(),
    }
}

pub(super) fn children(attempt: Attempt, parent: &Shape) -> Checked<Vec<Shape>> {
    let mut it = ShapeIterator::new(parent, true, true);
    let mut children = Vec::new();
    while it.more() {
        children.push(
            attempt
                .native(ASSEMBLY, "cumulative native occurrence", it.value())?
                .to_shape(),
        );
        it.next();
    }
    Ok(children)
}

pub(super) fn faces(attempt: Attempt, root: &Shape) -> Checked<Vec<Shape>> {
    Ok(attempt
        .native(
            ASSEMBLY,
            "native face occurrences",
            Explorer::new(root, ShapeEnum::Face, ShapeEnum::Shape),
        )?
        .map(|face| face.to_shape())
        .collect())
}

pub(super) fn named(
    attempt: Attempt,
    tool: &DetachedWrappedBodies,
    region: SurfaceRegionId,
    role: HistoryRole,
) -> Checked<&Shape> {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Region(region),
        role,
    };
    tool.entities.get(&key).ok_or_else(|| {
        attempt.history(
            CylinderFeatureSource::Tool(key),
            "exact named region entity",
        )
    })
}
