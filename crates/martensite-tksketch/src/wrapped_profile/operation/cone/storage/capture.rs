//! Decode analytic cone datums; generic retained curve and ruled-net checks are shared.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    geom::{GeomConicalSurface, GeomElementarySurface},
    top_abs::Orientation,
};
pub(super) struct SourceCapture<'a> {
    pub original: &'a DevelopedRegionRequest,
    pub prepared: &'a Prepared,
    pub bindings: &'a Bindings<'a>,
    pub carriers: [ConicalTrimCarrier; 2],
    pub occurrences: &'a coverage::Occurrences,
}
pub(super) fn carriers(
    prepared: &Prepared,
    bindings: &Bindings<'_>,
) -> ConeChecked<[ConicalTrimCarrier; 2]> {
    Ok([
        carrier(prepared, 0, bindings)?,
        carrier(prepared, 1, bindings)?,
    ])
}
fn carrier(
    prepared: &Prepared,
    depth: usize,
    bindings: &Bindings<'_>,
) -> ConeChecked<ConicalTrimCarrier> {
    let key = region_key(
        prepared.source.region,
        [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace][depth],
    );
    let (cone, located) = cone_surface(prepared.source.region, key, bindings)?;
    let cone = retained_cone(&cone, prepared.source.region)?;
    let frame = cone.position();
    let result = ConicalTrimCarrier {
        radius: cone.ref_radius(),
        axial_origin: frame.location().z(),
        semi_angle: cone.semi_angle(),
    };
    check_carrier(prepared, depth, result, &frame, located)?;
    Ok(result)
}
fn retained_cone(
    cone: &Handle<GeomConicalSurface>,
    region: SurfaceRegionId,
) -> ConeChecked<&GeomConicalSurface> {
    cone.get().ok_or_else(|| {
        mismatch(
            region,
            None,
            "decoded analytic cone class",
            vec![0],
            vec![1],
        )
    })
}
fn cone_surface(
    region: SurfaceRegionId,
    key: BindingKey,
    bindings: &Bindings<'_>,
) -> ConeChecked<(Handle<GeomConicalSurface>, bool)> {
    let shape = retained::lookup(key, bindings).map_err(|cause| captured(region, cause))?;
    let face = topods::face(shape).map_err(|cause| native_failure(region, cause))?;
    let (surface, location) =
        BRepTool::surface_with_location(&face).map_err(|cause| native_failure(region, cause))?;
    Ok((surface.downcast(), location.is_identity()))
}
fn native_failure(region: SurfaceRegionId, cause: Failure) -> ConeFailure {
    failure(region, None, ConeCause::Native(Box::new(cause)))
}
fn check_carrier(
    prepared: &Prepared,
    depth: usize,
    carrier: ConicalTrimCarrier,
    frame: &martensite_tkmath::gp::Axis3,
    located: bool,
) -> ConeChecked<()> {
    let mut actual = vec![carrier.radius, carrier.axial_origin, carrier.semi_angle];
    actual.extend(frame_coefficients(frame));
    let expected = expected_coefficients(prepared, depth);
    if actual != expected || !located {
        return Err(carrier_mismatch(
            prepared.source.region,
            actual,
            expected,
            located,
        ));
    }
    Ok(())
}
fn frame_coefficients(frame: &martensite_tkmath::gp::Axis3) -> Vec<f64> {
    let p = frame.location();
    let mut values = vec![p.x(), p.y()];
    for direction in [frame.x_direction(), frame.y_direction(), frame.direction()] {
        values.extend([direction.x(), direction.y(), direction.z()]);
    }
    values
}
fn expected_coefficients(prepared: &Prepared, depth: usize) -> Vec<f64> {
    let datum = native::cap_datum(prepared, prepared.semi_angle, depth);
    let mut expected = vec![datum.radius, datum.axial_origin, datum.semi_angle];
    expected.extend([0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 1.]);
    expected
}
fn carrier_mismatch(
    region: SurfaceRegionId,
    actual: Vec<f64>,
    expected: Vec<f64>,
    located: bool,
) -> ConeFailure {
    let actual = actual
        .into_iter()
        .map(f64::to_bits)
        .chain([u64::from(located)])
        .collect();
    let expected = expected.into_iter().map(f64::to_bits).chain([1]).collect();
    mismatch(
        region,
        None,
        "decoded cone datum/frame/location",
        actual,
        expected,
    )
}
impl SourceCapture<'_> {
    pub(super) fn span(
        &self,
        piece: &AuthoredPiece,
        work: &mut Work,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<SpanProof> {
        let curves = retained::captured_span(piece, self.bindings)
            .map_err(|cause| captured(self.original.region, cause))?;
        let boundary = boundary::certify(
            self.original,
            self.prepared,
            piece,
            &curves,
            work,
            cancelled,
        )?;
        let storage = self.storage(piece.span, &boundary)?;
        let trims = trims::certify(self, piece, &curves, &boundary, work, cancelled)?;
        self.finish(piece.span, curves, boundary, trims, storage)
    }
    fn finish(
        &self,
        span: CurveSpan,
        curves: [retained::OwnedCurve; 2],
        boundary: std::sync::Arc<ConicalBoundaryEvidence>,
        trims: trims::Trims,
        storage: allocation::StorageEvidence,
    ) -> ConeChecked<SpanProof> {
        let (side_reversed, cap_reversed) = self.orientations(span)?;
        Ok(SpanProof {
            span,
            boundary,
            storage,
            trims: trims.proofs,
            developed_boundary: trims.developed_boundary,
            curves,
            pcurves: trims.pcurves,
            cap_pcurves: trims.count,
            side_pcurves: trims.side_count,
            maximum_tolerance: trims.maximum_tolerance,
            side_reversed,
            cap_reversed,
        })
    }
    fn storage(
        &self,
        span: CurveSpan,
        boundary: &ConicalBoundaryEvidence,
    ) -> ConeChecked<allocation::StorageEvidence> {
        allocation::certify(
            self.original.region,
            span,
            boundary,
            self.prepared.source.tolerance,
        )
    }
    fn orientations(&self, span: CurveSpan) -> ConeChecked<(bool, [bool; 2])> {
        let key = retained::key(span, HistoryRole::SideFace);
        let actual = self
            .occurrences
            .face_orientation(key)
            .map_err(|cause| self.coverage(cause))?;
        Ok((
            reversed(self.original.region, span, actual)?,
            self.cap_orientation(span)?,
        ))
    }
    fn cap_orientation(&self, span: CurveSpan) -> ConeChecked<[bool; 2]> {
        Ok([
            self.cap_use(
                span,
                HistoryRole::FirstDepthEdge,
                HistoryRole::FirstDepthFace,
            )?,
            self.cap_use(span, HistoryRole::LastDepthEdge, HistoryRole::LastDepthFace)?,
        ])
    }
    fn cap_use(&self, span: CurveSpan, edge: HistoryRole, face: HistoryRole) -> ConeChecked<bool> {
        let edge = retained::key(span, edge);
        let face = region_key(self.original.region, face);
        let actual = self
            .occurrences
            .edge_orientation(edge, face)
            .map_err(|cause| self.coverage(cause))?;
        reversed(self.original.region, span, actual)
    }
    fn coverage(&self, cause: coverage::CoverageFailure) -> ConeFailure {
        failure(
            self.original.region,
            None,
            ConeCause::Coverage(Box::new(cause)),
        )
    }
}
fn reversed(region: SurfaceRegionId, span: CurveSpan, actual: Orientation) -> ConeChecked<bool> {
    match actual {
        Orientation::Forward => Ok(false),
        Orientation::Reversed => Ok(true),
        _ => Err(mismatch(
            region,
            Some(span),
            "flux actual occurrence orientation",
            vec![actual as u64],
            vec![Orientation::Forward as u64, Orientation::Reversed as u64],
        )),
    }
}
pub(super) fn region_key(region: SurfaceRegionId, role: HistoryRole) -> BindingKey {
    BindingKey {
        source: Source::Region(region),
        role,
    }
}
