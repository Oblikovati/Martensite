//! IgesDrawDrawing.
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
/// defines IGESDrawing, Type `404` Form `0`
///
/// Specifies a drawing as a collection of annotation entities
/// defined in drawing space, and views which together
/// constitute a single representation of a part
IgesDrawDrawing, IgesDrawDrawingState, "IgesDrawDrawing", parent IgesDataIgesEntity, {
the_views: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
the_view_origins: Handle<HArray1<Xy>>,
the_annotations: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesDrawDrawing {
    /// This method is used to set the fields of the class
    /// Drawing
    /// - allViews       : Pointers to DEs of View entities
    /// - allViewOrigins : Origin coordinates of transformed Views
    /// - allAnnotations : Pointers to DEs of Annotation entities
    ///   raises exception if Lengths of allViews and allViewOrigins are
    ///   not same.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_views: &Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>>,
        all_view_origins: &Handle<HArray1<Xy>>,
        all_annotations: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_views.is_null() {
            let mut len: i32 = iges_array1_length(all_views, "IgesDrawDrawing::init")?;
            let mut flag: bool =
                iges_array1_length(all_view_origins, "IgesDrawDrawing::init")? == len;
            if ((!flag) || (iges_array1_lower(all_views, "IgesDrawDrawing::init")? != 1))
                || (iges_array1_lower(all_view_origins, "IgesDrawDrawing::init")? != 1)
            {
                return Err(exceptions::dimension_mismatch("IgesDrawDrawing : Init"));
            }
        }
        if !all_annotations.is_null() {
            if iges_array1_lower(all_annotations, "IgesDrawDrawing::init")? != 1 {
                return Err(exceptions::dimension_mismatch("IgesDrawDrawing : Init"));
            }
        }
        {
            let mut state = self.state.write();
            state.the_views = all_views.clone();
            state.the_view_origins = all_view_origins.clone();
            state.the_annotations = all_annotations.clone();
        }
        self.init_type_and_form(404, 0);
        Ok(())
    }
    /// returns the number of view pointers in `me`
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_views(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_views.is_null() {
            0
        } else {
            iges_array1_length(&self.state.read().the_views, "IgesDrawDrawing::nb_views")?
        })
    }
    /// returns the ViewKindEntity indicated by ViewIndex
    /// raises an exception if ViewIndex at most 0 or ViewIndex &gt; NbViews().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn view_item(
        &self,
        view_index: i32,
    ) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_views,
            view_index,
            "IgesDrawDrawing::view_item",
        )
    }
    /// returns the Drawing space coordinates of the origin of the
    /// Transformed view indicated by TViewIndex
    /// raises an exception if TViewIndex at most 0 or TViewIndex &gt; NbViews().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn view_origin(&self, t_view_index: i32) -> Result<Point2, Failure> {
        Ok(Point2::from(iges_array1_item(
            &self.state.read().the_view_origins,
            t_view_index,
            "IgesDrawDrawing::view_origin",
        )?))
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
                "IgesDrawDrawing::nb_annotations",
            )?
        })
    }
    /// returns the Annotation entity in this Drawing, indicated by the
    /// AnnotationIndex
    /// raises an exception if AnnotationIndex at most 0 or
    /// AnnotationIndex &gt; NbAnnotations().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn annotation(&self, annotation_index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_annotations,
            annotation_index,
            "IgesDrawDrawing::annotation",
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
            "IgesDrawDrawing::view_to_drawing",
        )?;
        let mut x_origin: f64 = this_origin.x();
        let mut y_origin: f64 = this_origin.y();
        let mut the_scale_factor: f64 = 0.0;
        let mut temp_view: Handle<dyn IgesDataViewKindEntity> = iges_array1_item(
            &self.state.read().the_views,
            num_view,
            "IgesDrawDrawing::view_to_drawing",
        )?;
        if temp_view.is_kind(<IgesDrawView>::static_type()) {
            let mut this_view: Handle<IgesDrawView> = temp_view.cast();
            the_scale_factor =
                iges_live(&this_view, "IgesDrawDrawing::view_to_drawing")?.scale_factor();
        } else {
            if temp_view.is_kind(<IgesDrawPerspectiveView>::static_type()) {
                let mut this_view: Handle<IgesDrawPerspectiveView> = temp_view.cast();
                the_scale_factor =
                    iges_live(&this_view, "IgesDrawDrawing::view_to_drawing")?.scale_factor();
            }
        }
        let mut xv: f64 = view_coords.x();
        let mut yv: f64 = view_coords.y();
        let mut xd: f64 = x_origin + (the_scale_factor * xv);
        let mut yd: f64 = y_origin + (the_scale_factor * yv);
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
        *val = iges_live(&units, "IgesDrawDrawing::drawing_unit")?.unit_value();
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
        *x = iges_live(&size, "IgesDrawDrawing::drawing_size")?.x_size();
        *y = iges_live(&size, "IgesDrawDrawing::drawing_size")?.y_size();
        return Ok(true);
    }
}
