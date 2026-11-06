//! IgesDimenSection.
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
use martensite_tkmath::gp::{Point3, Xy, Xyz};

iges_entity! {
/// defines Section, Type `106` Form &lt;31-38&gt;
/// Contains information to display sectioned sides
IgesDimenSection, IgesDimenSectionState, "IgesDimenSection", parent IgesDataIgesEntity, {
the_datatype: i32,
the_z_displacement: f64,
the_data_points: Handle<HArray1<Xy>>,
}
}

impl IgesDimenSection {
    iges_methods! {
    get datatype = the_datatype: i32;
    get z_displacement = the_z_displacement: f64;
    }
    /// This method is used to set the fields of the class
    /// Section
    /// - dataType   : Interpretation Flag, always = 1
    /// - aDisp      : Common z displacement
    /// - dataPoints : Data points
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        data_type: i32,
        a_disp: f64,
        data_points: &Handle<HArray1<Xy>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(data_points, "IgesDimenSection::init")? != 1 {
            return Err(exceptions::dimension_mismatch("IgesDimenSection : Init"));
        }
        {
            let mut state = self.state.write();
            state.the_datatype = data_type;
            state.the_z_displacement = a_disp;
            state.the_data_points = data_points.clone();
        }
        self.init_type_and_form(106, self.form_number());
        Ok(())
    }
    /// Changes FormNumber (indicates the Type of the Hatches)
    /// Error if not in range \[31-38\]
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if (form < 31) || (form > 38) {
            return Err(exceptions::out_of_range("IgesDimenSection : SetFormNumber"));
        }
        self.init_type_and_form(106, form);
        Ok(())
    }
    /// returns number of Data Points
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_points(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_data_points,
            "IgesDimenSection::nb_points",
        )
    }
    /// returns Index'th data point
    /// raises exception if Index at most 0 or Index &gt; NbPoints()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn point(&self, index: i32) -> Result<Point3, Failure> {
        let mut temp_xy: Xy = iges_array1_item(
            &self.state.read().the_data_points,
            index,
            "IgesDimenSection::point",
        )?;
        let mut point: Point3 = Point3::new(
            temp_xy.x(),
            temp_xy.y(),
            self.state.read().the_z_displacement,
        );
        Ok(point)
    }
    /// returns Index'th data point after Transformation
    /// raises exception if Index at most 0 or Index &gt; NbPoints()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_point(&self, index: i32) -> Result<Point3, Failure> {
        let mut point2d: Xy = iges_array1_item(
            &self.state.read().the_data_points,
            index,
            "IgesDimenSection::transformed_point",
        )?;
        let mut point: Xyz = Xyz::new(
            point2d.x(),
            point2d.y(),
            self.state.read().the_z_displacement,
        );
        if self.has_transf() {
            self.location()?.transforms(&mut point);
        }
        Ok(Point3::from(point))
    }
}
