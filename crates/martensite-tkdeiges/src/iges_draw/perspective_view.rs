//! IgesDrawPerspectiveView.
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
use crate::{iges_entity, iges_methods};
use martensite_tkernel::standard::{Failure, Handle, WeakHandle};
use martensite_tkmath::gp::{Point2, Point3, Vector3, Xy, Xyz};

iges_entity! {
/// defines IGESPerspectiveView, Type `410` Form `1`
///
/// Supports a perspective view.
/// Any geometric projection is defined by a view plane
/// and the projectors that pass through the view plane.
/// Projectors can be visualized as rays of light that
/// form an image by passing through the viewed object
/// and striking the view plane.
/// The projectors are defined via a point called the
/// Centre-of-Projection or the eye-point.
/// A perspective view is formed by all projectors that
/// emanate from the Centre-of-Projection and pass
/// through the view plane.
IgesDrawPerspectiveView, IgesDrawPerspectiveViewState, "IgesDrawPerspectiveView", kind dyn IgesDataViewKindEntity, {
the_view_number: i32,
the_scale_factor: f64,
the_view_normal_vector: Xyz,
the_view_reference_point: Xyz,
the_center_of_projection: Xyz,
the_view_up_vector: Xyz,
the_view_plane_distance: f64,
the_top_left: Xy,
the_bottom_right: Xy,
the_depth_clip: i32,
the_back_plane_distance: f64,
the_front_plane_distance: f64,
the_this: WeakHandle<dyn IgesDataViewKindEntity>,
}
}

impl IgesDataViewKindEntity for IgesDrawPerspectiveView {
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

impl IgesDrawPerspectiveView {
    iges_methods! {
    get view_number = the_view_number: i32;
    get scale_factor = the_scale_factor: f64;
    get view_plane_distance = the_view_plane_distance: f64;
    get depth_clip = the_depth_clip: i32;
    get back_plane_distance = the_back_plane_distance: f64;
    get front_plane_distance = the_front_plane_distance: f64;
    }
    /// This method is used to set the fields of the class
    /// PerspectiveView
    /// - aViewNumber         : The desired view
    /// - aScaleFactor        : Scale factor
    /// - aViewNormalVector   : View plane normal vector (model space)
    /// - aViewReferencePoint : View reference point     (model space)
    /// - aCenterOfProjection : Center Of Projection     (model space)
    /// - aViewUpVector       : View up vector           (model space)
    /// - aViewPlaneDistance  : View plane distance      (model space)
    /// - aTopLeft            : Top-left point of clipping window
    /// - aBottomRight        : Bottom-right point of clipping window
    /// - aDepthClip          : Depth clipping indicator
    /// - aBackPlaneDistance  : Distance of back clipping plane
    /// - aFrontPlaneDistance : Distance of front clipping plane
    pub fn init(
        &self,
        a_view_number: i32,
        a_scale_factor: f64,
        a_view_normal_vector: Xyz,
        a_view_reference_point: Xyz,
        a_center_of_projection: Xyz,
        a_view_up_vector: Xyz,
        a_view_plane_distance: f64,
        a_top_left: Xy,
        a_bottom_right: Xy,
        a_depth_clip: i32,
        a_back_plane_distance: f64,
        a_front_plane_distance: f64,
    ) {
        {
            let mut state = self.state.write();
            state.the_view_number = a_view_number;
            state.the_scale_factor = a_scale_factor;
            state.the_view_normal_vector = a_view_normal_vector;
            state.the_view_reference_point = a_view_reference_point;
            state.the_center_of_projection = a_center_of_projection;
            state.the_view_up_vector = a_view_up_vector;
            state.the_view_plane_distance = a_view_plane_distance;
            state.the_top_left = a_top_left;
            state.the_bottom_right = a_bottom_right;
            state.the_depth_clip = a_depth_clip;
            state.the_back_plane_distance = a_back_plane_distance;
            state.the_front_plane_distance = a_front_plane_distance;
        }
        self.init_type_and_form(410, 1);
    }
    /// returns the View plane normal vector (model space)
    pub fn view_normal_vector(&self) -> Vector3 {
        let mut temp_res: Vector3 = Vector3::from(self.state.read().the_view_normal_vector);
        temp_res
    }
    /// returns the View reference point (model space)
    pub fn view_reference_point(&self) -> Point3 {
        let mut temp_res: Point3 = Point3::from(self.state.read().the_view_reference_point);
        temp_res
    }
    /// returns the Center Of Projection (model space)
    pub fn center_of_projection(&self) -> Point3 {
        let mut temp_res: Point3 = Point3::from(self.state.read().the_center_of_projection);
        temp_res
    }
    /// returns the View up vector (model space)
    pub fn view_up_vector(&self) -> Vector3 {
        let mut temp_res: Vector3 = Vector3::from(self.state.read().the_view_up_vector);
        temp_res
    }
    /// returns the top left point of the clipping window
    pub fn top_left(&self) -> Point2 {
        let mut temp_res: Point2 = Point2::from(self.state.read().the_top_left);
        temp_res
    }
    /// returns the bottom right point of the clipping window
    pub fn bottom_right(&self) -> Point2 {
        let mut temp_res: Point2 = Point2::from(self.state.read().the_bottom_right);
        temp_res
    }
    /// returns the Transformation Matrix
    pub fn view_matrix(&self) -> Handle<dyn IgesDataTransfEntity> {
        self.transf()
    }
    /// returns XYX from the Model space to the View space by
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
