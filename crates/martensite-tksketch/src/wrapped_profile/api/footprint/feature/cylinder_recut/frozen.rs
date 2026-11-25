//! Rebind only the certified stock boundary after the final tool faces are installed.
use super::*;

pub(in super::super) struct FrozenRecut<'a> {
    receipt: &'a [u8],
    band: CylinderBodyRef,
    seam: CylinderBodyRef,
    parts: Vec<CylinderBodyRef>,
}

impl RecutStock {
    pub(in super::super) fn bind_final(
        &self,
        attempt: Attempt,
        index: &body::Index,
        band: &Face,
    ) -> FeatureChecked<FrozenRecut<'_>> {
        bind_stock(
            attempt,
            index,
            band,
            self.new_seam(attempt)?,
            non_band_images(&self.band, &self.images),
            &self.receipt,
        )
    }

    fn new_seam(&self, attempt: Attempt) -> FeatureChecked<&Shape> {
        let mut seams = self.images.iter().filter(|link| {
            link.source.kind == ShapeEnum::Edge
                && link.relation == CylinderFeatureRelation::Generated
        });
        seams
            .next()
            .filter(|_| seams.next().is_none())
            .map(|link| &link.image)
            .ok_or_else(|| mismatch(attempt))
    }
}

pub(super) fn bind_stock<'a>(
    attempt: Attempt,
    index: &body::Index,
    band: &Face,
    seam: &Shape,
    parts: impl IntoIterator<Item = &'a Shape>,
    receipt: &'a [u8],
) -> FeatureChecked<FrozenRecut<'a>> {
    let bind = |entity: &Shape| {
        index
            .reference(entity, attempt)?
            .ok_or_else(|| mismatch(attempt))
    };
    Ok(FrozenRecut {
        receipt,
        band: bind(band)?,
        seam: bind(seam)?,
        parts: parts.into_iter().map(bind).collect::<FeatureChecked<_>>()?,
    })
}

impl FrozenRecut<'_> {
    pub(in super::super) fn rebind(
        &self,
        attempt: Attempt,
        decoded: &DetachedCylinderFeature,
    ) -> FeatureChecked<()> {
        let reference = |key| decoded.entities.get(key).ok_or_else(|| mismatch(attempt));
        let band = attempt.native(
            STAGE,
            "decoded original stock cylinder",
            topods::face(reference(&self.band)?),
        )?;
        let seam = reference(&self.seam)?;
        let parts = self
            .parts
            .iter()
            .map(reference)
            .collect::<FeatureChecked<Vec<_>>>()?;
        if stock_receipt(attempt, &band, seam, parts)? != self.receipt {
            return Err(mismatch(attempt));
        }
        Ok(())
    }
}

pub(super) fn stock_receipt<'a>(
    attempt: Attempt,
    band: &Face,
    seam: &Shape,
    parts: impl IntoIterator<Item = &'a Shape>,
) -> FeatureChecked<Vec<u8>> {
    let check = RecutCheck(attempt);
    let carrier = check.native("untrimmed original cylinder carrier", band.empty_copied())?;
    let outer = seam_wire(check, band, seam)?;
    let parts: Vec<_> = std::iter::once(((), outer))
        .chain(parts.into_iter().map(|part| ((), local_occurrence(part))))
        .collect();
    encoding::encode(&carrier, &parts).map_err(|cause| attempt.encoding(cause))
}

fn seam_wire(attempt: RecutCheck, band: &Face, seam: &Shape) -> FeatureChecked<Shape> {
    let mut found = None;
    for wire in children(attempt, band)? {
        let count = children(attempt, &wire)?
            .iter()
            .filter(|edge| edge.is_same(seam))
            .count();
        if count == 0 {
            continue;
        }
        if count != 2 || found.replace(wire).is_some() {
            return Err(mismatch(attempt.0));
        }
    }
    found.ok_or_else(|| mismatch(attempt.0))
}

fn local_occurrence(entity: &Shape) -> Shape {
    // Archive indexing chooses one occurrence of each edge and vertex. Their directed
    // occurrences are stored separately in the stock band wire and end disk boundaries.
    if matches!(entity.shape_type(), Ok(ShapeEnum::Edge | ShapeEnum::Vertex)) {
        entity.oriented(Orientation::Forward)
    } else {
        entity.to_shape()
    }
}

fn mismatch(attempt: Attempt) -> CylinderFeatureFailure {
    attempt.fail(
        CylinderFeatureStage::Encoding,
        CylinderFeatureCause::ArchiveMismatch,
    )
}
