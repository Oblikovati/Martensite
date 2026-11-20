//! Freeze one solid, validate its decoded topology and retain exact image references.
use super::*;
use martensite_tkbo::{body_check::validate_body, profile_extrusion::Resolution};
use martensite_tkbrep::{
    bintools::{CurveFrameReadPolicy, LocationTransformReadPolicy, SurfaceFrameReadPolicy},
    topexp::map_shapes_of_type,
    topods::ShapeIterator,
    toptools::IndexedMapOfShape,
};

pub(super) fn single_solid(
    attempt: Attempt,
    root: &Shape,
    stage: CylinderFeatureStage,
) -> FeatureChecked<Shape> {
    let kind = attempt.native(stage, "material body kind", root.shape_type())?;
    if kind == ShapeEnum::Solid {
        return Ok(root.clone());
    }
    let mut children = ShapeIterator::new(root, true, true);
    if kind != ShapeEnum::Compound || !children.more() {
        return Err(attempt.fail(
            stage,
            CylinderFeatureCause::SingleSolid {
                root: kind,
                members: 0,
            },
        ));
    }
    let first = attempt
        .native(stage, "Boolean compound member", children.value())?
        .clone();
    children.next();
    if children.more()
        || attempt.native(stage, "compound member kind", first.shape_type())? != ShapeEnum::Solid
    {
        return Err(attempt.fail(
            stage,
            CylinderFeatureCause::SingleSolid {
                root: kind,
                members: if children.more() { 2 } else { 1 },
            },
        ));
    }
    // The compound member contains the composed location and orientation supplied by the iterator.
    Ok(first)
}

const KINDS: [ShapeEnum; 4] = [
    ShapeEnum::Solid,
    ShapeEnum::Face,
    ShapeEnum::Edge,
    ShapeEnum::Vertex,
];

pub(super) struct Index {
    shapes: BTreeMap<ShapeEnum, IndexedMapOfShape>,
    attempt: Attempt,
    stage: CylinderFeatureStage,
}

impl Index {
    pub(super) fn new(
        attempt: Attempt,
        root: &Shape,
        stage: CylinderFeatureStage,
    ) -> FeatureChecked<Self> {
        let mut shapes = BTreeMap::new();
        for kind in KINDS {
            let mut members = IndexedMapOfShape::default();
            attempt.native(
                stage,
                "index body topology",
                map_shapes_of_type(root, kind, &mut members),
            )?;
            shapes.insert(kind, members);
        }
        Ok(Self {
            shapes,
            attempt,
            stage,
        })
    }

    pub(super) fn members(
        &self,
    ) -> impl Iterator<Item = FeatureChecked<(CylinderBodyRef, &Shape)>> {
        self.shapes.iter().flat_map(move |(&kind, members)| {
            (1..=members.extent()).map(move |ordinal| {
                let shape = self.attempt.native(
                    self.stage,
                    "indexed body member",
                    members.find_key(ordinal),
                )?;
                Ok((CylinderBodyRef { kind, ordinal }, shape))
            })
        })
    }

    pub(super) fn reference(
        &self,
        shape: &Shape,
        attempt: Attempt,
    ) -> FeatureChecked<Option<CylinderBodyRef>> {
        let kind = attempt.native(self.stage, "shape reference kind", shape.shape_type())?;
        let ordinal = self
            .shapes
            .get(&kind)
            .map_or(0, |members| members.find_index(shape));
        Ok((ordinal != 0).then_some(CylinderBodyRef { kind, ordinal }))
    }
}

pub(super) fn freeze(
    attempt: Attempt,
    captured: &history::Captured,
    resolution: &Resolution,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<(Vec<u8>, ValidationReport)> {
    let stage = CylinderFeatureStage::Encoding;
    attempt.poll(cancelled, stage)?;
    let bindings = captured
        .index
        .members()
        .map(|item| item.map(|(key, shape)| (key, shape.clone())))
        .collect::<FeatureChecked<Vec<_>>>()?;
    let bytes =
        encoding::encode(&captured.solid, &bindings).map_err(|cause| attempt.encoding(cause))?;
    let keys = bindings.iter().map(|(key, _)| *key).collect();
    let decoded = decode(&bytes, keys, attempt)?;
    let validation = validate(attempt, &decoded.body, resolution, cancelled)?;
    let references: Vec<_> = decoded.entities.into_iter().collect();
    let replay =
        encoding::encode(&decoded.body, &references).map_err(|cause| attempt.encoding(cause))?;
    if replay != bytes {
        return Err(attempt.fail(stage, CylinderFeatureCause::ArchiveMismatch));
    }
    attempt.poll(cancelled, CylinderFeatureStage::Publication)?;
    Ok((bytes, validation))
}

fn validate(
    attempt: Attempt,
    body: &Shape,
    resolution: &Resolution,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<ValidationReport> {
    let stage = CylinderFeatureStage::Validation;
    attempt.poll(cancelled, stage)?;
    let report = validate_body(body, resolution)
        .map_err(|cause| attempt.fail(stage, CylinderFeatureCause::Resolution(cause)))?;
    if !report.is_valid() {
        return Err(attempt.fail(stage, CylinderFeatureCause::Validation(Box::new(report))));
    }
    Ok(report)
}

pub(super) fn decode(
    bytes: &[u8],
    keys: Vec<CylinderBodyRef>,
    attempt: Attempt,
) -> FeatureChecked<DetachedCylinderFeature> {
    let (body, entities) = encoding::decode_with_geometry_policies(
        bytes,
        &keys,
        SurfaceFrameReadPolicy::PreserveStoredV1,
        LocationTransformReadPolicy::PreserveStoredRigidV1,
        CurveFrameReadPolicy::PreserveStoredV1,
    )
    .map_err(|cause| attempt.encoding(cause))?;
    let stage = CylinderFeatureStage::Encoding;
    let index = Index::new(attempt, &body, stage)?;
    for (key, entity) in &entities {
        if index.reference(entity, attempt)? != Some(*key) {
            return Err(attempt.fail(stage, CylinderFeatureCause::ArchiveMismatch));
        }
    }
    Ok(DetachedCylinderFeature {
        body,
        entities: entities.into_iter().collect(),
    })
}
