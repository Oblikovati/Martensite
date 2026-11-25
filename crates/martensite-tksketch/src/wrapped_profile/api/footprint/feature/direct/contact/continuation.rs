//! Keep each possible near contact inside the declared closed stock trim.
use super::{
    interval::I,
    prefix as p, retraction as r,
    windows::{Windows, contact_failure, ordinal},
    *,
};

struct ContactPolicy {
    continuation_floor: f64,
    upper: f64,
    depth: f64,
}

impl SourceGraph {
    pub(in super::super) fn qualify_contact(
        &self,
        attempt: Attempt,
        windows: &Windows,
        chart: stock::StockChart,
        collars: &BTreeMap<CylinderBodyRef, collar::Receipt>,
    ) -> FeatureChecked<p::Summary> {
        let datum = chart.datum;
        let issuer = r::Issuer::new(r::ConeSource {
            angle: datum.semi_angle,
            radius: datum.radius,
            origin_z: datum.axial_origin,
        })
        .map_err(|cause| contact_failure(attempt, "conical retraction admission", cause))?;
        let fuzzy = martensite_tkernel::precision::confusion();
        let max = self.limits.class_maxima.into_iter().fold(0.0_f64, f64::max);
        let upper = collar::native_comparator_upper(max, max, fuzzy);
        let policy = ContactPolicy {
            upper,
            continuation_floor: 3. * fuzzy,
            depth: collars
                .values()
                .map(|r| r.all().normalized_depth_upper)
                .fold(0.0_f64, f64::max),
        };
        let sides = self.sides(attempt, &issuer, collars, &policy)?;
        let guard = sides
            .iter()
            .flat_map(|row| row.vertex_corner_upper)
            .fold(0.0_f64, f64::max);
        let arms = self.incident_arms(attempt, &issuer, &windows.census.windows, &policy, guard)?;
        let stock = self
            .owner_tolerances
            .iter()
            .map(|(&id, &tolerance)| {
                Ok(p::StockFace {
                    id: p::Face(ordinal(attempt, id)?),
                    tolerance,
                })
            })
            .collect::<FeatureChecked<Vec<_>>>()?;
        let mut report = p::certify(p::Input {
            cone: windows.cone,
            stock: &stock,
            sides: &sides,
            pieces: &windows.pieces,
            windows: &windows.census.windows,
            arms: &arms,
            separation: windows.census.separation,
            fuzzy,
            collar_threshold: upper,
            continuation_floor: policy.continuation_floor,
        })
        .map_err(|cause| {
            contact_failure(attempt, "closed-trim contact retraction coverage", cause)
        })?;
        if report.closed_piece_pairs != windows.census.pairs {
            return Err(refused(
                attempt,
                "complete replay of issued stock trim pair census",
            ));
        }
        report.interval_split_visits += windows.census.split_visits;
        Ok(report)
    }

    fn sides(
        &self,
        attempt: Attempt,
        issuer: &r::Issuer,
        collars: &BTreeMap<CylinderBodyRef, collar::Receipt>,
        policy: &ContactPolicy,
    ) -> FeatureChecked<Vec<p::Side>> {
        self.rails
            .iter()
            .map(|(&edge, rail)| {
                let receipt = collars
                    .get(&edge)
                    .ok_or_else(|| refused(attempt, "paired side depth collar"))?;
                self.side(attempt, issuer, edge, rail, receipt, policy)
            })
            .collect()
    }

    fn side(
        &self,
        attempt: Attempt,
        issuer: &r::Issuer,
        edge: CylinderBodyRef,
        rail: &Rail,
        receipt: &collar::Receipt,
        policy: &ContactPolicy,
    ) -> FeatureChecked<p::Side> {
        let corner_error = corner_to_native(rail);
        let bounds = *receipt.all();
        let speed = receipt.motion_upper()[0];
        let rate = receipt.signed_derivative_lower();
        let using = r::OwnLimits {
            broad_comparator_upper: policy.upper,
            minimum_signed_depth_rate: rate,
            maximum_speed: speed,
            maximum_u_anchor_pad: bounds.uv_from_support_curve_upper[0],
            xyz_anchor_pad: bounds.from_support_curve_upper,
            sew_upper: rail.gap_upper,
            vertex_guard: corner_error[0].max(corner_error[1]),
            continuation_floor: policy.continuation_floor,
        };
        let own = issuer.own(retraction_side(rail), using).map_err(|cause| {
            contact_failure(
                attempt,
                "declared native rail retraction",
                (rail.key, cause),
            )
        })?;
        let [ru, rv] = rail.rulings;
        let [vu, vv] = rail.corners;
        Ok(p::Side {
            edge: p::Edge(ordinal(attempt, edge)?),
            face: p::Face(ordinal(attempt, rail.support)?),
            face_tolerance: self.limits.entity(rail.side)?,
            rail_tolerance: self.limits.entity(edge)?,
            ruling_tolerance: [self.limits.entity(ru)?, self.limits.entity(rv)?],
            vertex_tolerance: [self.limits.entity(vu)?, self.limits.entity(vv)?],
            vertex_corner_upper: corner_error,
            depth_upper: bounds.normalized_depth_upper,
            signed_depth_rate_lower: rate,
            speed_upper: speed,
            uv_anchor_pad_upper: bounds.uv_from_support_curve_upper,
            xyz_anchor_pad_upper: bounds.from_support_curve_upper,
            sew_upper: rail.gap_upper,
            own_ratio_upper: own.ratio_upper,
            own_margin_lower: own.margin_lower,
        })
    }

    fn incident_arms(
        &self,
        attempt: Attempt,
        issuer: &r::Issuer,
        windows: &[p::Window],
        policy: &ContactPolicy,
        vertex_guard: f64,
    ) -> FeatureChecked<Vec<p::ArmProof>> {
        let mut arms = Vec::with_capacity(windows.len() * 2);
        for window in windows {
            let p::Incidence([a, b], anchor) = window.pair;
            let ends = [
                self.incident(attempt, a, anchor)?,
                self.incident(attempt, b, anchor)?,
            ];
            let results = issuer
                .incident(
                    ends,
                    r::IncidentLimits {
                        window_fraction: window.fraction,
                        depth_upper: policy.depth,
                        vertex_guard,
                        continuation_floor: policy.continuation_floor,
                    },
                )
                .map_err(|cause| {
                    contact_failure(
                        attempt,
                        "native incident contact retraction",
                        (window.pair, cause),
                    )
                })?;
            arms.extend(results.iter().enumerate().map(|(source_end, result)| {
                let leaf = result.mixed;
                p::ArmProof {
                    pair: window.pair,
                    source_end,
                    window_fraction: window.fraction,
                    linear_min_lower: leaf.linear_min_lower,
                    linear_sum_upper: leaf.linear_sum_upper,
                    quadratic_upper: leaf.quadratic_upper,
                    derivative_remainder_upper: leaf.derivative_remainder_upper,
                    anchor_upper: leaf.anchor_upper,
                }
            }));
        }
        Ok(arms)
    }

    fn incident(
        &self,
        attempt: Attempt,
        p::PieceId(p::Edge(edge), span): p::PieceId,
        p::Vertex(vertex): p::Vertex,
    ) -> FeatureChecked<r::IncidentEnd<'_>> {
        let key = CylinderBodyRef {
            kind: ShapeEnum::Edge,
            ordinal: edge as usize,
        };
        let rail = self
            .rails
            .get(&key)
            .ok_or_else(|| refused(attempt, "bound incident curve owner"))?;
        let end = match rail.corners.map(|id| id.ordinal == vertex as usize) {
            [true, false] => r::Endpoint::First,
            [false, true] => r::Endpoint::Last,
            _ => {
                return Err(refused(
                    attempt,
                    "actual distinct native vertex at incident trim window",
                ));
            }
        };
        Ok(r::IncidentEnd {
            side: retraction_side(rail),
            piece_index: usize::from(span),
            end,
        })
    }
}

fn retraction_side(rail: &Rail) -> r::SideSource<'_> {
    r::SideSource {
        affine: rail.first.location,
        first: &rail.first.poles,
        last: &rail.last.poles,
        knots: &rail.knots,
        uv: &rail.poles,
    }
}

fn corner_to_native(rail: &Rail) -> [f64; 2] {
    let n = rail.first.poles.len();
    let affine = rail.first.location;
    std::array::from_fn(|end| {
        let [x, y, z, _] = rail.first.poles[end * (n - 1)];
        let actual = rail.first.endpoints[end];
        actual
            .into_iter()
            .enumerate()
            .fold(I::Z, |norm, (axis, coordinate)| {
                let residual = [x, y, z]
                    .into_iter()
                    .enumerate()
                    .fold(I::p(affine[1 + axis]), |value, (j, pole)| {
                        value.add(I::p(pole).mul(I::p(affine[4 + axis * 3 + j])))
                    })
                    .sub(I::p(coordinate));
                norm.add(I::p(residual.abs()))
            })
            .hi
    })
}
