use super::*;
use crate::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon};
use martensite_tkbrep::topods::{edge, vertex, wire};
use martensite_tkmath::gp::Point3;

const KINDS: [ShapeEnum; 3] = [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Wire];

#[test]
fn enumerated_native_members_match_the_public_context_statuses() {
    let context = triangle();
    for kind in KINDS {
        for member in sub_shapes(&context, kind).unwrap() {
            let occurrence = EnumeratedContext {
                context: &context,
                member: &member,
            };
            let ordinary = checker(&member);
            ordinary.get().unwrap().in_context(&context).unwrap();
            let enumerated = checker(&member);
            check_enumerated(&enumerated, &occurrence, None).unwrap();
            assert_eq!(
                statuses(&enumerated, &context),
                statuses(&ordinary, &context),
                "{kind:?}"
            );
        }
    }
}

#[test]
fn an_enumerated_member_does_not_admit_a_foreign_checker_or_context() {
    let context = triangle();
    let foreign = triangle();
    for kind in KINDS {
        let member = &sub_shapes(&context, kind).unwrap()[0];
        let other = &sub_shapes(&foreign, kind).unwrap()[0];
        assert!(!member.is_same(other));
        let occurrence = EnumeratedContext {
            context: &context,
            member,
        };
        assert!(!occurrence.includes(&foreign, member));
        let mode = checker(other);
        check_enumerated(&mode, &occurrence, None).unwrap();
        assert_eq!(
            statuses(&mode, &context),
            [BRepCheckStatus::SubshapeNotInShape],
            "{kind:?}"
        );
        let ordinary = checker(other);
        ordinary.get().unwrap().in_context(&context).unwrap();
        assert_eq!(statuses(&ordinary, &context), statuses(&mode, &context));
    }
}

pub(super) fn triangle() -> Shape {
    let polygon = BRepBuilderAPIMakePolygon::from_three_points(
        Point3::new(0., 0., 0.),
        Point3::new(1., 0., 0.),
        Point3::new(0., 1., 0.),
        true,
    )
    .unwrap();
    BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

fn checker(shape: &Shape) -> Handle<dyn BRepCheckResult> {
    match shape.shape_type().unwrap() {
        ShapeEnum::Vertex => {
            Handle::new(BRepCheckVertex::new(&vertex(shape).unwrap()).unwrap()).into_upcast()
        }
        ShapeEnum::Edge => {
            Handle::new(BRepCheckEdge::new(&edge(shape).unwrap()).unwrap()).into_upcast()
        }
        ShapeEnum::Wire => {
            Handle::new(BRepCheckWire::new(&wire(shape).unwrap()).unwrap()).into_upcast()
        }
        _ => panic!("unexpected analyzer member kind"),
    }
}

fn statuses(checker: &Handle<dyn BRepCheckResult>, context: &Shape) -> Vec<BRepCheckStatus> {
    checker
        .get()
        .unwrap()
        .status_on_shape_of(context)
        .unwrap()
        .iter()
        .copied()
        .collect()
}
