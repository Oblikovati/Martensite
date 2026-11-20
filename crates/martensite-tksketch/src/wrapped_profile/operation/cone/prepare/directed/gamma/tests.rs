//! The issued gamma carrier binds the actual native constructor and stored-frame decoder.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomConicalSurface, GeomElementarySurface};
#[test]
fn issued_gamma_matches_native_cap_frame_bits_in_both_senses_and_rows() {
    for sense in [NormalSense::Outward, NormalSense::Inward] {
        check_carriers(sense);
    }
}
fn check_carriers(sense: NormalSense) {
    let mut request = crate::surface_sketch::wrapped_profile::operation::cone::tests::request();
    request.support.normal = sense;
    let inputs = input::capture(&request, &mut || false).unwrap();
    let mut work = Work::new(request.proof.arithmetic);
    let prepared = nearest(&request, &inputs.originals, &mut work);
    let inventory = Inventory::capture(&prepared).unwrap();
    let members = operation::source::by_region(&inputs.originals);
    let issued = certify(
        &inputs.originals,
        &members,
        &prepared,
        &inventory,
        &mut work,
        &mut || false,
    )
    .unwrap();
    let spans = spans(&members, &prepared, &inventory).unwrap();
    let actual = captured_caps(&request, &prepared, &inventory);
    assert_rebind(&issued, &spans, actual);
}
fn assert_rebind(
    issued: &ConicalDirectedGammaEvidence,
    spans: &[ConicalDirectedGammaSpan<'_>],
    actual: Vec<ConicalActualCapCarrier>,
) {
    let query = ConicalDirectedGammaQuery {
        spans,
        caps: &actual,
        refinement: ConicalPrimitiveRefinement::HalfSplitV1,
        options: issued.options(),
    };
    assert!(issued.matches(&query));
    let mut changed = actual.clone();
    changed[0].frame[1][2] = f64::from_bits(changed[0].frame[1][2].to_bits() ^ (1 << 63));
    assert!(!issued.matches(&ConicalDirectedGammaQuery {
        caps: &changed,
        ..query
    }));
}
fn nearest(
    request: &WrappedConeNormalRequest,
    originals: &[DevelopedRegionRequest],
    work: &mut Work,
) -> Vec<Prepared> {
    let domain = domain::certify(
        &ConeRun::from(request),
        originals,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        work,
        &mut || false,
    )
    .unwrap();
    prepare_regions(
        &ConeRun::from(request),
        originals,
        &domain,
        WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1,
        work,
        &mut || false,
    )
    .unwrap()
}
fn captured_caps(
    request: &WrappedConeNormalRequest,
    prepared: &[Prepared],
    inventory: &Inventory,
) -> Vec<ConicalActualCapCarrier> {
    let mut caps = Vec::new();
    for member in prepared {
        let built = native::construct(member, request.support).unwrap();
        let keys: Vec<_> = built.bindings.iter().map(|(key, _)| *key).collect();
        let bytes = crate::surface_sketch::encoding::encode(&built.solid, &built.bindings).unwrap();
        let (_, bindings) =
            crate::surface_sketch::wrapped_profile::pipeline::decode(&bytes, &keys).unwrap();
        append_caps(member, inventory, &bindings, &mut caps);
    }
    caps
}
fn append_caps(
    member: &Prepared,
    inventory: &Inventory,
    bindings: &[(BindingKey, Shape)],
    caps: &mut Vec<ConicalActualCapCarrier>,
) {
    for (row, role) in [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace]
        .into_iter()
        .enumerate()
    {
        let key = BindingKey {
            source: Source::Region(member.source.region),
            role,
        };
        let shape = &bindings
            .iter()
            .find(|(actual, _)| *actual == key)
            .unwrap()
            .1;
        let id = inventory.cap_pair(member.source.region).unwrap()[row];
        caps.push(captured_cap(member, row, id, shape));
    }
}
fn captured_cap(
    member: &Prepared,
    row: usize,
    id: ConicalActualCapId,
    shape: &Shape,
) -> ConicalActualCapCarrier {
    let mut expected = cap(member, row, id).unwrap();
    let surface = captured_surface(shape);
    expected.frame = captured_frame(&surface);
    expected.radius = surface.ref_radius();
    expected.axial_origin = surface.position().location().z();
    expected.semi_angle = surface.semi_angle();
    expected
}
fn captured_surface(shape: &Shape) -> GeomConicalSurface {
    let face = topods::face(shape).unwrap();
    let (handle, location) = BRepTool::surface_with_location(&face).unwrap();
    assert!(location.is_identity());
    let cone: Handle<GeomConicalSurface> = handle.cast();
    cone.get().unwrap().clone()
}
fn captured_frame(cone: &GeomConicalSurface) -> [[f64; 3]; 4] {
    let axis = cone.position();
    let origin = axis.location();
    let mut frame = [[origin.x(), origin.y(), origin.z()]; 4];
    for (row, direction) in
        frame[1..]
            .iter_mut()
            .zip([axis.x_direction(), axis.y_direction(), axis.direction()])
    {
        *row = [direction.x(), direction.y(), direction.z()];
    }
    frame
}
