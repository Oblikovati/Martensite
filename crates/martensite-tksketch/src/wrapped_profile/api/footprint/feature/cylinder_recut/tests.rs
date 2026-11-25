use super::super::tests::{REVISION, indirect_extruded_circle, volume};
use super::*;
use martensite_tkbo::body_check::validate_body;
use std::collections::BTreeSet;
use std::f64::consts::PI;

#[test]
fn recut_preserves_the_indirect_circle_extrude_stock() {
    let attachment = indirect_extruded_circle();
    let original = attachment.export_native(REVISION).unwrap();
    let attempt = attempt(&attachment);
    let source_bytes = archive(attempt, &original.body).0;
    let sew = attachment.record().resolution.sew;
    let result = prepare(
        attempt,
        &original.body,
        &original.face,
        [-0.025, 0.025],
        sew,
    )
    .unwrap();
    assert_eq!(result.sheet, [-PI, PI]);
    assert!(result.spatial_upper < sew);
    assert!(result.clear_distance > 1.);
    assert_eq!(
        BRepTool::surface_with_location(&result.band).unwrap(),
        BRepTool::surface_with_location(&original.face).unwrap()
    );
    assert_eq!(archive(attempt, &original.body).0, source_bytes);
    assert!((volume(&result.solid) - 400. * PI).abs() < 1e-7);
    let (_, decoded) = archive(attempt, &result.solid);
    check_images(attempt, &original.body, &result, &decoded);
    let validation = validate_body(&decoded.body, &attachment.record().resolution).unwrap();
    assert!(validation.is_valid(), "{validation:?}");
}

#[test]
fn recut_refuses_a_hull_at_the_seam_and_a_changed_stock_occurrence() {
    let attachment = indirect_extruded_circle();
    let original = attachment.export_native(REVISION).unwrap();
    let attempt = attempt(&attachment);
    let sew = attachment.record().resolution.sew;
    let wrong = topods::face(&original.face.reversed()).unwrap();
    for (face, hull, expected) in [
        (
            &original.face,
            [PI.next_up(), PI + 0.01],
            "strict clearance from both images of the replacement seam",
        ),
        (
            &wrong,
            [-0.025, 0.025],
            "the exact selected band and two stock disks",
        ),
    ] {
        let cause = prepare(attempt, &original.body, face, hull, sew)
            .err()
            .unwrap();
        assert!(
            matches!(*cause.cause, CylinderFeatureCause::DirectAttachment { expected: x } if x == expected)
        );
    }
}

fn attempt(attachment: &CylinderFaceAttachment) -> Attempt {
    Attempt {
        attachment: attachment.record().id,
        revision: REVISION,
        kind: CylinderFeatureKind::Boss,
    }
}

fn archive(attempt: Attempt, solid: &Shape) -> (Vec<u8>, DetachedCylinderFeature) {
    let captured = history::Captured {
        index: body::Index::new(attempt, solid, STAGE).unwrap(),
        solid: solid.clone(),
        history: CylinderFeatureHistory {
            images: BTreeMap::new(),
            removed: vec![],
        },
    };
    body::detach(attempt, &captured, &mut || false).unwrap()
}

fn check_images(
    attempt: Attempt,
    source: &Shape,
    result: &RecutStock,
    decoded: &DetachedCylinderFeature,
) {
    let index = body::Index::new(attempt, source, STAGE).unwrap();
    let originals: BTreeMap<_, _> = index.members().map(Result::unwrap).collect();
    let index = body::Index::new(attempt, &result.solid, STAGE).unwrap();
    let published =
        |shape: &Shape| &decoded.entities[&index.reference(shape, attempt).unwrap().unwrap()];
    let sources: BTreeSet<_> = result.images.iter().map(|link| link.source).collect();
    assert_eq!(sources.len(), 8);
    assert_eq!(result.images.len(), sources.len());
    for link in &result.images {
        check_image(
            link,
            originals[&link.source],
            published(&link.image),
            result.sheet,
        );
        assert!(
            index
                .reference(originals[&link.source], attempt)
                .unwrap()
                .is_none()
        );
    }
    let band = topods::face(published(&result.band)).unwrap();
    check_paired_trim(attempt, &band, result.sheet);
}

fn check_image(link: &StockImage, old: &Shape, new: &Shape, sheet: [f64; 2]) {
    use CylinderFeatureRelation::{Generated, Modified};
    assert_eq!(new.shape_type().unwrap(), link.source.kind);
    assert_eq!(old.orientation(), new.orientation());
    assert_eq!(old.location(), new.location());
    match (link.source.kind, link.relation) {
        (ShapeEnum::Face, Modified) => {
            let old = topods::face(old).unwrap();
            let new = topods::face(new).unwrap();
            assert_eq!(
                face_tshape(&old, "test").unwrap().tolerance(),
                face_tshape(&new, "test").unwrap().tolerance()
            );
            assert_eq!(
                BRepTool::surface_with_location(&old).unwrap().1,
                BRepTool::surface_with_location(&new).unwrap().1
            );
        }
        (ShapeEnum::Edge, Modified | Generated) => {
            let old = topods::edge(old).unwrap();
            let new = topods::edge(new).unwrap();
            assert_eq!(
                edge_tshape(&old, "test").unwrap().tolerance(),
                edge_tshape(&new, "test").unwrap().tolerance()
            );
            assert!(BRepTool::same_parameter(&new).unwrap() && BRepTool::same_range(&new).unwrap());
            let curve = BRepTool::curve_with_location(&new).unwrap();
            if link.relation == Modified {
                assert_eq!([curve.first, curve.last], sheet);
                assert!(curve.curve.downcast_ref::<GeomCircle>().is_some());
            } else {
                assert_eq!([curve.first, curve.last], [0., 4.]);
                assert!(curve.curve.downcast_ref::<GeomLine>().is_some());
            }
        }
        (ShapeEnum::Vertex, Generated) => {
            let old = topods::vertex(old).unwrap();
            let new = topods::vertex(new).unwrap();
            assert_eq!(
                vertex_tshape(&old, "test").unwrap().tolerance(),
                vertex_tshape(&new, "test").unwrap().tolerance()
            );
            let start = BRepTool::pnt(&old).unwrap();
            let end = BRepTool::pnt(&new).unwrap();
            assert_eq!([start.x(), end.x()], [10., -10.]);
            assert_eq!([start.y(), start.z()], [end.y(), end.z()]);
        }
        item => panic!("unexpected stock ancestry {item:?}"),
    }
}

fn check_paired_trim(attempt: Attempt, band: &Face, sheet: [f64; 2]) {
    let [wire]: [Shape; 1] = children(attempt, band).unwrap().try_into().unwrap();
    let mut seam_uses = Vec::new();
    let mut rims = 0;
    for edge in children(attempt, &wire).unwrap() {
        let edge = topods::edge(&edge).unwrap();
        let uv = BRepTool::curve_on_face(&edge, band).unwrap();
        assert!(uv.is_stored);
        if BRepTool::is_closed_on_face(&edge, band).unwrap() {
            let line = uv.pcurve.downcast_ref::<Geom2dLine>().unwrap();
            seam_uses.push((line.location().x(), edge));
            assert_eq!([uv.first, uv.last], [0., 4.]);
        } else {
            assert!(BRepTool::is_closed(&edge).unwrap());
            assert_eq!([uv.first, uv.last], sheet);
            rims += 1;
        }
    }
    assert_eq!(rims, 2);
    let [(ua, a), (ub, b)] = &seam_uses[..] else {
        panic!("one paired seam, found {}", seam_uses.len());
    };
    assert!(a.is_same(b) && a.orientation() != b.orientation());
    assert_eq!([ua.min(*ub), ua.max(*ub)], sheet);
}
