//! The issued pre-sew tool root lends authority only to its exact retained occurrences.
use super::*;
use crate::surface_sketch::wrapped_profile::placement::attachment_tool::AttachmentTool;
use martensite_tkbo::body_check::CertifiedConicalBody;
use martensite_tkernel::standard::Failure;
use std::{collections::BTreeSet, sync::Arc};
mod native_snapshot;
mod source;
mod trim;
use native_snapshot::*;
use trim::{Face as FaceSnapshot, ShapeKeys, normalized_wires};
type ToolFace = FaceSnapshot<WrappedHistoryKey, WrappedHistoryKey>;

pub(super) struct Retained {
    _authority: Arc<CertifiedConicalBody>,
    faces: BTreeMap<WrappedHistoryKey, ToolFace>,
    edges: BTreeMap<WrappedHistoryKey, EdgeGeometry>,
    vertices: BTreeMap<WrappedHistoryKey, VertexGeometry>,
    removed: BTreeSet<WrappedHistoryKey>,
    cap_residuals: BTreeMap<WrappedHistoryKey, f64>,
}

pub(super) struct BoundSubset {
    _authority: Arc<CertifiedConicalBody>,
    census: [usize; 4],
    images: BTreeMap<WrappedHistoryKey, CylinderBodyRef>,
    sources: BTreeMap<CylinderBodyRef, WrappedHistoryKey>,
    cap_residuals: BTreeMap<WrappedHistoryKey, f64>,
}
impl BoundSubset {
    pub(super) fn census(&self) -> [usize; 4] {
        self.census
    }
    pub(super) fn tool_image(&self, key: WrappedHistoryKey) -> Option<CylinderBodyRef> {
        self.images.get(&key).copied()
    }
    pub(super) fn tool_key(&self, reference: CylinderBodyRef) -> Option<WrappedHistoryKey> {
        self.sources.get(&reference).copied()
    }

    pub(super) fn tool_images(
        &self,
        role: HistoryRole,
    ) -> impl Iterator<Item = CylinderBodyRef> + '_ {
        self.images
            .iter()
            .filter_map(move |(key, &image)| (key.role == role).then_some(image))
    }

    pub(super) fn cap_residual_upper(&self, image: CylinderBodyRef) -> Option<f64> {
        self.tool_key(image)
            .and_then(|key| self.cap_residuals.get(&key).copied())
    }
}

type Bound<T> = Result<T, Rejected>;
#[derive(Debug)]
pub(super) enum Rejected {
    Authority,
    Source(WrappedNormalFailure),
    Native(Failure),
    Missing(WrappedHistoryKey),
    Face(WrappedHistoryKey),
    Edge(WrappedHistoryKey),
    Vertex(WrappedHistoryKey),
    History,
    Seam(WrappedHistoryKey),
}
impl From<Failure> for Rejected {
    fn from(error: Failure) -> Self {
        Self::Native(error)
    }
}
impl Rejected {
    pub(super) fn feature(self, attempt: Attempt) -> CylinderFeatureFailure {
        // The experiment preserves a typed refusal up to this existing operation boundary.
        let expected = match self {
            Self::Source(error) => return attempt.tool(error),
            Self::Native(error) => {
                return attempt.fail(
                    ASSEMBLY,
                    CylinderFeatureCause::Native {
                        query: "certified retained-tool snapshot",
                        cause: Box::new(error),
                    },
                );
            }
            Self::Missing(key)
            | Self::Face(key)
            | Self::Edge(key)
            | Self::Vertex(key)
            | Self::Seam(key) => {
                return attempt.history(
                    CylinderFeatureSource::Tool(key),
                    "actual retained tool provenance",
                );
            }
            Self::Authority => "complete canonical conical family certificate",
            Self::History => "complete and unique retained source history",
        };
        refused(attempt, expected)
    }
}

type Images<'a> = BTreeMap<WrappedHistoryKey, (CylinderBodyRef, &'a Shape)>;

impl Retained {
    pub(super) fn capture_attachment(owner: &AttachmentTool) -> Bound<Self> {
        let rebound = owner.rebind(owner.native()).map_err(Rejected::Source)?;
        let authority = Arc::clone(rebound.certified_body());
        let mut source = Self::capture_verified(authority, owner.history(), owner.native())?;
        for (key, upper) in rebound.cap_residuals() {
            if !source.edges.contains_key(&key)
                || !upper.is_finite()
                || upper < 0.
                || source.cap_residuals.insert(key, upper).is_some()
            {
                return Err(Rejected::Edge(key));
            }
        }
        let required = source
            .edges
            .keys()
            .filter(|key| {
                matches!(
                    key.role,
                    HistoryRole::FirstDepthEdge | HistoryRole::LastDepthEdge
                )
            })
            .count();
        if source.cap_residuals.len() != required {
            return Err(Rejected::Authority);
        }
        Ok(source)
    }

    fn capture_verified(
        authority: Arc<CertifiedConicalBody>,
        records: &[crate::surface_sketch::WrappedHistoryRecord],
        native: &DetachedWrappedBodies,
    ) -> Bound<Self> {
        let mut result = Self {
            _authority: authority,
            faces: BTreeMap::new(),
            edges: BTreeMap::new(),
            vertices: BTreeMap::new(),
            removed: BTreeSet::new(),
            cap_residuals: BTreeMap::new(),
        };
        let ids = tool_keys(native.entities.iter().map(|(key, shape)| (*key, shape)))?;
        let source_faces = source::shell_faces(records, native)?;
        for (&key, shape) in &native.entities {
            match shape.shape_type()? {
                ShapeEnum::Edge => {
                    result
                        .edges
                        .insert(key, edge_geometry(&topods::edge(shape)?, None)?);
                }
                ShapeEnum::Vertex => {
                    result
                        .vertices
                        .insert(key, vertex_geometry(&topods::vertex(shape)?)?);
                }
                ShapeEnum::Face if key.role == HistoryRole::FirstDepthFace => {
                    result.removed.insert(key);
                }
                ShapeEnum::Face => {
                    let face = topods::face(source_faces.get(&key).ok_or(Rejected::Missing(key))?)?;
                    result.faces.insert(
                        key,
                        FaceSnapshot::capture(
                            &face,
                            normalized_wires(&face)?,
                            (&ids.0, &ids.1),
                            false,
                        )?,
                    );
                }
                ShapeEnum::Solid if key.role == HistoryRole::Solid => {}
                _ => return Err(Rejected::Missing(key)),
            }
        }
        Ok(result)
    }

    pub(super) fn rebind(
        &self,
        attempt: Attempt,
        result: &DetachedCylinderFeature,
        history: &CylinderFeatureHistory,
        welded: &[seams::ReboundSeam],
        incidence: &seams::FrozenIncidence,
    ) -> Bound<BoundSubset> {
        let images = retained_images(attempt, result, history)?;
        if images.len() != self.faces.len() + self.edges.len() + self.vertices.len()
            || removed_tool(history) != self.removed
        {
            return Err(Rejected::History);
        }
        let ids = tool_keys(images.iter().map(|(key, (_, shape))| (*key, *shape)))?;
        self.faces(attempt.kind, &images, &ids)?;
        let seams = welded
            .iter()
            .map(|seam| (seam.edge(), seam))
            .collect::<BTreeMap<_, _>>();
        if seams.len() != welded.len() {
            return Err(Rejected::History);
        }
        self.edges(result, &images, &seams, incidence)?;
        for (&key, expected) in &self.vertices {
            let (_, actual) = image(&images, key)?;
            if vertex_geometry(&topods::vertex(actual)?)? != *expected {
                return Err(Rejected::Vertex(key));
            }
        }
        Ok(BoundSubset {
            _authority: Arc::clone(&self._authority),
            cap_residuals: self.cap_residuals.clone(),
            sources: images
                .iter()
                .map(|(&key, &(image, _))| (image, key))
                .collect(),
            images: images
                .iter()
                .map(|(&key, &(image, _))| (key, image))
                .collect(),
            census: [
                self.faces.len(),
                self.edges.len(),
                self.vertices.len(),
                seams.len(),
            ],
        })
    }

    fn faces(
        &self,
        kind: CylinderFeatureKind,
        images: &Images<'_>,
        ids: &(ShapeKeys<WrappedHistoryKey>, ShapeKeys<WrappedHistoryKey>),
    ) -> Bound<()> {
        for (&key, expected) in &self.faces {
            let (_, shape) = image(images, key)?;
            let face = topods::face(shape)?;
            let mut actual =
                FaceSnapshot::capture(&face, normalized_wires(&face)?, (&ids.0, &ids.1), false)?;
            if kind == CylinderFeatureKind::Pocket {
                actual.surface.orientation = shape.reversed().orientation();
            }
            if actual != *expected {
                return Err(Rejected::Face(key));
            }
        }
        Ok(())
    }

    fn edges(
        &self,
        result: &DetachedCylinderFeature,
        images: &Images<'_>,
        welded: &BTreeMap<CylinderBodyRef, &seams::ReboundSeam>,
        incidence: &seams::FrozenIncidence,
    ) -> Bound<()> {
        let mut used = 0;
        for (&key, expected) in &self.edges {
            let (id, actual) = image(images, key)?;
            let edge = topods::edge(actual)?;
            let stock = match (key.role, welded.get(&id)) {
                (HistoryRole::FirstDepthEdge, Some(issued)) => {
                    let side_key = WrappedHistoryKey {
                        source: key.source,
                        role: HistoryRole::SideFace,
                    };
                    let (side, _) = image(images, side_key)?;
                    if !incidence.opposite_side(issued, side) {
                        return Err(Rejected::Seam(key));
                    }
                    used += 1;
                    let face = result
                        .entities
                        .get(&issued.face())
                        .ok_or(Rejected::Seam(key))?;
                    let (surface, location) =
                        BRepTool::surface_with_location(&topods::face(face)?)?;
                    Some((surface, location.predivided(edge.location())?))
                }
                (HistoryRole::FirstDepthEdge, None) | (_, Some(_)) => {
                    return Err(Rejected::Seam(key));
                }
                (_, None) => None,
            };
            let added = stock.as_ref().map(|(carrier, relative)| AddedSupport {
                carrier,
                relative: relative.clone(),
            });
            if edge_geometry(&edge, added)? != *expected {
                return Err(Rejected::Edge(key));
            }
        }
        if used != welded.len() {
            return Err(Rejected::History);
        }
        Ok(())
    }
}

fn image<'a>(images: &Images<'a>, key: WrappedHistoryKey) -> Bound<(CylinderBodyRef, &'a Shape)> {
    images.get(&key).copied().ok_or(Rejected::Missing(key))
}

fn removed_tool(history: &CylinderFeatureHistory) -> BTreeSet<WrappedHistoryKey> {
    history
        .removed
        .iter()
        .filter_map(|source| match source {
            CylinderFeatureSource::Tool(key) => Some(*key),
            _ => None,
        })
        .collect()
}

fn retained_images<'a>(
    attempt: Attempt,
    result: &'a DetachedCylinderFeature,
    history: &CylinderFeatureHistory,
) -> Bound<Images<'a>> {
    let index = body::Index::new(attempt, &result.body, CylinderFeatureStage::Encoding)
        .map_err(|_| Rejected::History)?;
    let verified = index
        .members()
        .collect::<FeatureChecked<BTreeMap<_, _>>>()
        .map_err(|_| Rejected::History)?;
    let mut images = BTreeMap::new();
    for (reference, parents) in &history.images {
        if reference.kind == ShapeEnum::Solid {
            continue;
        }
        let mut keys = parents.iter().filter_map(|parent| match parent.source {
            CylinderFeatureSource::Tool(key) => Some((key, parent.relation)),
            _ => None,
        });
        let Some((key, relation)) = keys.next() else {
            continue;
        };
        if parents.len() != 1 || relation != CylinderFeatureRelation::Unchanged {
            return Err(Rejected::History);
        }
        let actual = result
            .entities
            .get(reference)
            .ok_or(Rejected::Missing(key))?;
        let root_member = verified.get(reference).ok_or(Rejected::Missing(key))?;
        if !root_member.is_equal(actual) || images.insert(key, (*reference, actual)).is_some() {
            return Err(Rejected::History);
        }
    }
    Ok(images)
}

fn tool_keys<'a>(
    pairs: impl Iterator<Item = (WrappedHistoryKey, &'a Shape)>,
) -> Bound<(ShapeKeys<WrappedHistoryKey>, ShapeKeys<WrappedHistoryKey>)> {
    let mut edges = ShapeKeys::new();
    let mut vertices = ShapeKeys::new();
    for (key, shape) in pairs {
        let ok = match shape.shape_type()? {
            ShapeEnum::Edge => edges.add(shape, key),
            ShapeEnum::Vertex => vertices.add(shape, key),
            _ => true,
        };
        if !ok {
            return Err(Rejected::Missing(key));
        }
    }
    Ok((edges, vertices))
}
