//! Public virtual policies must observe settings changed by preceding callbacks.
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState},
};
use martensite_tkmath::gp::Plane3;
use martensite_tkshhealing::shape_upgrade::{
    FaceDivideData, FaceDivider, ShapeUpgradeFaceDivide, ShapeUpgradeShapeDivide, ShapeUpgradeTool,
    ShapeUpgradeWireDivide, WireDivider,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::sync::{Arc, Weak};

struct ReconfiguringFace {
    parent: ShapeUpgradeFaceDivide,
    operation: Weak<ShapeUpgradeShapeDivide>,
    observed: SharedState<Vec<(f64, i32)>>,
}
impl_transient!(ReconfiguringFace,"HealingTest_ReconfiguringFace",ShapeUpgradeFaceDivide,parent,interfaces:[dyn FaceDivider]);
impl FaceDivider for ReconfiguringFace {
    fn face_divide_data(&self) -> &FaceDivideData {
        self.parent.face_divide_data()
    }
    fn face_divide_base(&self) -> &ShapeUpgradeTool {
        self.parent.face_divide_base()
    }
    fn wire_divide_tool(&self) -> Handle<ShapeUpgradeWireDivide> {
        let operation = self.operation.upgrade().unwrap();
        operation.set_min_tolerance(0.03125);
        operation.set_edge_mode(7);
        self.parent.wire_divide_tool()
    }
    fn perform(&self, _area: f64) -> Result<bool, Failure> {
        let wire = self.parent.wire_divide_tool();
        let mode = wire.wire_divide_data().with(|fields| fields.edge_mode);
        self.observed.write().push((wire.min_tolerance(), mode));
        Ok(false)
    }
}

fn reconfiguring_operation() -> (Arc<ShapeUpgradeShapeDivide>, Handle<ReconfiguringFace>) {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 3.))
        .unwrap()
        .face()
        .unwrap();
    let operation = Arc::new(ShapeUpgradeShapeDivide::new(face.into_shape()));
    operation.set_min_tolerance(0.015625);
    operation.set_edge_mode(1);
    let policy = Handle::new(ReconfiguringFace {
        parent: ShapeUpgradeFaceDivide::empty(),
        operation: Arc::downgrade(&operation),
        observed: SharedState::new(Vec::new()),
    });
    operation.set_split_face_tool(&policy.cast());
    (operation, policy)
}

#[test]
fn wire_selection_callback_updates_settings_before_face_execution() {
    let (operation, policy) = reconfiguring_operation();
    assert!(!operation.perform(true).unwrap());
    assert_eq!(*policy.observed.read(), vec![(0.03125, 7)]);
}

struct CountingFace {
    parent: ShapeUpgradeFaceDivide,
    selections: SharedState<usize>,
}
impl_transient!(CountingFace,"HealingTest_CountingFace",ShapeUpgradeFaceDivide,parent,interfaces:[dyn FaceDivider]);
impl FaceDivider for CountingFace {
    fn face_divide_data(&self) -> &FaceDivideData {
        self.parent.face_divide_data()
    }
    fn face_divide_base(&self) -> &ShapeUpgradeTool {
        self.parent.face_divide_base()
    }
    fn wire_divide_tool(&self) -> Handle<ShapeUpgradeWireDivide> {
        *self.selections.write() += 1;
        self.parent.wire_divide_tool()
    }
}

fn cylindrical_face() -> martensite_tkbrep::topods::Face {
    use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomSurface};
    use martensite_tkmath::gp::Axis3;
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(Axis3::default(), 1.).unwrap()).cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (0., 3., 0., 2.), 1e-7)
        .unwrap()
        .face()
        .unwrap()
}

fn cylindrical_policy() -> CountingFace {
    use martensite_tkshhealing::{
        shape_build::ShapeBuildReShape, shape_upgrade::ShapeUpgradeSplitSurfaceAngle,
    };
    let policy = CountingFace {
        parent: ShapeUpgradeFaceDivide::new(cylindrical_face()),
        selections: SharedState::new(0),
    };
    policy
        .parent
        .set_context(&Handle::new(ShapeBuildReShape::new()));
    policy
        .parent
        .set_split_surface_tool(&Handle::new(ShapeUpgradeSplitSurfaceAngle::new(1.5)).cast());
    policy
}

#[test]
fn surface_composition_selects_wire_once_for_guard_and_again_for_transfer() {
    let policy = cylindrical_policy();
    assert!(policy.split_surface(0.).unwrap());
    assert_eq!(*policy.selections.read(), 2);
}
