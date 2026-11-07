//! IgesDimenCenterLine.
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
/// defines CenterLine, Type `106` Form &lt;20-21&gt;
/// Is an entity appearing as crosshairs or as a
/// construction between 2 positions
IgesDimenCenterLine, IgesDimenCenterLineState, "IgesDimenCenterLine", parent IgesDataIgesEntity, {
the_datatype: i32,
the_z_displacement: f64,
the_data_points: Handle<HArray1<Xy>>,
}
}

impl IgesDimenCenterLine {
    iges_methods! {
    get datatype = the_datatype: i32;
    get z_displacement = the_z_displacement: f64;
    }
    /// This method is used to set the fields of the class
    /// CenterLine
    /// - aDataType      : Interpretation Flag, always = 1
    /// - aZDisplacement : Common z displacement
    /// - dataPnts       : Data points (x and y)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_data_type: i32,
        a_zdisp: f64,
        data_pnts: &Handle<HArray1<Xy>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(data_pnts, "IgesDimenCenterLine::init")? != 1 {
            return Err(exceptions::dimension_mismatch("IgesDimenCenterLine : Init"));
        }
        {
            let mut state = self.state.write();
            state.the_datatype = a_data_type;
            state.the_z_displacement = a_zdisp;
            state.the_data_points = data_pnts.clone();
        }
        self.init_type_and_form(106, self.form_number());
        Ok(())
    }
    /// Sets FormNumber to 20 if `mode` is True, 21 else
    pub fn set_cross_hair(&self, mode: bool) {
        self.init_type_and_form(106, if mode { 20 } else { 21 });
    }
    /// returns Number of Data Points.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_points(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_data_points,
            "IgesDimenCenterLine::nb_points",
        )
    }
    /// returns the data point as Pnt from gp.
    /// raises exception if Index at most 0 or Index &gt; NbPoints()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn point(&self, index: i32) -> Result<Point3, Failure> {
        let mut temp_xy: Xy = iges_array1_item(
            &self.state.read().the_data_points,
            index,
            "IgesDimenCenterLine::point",
        )?;
        let mut point: Point3 = Point3::new(
            temp_xy.x(),
            temp_xy.y(),
            self.state.read().the_z_displacement,
        );
        Ok(point)
    }
    /// returns the data point as Pnt from gp after Transformation.
    /// raises exception if Index at most 0 or Index &gt; NbPoints()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_point(&self, index: i32) -> Result<Point3, Failure> {
        let mut point2d: Xy = iges_array1_item(
            &self.state.read().the_data_points,
            index,
            "IgesDimenCenterLine::transformed_point",
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
    /// returns True if Form is 20.
    pub fn is_cross_hair(&self) -> bool {
        self.form_number() == 20
    }
}
