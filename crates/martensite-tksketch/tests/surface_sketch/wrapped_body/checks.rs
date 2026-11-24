//! Native validation and an independent ideal-volume oracle complement isolation checks.
use super::*;
use martensite_tkbo::body_check::validate_body;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};
pub(super) fn body(body: &Shape, delta: f64) {
    let resolution = super::super::refusals::options().resolution;
    let report = validate_body(body, &resolution).unwrap();
    assert!(report.is_valid(), "{report:?}");
    assert!(!report.is_exact());
    assert_eq!(report.census.len(), 1);
    assert_eq!(report.census[0].shells, 1);
    volume(body, delta);
}
fn mass(body: &Shape) -> f64 {
    let mut properties = GProps::new();
    let error = BRepGProp::volume_properties_gk(
        body,
        &mut properties,
        1e-8,
        true,
        VolumeGKFlags::default(),
    )
    .unwrap();
    assert!(
        error.is_finite() && (0. ..=1e-8).contains(&error),
        "integration={error}"
    );
    properties.mass()
}
fn volume(body: &Shape, delta: f64) {
    let actual = mass(body);
    let expected = 400. * std::f64::consts::PI + delta;
    assert!(
        (actual - expected).abs() <= 0.01,
        "volume={actual} ideal={expected}"
    );
}
pub(super) fn isolated(
    attachment: &CylinderFaceAttachment,
    source: &DetachedCylinderSupport,
    tool: &WrappedNormalResult,
    native: &DetachedWrappedBodies,
) {
    // Source byte identity is checked before reattachment consumes any history.
    let restored = CylinderFaceAttachment::restore(attachment.record().clone()).unwrap();
    assert_eq!(restored.record(), attachment.record());
    let again = attachment.export_native(SupportRevision(3)).unwrap();
    assert!(!again.body.is_partner(&source.body));
    let fresh_tool = tool.export_native().unwrap();
    assert!(!fresh_tool.bodies.is_partner(&native.bodies));
}

pub(super) fn history(operation: &BRepAlgoAPIBooleanOperation, tool: &DetachedWrappedBodies) {
    use martensite_tkbrep::{topexp, toptools::IndexedMapOfShape};
    let mut output = IndexedMapOfShape::default();
    topexp::map_shapes(operation.shape(), &mut output, true, true);
    let history = operation.history().unwrap();
    for (key, entity) in &tool.entities {
        let mut images = history.modified(entity).unwrap();
        images.extend(history.generated(entity).unwrap());
        for image in &images {
            assert!(
                output.contains(image),
                "{key:?} has a non-result history image"
            );
        }
        assert!(
            !images.is_empty() || history.is_removed(entity).unwrap() || output.contains(entity),
            "{key:?} missing Boolean correspondence"
        );
    }
}

pub(super) fn rebound(
    attachment: &CylinderFaceAttachment,
    source: &DetachedCylinderSupport,
    result: &BRepAlgoAPIBooleanOperation,
) -> CylinderFaceAttachment {
    let rebound = attachment
        .reattach(
            source,
            result.shape(),
            SupportRevision(4),
            result.history().unwrap(),
        )
        .unwrap();
    assert!(rebound.validation().is_valid());
    rebound
}
pub(super) fn tool_signature(native: &DetachedWrappedBodies) -> String {
    super::super::replay::geometry_bytes::bound_signature(&native.bodies, native.entities.iter())
}
