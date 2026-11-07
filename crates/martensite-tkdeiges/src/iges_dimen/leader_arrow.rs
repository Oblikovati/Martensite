//! IgesDimenLeaderArrow.
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
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{Point2, Point3, Xy, Xyz};

iges_entity! {
/// defines LeaderArrow, Type `214` Form &lt;1-12&gt;
/// Consists of one or more line segments except when
/// leader is part of an angular dimension, with links to
/// presumed text item
IgesDimenLeaderArrow, IgesDimenLeaderArrowState, "IgesDimenLeaderArrow", parent IgesDataIgesEntity, {
the_arrow_head_height: f64,
the_arrow_head_width: f64,
the_z_depth: f64,
the_arrow_head: Xy,
the_segment_tails: Handle<HArray1<Xy>>,
}
}

impl IgesDimenLeaderArrow {
    iges_methods! {
    get arrow_head_height = the_arrow_head_height: f64;
    get arrow_head_width = the_arrow_head_width: f64;
    get z_depth = the_z_depth: f64;
    }
    /// This method is used to set the fields of the class
    /// LeaderArrow
    /// - height      : ArrowHead height
    /// - width       : ArrowHead width
    /// - depth       : Z Depth
    /// - position    : ArrowHead coordinates
    /// - segments    : Segment tail coordinate pairs
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        height: f64,
        width: f64,
        depth: f64,
        position: Xy,
        segments: &Handle<HArray1<Xy>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(segments, "IgesDimenLeaderArrow::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesDimenLeaderArrow : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_arrow_head_height = height;
            state.the_arrow_head_width = width;
            state.the_z_depth = depth;
            state.the_arrow_head = position;
            state.the_segment_tails = segments.clone();
        }
        self.init_type_and_form(214, self.form_number());
        Ok(())
    }
    /// Changes FormNumber (indicates the Shape of the Arrow)
    /// Error if not in range \[0-12\]
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if (form < 1) || (form > 12) {
            return Err(exceptions::out_of_range(
                "IgesDimenLeaderArrow : SetFormNumber",
            ));
        }
        self.init_type_and_form(214, form);
        Ok(())
    }
    /// returns number of segments
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_segments(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_segment_tails,
            "IgesDimenLeaderArrow::nb_segments",
        )
    }
    /// returns ArrowHead coordinates
    pub fn arrow_head(&self) -> Point2 {
        let mut ah_pnt2d: Point2 = Point2::from(self.state.read().the_arrow_head);
        ah_pnt2d
    }
    /// returns ArrowHead coordinates after Transformation
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_arrow_head(&self) -> Result<Point3, Failure> {
        let mut point: Xyz = Xyz::new(
            self.state.read().the_arrow_head.x(),
            self.state.read().the_arrow_head.y(),
            self.z_depth(),
        );
        if self.has_transf() {
            self.location()?.transforms(&mut point);
        }
        Ok(Point3::from(point))
    }
    /// returns segment tail coordinates.
    /// raises exception if Index at most 0 or Index &gt; NbSegments
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn segment_tail(&self, index: i32) -> Result<Point2, Failure> {
        let mut st_pnt2d: Point2 = Point2::from(iges_array1_item(
            &self.state.read().the_segment_tails,
            index,
            "IgesDimenLeaderArrow::segment_tail",
        )?);
        Ok(st_pnt2d)
    }
    /// returns segment tail coordinates after Transformation.
    /// raises exception if Index at most 0 or Index &gt; NbSegments
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_segment_tail(&self, index: i32) -> Result<Point3, Failure> {
        let mut point2d: Xy = iges_array1_item(
            &self.state.read().the_segment_tails,
            index,
            "IgesDimenLeaderArrow::transformed_segment_tail",
        )?;
        let mut point: Xyz = Xyz::new(point2d.x(), point2d.y(), self.z_depth());
        if self.has_transf() {
            self.location()?.transforms(&mut point);
        }
        Ok(Point3::from(point))
    }
}
