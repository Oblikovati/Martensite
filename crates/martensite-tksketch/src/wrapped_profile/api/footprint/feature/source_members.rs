//! Transfer caller-owned identities into the frozen support's exact reference table (#420).
use super::*;
use martensite_tkbrep::bintools::{
    CurveFrameReadPolicy, LocationTransformReadPolicy, SurfaceFrameReadPolicy,
};

impl CylinderFaceAttachment {
    /// Resolve exact source names for a subsequent wrapped feature's history.
    ///
    /// Supply the source body and selected face from capture, plus the named members of
    /// that body. The returned references are local to this support revision.
    /// Example: `attachment.bind_feature_sources(revision, kind, &body, &face, &[(label, edge)])?`.
    pub fn bind_feature_sources<Key: Copy + Ord>(
        &self,
        revision: SupportRevision,
        kind: CylinderFeatureKind,
        body: &Shape,
        selected: &Face,
        members: &[(Key, Shape)],
    ) -> Result<BTreeMap<Key, CylinderFeatureSource>, CylinderFeatureFailure> {
        let attempt = Attempt {
            attachment: self.record().id,
            revision,
            kind,
        };
        if revision != self.record().revision {
            return Err(attempt.history(
                CylinderFeatureSource::SupportSolid,
                "the attachment's current support revision",
            ));
        }
        let (body, selected, members) = decode_members(attempt, self, body, selected, members)?;
        let solid = body::single_solid(attempt, &body, CylinderFeatureStage::History)?;
        let index = body::Index::new(attempt, &solid, CylinderFeatureStage::History)?;
        bind_members(attempt, &index, &selected, members)
    }
}

type SelectedMembers<Key> = (Shape, Shape, Vec<(Key, Shape)>);

fn decode_members<Key: Copy>(
    attempt: Attempt,
    attachment: &CylinderFaceAttachment,
    body: &Shape,
    selected: &Face,
    members: &[(Key, Shape)],
) -> FeatureChecked<SelectedMembers<Key>> {
    let mut references = Vec::with_capacity(members.len() + 1);
    references.push((None, selected.clone().into_shape()));
    references.extend(
        members
            .iter()
            .map(|(key, member)| (Some(*key), member.clone())),
    );
    let encoded = encoding::encode(body, &references).map_err(|cause| attempt.encoding(cause))?;
    if !encoded.starts_with(&attachment.record().brep) {
        return Err(attempt.history(
            CylinderFeatureSource::SupportSolid,
            "the frozen support's exact body and selected-face references",
        ));
    }
    let keys: Vec<_> = references.iter().map(|(key, _)| *key).collect();
    let (root, decoded) = encoding::decode_with_geometry_policies(
        &encoded,
        &keys,
        SurfaceFrameReadPolicy::PreserveStoredV1,
        LocationTransformReadPolicy::PreserveStoredRigidV1,
        CurveFrameReadPolicy::PreserveStoredV1,
    )
    .map_err(|cause| attempt.encoding(cause))?;
    unpack_members(attempt, root, decoded)
}

fn unpack_members<Key>(
    attempt: Attempt,
    root: Shape,
    decoded: Vec<(Option<Key>, Shape)>,
) -> FeatureChecked<SelectedMembers<Key>> {
    let mut decoded = decoded.into_iter();
    let Some((None, selected)) = decoded.next() else {
        return Err(attempt.history(
            CylinderFeatureSource::SupportFace(attempt.attachment),
            "one explicit selected-face reference",
        ));
    };
    Ok((
        root,
        selected,
        decoded
            .filter_map(|(key, shape)| key.map(|key| (key, shape)))
            .collect(),
    ))
}

fn bind_members<Key: Ord>(
    attempt: Attempt,
    index: &body::Index,
    selected: &Shape,
    members: Vec<(Key, Shape)>,
) -> FeatureChecked<BTreeMap<Key, CylinderFeatureSource>> {
    let mut names = BTreeMap::new();
    for (key, member) in members {
        let source = source_key(attempt, index, selected, &member)?;
        if names.insert(key, source).is_some() {
            return Err(attempt.history(source, "one binding per caller source identity"));
        }
    }
    Ok(names)
}

fn source_key(
    attempt: Attempt,
    index: &body::Index,
    selected: &Shape,
    member: &Shape,
) -> FeatureChecked<CylinderFeatureSource> {
    let reference = index.reference(member, attempt)?.ok_or_else(|| {
        attempt.history(
            CylinderFeatureSource::SupportSolid,
            "an exact support member",
        )
    })?;
    Ok(if member.is_same(selected) {
        CylinderFeatureSource::SupportFace(attempt.attachment)
    } else if reference.kind == ShapeEnum::Solid {
        CylinderFeatureSource::SupportSolid
    } else {
        CylinderFeatureSource::SupportMember(reference)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface_sketch::NormalSense;
    use crate::surface_sketch::wrapped_profile::api::footprint::tests::{attachment, request};
    use martensite_tkbrep::topexp::Explorer;

    #[test]
    fn caller_names_follow_frozen_references_instead_of_their_input_order() {
        let attachment = attachment(None);
        let revision = attachment.record().revision;
        let native = attachment.export_native(revision).unwrap();
        let selected = native.face.clone().into_shape();
        let members = [(3, selected), (2, native.body.clone())];
        let bind = |members: &[_]| {
            attachment.bind_feature_sources(
                revision,
                CylinderFeatureKind::Boss,
                &native.body,
                &native.face,
                members,
            )
        };
        let named = bind(&members).unwrap();
        assert_eq!(named[&2], CylinderFeatureSource::SupportSolid);
        assert_eq!(
            named[&3],
            CylinderFeatureSource::SupportFace(attachment.record().id)
        );
        assert_eq!(
            bind(&[members[1].clone(), members[0].clone()]).unwrap(),
            named
        );
        assert!(bind(&[members[0].clone(), members[0].clone()]).is_err());
    }

    #[test]
    fn bound_source_members_are_used_by_the_boss_history() {
        let attachment = attachment(None);
        let revision = attachment.record().revision;
        let native = attachment.export_native(revision).unwrap();
        let edge = first_edge(&native.body);
        let members = [(7, edge)];
        let names = attachment
            .bind_feature_sources(
                revision,
                CylinderFeatureKind::Boss,
                &native.body,
                &native.face,
                &members,
            )
            .unwrap();
        let reference = names[&7];
        assert!(matches!(reference, CylinderFeatureSource::SupportMember(_)));
        let mut proposal = request(&attachment, [-0.25, 0.25], [1., 2.]);
        proposal.support.normal = NormalSense::Outward;
        let feature = attachment
            .apply_wrapped_profile(revision, CylinderFeatureKind::Boss, &proposal)
            .unwrap();
        let history = feature.history();
        assert!(
            history
                .images
                .values()
                .flatten()
                .any(|parent| parent.source == reference)
                || history.removed.contains(&reference)
        );
        assert!(
            attachment
                .bind_feature_sources(
                    SupportRevision(revision.0 + 1),
                    CylinderFeatureKind::Boss,
                    &native.body,
                    &native.face,
                    &members
                )
                .is_err()
        );
    }

    fn first_edge(body: &Shape) -> Shape {
        let edges = Explorer::new(body, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
        assert!(edges.more(), "cylindrical support fixture has named edges");
        edges.current().clone()
    }
}
