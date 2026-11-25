//! Verify the stored face and edge geometry of the freshly decoded solid.
use super::*;
use martensite_tkbrep::brep::access::{edge_tshape, face_tshape};

mod literal;

pub(super) fn qualify(
    attempt: Attempt,
    body: &DetachedCylinderFeature,
    topology: &topology::Topology,
    stock: &stock_binding::BoundStock,
    subset: &retained::BoundSubset,
) -> FeatureChecked<[usize; 3]> {
    let [base, lid] = stock.ends;
    let native = |id| read_face(attempt, body, id);
    let band = native(stock.band)?;
    let original = literal::qualify_stock_storage(&band, &native(base)?, &native(lid)?)
        .map_err(|cause| geometry_failure(attempt, "original stock representations", cause))?;
    let mut sides = 0;
    for reference in subset.tool_images(HistoryRole::SideFace) {
        literal::qualify_side_geometry(&native(reference)?).map_err(|cause| {
            geometry_failure(attempt, "ruled side boundary geometry", (reference, cause))
        })?;
        sides += 1;
    }
    let caps = subset
        .tool_images(HistoryRole::LastDepthFace)
        .map(|reference| {
            cap(
                attempt,
                body,
                topology,
                subset,
                reference,
                &native(reference)?,
            )
        })
        .collect::<FeatureChecked<Vec<_>>>()?
        .into_iter()
        .sum();
    Ok([sides, caps, original.surface_uses])
}

fn cap(
    attempt: Attempt,
    body: &DetachedCylinderFeature,
    topology: &topology::Topology,
    subset: &retained::BoundSubset,
    reference: CylinderBodyRef,
    face: &Face,
) -> FeatureChecked<usize> {
    let membership = topology
        .wires(reference)
        .ok_or_else(|| refused(attempt, "actual retained cap boundary incidence"))?;
    let (_, surface_place) = attempt.native(
        ASSEMBLY,
        "retained cap carrier",
        BRepTool::surface_with_location(face),
    )?;
    if !attempt
        .native(
            ASSEMBLY,
            "retained cap mesh absence",
            face_tshape(face, "retained cap"),
        )?
        .triangulations()
        .is_empty()
    {
        return Err(refused(attempt, "unmeshed actual cap geometry"));
    }
    for edge in membership.iter().flatten() {
        let upper = subset
            .cap_residual_upper(edge.edge)
            .ok_or_else(|| refused(attempt, "actual residual proof of each retained cap trim"))?;
        let shape = body
            .entities
            .get(&edge.edge)
            .ok_or_else(|| refused(attempt, "actual retained cap curve"))?;
        let native = attempt.native(ASSEMBLY, "actual retained cap edge", topods::edge(shape))?;
        let curve = attempt.native(
            ASSEMBLY,
            "actual retained cap curve",
            BRepTool::curve_with_location(&native),
        )?;
        let affine = literal::retained_cap_affine_norm(&curve.location, &surface_place).map_err(
            |cause| {
                geometry_failure(
                    attempt,
                    "common full affine cap transform",
                    (edge.edge, cause),
                )
            },
        )?;
        let tolerance = attempt
            .native(
                ASSEMBLY,
                "raw retained cap edge budget",
                edge_tshape(&native, "retained cap"),
            )?
            .tolerance();
        let achieved = (upper * affine).next_up();
        if !upper.is_finite()
            || upper < 0.
            || !affine.is_finite()
            || affine <= 0.
            || !achieved.is_finite()
            || !tolerance.is_finite()
            || tolerance < 0.
            || achieved > tolerance
        {
            return Err(geometry_failure(
                attempt,
                "retained cap residual budget",
                (edge.edge, upper, affine, tolerance),
            ));
        }
    }
    Ok(membership.iter().map(Vec::len).sum())
}

fn read_face(
    attempt: Attempt,
    body: &DetachedCylinderFeature,
    id: CylinderBodyRef,
) -> FeatureChecked<Face> {
    let shape = body
        .entities
        .get(&id)
        .ok_or_else(|| refused(attempt, "actual direct retained face"))?;
    attempt.native(ASSEMBLY, "actual direct retained face", topods::face(shape))
}

fn geometry_failure(
    attempt: Attempt,
    query: &'static str,
    cause: impl std::fmt::Debug,
) -> CylinderFeatureFailure {
    attempt.fail(
        CylinderFeatureStage::Validation,
        CylinderFeatureCause::Native {
            query,
            cause: Box::new(Failure::new(&format!("{cause:?}"))),
        },
    )
}
