//! IgesDrawDrawingWithRotation.
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
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, IgesDataViewKindEntity, iges_array1_item, iges_live};
use crate::iges_draw::{IgesDrawPerspectiveView, IgesDrawView};
use crate::iges_entity;
use crate::iges_graph::{IgesGraphDrawingSize, IgesGraphDrawingUnits};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeType, RuntimeTyped, exceptions};
use martensite_tkmath::gp::{Point2, Xy, Xyz};

iges_entity! {
/// defines IGESDrawingWithRotation, Type `404` Form `1`
///
/// Permits rotation, in addition to transformation and
/// scaling, between the view and drawing coordinate systems
IgesDrawDrawingWithRotation, IgesDrawDrawingWithRotationState, "IgesDrawDrawingWithRotation", parent IgesDataIgesEntity, {
the_views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
the_view_origins: Handle<HArray1<Xy>>,
the_orientation_angles: Handle<HArray1<f64>>,
the_annotations: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesDrawDrawingWithRotation {
    /// This method is used to set the fields of the class
    /// DrawingWithRotation
    /// - allViews             : Pointers to View entities
    /// - allViewOrigins       : Origin coords of transformed views
    /// - allOrientationAngles : Orientation angles of transformed views
    /// - allAnnotations       : Pointers to Annotation entities
    ///   raises exception if Lengths of allViews, allViewOrigins and
    ///   allOrientationAngles are not same.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_views: &Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
        all_view_origins: &Handle<HArray1<Xy>>,
        all_orientation_angles: &Handle<HArray1<f64>>,
        all_annotations: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        let mut len: i32 = iges_array1_length(all_views, "IgesDrawDrawingWithRotation::init")?;
        if ((iges_array1_lower(all_views, "IgesDrawDrawingWithRotation::init")? != 1)
            || ((iges_array1_lower(all_view_origins, "IgesDrawDrawingWithRotation::init")? != 1)
                || (iges_array1_length(all_view_origins, "IgesDrawDrawingWithRotation::init")?
                    != len)))
            || ((iges_array1_lower(all_orientation_angles, "IgesDrawDrawingWithRotation::init")?
                != 1)
                || (iges_array1_length(
                    all_orientation_angles,
                    "IgesDrawDrawingWithRotation::init",
                )? != len))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDrawDrawingWithRotation : Init",
            ));
        }
        if !all_annotations.is_null() {
            if iges_array1_lower(all_annotations, "IgesDrawDrawingWithRotation::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawDrawingWithRotation : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_views = all_views.clone();
            state.the_view_origins = all_view_origins.clone();
            state.the_orientation_angles = all_orientation_angles.clone();
            state.the_annotations = all_annotations.clone();
        }
        self.init_type_and_form(404, 1);
        Ok(())
    }
    /// returns the number of view pointers in `me`
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_views(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_views,
            "IgesDrawDrawingWithRotation::nb_views",
        )
    }
    /// returns the View entity indicated by Index
    /// raises an exception if Index at most 0 or Index &gt; NbViews().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn view_item(&self, index: i32) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_views,
            index,
            "IgesDrawDrawingWithRotation::view_item",
        )
    }
    /// returns the Drawing space coordinates of the origin of the
    /// Transformed view indicated by Index
    /// raises an exception if Index at most 0 or Index &gt; NbViews().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn view_origin(&self, index: i32) -> Result<Point2, Failure> {
        Ok(Point2::from(iges_array1_item(
            &self.state.read().the_view_origins,
            index,
            "IgesDrawDrawingWithRotation::view_origin",
        )?))
    }
    /// returns the Orientation angle for the Transformed view
    /// indicated by Index
    /// raises an exception if Index at most 0 or Index &gt; NbViews().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn orientation_angle(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_orientation_angles,
            index,
            "IgesDrawDrawingWithRotation::orientation_angle",
        )
    }
    /// returns the number of Annotation entities in `me`
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_annotations(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_annotations.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_annotations,
                "IgesDrawDrawingWithRotation::nb_annotations",
            )?
        })
    }
    /// returns the Annotation entity in this Drawing, indicated by Index
    /// raises an exception if Index at most 0 or Index &gt; NbAnnotations().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn annotation(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_annotations,
            index,
            "IgesDrawDrawingWithRotation::annotation",
        )
    }
    /// `view_to_drawing`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn view_to_drawing(&self, num_view: i32, view_coords: Xyz) -> Result<Xy, Failure> {
        let mut this_origin: Xy = iges_array1_item(
            &self.state.read().the_view_origins,
            num_view,
            "IgesDrawDrawingWithRotation::view_to_drawing",
        )?;
        let mut x_origin: f64 = this_origin.x();
        let mut y_origin: f64 = this_origin.y();
        let mut the_scale_factor: f64 = 0.0;
        let mut temp_view: Handle<dyn IgesDataViewKindEntity> = iges_array1_item(
            &self.state.read().the_views,
            num_view,
            "IgesDrawDrawingWithRotation::view_to_drawing",
        )?;
        if temp_view.is_kind(<IgesDrawView>::static_type()) {
            let mut this_view: Handle<IgesDrawView> = temp_view.cast();
            the_scale_factor =
                iges_live(&this_view, "IgesDrawDrawingWithRotation::view_to_drawing")?
                    .scale_factor();
        } else {
            if temp_view.is_kind(<IgesDrawPerspectiveView>::static_type()) {
                let mut this_view: Handle<IgesDrawPerspectiveView> = temp_view.cast();
                the_scale_factor =
                    iges_live(&this_view, "IgesDrawDrawingWithRotation::view_to_drawing")?
                        .scale_factor();
            }
        }
        let mut xv: f64 = view_coords.x();
        let mut yv: f64 = view_coords.y();
        let mut theta: f64 = iges_array1_item(
            &self.state.read().the_orientation_angles,
            num_view,
            "IgesDrawDrawingWithRotation::view_to_drawing",
        )?;
        let mut xd: f64 = x_origin
            + (the_scale_factor
                * ((xv * (martensite_cmath::cos(theta))) - (yv * (martensite_cmath::sin(theta)))));
        let mut yd: f64 = y_origin
            + (the_scale_factor
                * ((xv * (martensite_cmath::sin(theta))) + (yv * (martensite_cmath::cos(theta)))));
        Ok(Xy::new(xd, yd))
    }
    /// Returns the Drawing Unit Value if it is specified (by a
    /// specific property entity)
    /// If not specified, returns False, and val as zero :
    /// unit to consider is then the model unit in GlobalSection
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn drawing_unit(&self, val: &mut f64) -> Result<bool, Failure> {
        *val = 0.0;
        let mut typunit: &'static RuntimeType = <IgesGraphDrawingUnits>::static_type();
        if self.nb_typed_properties(typunit) != 1 {
            return Ok(false);
        }
        let mut units: Handle<IgesGraphDrawingUnits> = self.typed_property(typunit, 0)?.cast();
        if units.is_null() {
            return Ok(false);
        }
        *val = iges_live(&units, "IgesDrawDrawingWithRotation::drawing_unit")?.unit_value();
        return Ok(true);
    }
    /// Returns the Drawing Size if it is specified (by a
    /// specific property entity)
    /// If not specified, returns False, and X,Y as zero :
    /// unit to consider is then the model unit in GlobalSection
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn drawing_size(&self, x: &mut f64, y: &mut f64) -> Result<bool, Failure> {
        *y = 0.0;
        *x = *y;
        let mut typsize: &'static RuntimeType = <IgesGraphDrawingSize>::static_type();
        if self.nb_typed_properties(typsize) != 1 {
            return Ok(false);
        }
        let mut size: Handle<IgesGraphDrawingSize> = self.typed_property(typsize, 0)?.cast();
        if size.is_null() {
            return Ok(false);
        }
        *x = iges_live(&size, "IgesDrawDrawingWithRotation::drawing_size")?.x_size();
        *y = iges_live(&size, "IgesDrawDrawingWithRotation::drawing_size")?.y_size();
        return Ok(true);
    }
}
