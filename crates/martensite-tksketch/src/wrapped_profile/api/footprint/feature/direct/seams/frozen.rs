//! Bind the typed installation to the published archive’s actual edge and pcurve.
use super::*;

pub(crate) struct InstalledSeam<'a> {
    pub(super) target: Face,
    pub(super) original: StrictSeam<'a>,
}

impl<'a> InstalledSeam<'a> {
    pub(crate) fn bind(
        self,
        attempt: Attempt,
        index: &body::Index,
    ) -> FeatureChecked<FrozenSeam<'a>> {
        let image = |shape: &Shape| {
            index.reference(shape, attempt)?.ok_or_else(|| {
                attempt.fail(
                    CylinderFeatureStage::Encoding,
                    CylinderFeatureCause::ArchiveMismatch,
                )
            })
        };
        Ok(FrozenSeam {
            edge: image(&self.original.edge)?,
            face: image(&self.target)?,
            direction: self.original.edge.orientation(),
            original: self.original,
        })
    }
}

pub(crate) struct FrozenSeam<'a> {
    edge: CylinderBodyRef,
    face: CylinderBodyRef,
    direction: Orientation,
    original: StrictSeam<'a>,
}

pub(in super::super) struct ReboundSeam {
    edge: CylinderBodyRef,
    face: CylinderBodyRef,
    direction: Orientation,
    gap_upper: f64,
}
impl ReboundSeam {
    pub(in super::super) fn edge(&self) -> CylinderBodyRef {
        self.edge
    }
    pub(in super::super) fn face(&self) -> CylinderBodyRef {
        self.face
    }
    pub(in super::super) fn direction(&self) -> Orientation {
        self.direction
    }

    pub(in super::super) fn gap_upper(&self) -> f64 {
        self.gap_upper
    }
}

impl FrozenSeam<'_> {
    pub(crate) fn rebind(
        self,
        attempt: Attempt,
        result: &DetachedCylinderFeature,
        incidence: &FrozenIncidence,
    ) -> FeatureChecked<ReboundSeam> {
        let refusal = || {
            attempt.fail(
                CylinderFeatureStage::Encoding,
                CylinderFeatureCause::ArchiveMismatch,
            )
        };
        if !incidence.opposite(self.edge, self.face, self.direction) {
            return Err(refusal());
        }
        let edge = topods::edge(result.entities.get(&self.edge).ok_or_else(refusal)?)
            .map_err(|_| refusal())?;
        let face = topods::face(result.entities.get(&self.face).ok_or_else(refusal)?)
            .map_err(|_| refusal())?;
        let source = EdgeSource::capture(attempt, &edge).map_err(|_| refusal())?;
        if !source.stored_bits().eq(self.original.source.stored_bits())
            || !same_carrier(&face, self.original.carrier.datum).map_err(|_| refusal())?
        {
            return Err(refusal());
        }
        let expected = self.original.layout(attempt)?;
        let trace = BRepTool::curve_on_face(&edge, &face).map_err(|_| refusal())?;
        let actual = trace.pcurve.downcast::<Geom2dBSplineCurve>();
        if !trace.is_stored
            || [trace.first, trace.last] != [0., 1.]
            || actual.is_null()
            || !expected.matches(&actual).map_err(|_| refusal())?
        {
            return Err(refusal());
        }
        Ok(ReboundSeam {
            edge: self.edge,
            face: self.face,
            direction: self.direction,
            gap_upper: self.original.pieces.iter().fold(0.0_f64, |upper, span| {
                upper.max((span.proof.residual_bounds()[2] + span.fit.roundoff).next_up())
            }),
        })
    }
}

type EdgeUses = BTreeMap<CylinderBodyRef, Vec<(CylinderBodyRef, Orientation)>>;

pub(in super::super) struct FrozenIncidence(EdgeUses);

impl FrozenIncidence {
    pub(in super::super) fn capture(
        attempt: Attempt,
        result: &DetachedCylinderFeature,
    ) -> FeatureChecked<Self> {
        let stage = CylinderFeatureStage::Encoding;
        let index = body::Index::new(attempt, &result.body, stage)?;
        let mut uses = EdgeUses::new();
        for (face_ref, face) in &result.entities {
            if face_ref.kind != ShapeEnum::Face {
                continue;
            }
            for wire in direct_children(attempt, face, ShapeEnum::Wire)? {
                for edge in direct_children(attempt, &wire, ShapeEnum::Edge)? {
                    let key = index.reference(&edge, attempt)?.ok_or_else(|| {
                        attempt.fail(stage, CylinderFeatureCause::ArchiveMismatch)
                    })?;
                    uses.entry(key)
                        .or_default()
                        .push((*face_ref, edge.orientation()));
                }
            }
        }
        Ok(Self(uses))
    }

    pub(in super::super) fn opposite_side(
        &self,
        weld: &ReboundSeam,
        side: CylinderBodyRef,
    ) -> bool {
        let Some([a, b]) = self.0.get(&weld.edge).map(Vec::as_slice) else {
            return false;
        };
        (a.0 == side && expected_uses(*b, *a, weld.face, weld.direction))
            || (b.0 == side && expected_uses(*a, *b, weld.face, weld.direction))
    }

    fn opposite(
        &self,
        edge: CylinderBodyRef,
        face: CylinderBodyRef,
        direction: Orientation,
    ) -> bool {
        let Some([a, b]) = self.0.get(&edge).map(Vec::as_slice) else {
            return false;
        };
        expected_uses(*a, *b, face, direction) || expected_uses(*b, *a, face, direction)
    }
}

fn expected_uses(
    target: (CylinderBodyRef, Orientation),
    other: (CylinderBodyRef, Orientation),
    face: CylinderBodyRef,
    direction: Orientation,
) -> bool {
    target == (face, direction)
        && other.0 != face
        && matches!(
            (direction, other.1),
            (Orientation::Forward, Orientation::Reversed)
                | (Orientation::Reversed, Orientation::Forward)
        )
}

pub(in super::super) fn same_carrier(
    face: &Face,
    datum: ConicalTrimCarrier,
) -> Result<bool, Failure> {
    let (surface, location) = BRepTool::surface_with_location(face)?;
    let cone = surface.downcast::<GeomConicalSurface>();
    if cone.is_null() || !location.is_identity() {
        return Ok(false);
    }
    let value = cone.cone()?;
    let pos = value.position();
    let scalars = [value.ref_radius(), pos.location().z(), value.semi_angle()].map(f64::to_bits);
    Ok(
        scalars == [datum.radius, datum.axial_origin, datum.semi_angle].map(f64::to_bits)
            && pos.location().x() == 0.
            && pos.location().y() == 0.
            && *pos.x_direction().coordinates() == [1., 0., 0.]
            && *pos.y_direction().coordinates() == [0., 1., 0.]
            && *pos.direction().coordinates() == [0., 0., 1.],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_on_same_carrier_cannot_replace_the_actual_face_use() {
        let face = |ordinal| CylinderBodyRef {
            kind: ShapeEnum::Face,
            ordinal,
        };
        let edge = CylinderBodyRef {
            kind: ShapeEnum::Edge,
            ordinal: 1,
        };
        let actual = (face(1), Orientation::Forward);
        let reversed = (face(2), Orientation::Reversed);
        let uses = FrozenIncidence(BTreeMap::from([(edge, vec![actual, reversed])]));
        assert!(uses.opposite(edge, face(1), Orientation::Forward));
        let welded = ReboundSeam {
            edge,
            face: face(1),
            direction: Orientation::Forward,
            gap_upper: 0.,
        };
        assert!(uses.opposite_side(&welded, face(2)));
        assert!(!uses.opposite_side(&welded, face(3)));
        assert!(!uses.opposite(edge, face(3), Orientation::Forward));
        assert!(!uses.opposite(edge, face(1), Orientation::Reversed));
        let duplicate = FrozenIncidence(BTreeMap::from([(edge, vec![actual, reversed, reversed])]));
        assert!(!duplicate.opposite(edge, face(1), Orientation::Forward));
    }
}
