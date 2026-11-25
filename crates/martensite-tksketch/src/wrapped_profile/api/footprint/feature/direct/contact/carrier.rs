//! Enclose the actual stock separation for each retained side and far cap.
use super::*;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkmath::predicates::roots::metric_bounds::conical_separation::{
    CarrierSeparationQuery, CarrierSeparationReceipt, DecodedCapTrim, DecodedCoaxialLocation,
    DecodedConeFrame, DecodedRimPlane, DecodedRuledSide, SeparationOptions, SeparationSense,
    certify_separation,
};
use martensite_tkmath::toploc::Location;

pub(in super::super) struct Separation {
    pub(super) proof: CarrierSeparationReceipt,
    pub(super) stock: DecodedConeFrame,
}

impl SourceGraph {
    pub(in super::super) fn separate(
        &self,
        attempt: Attempt,
        final_body: &DetachedCylinderFeature,
        stock: &stock_binding::BoundStock,
        subset: &retained::BoundSubset,
        sew: f64,
        cancelled: &mut CancelPoll<'_>,
    ) -> FeatureChecked<Separation> {
        attempt.poll(cancelled, CylinderFeatureStage::Validation)?;
        let carrier = frame(attempt, &face(attempt, final_body, stock.band)?)?;
        let [bottom, top] = stock.ends;
        let rims = [
            rim(attempt, &face(attempt, final_body, bottom)?)?,
            rim(attempt, &face(attempt, final_body, top)?)?,
        ];
        let sides = self
            .rails
            .iter()
            .map(|(id, rail)| DecodedRuledSide {
                id: id.ordinal as u64,
                location: affine(rail.first.location),
                rows: [&rail.first.poles, &rail.last.poles],
                domain: [[0., 1.]; 2],
            })
            .collect::<Vec<_>>();
        let caps = subset
            .tool_images(HistoryRole::LastDepthFace)
            .map(|reference| {
                cap(
                    attempt,
                    &face(attempt, final_body, reference)?,
                    reference.ordinal as u64,
                )
            })
            .collect::<FeatureChecked<BTreeMap<_, _>>>()?
            .into_values()
            .collect::<Vec<_>>();
        if caps.len() + self.rails.len() != subset.census()[0] {
            return Err(refused(
                attempt,
                "complete retained side and far-cap carrier cover",
            ));
        }
        let sense = match attempt.kind {
            CylinderFeatureKind::Boss => SeparationSense::Outside,
            CylinderFeatureKind::Pocket => SeparationSense::Inside,
        };
        let receipt = certify_separation(
            CarrierSeparationQuery {
                stock: carrier,
                rims,
                sides: &sides,
                caps: &caps,
                sense,
                options: SeparationOptions {
                    max_scalar_steps: 200_000,
                    max_dyadic_operations: 100_000,
                },
            },
            cancelled,
        )
        .map_err(|cause| {
            attempt.fail(
                CylinderFeatureStage::Validation,
                CylinderFeatureCause::Native {
                    query: "actual side and cap separation",
                    cause: Box::new(Failure::new(&format!("{cause:?}"))),
                },
            )
        })?;
        if receipt.sides().len() != self.rails.len()
            || receipt.caps().len() != caps.len()
            || !receipt
                .sides()
                .iter()
                .zip(&self.rails)
                .all(|(evidence, (id, rail))| {
                    evidence.id() == id.ordinal as u64
                        && evidence.zero_normal_upper() < sew.min(rail.first.tolerance)
                })
        {
            return Err(refused(
                attempt,
                "actual carrier receipt for every side and cap",
            ));
        }
        Ok(Separation {
            proof: receipt,
            stock: carrier,
        })
    }
}

fn face(
    attempt: Attempt,
    body: &DetachedCylinderFeature,
    id: CylinderBodyRef,
) -> FeatureChecked<Face> {
    let member = body
        .entities
        .get(&id)
        .ok_or_else(|| refused(attempt, "bound decoded separation face"))?;
    attempt.native(
        ASSEMBLY,
        "bound decoded separation face",
        topods::face(member),
    )
}

fn cap(attempt: Attempt, face: &Face, id: u64) -> FeatureChecked<(u64, DecodedCapTrim)> {
    let mut bounds = [f64::INFINITY, f64::NEG_INFINITY];
    for shape in occurrences(attempt, face, ShapeEnum::Edge)? {
        let edge = attempt.native(ASSEMBLY, "cap boundary in carrier", topods::edge(&shape))?;
        let trace = attempt.native(
            ASSEMBLY,
            "cap carrier planar chart use",
            BRepTool::curve_on_face(&edge, face),
        )?;
        let curve = trace.pcurve.downcast::<Geom2dBezierCurve>();
        if !trace.is_stored
            || [trace.first, trace.last] != [0., 1.]
            || curve.is_null()
            || curve.has_eval_representation()
            || curve.is_rational()
            || !(2..=17).contains(&curve.nb_poles())
        {
            return Err(refused(attempt, "stored full polynomial cap trim"));
        }
        for index in 1..=curve.nb_poles() {
            let pole = attempt.native(ASSEMBLY, "cap axial bound pole", curve.pole(index))?;
            if !pole.y().is_finite() {
                return Err(refused(attempt, "finite cap trim axial extent"));
            }
            bounds = [bounds[0].min(pole.y()), bounds[1].max(pole.y())];
        }
    }
    if bounds[0] >= bounds[1] {
        return Err(refused(attempt, "ordered trimmed cap axial bounds"));
    }
    Ok((
        id,
        DecodedCapTrim {
            id,
            frame: frame(attempt, face)?,
            axial_parameters: bounds,
        },
    ))
}

fn rim(attempt: Attempt, face: &Face) -> FeatureChecked<DecodedRimPlane> {
    let (surface, location) = attempt.native(
        ASSEMBLY,
        "stock disk carrier",
        BRepTool::surface_with_location(face),
    )?;
    let surface = surface.downcast::<GeomPlane>();
    if !location.is_identity() || surface.is_null() {
        return Err(refused(attempt, "world-axis retained stock disk carrier"));
    }
    let frame = surface.pln();
    let normal = *frame.position().direction().coordinates();
    Ok(DecodedRimPlane {
        elevation: frame.position().location().z(),
        normal,
    })
}

fn frame(attempt: Attempt, face: &Face) -> FeatureChecked<DecodedConeFrame> {
    let (surface, location) = attempt.native(
        ASSEMBLY,
        "separation face carrier",
        BRepTool::surface_with_location(face),
    )?;
    let cone = surface.downcast::<GeomConicalSurface>();
    if cone.is_null() {
        return Err(refused(attempt, "literal conical separation carrier"));
    }
    let cone = attempt.native(ASSEMBLY, "separation cone", cone.cone())?;
    let frame = cone.position();
    Ok(DecodedConeFrame {
        semi_angle: cone.semi_angle(),
        radius: cone.ref_radius(),
        origin: *frame.location().coordinates(),
        axes: [
            *frame.x_direction().coordinates(),
            *frame.y_direction().coordinates(),
            *frame.direction().coordinates(),
        ],
        location: affine(location_words(&location)),
    })
}

fn affine(values: [f64; 13]) -> DecodedCoaxialLocation {
    DecodedCoaxialLocation {
        scale: values[0],
        translation: [values[1], values[2], values[3]],
        matrix: std::array::from_fn(|i| [values[4 + 3 * i], values[5 + 3 * i], values[6 + 3 * i]]),
    }
}

fn location_words(location: &Location) -> [f64; 13] {
    let trans = location.transformation();
    let mut words = [trans.scale_factor(); 13];
    words[1..4].copy_from_slice(trans.translation_part().coordinates());
    for (index, row) in trans.matrix().coefficients().iter().enumerate() {
        words[4 + 3 * index..7 + 3 * index].copy_from_slice(row);
    }
    words
}
