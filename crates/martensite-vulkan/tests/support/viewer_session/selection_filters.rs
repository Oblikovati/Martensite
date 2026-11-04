//! What picking may return: shape-type filters, per-object selection modes and the
//! picking strategy of overlapping entities.
use super::*;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::ais::AisSelectionModesConcurrency;
use martensite_tkv3d::select_mgr::{SelectMgrFilter, SelectMgrPickingStrategy};
use martensite_tkv3d::std_select::StdSelectShapeTypeFilter;

impl ViewerSession {
    /// Remove every selection filter of the context; `session.clear_selection_filters()`.
    pub fn clear_selection_filters(&self) {
        self.context.remove_filters();
    }

    /// Accept only owners of sub-shapes of `shape_type`, or interactive shapes for `Shape`.
    /// Example: `session.add_shape_type_filter(ShapeEnum::Face)`.
    pub fn add_shape_type_filter(&self, shape_type: ShapeEnum) {
        let filter: Handle<dyn SelectMgrFilter> = if shape_type == ShapeEnum::Shape {
            Handle::new(AisTypeFilter::new(AisKindOfInteractive::Shape)).cast()
        } else {
            Handle::new(StdSelectShapeTypeFilter::new(shape_type)).cast()
        };
        self.context.add_filter(&filter);
    }

    /// Choose which of overlapping detected entities picking keeps.
    /// Example: `session.set_picking_strategy(SelectMgrPickingStrategy::OnlyTopmost)`.
    pub fn set_picking_strategy(&self, strategy: SelectMgrPickingStrategy) {
        self.context.set_picking_strategy(strategy);
    }

    /// Turn selection `mode` on or off for the object named `name` only.
    /// Example: `session.set_object_selection_mode("b1", 4, true)?`.
    pub fn set_object_selection_mode(
        &self,
        name: &str,
        mode: i32,
        active: bool,
    ) -> Result<(), Failure> {
        let object = self.object(name).cloned().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "selection mode of {name:?}; expected a displayed object name"
            ))
        })?;
        self.context.set_selection_mode_active(
            &object,
            mode,
            active,
            AisSelectionModesConcurrency::GlobalOrLocal,
            false,
        )
    }
}
