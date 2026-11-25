//! Rebind the closed set of ruled side faces to both cap traces and their exact shared connectors.
use super::{contact, trims::Trace, *};
use crate::surface_sketch::CylinderSupport;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkg3d::geom::GeomBezierSurface;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CylindricalRadialSense, CylindricalSliceQuery, MetricBezier,
    certify_cylindrical_slice_direction,
};

mod boundary;
mod separation;

pub(super) type CapTraces = [BTreeMap<WrappedHistoryKey, Trace>; 2];

pub(super) fn validate(
    attempt: Attempt,
    input: &Admitted,
    tool: &Tool,
    candidate: &Candidate,
    index: &body::Index,
    caps: &CapTraces,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<f64> {
    let images = Images::read(attempt, candidate)?;
    let common = separation::Cover::read(attempt, input.support, tool, candidate)?;
    let sense = match attempt.kind {
        CylinderFeatureKind::Boss => CylindricalRadialSense::Outward,
        CylinderFeatureKind::Pocket => CylindricalRadialSense::Inward,
    };
    let parts: Vec<_> = tool.spans.iter().collect();
    let checked = work::ordered(
        attempt,
        VALIDATION,
        &parts,
        cancelled,
        |&(&key, stored), poll| {
            let part = Part::read(attempt, &images, caps, key)?;
            let tube = part.bind(attempt, input.support, sense, stored, common, poll)?;
            Ok((key, part, tube))
        },
    )?;
    let mut boundary = boundary::Boundary::new(index, candidate);
    let mut tubes = BTreeMap::new();
    for (key, part, tube) in checked {
        attempt.poll(cancelled, VALIDATION)?;
        boundary.add(attempt, key, &part.face, part.edges, sense)?;
        tubes.insert(key, tube);
    }
    let actual = boundary.pairs(attempt)?;
    separation::validate(attempt, tool, common, &actual)?;
    contact::certify(
        attempt,
        input.support,
        common.contact,
        &tubes,
        &tool.joins,
        cancelled,
    )?;
    Ok(common.contact)
}

struct Images<'a> {
    entries: BTreeMap<WrappedHistoryKey, &'a Shape>,
    cap_floor: [f64; 2],
}

impl<'a> Images<'a> {
    fn read(attempt: Attempt, candidate: &'a Candidate) -> Checked<Self> {
        let mut result = Self {
            entries: BTreeMap::new(),
            cap_floor: [f64::INFINITY; 2],
        };
        for (&identity, parents) in &candidate.history.images {
            let shape = entity(attempt, &candidate.native, identity)?;
            for parent in parents {
                result.add(attempt, shape, parent.source)?;
            }
        }
        require(
            attempt,
            VALIDATION,
            result.cap_floor.into_iter().all(valid_tol),
            "positive finite continuation floor of each cap class",
        )?;
        Ok(result)
    }

    fn add(
        &mut self,
        attempt: Attempt,
        shape: &'a Shape,
        source: CylinderFeatureSource,
    ) -> Checked<()> {
        if let CylinderFeatureSource::Tool(key) = source {
            require(
                attempt,
                VALIDATION,
                self.entries.insert(key, shape).is_none(),
                "single decoded image for every retained tool entity",
            )?;
        }
        let depth = match source {
            CylinderFeatureSource::SupportFace(_) => 0,
            CylinderFeatureSource::Tool(WrappedHistoryKey {
                role: HistoryRole::LastDepthFace,
                ..
            }) => 1,
            _ => return Ok(()),
        };
        let face = attempt.native(
            VALIDATION,
            "actual cap continuation face",
            topods::face(shape),
        )?;
        let raw = face_tolerance(attempt, &face)?;
        self.cap_floor[depth] = self.cap_floor[depth].min(raw);
        Ok(())
    }

    fn shape(&self, attempt: Attempt, key: WrappedHistoryKey) -> Checked<&'a Shape> {
        self.entries.get(&key).copied().ok_or_else(|| {
            attempt.history(
                CylinderFeatureSource::Tool(key),
                "retained native tool member",
            )
        })
    }
}

pub(super) fn valid_tol(value: f64) -> bool {
    value.is_finite() && value > 0.
}

fn face_tolerance(attempt: Attempt, face: &Face) -> Checked<f64> {
    let value = attempt.native(
        VALIDATION,
        "actual decoded face tolerance",
        BRepTool::face_tolerance(face),
    )?;
    require(
        attempt,
        VALIDATION,
        valid_tol(value),
        "positive finite native face tolerance",
    )?;
    Ok(value)
}

struct Part<'a> {
    face: Face,
    edges: [&'a Shape; 2],
    caps: [&'a Trace; 2],
    cap_floor: [f64; 2],
}

impl<'a> Part<'a> {
    fn read(
        attempt: Attempt,
        images: &Images<'a>,
        caps: &'a CapTraces,
        key: WrappedHistoryKey,
    ) -> Checked<Self> {
        let face = attempt.native(
            VALIDATION,
            "decoded exact side face",
            topods::face(images.shape(attempt, key)?),
        )?;
        let first = WrappedHistoryKey {
            role: HistoryRole::FirstDepthEdge,
            source: key.source,
        };
        let last = WrappedHistoryKey {
            role: HistoryRole::LastDepthEdge,
            source: key.source,
        };
        let edges = [images.shape(attempt, first)?, images.shape(attempt, last)?];
        let trace = |depth: usize, edge| {
            caps[depth].get(&edge).ok_or_else(|| {
                attempt.history(
                    CylinderFeatureSource::Tool(edge),
                    "both decoded actual side rails",
                )
            })
        };
        Ok(Self {
            face,
            edges,
            caps: [trace(0, first)?, trace(1, last)?],
            cap_floor: images.cap_floor,
        })
    }

    fn bind(
        &self,
        attempt: Attempt,
        support: CylinderSupport,
        sense: CylindricalRadialSense,
        span: &'a material::Span,
        cover: separation::Cover,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<contact::Tube<'a>> {
        let rails = self.caps.map(|cap| cap.query().curve);
        bind_net(attempt, &self.face, support.origin, rails, sense)?;
        prove_slices(
            attempt,
            support.radius,
            rails,
            sense,
            span,
            cover.allowance,
            cancelled,
        )?;
        let raw = face_tolerance(attempt, &self.face)?;
        let floor = self.cap_floor.map(|cap| {
            ((cap + raw).next_down() + martensite_tkernel::precision::confusion()).next_down()
        });
        Ok(contact::Tube {
            caps: self.caps.map(|cap| contact::Trace {
                query: cap.query(),
                proof: &cap.proof,
            }),
            storage: &span.storage.certificate,
            directions: &span.directions,
            floor,
        })
    }
}

fn bind_net(
    attempt: Attempt,
    face: &Face,
    origin: [f64; 3],
    rails: [MetricBezier<'_, 3>; 2],
    sense: CylindricalRadialSense,
) -> Checked<()> {
    let (surface, location) = attempt.native(
        VALIDATION,
        "decoded analytic side carrier",
        BRepTool::surface_with_location(face),
    )?;
    let located = assembly::exact_translation(attempt, [0.; 3], location.transformation())?;
    let net = surface.downcast::<GeomBezierSurface>();
    require(
        attempt,
        VALIDATION,
        located == origin && !net.is_null() && net.nb_v_poles() == 2,
        "same placed degree-one radial ruled side",
    )?;
    let rows = match sense {
        CylindricalRadialSense::Outward => [1, 2],
        CylindricalRadialSense::Inward => [2, 1],
    };
    for (row, rail) in rows.into_iter().zip(rails) {
        bind_row(attempt, &net, row, rail)?;
    }
    require(
        attempt,
        VALIDATION,
        rails[0]
            .poles
            .iter()
            .zip(rails[1].poles)
            .all(|(a, b)| a[2].to_bits() == b[2].to_bits()),
        "exact common height along both ruled rails",
    )
}

fn bind_row(
    attempt: Attempt,
    net: &GeomBezierSurface,
    row: i32,
    rail: MetricBezier<'_, 3>,
) -> Checked<()> {
    let weights = rail
        .weights
        .ok_or_else(|| missing(attempt, VALIDATION, "all raw side weights"))?;
    require(
        attempt,
        VALIDATION,
        usize::try_from(net.nb_u_poles()).ok() == Some(rail.poles.len())
            && weights.len() == rail.poles.len(),
        "complete actual side control row",
    )?;
    for column in 1..=net.nb_u_poles() {
        let point = attempt.native(
            VALIDATION,
            "raw decoded ruled side pole",
            net.pole(column, row),
        )?;
        let weight = attempt.native(
            VALIDATION,
            "raw decoded ruled side weight",
            net.weight(column, row),
        )?;
        let index = column as usize - 1;
        require(
            attempt,
            VALIDATION,
            point.coordinates().map(f64::to_bits) == rail.poles[index].map(f64::to_bits)
                && weight.to_bits() == weights[index].to_bits()
                && valid_tol(weight),
            "exact native side and seam coefficient bits",
        )?;
    }
    Ok(())
}

fn prove_slices(
    attempt: Attempt,
    radius: f64,
    rails: [MetricBezier<'_, 3>; 2],
    sense: CylindricalRadialSense,
    span: &material::Span,
    allowance: [f64; 2],
    cancelled: &mut CancelPoll<'_>,
) -> Checked<()> {
    require(
        attempt,
        VALIDATION,
        span.directions.len() == 3,
        "one side and both oriented adjacent join directions",
    )?;
    for &direction in &span.directions {
        let slice = CylindricalSliceQuery {
            rails,
            sense,
            chart_radius: radius,
            direction,
            depth: [-1e-5, 1. + 1e-5],
            options: Default::default(),
        };
        let slope = certify(
            attempt,
            VALIDATION,
            "actual fixed-radius side trace projection",
            certify_cylindrical_slice_direction(slice, &mut *cancelled),
        )?;
        certify(
            attempt,
            VALIDATION,
            "complete common radial interval including terminal collars",
            slope.require_common_radial_interval(
                slice,
                &span.storage.certificate,
                allowance,
                &mut *cancelled,
            ),
        )?;
    }
    Ok(())
}
