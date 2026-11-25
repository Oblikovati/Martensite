//! Link exact original stock roles to the final images of the certified re-cut.
use super::*;

pub(in super::super) struct RecutResult<'a> {
    pub(in super::super) solid: Shape,
    pub(in super::super) replacement: &'a [Shape],
    pub(in super::super) recut: &'a cylinder_recut::RecutStock,
}

pub(in super::super) fn capture_recut_direct(
    attempt: Attempt,
    selected: &Face,
    original_stock: &Shape,
    tool: &DetachedWrappedBodies,
    output: RecutResult<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<Captured> {
    if !output.recut.matches_source(original_stock, selected) {
        return Err(attempt.history(
            CylinderFeatureSource::SupportFace(attempt.attachment),
            "re-cut bound to the exact original stock and selected face occurrence",
        ));
    }
    let source = ImageSource::Recut {
        replacement: output.replacement,
        support: &output.recut.images,
    };
    capture_direct_images(
        attempt,
        selected,
        original_stock,
        tool,
        output.solid,
        source,
        cancelled,
    )
}

impl Audit<'_> {
    pub(super) fn read_recut(
        &self,
        replacement: &[Shape],
        recut: &[cylinder_recut::StockImage],
        source: CylinderFeatureSource,
        shape: &Shape,
    ) -> FeatureChecked<NativeImages> {
        let CylinderFeatureSource::SupportMember(reference) = source else {
            return self.read_direct(replacement, source, shape);
        };
        let mut links = recut.iter().filter(|link| link.source == reference);
        let link = links
            .next()
            .filter(|_| links.next().is_none())
            .ok_or_else(|| {
                self.attempt
                    .history(source, "one owned post-re-cut stock image")
            })?;
        if self.output.reference(shape, self.attempt)?.is_some() {
            return Err(self
                .attempt
                .history(source, "re-cut original stock member absent"));
        }
        let images = self.images(source, vec![link.image.clone()])?;
        Self::replace_recut_image(link.relation, images).ok_or_else(|| {
            self.attempt
                .history(source, "modified or replaced post-re-cut stock member")
        })
    }

    fn replace_recut_image(
        relation: CylinderFeatureRelation,
        images: Vec<CylinderBodyRef>,
    ) -> Option<NativeImages> {
        use CylinderFeatureRelation::{Generated, Modified};
        match relation {
            Modified => Some(NativeImages {
                unchanged: None,
                modified: images,
                generated: vec![],
                removed: false,
            }),
            Generated => Some(NativeImages {
                unchanged: None,
                modified: vec![],
                generated: images,
                removed: true,
            }),
            _ => None,
        }
    }
}
