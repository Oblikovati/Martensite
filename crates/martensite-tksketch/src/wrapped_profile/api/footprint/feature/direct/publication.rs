//! Encode and replay once, then issue the complete attachment report in check order.
use super::*;
use martensite_tkbo::body_check::ResolutionRejected;

struct Encoded {
    bytes: Vec<u8>,
    history: CylinderFeatureHistory,
    report: CertifiedConeAttachmentReport,
}

pub(super) fn build(
    input: Inputs<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<(
    Vec<u8>,
    CylinderFeatureHistory,
    CertifiedConeAttachmentReport,
)> {
    let result = encode_and_qualify(input, cancelled)?;
    Ok((result.bytes, result.history, result.report))
}

fn encode_and_qualify(
    input: Inputs<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<Encoded> {
    let attempt = input.attempt;
    let sewing = seams::Carrier::capture(attempt, input.cone, input.resolution)?;
    let stock_proof = StockFrustumReceipt::issue(input.stock, input.cone, sewing.stock_chart())
        .map_err(|cause| stock_refusal(attempt, CylinderFeatureStage::Admission, cause))?;
    let mut shell = Frustum::admit(attempt, input.cone, input.stock)?;
    let [base, lid] = stock_disks(input.stock, input.cone, attempt)?;
    let installed_stock = InstalledStock {
        proof: stock_proof,
        band: shell.remainder.clone(),
        base,
        lid,
    };
    for &region in input.regions {
        let member = Member::capture(attempt, region, input.tool)?;
        shell.attach(attempt, member, &sewing, cancelled)?;
    }
    let (solid, images, traces) = shell.finish(attempt)?;
    let captured = history::capture_direct(
        attempt,
        input.cone,
        input.stock,
        input.tool,
        solid,
        &images,
        cancelled,
    )?;
    let stock = installed_stock.bind(attempt, &captured.index)?;
    let seams = traces
        .into_iter()
        .map(|trace| trace.bind(attempt, &captured.index))
        .collect::<FeatureChecked<Vec<_>>>()?;
    let (bytes, frozen) = body::detach(attempt, &captured, cancelled)?;
    let report = Ordered {
        input,
        frozen: &frozen,
        history: &captured.history,
        sewing: &sewing,
    }
    .qualify(stock, seams, cancelled)
    .map_err(classify_decoded_refusal)?;
    Ok(Encoded {
        bytes,
        history: captured.history,
        report,
    })
}

struct Ordered<'a> {
    input: Inputs<'a>,
    frozen: &'a DetachedCylinderFeature,
    history: &'a CylinderFeatureHistory,
    sewing: &'a seams::Carrier,
}

impl Ordered<'_> {
    fn qualify(
        self,
        original: stock_binding::FrozenStock,
        traces: Vec<seams::FrozenSeam<'_>>,
        cancelled: &mut CancelPoll<'_>,
    ) -> FeatureChecked<CertifiedConeAttachmentReport> {
        let attempt = self.input.attempt;
        self.input.resolution.admit().map_err(|(field, value)| {
            attempt.fail(
                CylinderFeatureStage::Validation,
                CylinderFeatureCause::Resolution(ResolutionRejected { field, value }),
            )
        })?;
        let limits = contact::LoadedTolerances::capture(attempt, self.frozen, self.history)?;
        let stock = original.rebind(attempt, self.frozen)?;
        let topology = topology::Topology::capture(attempt, self.frozen, &stock.boundary)
            .map_err(|cause| cause.feature(attempt))?;
        let incidence = seams::FrozenIncidence::capture(attempt, self.frozen)?;
        let installed = traces
            .into_iter()
            .map(|trace| trace.rebind(attempt, self.frozen, &incidence))
            .collect::<FeatureChecked<Vec<_>>>()?;
        let retained = self
            .input
            .retained
            .rebind(attempt, self.frozen, self.history, &installed, &incidence)
            .map_err(|cause| cause.feature(attempt))?;
        let geometry = geometry::qualify(attempt, self.frozen, &topology, &stock, &retained)?;
        let stock_uv = stock_uv::qualify(
            attempt,
            stock_uv::Input {
                history: self.history,
                body: self.frozen,
                topology: &topology,
                band: stock.band,
                chart: self.sewing.stock_chart(),
                welded: &installed,
                records: self.input.records,
                resolution: self.input.resolution,
            },
            cancelled,
        )?;
        let contact = contact::SourceGraph::capture(
            attempt,
            self.frozen,
            limits,
            &retained,
            &installed,
            &incidence,
        )?;
        let summary = self.contacts(&contact, &topology, &stock, &retained, cancelled)?;
        check_coverage(attempt, stock_uv, geometry[0], summary.sides)?;
        let maximum_tolerance = contact.limits.finish(&topology, self.input.resolution)?;
        let [faces, edges, vertices, loops] = topology.counts();
        let seam_residual_upper = installed
            .iter()
            .map(|trace| trace.gap_upper())
            .fold(0.0_f64, f64::max);
        attempt.poll(cancelled, CylinderFeatureStage::Publication)?;
        Ok(CertifiedConeAttachmentReport {
            faces,
            edges,
            vertices,
            loops,
            genus: topology.genus(),
            tool_sides: geometry[0],
            cap_edge_uses: geometry[1],
            stock_trim_pieces: summary.pieces,
            trim_piece_pairs: summary.closed_piece_pairs,
            incident_paths: summary.incident_cases,
            stock_contact_faces: summary.faces,
            contact_interval_visits: summary.interval_split_visits,
            contact_thresholds: [summary.threshold_min, summary.threshold_max],
            prefix_distance_upper: summary.prefix_hit_upper,
            foreign_clearance_query_upper: summary.foreign_cross_upper,
            contact_convexity_lower: [
                summary.normal_half_hessian_lower,
                summary.vertex_half_hessian_lower,
            ],
            contact_continuation_upper: [summary.own_core_upper, summary.incident_core_upper],
            maximum_tolerance,
            authored_pairs: 0,
            cap_pairs: 0,
            pair_bisections: 0,
            integration_cells: 0,
            arithmetic_operations: 0,
            family_operations: 0,
            requested_sew_limit: self.input.resolution.sew,
            seam_residual_upper,
        })
    }

    fn contacts(
        &self,
        source: &contact::SourceGraph,
        topology: &topology::Topology,
        stock: &stock_binding::BoundStock,
        retained: &retained::BoundSubset,
        cancelled: &mut CancelPoll<'_>,
    ) -> FeatureChecked<contact::prefix::Summary> {
        let attempt = self.input.attempt;
        let chart = self.sewing.stock_chart();
        attempt.poll(cancelled, CylinderFeatureStage::Validation)?;
        let separation =
            source.separate(attempt, self.frozen, stock, retained, chart.sew, cancelled)?;
        let collars = source.localize(attempt, &separation, stock, chart.sew)?;
        let windows = source
            .window_source(attempt, topology, chart, stock, &collars)?
            .census(attempt)?;
        attempt.poll(cancelled, CylinderFeatureStage::Validation)?;
        let summary = source.qualify_contact(attempt, &windows, chart, &collars)?;
        source.qualify_remaining_domains(attempt, &separation, stock, topology, retained)?;
        Ok(summary)
    }
}

fn classify_decoded_refusal(mut failure: CylinderFeatureFailure) -> CylinderFeatureFailure {
    // Shared assembly checks can be repeated against the frozen body during validation.
    if failure.stage == ASSEMBLY {
        failure.stage = CylinderFeatureStage::Validation;
    }
    failure
}

fn check_coverage(
    attempt: Attempt,
    stock_trim_domains: usize,
    retained_sides: usize,
    qualified_sides: usize,
) -> FeatureChecked<()> {
    if stock_trim_domains != 0 && retained_sides == qualified_sides {
        return Ok(());
    }
    Err(attempt.fail(
        CylinderFeatureStage::Validation,
        CylinderFeatureCause::ConicalCoverage {
            stock_trim_domains,
            retained_sides,
            qualified_sides,
        },
    ))
}

fn stock_disks(stock: &Shape, cone: &Face, attempt: Attempt) -> FeatureChecked<[Face; 2]> {
    let disks = occurrences(attempt, stock, ShapeEnum::Face)?
        .into_iter()
        .filter(|face| !face.is_same(cone))
        .map(|face| attempt.native(ASSEMBLY, "source stock disk", topods::face(&face)))
        .collect::<FeatureChecked<Vec<_>>>()?
        .try_into()
        .map_err(|_| refused(attempt, "two stock disks"))?;
    sort_stock_disks(attempt, disks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ATTEMPT: Attempt = Attempt {
        attachment: FaceAttachmentId(3),
        revision: SupportRevision(5),
        kind: CylinderFeatureKind::Boss,
    };

    #[test]
    fn decoded_stock_coverage_reports_actual_counts_at_validation() {
        assert!(check_coverage(ATTEMPT, 2, 7, 7).is_ok());
        for (domains, sides) in [(0, 7), (2, 6)] {
            let failure = check_coverage(ATTEMPT, domains, 7, sides).unwrap_err();
            assert_eq!(failure.stage, CylinderFeatureStage::Validation);
            assert!(matches!(
                *failure.cause,
                CylinderFeatureCause::ConicalCoverage {
                    stock_trim_domains,
                    retained_sides: 7,
                    qualified_sides,
                } if stock_trim_domains == domains && qualified_sides == sides
            ));
        }
    }

    #[test]
    fn decoded_contact_refusal_reclassifies_an_assembly_check_only() {
        let expected = "complete stock and retained tool contact partition";
        let contact = classify_decoded_refusal(refused(ATTEMPT, expected));
        assert_eq!(contact.stage, CylinderFeatureStage::Validation);
        assert!(matches!(
            *contact.cause,
            CylinderFeatureCause::DirectAttachment { expected: invariant } if invariant == expected
        ));
        let mismatch = ATTEMPT.fail(
            CylinderFeatureStage::Encoding,
            CylinderFeatureCause::ArchiveMismatch,
        );
        assert_eq!(
            classify_decoded_refusal(mismatch).stage,
            CylinderFeatureStage::Encoding
        );
    }
}
