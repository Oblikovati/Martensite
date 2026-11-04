//! Dynamic hover arrows use the selector's actual picked owner, point and normal.
use super::{object::TutorialObject, owner::TutorialOwner};
use martensite_tkernel::{
    ncollection::Vec2,
    standard::{Failure, Handle},
};
use martensite_tkmath::{
    gp::{Axis1, Direction3},
    toploc::{Datum3, Location},
};
use martensite_tkservice::{
    aspect::{AspectVKeyFlags, AspectVKeyMouse},
    graphic3d::*,
};
use martensite_tkv3d::{ais::AisInteractiveObjectMembers, prs_mgr::*, prs3d::*, select_mgr::*};

impl SelectMgrEntityOwnerSource for TutorialOwner {
    fn owner_base(&self) -> &SelectMgrEntityOwnerBase {
        &self.base
    }
    fn is_forced_hilight(&self) -> bool {
        true
    }
    fn hilight_with_color(
        &self,
        manager: &Handle<PrsMgrPresentationManager>,
        style: &Handle<Prs3dDrawer>,
        _: i32,
    ) -> Result<(), Failure> {
        let object = self.selectable().cast::<TutorialObject>();
        self.ensure_box(manager, &object)?;
        if manager.is_immediate_mode_on() {
            return self.hover_arrow(manager, style, &object);
        }
        let prs = self.presentation.read().clone();
        prs.set_transformation(&object.transformation_geom());
        prs.display();
        Ok(())
    }
    fn unhilight(&self, _: &Handle<PrsMgrPresentationManager>, _: i32) {
        let prs = self.presentation.read().clone();
        if !prs.is_null() {
            prs.erase();
        }
    }
    fn set_location(&self, location: &Location) {
        let prs = self.presentation.read().clone();
        if !prs.is_null() {
            prs.set_transformation(&Handle::new(Datum3::new(location.transformation())));
        }
    }
    fn handle_mouse_click(
        &self,
        _: Vec2<i32>,
        _: AspectVKeyMouse,
        _: AspectVKeyFlags,
        _: bool,
    ) -> bool {
        self.click();
        true
    }
}
impl TutorialOwner {
    fn ensure_box(
        &self,
        manager: &Handle<PrsMgrPresentationManager>,
        object: &Handle<TutorialObject>,
    ) -> Result<(), Failure> {
        if !self.presentation.read().is_null() {
            return Ok(());
        }
        let prs = Graphic3dStructure::new(&manager.structure_manager(), &Handle::default())?;
        object.compute(manager, &prs.cast(), 1)?;
        *self.presentation.write() = prs.cast();
        Ok(())
    }
    fn hover_arrow(
        &self,
        manager: &Handle<PrsMgrPresentationManager>,
        style: &Handle<Prs3dDrawer>,
        object: &Handle<TutorialObject>,
    ) -> Result<(), Failure> {
        let triangles = self.picked_arrow(object)?;
        let prs = object.get_hilight_presentation(manager)?;
        prs.clear(true);
        let group = prs.new_group()?;
        group.set_group_primitives_aspect(&style.arrow_aspect().aspect().cast());
        group.add_primitive_array(&triangles.cast(), true);
        prs.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOP);
        manager.add_to_immediate_list(&prs);
        Ok(())
    }
    fn picked_arrow(
        &self,
        object: &Handle<TutorialObject>,
    ) -> Result<Handle<Graphic3dArrayOfTriangles>, Failure> {
        let criterion = self.actual_pick(object)?;
        let normal = Direction3::new(
            criterion.normal.x as f64,
            criterion.normal.y as f64,
            criterion.normal.z as f64,
        )?;
        let axis = Axis1::new(criterion.point, normal)
            .transformed(object.inversed_transformation().trsf()?);
        Prs3dArrow::draw_shaded(&axis, 1., 15., 3., 4., 10)
    }
    fn actual_pick(
        &self,
        object: &Handle<TutorialObject>,
    ) -> Result<SelectMgrSortCriterion, Failure> {
        let selector = object.interactive_context().upgrade().main_selector();
        for rank in 1..=selector.nb_picked() {
            if selector.picked(rank)? == self.base.this() {
                return Ok(selector.picked_data(rank)?.clone());
            }
        }
        Ok(SelectMgrSortCriterion::default())
    }
}
