//! Bind the native Boolean to authored inputs and exact support snapshot members.
use super::*;
use body::Index;

/// One entity in an exact source or result snapshot. The ordinal is local to its kind and root.
/// Resolve it through the detached export. Persistent names belong to the application.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CylinderBodyRef {
    /// Topological kind of the referenced entity.
    pub kind: ShapeEnum,
    /// One-based `map_shapes_of_type` index in this snapshot.
    pub ordinal: usize,
}

/// Parent of one Boolean image, tied to the feature's support revision or authored keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CylinderFeatureSource {
    /// The selected face's application-defined identity.
    SupportFace(FaceAttachmentId),
    /// Another face, edge or vertex in the exact support solid snapshot.
    SupportMember(CylinderBodyRef),
    /// The explicit support solid operand.
    SupportSolid,
    /// Semantic tool identity. The extrusion history carries its original authored parents.
    Tool(WrappedHistoryKey),
}

/// Relationship of a parent to one published Boolean image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CylinderFeatureRelation {
    /// Exact native topology survived, at the same location.
    Unchanged,
    /// The Boolean returned this modified image, including splits and merges.
    Modified,
    /// The Boolean returned this generated image.
    Generated,
    /// An operand of the final material body.
    Operand,
}

/// A named input and its relationship to an output entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CylinderFeatureParent {
    /// Authored or support input.
    pub source: CylinderFeatureSource,
    /// How the input contributed to this image.
    pub relation: CylinderFeatureRelation,
}

/// All native Boolean parent relations for supported entities, in snapshot order.
/// Example: `feature.history().removed.contains(&CylinderFeatureSource::SupportFace(id))`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CylinderFeatureHistory {
    /// Parents of every final solid, face, edge and vertex.
    pub images: BTreeMap<CylinderBodyRef, Vec<CylinderFeatureParent>>,
    /// Boolean-removed source entities. A removed source can still generate other images.
    pub removed: Vec<CylinderFeatureSource>,
}

pub(super) struct Captured {
    pub solid: Shape,
    pub index: Index,
    pub history: CylinderFeatureHistory,
}

pub(super) fn capture(
    attempt: Attempt,
    selected_face: &Face,
    source_solid: &Shape,
    tool: &DetachedWrappedBodies,
    boolean: &BRepAlgoAPIBooleanOperation,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<Captured> {
    let stage = CylinderFeatureStage::History;
    let solid = body::single_solid(attempt, boolean.shape(), stage)?;
    let index = Index::new(attempt, &solid, stage)?;
    let native = boolean
        .history()
        .and_then(|handle| handle.get())
        .ok_or_else(|| {
            attempt.history(
                CylinderFeatureSource::SupportSolid,
                "filled native Boolean history",
            )
        })?;
    let mut audit = Audit::new(attempt, &index, native, &solid)?;
    audit.support(selected_face, source_solid, cancelled)?;
    audit.tool(tool, cancelled)?;
    let history = audit.finish()?;
    Ok(Captured {
        solid,
        index,
        history,
    })
}

struct Audit<'a> {
    attempt: Attempt,
    output: &'a Index,
    native: &'a History,
    root: CylinderBodyRef,
    history: CylinderFeatureHistory,
}

impl<'a> Audit<'a> {
    fn new(
        attempt: Attempt,
        output: &'a Index,
        native: &'a History,
        solid: &Shape,
    ) -> FeatureChecked<Self> {
        let root = output.reference(solid, attempt)?.ok_or_else(|| {
            attempt.history(
                CylinderFeatureSource::SupportSolid,
                "registered final solid",
            )
        })?;
        let mut result = Self {
            attempt,
            output,
            native,
            root,
            history: CylinderFeatureHistory {
                images: BTreeMap::new(),
                removed: Vec::new(),
            },
        };
        result.parent(
            root,
            CylinderFeatureSource::SupportSolid,
            CylinderFeatureRelation::Operand,
        );
        Ok(result)
    }

    fn support(
        &mut self,
        selected_face: &Face,
        solid: &Shape,
        cancelled: &mut CancelPoll<'_>,
    ) -> FeatureChecked<()> {
        let index = Index::new(self.attempt, solid, CylinderFeatureStage::History)?;
        for item in index.members() {
            let (reference, shape) = item?;
            if reference.kind == ShapeEnum::Solid {
                continue;
            }
            let source = if shape.is_same(selected_face) {
                CylinderFeatureSource::SupportFace(self.attempt.attachment)
            } else {
                CylinderFeatureSource::SupportMember(reference)
            };
            self.attempt
                .poll(cancelled, CylinderFeatureStage::History)?;
            self.member(source, shape)?;
        }
        Ok(())
    }

    fn tool(
        &mut self,
        tool: &DetachedWrappedBodies,
        cancelled: &mut CancelPoll<'_>,
    ) -> FeatureChecked<()> {
        for (&key, shape) in &tool.entities {
            self.attempt
                .poll(cancelled, CylinderFeatureStage::History)?;
            let source = CylinderFeatureSource::Tool(key);
            let kind = self.attempt.native(
                CylinderFeatureStage::History,
                "authored tool entity kind",
                shape.shape_type(),
            )?;
            match kind {
                ShapeEnum::Solid => {
                    self.parent(self.root, source, CylinderFeatureRelation::Operand)
                }
                ShapeEnum::Face | ShapeEnum::Edge | ShapeEnum::Vertex => {
                    self.member(source, shape)?
                }
                _ => {
                    return Err(self
                        .attempt
                        .history(source, "a solid, face, edge or vertex tool binding"));
                }
            }
        }
        Ok(())
    }

    fn member(&mut self, source: CylinderFeatureSource, shape: &Shape) -> FeatureChecked<()> {
        let images = self.read_native(source, shape)?;
        self.check_required(source, &images)?;
        if images.removed {
            self.history.removed.push(source);
        }
        if let Some(image) = images.unchanged {
            self.parent(image, source, CylinderFeatureRelation::Unchanged);
        }
        for image in images.modified {
            self.parent(image, source, CylinderFeatureRelation::Modified);
        }
        for image in images.generated {
            self.parent(image, source, CylinderFeatureRelation::Generated);
        }
        Ok(())
    }

    fn check_required(
        &self,
        source: CylinderFeatureSource,
        images: &NativeImages,
    ) -> FeatureChecked<()> {
        if images.removed && images.unchanged.is_some() {
            return Err(self
                .attempt
                .history(source, "removed topology absent from the result"));
        }
        if images.is_empty() && !images.removed {
            return Err(self.attempt.history(
                source,
                "an exact surviving, modified, generated or removed source",
            ));
        }
        if matches!(source, CylinderFeatureSource::SupportFace(_))
            && images.modified.is_empty()
            && !images.removed
        {
            return Err(self
                .attempt
                .history(source, "a modified or removed selected support face"));
        }
        if let CylinderFeatureSource::Tool(WrappedHistoryKey {
            source: WrappedEntitySource::Region(region),
            role: HistoryRole::LastDepthFace,
        }) = source
            && !images.has_face()
        {
            return Err(self.attempt.fail(
                CylinderFeatureStage::History,
                CylinderFeatureCause::MissingCap(region),
            ));
        }
        Ok(())
    }

    fn read_native(
        &self,
        source: CylinderFeatureSource,
        shape: &Shape,
    ) -> FeatureChecked<NativeImages> {
        let stage = CylinderFeatureStage::History;
        let unchanged = self.output.reference(shape, self.attempt)?;
        let removed =
            self.attempt
                .native(stage, "Boolean removed", self.native.is_removed(shape))?;
        let modified =
            self.attempt
                .native(stage, "Boolean modified", self.native.modified(shape))?;
        let generated =
            self.attempt
                .native(stage, "Boolean generated", self.native.generated(shape))?;
        Ok(NativeImages {
            unchanged,
            removed,
            modified: self.images(source, modified)?,
            generated: self.images(source, generated)?,
        })
    }

    fn images(
        &self,
        source: CylinderFeatureSource,
        images: Vec<Shape>,
    ) -> FeatureChecked<Vec<CylinderBodyRef>> {
        images
            .iter()
            .map(|image| {
                self.output.reference(image, self.attempt)?.ok_or_else(|| {
                    self.attempt
                        .history(source, "every Boolean image in the final solid")
                })
            })
            .collect()
    }

    fn parent(
        &mut self,
        target: CylinderBodyRef,
        source: CylinderFeatureSource,
        relation: CylinderFeatureRelation,
    ) {
        self.history
            .images
            .entry(target)
            .or_default()
            .push(CylinderFeatureParent { source, relation });
    }

    fn finish(mut self) -> FeatureChecked<CylinderFeatureHistory> {
        for item in self.output.members() {
            let (image, _) = item?;
            let parents = self.history.images.get_mut(&image).ok_or_else(|| {
                self.attempt.fail(
                    CylinderFeatureStage::History,
                    CylinderFeatureCause::UnownedImage(image),
                )
            })?;
            parents.sort_unstable();
            parents.dedup();
        }
        self.history.removed.sort_unstable();
        Ok(self.history)
    }
}

struct NativeImages {
    unchanged: Option<CylinderBodyRef>,
    modified: Vec<CylinderBodyRef>,
    generated: Vec<CylinderBodyRef>,
    removed: bool,
}

impl NativeImages {
    fn is_empty(&self) -> bool {
        self.unchanged.is_none() && self.modified.is_empty() && self.generated.is_empty()
    }
    fn has_face(&self) -> bool {
        self.unchanged
            .iter()
            .chain(&self.modified)
            .chain(&self.generated)
            .any(|image| image.kind == ShapeEnum::Face)
    }
}
