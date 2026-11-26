// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::{
    ArcDefinition, ArcFitTolerance, ArcId, CurveId, Edit, MinimumLength, PointRole, SketchArc,
    SketchPoint, Snapshot,
};
use std::num::NonZeroU64;

fn point(id: u64, position: Point3, role: PointRole) -> SketchPoint {
    SketchPoint {
        id: PointId::new(NonZeroU64::new(id).unwrap()),
        position,
        role,
    }
}

fn quarter() -> (MixedWireRecipe, ArcId) {
    let first = point(1, Point3::new(2., 0., 0.), PointRole::Defining);
    let last = point(2, Point3::new(0., 2., 0.), PointRole::Defining);
    let center = point(3, Point3::default(), PointRole::CenterMarker);
    let id = ArcId::new(NonZeroU64::new(4).unwrap());
    let arc = SketchArc {
        id,
        endpoints: [first.id, last.id],
        definition: ArcDefinition::Center {
            center: center.id,
            normal: Direction3::Z,
        },
        fit_tolerance: ArcFitTolerance::new(1e-12).unwrap(),
    };
    let sketch = Snapshot::empty(MinimumLength::new(1e-6).unwrap())
        .apply(
            &[
                Edit::InsertPoint(first),
                Edit::InsertPoint(last),
                Edit::InsertPoint(center),
                Edit::InsertArc(arc),
            ],
            || false,
        )
        .unwrap();
    let source = DirectedCurveUse {
        curve: CurveId::Arc(id),
        direction: LineDirection::Forward,
    };
    let recipe = sketch
        .mixed_wire_recipe(&[source], WireClosure::Open, 0, || false)
        .unwrap();
    (recipe, id)
}

#[test]
fn actual_radius_and_trim_mutations_refuse_native_publication() {
    let (recipe, arc) = quarter();
    let (wire, edges, points) = build(&recipe, &mut || false).unwrap();
    let stored = BRepTool::curve_with_location(&edges[0]).unwrap();
    stored
        .curve
        .downcast_ref::<GeomCircle>()
        .unwrap()
        .set_radius(3.)
        .unwrap();
    assert!(
        matches!(readback(&recipe, &wire, edges, &points, &mut || false),
        Err(WireFailure::NativeMismatch { entity: Some(EntityId::Arc(id)), expected: "source circle coefficients" }) if id == arc)
    );

    let (wire, edges, points) = build(&recipe, &mut || false).unwrap();
    BRepBuilder::default()
        .range(&edges[0], 0., 1., true)
        .unwrap();
    assert!(
        matches!(readback(&recipe, &wire, edges, &points, &mut || false),
        Err(WireFailure::NativeMismatch { entity: Some(EntityId::Arc(id)), expected: "original model-space trim" }) if id == arc)
    );
}
