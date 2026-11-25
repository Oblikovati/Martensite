use super::*;
use martensite_tkbrep::topexp::{first_vertex, last_vertex};
use martensite_tkg2d::geom2d::Geom2dBezierCurve;

pub(super) struct Rail {
    pub(super) key: WrappedHistoryKey,
    pub(super) support: CylinderBodyRef,
    pub(super) side: CylinderBodyRef,
    pub(super) direction: Orientation,
    pub(super) gap_upper: f64,
    pub(super) first: seams::EdgeSource,
    pub(super) last: seams::EdgeSource,
    pub(super) rulings: [CylinderBodyRef; 2],
    pub(super) corners: [CylinderBodyRef; 2],
    pub(super) poles: Vec<[f64; 2]>,
    pub(super) knots: Vec<f64>,
}

pub(super) struct Source<'a> {
    pub(super) attempt: Attempt,
    pub(super) body: &'a DetachedCylinderFeature,
    pub(super) index: body::Index,
    pub(super) retained: &'a retained::BoundSubset,
}

impl Source<'_> {
    pub(super) fn owned_rail(
        &self,
        weld: &seams::ReboundSeam,
        incidence: &seams::FrozenIncidence,
    ) -> FeatureChecked<Rail> {
        let key = self
            .retained
            .tool_key(weld.edge())
            .filter(|k| k.role == HistoryRole::FirstDepthEdge)
            .ok_or_else(|| refused(self.attempt, "authored first-depth seam identity"))?;
        let (_, edge) = self.edge(key)?;
        let far_key = WrappedHistoryKey {
            role: HistoryRole::LastDepthEdge,
            ..key
        };
        let (_, far) = self.edge(far_key)?;
        let side_key = WrappedHistoryKey {
            role: HistoryRole::SideFace,
            ..key
        };
        let (side, owned) = self.image(side_key)?;
        if !incidence.opposite_side(weld, side) {
            return Err(refused(
                self.attempt,
                "opposite retained side for installed support seam",
            ));
        }
        let side_face = self
            .attempt
            .native(ASSEMBLY, "contact side image", topods::face(owned))?;
        let stock = self.attempt.native(
            ASSEMBLY,
            "contact stock image",
            topods::face(self.shape(weld.face())?),
        )?;
        let first = seams::EdgeSource::capture(self.attempt, &edge)?;
        let last = seams::EdgeSource::capture(self.attempt, &far)?;
        self.side_rows(&side_face, &edge, &far, &first, &last)?;
        let rulings = self.normal_rulings(&side_face, &edge, &far)?;
        let (poles, knots) = support_curve(self.attempt, &stock, &edge)?;
        Ok(Rail {
            key,
            support: weld.face(),
            side,
            direction: weld.direction(),
            gap_upper: weld.gap_upper(),
            first,
            last,
            rulings,
            corners: [
                self.corner(&first_vertex(&edge, false))?,
                self.corner(&last_vertex(&edge, false))?,
            ],
            poles,
            knots,
        })
    }

    fn image(&self, key: WrappedHistoryKey) -> FeatureChecked<(CylinderBodyRef, &Shape)> {
        let reference = self.retained.tool_image(key).ok_or_else(|| {
            self.attempt
                .history(CylinderFeatureSource::Tool(key), "retained contact domain")
        })?;
        Ok((reference, self.shape(reference)?))
    }

    fn shape(&self, reference: CylinderBodyRef) -> FeatureChecked<&Shape> {
        self.body
            .entities
            .get(&reference)
            .ok_or_else(|| refused(self.attempt, "native final contact entity"))
    }

    fn edge(&self, key: WrappedHistoryKey) -> FeatureChecked<(CylinderBodyRef, topods::Edge)> {
        let (reference, shape) = self.image(key)?;
        Ok((
            reference,
            self.attempt
                .native(ASSEMBLY, "contact rail edge", topods::edge(shape))?,
        ))
    }

    fn corner(&self, vertex: &topods::Vertex) -> FeatureChecked<CylinderBodyRef> {
        let reference = self
            .index
            .reference(vertex, self.attempt)?
            .ok_or_else(|| refused(self.attempt, "actual final contact vertex"))?;
        if self
            .retained
            .tool_key(reference)
            .is_none_or(|key| key.role != HistoryRole::FirstDepthVertex)
        {
            return Err(refused(
                self.attempt,
                "authored first-depth corner identity",
            ));
        }
        Ok(reference)
    }

    fn normal_rulings(
        &self,
        face: &Face,
        first: &topods::Edge,
        last: &topods::Edge,
    ) -> FeatureChecked<[CylinderBodyRef; 2]> {
        let wires = direct_children(self.attempt, face, ShapeEnum::Wire)?;
        let [wire] = wires.as_slice() else {
            return Err(refused(self.attempt, "one retained ruled-side trim wire"));
        };
        let mut ends = [None; 2];
        for shape in direct_children(self.attempt, wire, ShapeEnum::Edge)? {
            if shape.is_same(first) || shape.is_same(last) {
                continue;
            }
            let key = self
                .index
                .reference(&shape, self.attempt)?
                .ok_or_else(|| refused(self.attempt, "actual ruled side edge incidence"))?;
            if self
                .retained
                .tool_key(key)
                .is_none_or(|key| key.role != HistoryRole::NormalEdge)
            {
                return Err(refused(
                    self.attempt,
                    "intentional ruled side corner generator",
                ));
            }
            let edge = self
                .attempt
                .native(ASSEMBLY, "ruled side corner", topods::edge(&shape))?;
            let [a, b] = unit_side_trace(self.attempt, face, &edge)?;
            if a[0] != b[0]
                || ![0., 1.].contains(&a[0])
                || !([a[1], b[1]] == [0., 1.] || [a[1], b[1]] == [1., 0.])
                || ends[a[0] as usize].replace(key).is_some()
            {
                return Err(refused(
                    self.attempt,
                    "two distinct complete corner generators on a side",
                ));
            }
        }
        let [Some(a), Some(b)] = ends else {
            return Err(refused(
                self.attempt,
                "both native ruled side corner generators",
            ));
        };
        Ok([a, b])
    }

    fn side_rows(
        &self,
        face: &Face,
        edge: &topods::Edge,
        far: &topods::Edge,
        first: &seams::EdgeSource,
        last: &seams::EdgeSource,
    ) -> FeatureChecked<()> {
        let (native, location) = self.attempt.native(
            ASSEMBLY,
            "contact side surface",
            BRepTool::surface_with_location(face),
        )?;
        let surface = native.downcast::<GeomBezierSurface>();
        if surface.is_null()
            || surface.has_eval_representation()
            || surface.nb_v_poles() != 2
            || surface.nb_u_poles() as usize != first.poles.len()
            || first.location != last.location
            || first.poles.len() != last.poles.len()
            || !first
                .poles
                .iter()
                .zip(&last.poles)
                .all(|(a, b)| a[3].to_bits() == b[3].to_bits())
        {
            return Err(refused(
                self.attempt,
                "stored positive-weight two-row ruled Bézier side",
            ));
        }
        let first_loc = self.attempt.native(
            ASSEMBLY,
            "stored first rail location",
            BRepTool::curve_with_location(edge),
        )?;
        if !location.is_equal(&first_loc.location) {
            return Err(refused(
                self.attempt,
                "common native contact side and rail location",
            ));
        }
        let first_row = unit_side_row(self.attempt, face, edge)?;
        let last_row = unit_side_row(self.attempt, face, far)?;
        if first_row == last_row {
            return Err(refused(
                self.attempt,
                "opposite complete native side rail domains",
            ));
        }
        for (row, basis) in [(first_row, first), (last_row, last)] {
            for (i, poles) in basis.poles.iter().enumerate() {
                let (i, v) = (i as i32 + 1, row + 1);
                let p = self
                    .attempt
                    .native(ASSEMBLY, "side rail pole", surface.pole(i, v))?;
                let w = self
                    .attempt
                    .native(ASSEMBLY, "side rail weight", surface.weight(i, v))?;
                if [p.x(), p.y(), p.z(), w].map(f64::to_bits) != poles.map(f64::to_bits) {
                    return Err(refused(
                        self.attempt,
                        "exact common rail and surface stored rows",
                    ));
                }
            }
        }
        Ok(())
    }
}

fn unit_side_row(attempt: Attempt, face: &Face, edge: &topods::Edge) -> FeatureChecked<i32> {
    let [a, b] = unit_side_trace(attempt, face, edge)?;
    if a[0] != 0. || b[0] != 1. || a[1] != b[1] || ![0., 1.].contains(&a[1]) {
        return Err(refused(attempt, "full directed side-rail parameter domain"));
    }
    Ok(a[1] as i32)
}

fn unit_side_trace(
    attempt: Attempt,
    face: &Face,
    edge: &topods::Edge,
) -> FeatureChecked<[[f64; 2]; 2]> {
    let trace = attempt.native(
        ASSEMBLY,
        "side rail trace",
        BRepTool::curve_on_face(edge, face),
    )?;
    let line = trace.pcurve.downcast::<Geom2dBezierCurve>();
    if !trace.is_stored
        || [trace.first, trace.last] != [0., 1.]
        || line.is_null()
        || line.has_eval_representation()
        || line.is_rational()
        || line.nb_poles() != 2
    {
        return Err(refused(attempt, "stored polynomial unit side-rail pcurve"));
    }
    let a = attempt.native(ASSEMBLY, "side rail UV start", line.pole(1))?;
    let b = attempt.native(ASSEMBLY, "side rail UV finish", line.pole(2))?;
    Ok([[a.x(), a.y()], [b.x(), b.y()]])
}

fn support_curve(
    attempt: Attempt,
    face: &Face,
    edge: &topods::Edge,
) -> FeatureChecked<(Vec<[f64; 2]>, Vec<f64>)> {
    let stored = attempt.native(
        ASSEMBLY,
        "stock rail trace",
        BRepTool::curve_on_face(edge, face),
    )?;
    let spline = stored.pcurve.downcast::<Geom2dBSplineCurve>();
    if !stored.is_stored
        || [stored.first, stored.last] != [0., 1.]
        || spline.is_null()
        || spline.has_eval_representation()
        || spline.is_rational()
        || spline.degree() != 3
        || spline.is_periodic()
        || spline.nb_poles() != 3 * (spline.nb_knots() - 1) + 1
    {
        return Err(refused(attempt, "stored non-rational Hermite stock rail"));
    }
    let poles = (1..=spline.nb_poles())
        .map(|i| {
            let p = attempt.native(ASSEMBLY, "stock rail UV pole", spline.pole(i))?;
            Ok([p.x(), p.y()])
        })
        .collect::<FeatureChecked<Vec<_>>>()?;
    let knots = (1..=spline.nb_knots())
        .map(|i| {
            let expected = if i == 1 || i == spline.nb_knots() {
                4
            } else {
                3
            };
            if attempt.native(
                ASSEMBLY,
                "stock rail knot multiplicity",
                spline.multiplicity(i),
            )? != expected
            {
                return Err(refused(attempt, "cubic support rail knot multiplicity"));
            }
            attempt.native(ASSEMBLY, "stock rail knot", spline.knot(i))
        })
        .collect::<FeatureChecked<Vec<_>>>()?;
    if !poles.iter().flatten().all(|v| v.is_finite())
        || knots.first() != Some(&0.)
        || knots.last() != Some(&1.)
        || !knots.windows(2).all(|w| w[0] < w[1])
    {
        return Err(refused(
            attempt,
            "finite ordered unit support rail controls",
        ));
    }
    Ok((poles, knots))
}
