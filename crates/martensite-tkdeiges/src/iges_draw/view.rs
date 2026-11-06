//! IgesDrawView.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_data::{IgesDataTransfEntity, IgesDataViewKindEntity};
use crate::iges_geom::IgesGeomPlane;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle, WeakHandle};
use martensite_tkmath::gp::Xyz;

iges_entity! {
/// defines IGES View Entity, Type `410` Form `0`
///
/// Used to define a framework for specifying a viewing
/// orientation of an object in three dimensional model
/// space (X,Y,Z). The framework is also used to support
/// the projection of all or part of model space onto a
/// view volume.
IgesDrawView, IgesDrawViewState, "IgesDrawView", kind dyn IgesDataViewKindEntity, {
the_view_number: i32,
the_scale_factor: f64,
the_left_plane: Handle<IgesGeomPlane>,
the_top_plane: Handle<IgesGeomPlane>,
the_right_plane: Handle<IgesGeomPlane>,
the_bottom_plane: Handle<IgesGeomPlane>,
the_back_plane: Handle<IgesGeomPlane>,
the_front_plane: Handle<IgesGeomPlane>,
the_this: WeakHandle<dyn IgesDataViewKindEntity>,
}
}

impl IgesDataViewKindEntity for IgesDrawView {
    fn is_single(&self) -> bool {
        true
    }
    fn nb_views(&self) -> Result<i32, Failure> {
        Ok(1)
    }
    fn view_item(&self, _num: i32) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure> {
        Ok(self.state.read().the_this.upgrade())
    }
}

impl IgesDrawView {
    iges_methods! {
    get view_number = the_view_number: i32;
    get scale_factor = the_scale_factor: f64;
    get left_plane = the_left_plane: Handle<IgesGeomPlane>;
    get top_plane = the_top_plane: Handle<IgesGeomPlane>;
    get right_plane = the_right_plane: Handle<IgesGeomPlane>;
    get bottom_plane = the_bottom_plane: Handle<IgesGeomPlane>;
    get back_plane = the_back_plane: Handle<IgesGeomPlane>;
    get front_plane = the_front_plane: Handle<IgesGeomPlane>;
    }
    /// This method is used to set fields of the class View
    /// - aViewNum     : View number
    /// - aScale       : Scale factor
    /// - aLeftPlane   : Left   plane of view volume
    /// - aTopPlane    : Top    plane of view volume
    /// - aRightPlane  : Right  plane of view volume
    /// - aBottomPlane : Bottom plane of view volume
    /// - aBackPlane   : Back   plane of view volume
    /// - aFrontPlane  : Front  plane of view volume
    pub fn init(
        &self,
        a_view_num: i32,
        a_scale: f64,
        a_left_plane: &Handle<IgesGeomPlane>,
        a_top_plane: &Handle<IgesGeomPlane>,
        a_right_plane: &Handle<IgesGeomPlane>,
        a_bottom_plane: &Handle<IgesGeomPlane>,
        a_back_plane: &Handle<IgesGeomPlane>,
        a_front_plane: &Handle<IgesGeomPlane>,
    ) {
        {
            let mut state = self.state.write();
            state.the_view_number = a_view_num;
            state.the_scale_factor = a_scale;
            state.the_left_plane = a_left_plane.clone();
            state.the_top_plane = a_top_plane.clone();
            state.the_right_plane = a_right_plane.clone();
            state.the_bottom_plane = a_bottom_plane.clone();
            state.the_back_plane = a_back_plane.clone();
            state.the_front_plane = a_front_plane.clone();
        }
        self.init_type_and_form(410, 0);
    }
    /// returns False if left side of view volume is not present
    pub fn has_left_plane(&self) -> bool {
        !(self.state.read().the_left_plane.is_null())
    }
    /// returns False if top of view volume is not present
    pub fn has_top_plane(&self) -> bool {
        !(self.state.read().the_top_plane.is_null())
    }
    /// returns False if right side of view volume is not present
    pub fn has_right_plane(&self) -> bool {
        !(self.state.read().the_right_plane.is_null())
    }
    /// returns False if bottom of view volume is not present
    pub fn has_bottom_plane(&self) -> bool {
        !(self.state.read().the_bottom_plane.is_null())
    }
    /// returns False if back of view volume is not present
    pub fn has_back_plane(&self) -> bool {
        !(self.state.read().the_back_plane.is_null())
    }
    /// returns False if front of view volume is not present
    pub fn has_front_plane(&self) -> bool {
        !(self.state.read().the_front_plane.is_null())
    }
    /// returns the Transformation Matrix
    pub fn view_matrix(&self) -> Handle<dyn IgesDataTransfEntity> {
        self.transf()
    }
    /// returns XYZ from the Model space to the View space by
    /// applying the View Matrix
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn model_to_view(&self, coords: Xyz) -> Result<Xyz, Failure> {
        let mut temp_coords: Xyz = coords;
        self.location()?.transforms(&mut temp_coords);
        Ok(temp_coords)
    }
    /// A shared view that knows its own handle, as `view_item` answers with it.
    pub fn new_bound() -> Handle<Self> {
        let view = Handle::new(Self::new());
        view.state.write().the_this = view.cast::<dyn IgesDataViewKindEntity>().downgrade();
        view
    }
}
