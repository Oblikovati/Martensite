use super::*;

pub(in super::super) struct LoadedTolerances {
    attempt: Attempt,
    items: BTreeMap<CylinderBodyRef, f64>,
    support_faces: BTreeMap<CylinderBodyRef, f64>,
    pub(in super::super) class_maxima: [f64; 3],
}

impl LoadedTolerances {
    pub(in super::super) fn capture(
        attempt: Attempt,
        result: &DetachedCylinderFeature,
        history: &CylinderFeatureHistory,
    ) -> FeatureChecked<Self> {
        let mut checked = Self {
            attempt,
            items: BTreeMap::new(),
            support_faces: BTreeMap::new(),
            class_maxima: [0.; 3],
        };
        for (&reference, shape) in &result.entities {
            if reference.kind == ShapeEnum::Solid {
                continue;
            }
            let (class, raw) = stored_tolerance(attempt, reference, shape)?;
            if !raw.is_finite() || raw < 0. {
                return Err(refused(attempt, "finite nonnegative raw contact tolerance"));
            }
            let loaded = raw.max(martensite_tkernel::precision::confusion());
            checked.class_maxima[class] = checked.class_maxima[class].max(loaded);
            checked.items.insert(reference, loaded);
            if history.images.get(&reference).is_some_and(|parents| {
                parents.iter().any(|parent| {
                    parent.source == CylinderFeatureSource::SupportFace(attempt.attachment)
                })
            }) {
                if reference.kind != ShapeEnum::Face {
                    return Err(refused(attempt, "actual retained support-face tolerance"));
                }
                checked.support_faces.insert(reference, loaded);
            }
        }
        Ok(checked)
    }

    pub(super) fn entity(&self, reference: CylinderBodyRef) -> FeatureChecked<f64> {
        self.items
            .get(&reference)
            .copied()
            .ok_or_else(|| refused(self.attempt, "loaded native contact entity"))
    }

    pub(super) fn support_covers(&self, owners: &BTreeMap<CylinderBodyRef, f64>) -> bool {
        self.support_faces == *owners
    }

    pub(in super::super) fn finish(
        &self,
        topology: &topology::Topology,
        resolution: &Resolution,
    ) -> FeatureChecked<f64> {
        use martensite_tkbo::profile_extrusion::ToleranceCap;
        let max = self.class_maxima.into_iter().fold(0.0_f64, f64::max);
        let cap = match resolution.cap {
            ToleranceCap::NoLimit => None,
            ToleranceCap::Report(cap) | ToleranceCap::Refuse(cap) => Some(cap),
        };
        if cap.is_some_and(|limit| max > limit) {
            return Err(refused(
                self.attempt,
                "final loaded tolerance within the requested cap",
            ));
        }
        for (face, edge) in topology.face_uses() {
            let edge_budget = self.entity(edge.edge)?;
            let face_budget = self.entity(face)?;
            let [v0, v1] = edge.ends;
            if face_budget > edge_budget
                || edge_budget > self.entity(v0)?
                || edge_budget > self.entity(v1)?
            {
                return Err(refused(
                    self.attempt,
                    "every final incidence observes vertex–edge–face tolerance order",
                ));
            }
        }
        Ok(max)
    }
}

fn stored_tolerance(
    attempt: Attempt,
    key: CylinderBodyRef,
    shape: &Shape,
) -> FeatureChecked<(usize, f64)> {
    use martensite_tkbrep::brep::access::{edge_tshape, face_tshape, vertex_tshape};
    const QUERY: &str = "raw tolerance of final direct contact domain";
    match key.kind {
        ShapeEnum::Vertex => {
            let item = attempt.native(ASSEMBLY, QUERY, topods::vertex(shape))?;
            attempt
                .native(ASSEMBLY, QUERY, vertex_tshape(&item, QUERY))
                .map(|item| (0, item.tolerance()))
        }
        ShapeEnum::Edge => {
            let item = attempt.native(ASSEMBLY, QUERY, topods::edge(shape))?;
            attempt
                .native(ASSEMBLY, QUERY, edge_tshape(&item, QUERY))
                .map(|item| (1, item.tolerance()))
        }
        ShapeEnum::Face => {
            let item = attempt.native(ASSEMBLY, QUERY, topods::face(shape))?;
            attempt
                .native(ASSEMBLY, QUERY, face_tshape(&item, QUERY))
                .map(|item| (2, item.tolerance()))
        }
        _ => Err(refused(
            attempt,
            "named face, edge or vertex in contact inventory",
        )),
    }
}
