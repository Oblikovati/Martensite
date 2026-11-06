//! IgesDrawCircArraySubfigure.
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
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower, iges_array1_upper};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// Defines IGES Circular Array Subfigure Instance Entity,
///
/// Used to produce copies of object called the base entity,
/// arranging them around the edge of an imaginary circle
/// whose center and radius are specified
IgesDrawCircArraySubfigure, IgesDrawCircArraySubfigureState, "IgesDrawCircArraySubfigure", parent IgesDataIgesEntity, {
the_base_entity: Handle<IgesDataIgesEntity>,
the_nb_locations: i32,
the_center: Xyz,
the_radius: f64,
the_start_angle: f64,
the_delta_angle: f64,
the_do_dont_flag: bool,
the_positions: Handle<HArray1<i32>>,
}
}

impl IgesDrawCircArraySubfigure {
    iges_methods! {
    get base_entity = the_base_entity: Handle<IgesDataIgesEntity>;
    get nb_locations = the_nb_locations: i32;
    get circle_radius = the_radius: f64;
    get start_angle = the_start_angle: f64;
    get delta_angle = the_delta_angle: f64;
    get do_dont_flag = the_do_dont_flag: bool;
    }
    /// This method is used to set the fields of the class
    /// CircArraySubfigure
    /// - aBase     : Base entity
    /// - aNumLocs  : Total number of possible instance locations
    /// - aCenter   : Coordinates of Center of imaginary circle
    /// - aRadius   : Radius of imaginary circle
    /// - aStAngle  : Start angle in radians
    /// - aDelAngle : Delta angle in radians
    /// - aFlag     : DO-DON'T flag to control which portion to
    ///   display
    /// - allNumPos : All position to be or not to be processed
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_base: &Handle<IgesDataIgesEntity>,
        a_num_locs: i32,
        a_center: Xyz,
        a_radius: f64,
        a_st_angle: f64,
        a_del_angle: f64,
        a_flag: i32,
        all_num_pos: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        if !all_num_pos.is_null() {
            if iges_array1_lower(all_num_pos, "IgesDrawCircArraySubfigure::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawCircArraySubfigure : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_base_entity = a_base.clone();
            state.the_nb_locations = a_num_locs;
            state.the_center = a_center;
            state.the_radius = a_radius;
            state.the_start_angle = a_st_angle;
            state.the_delta_angle = a_del_angle;
            state.the_do_dont_flag = a_flag != 0;
            state.the_positions = all_num_pos.clone();
        }
        self.init_type_and_form(414, 0);
        Ok(())
    }
    /// returns the center of the imaginary circle
    pub fn center_point(&self) -> Point3 {
        let mut temp_center_point: Point3 = Point3::from(self.state.read().the_center);
        temp_center_point
    }
    /// returns the Transformed center of the imaginary circle
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_center_point(&self) -> Result<Point3, Failure> {
        let mut temp_center_point: Xyz = self.state.read().the_center;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_center_point);
        }
        let mut temp_res: Point3 = Point3::from(temp_center_point);
        Ok(temp_res)
    }
    /// returns 0 if all elements to be displayed
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn list_count(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_positions.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_positions,
                "IgesDrawCircArraySubfigure::list_count",
            )?
        })
    }
    /// returns True if (ListCount = 0) all elements are to be displayed
    pub fn display_flag(&self) -> bool {
        self.state.read().the_positions.is_null()
    }
    /// returns whether Index is to be processed (DO)
    /// or not to be processed(DON'T)
    /// if (ListCount = 0) return theDoDontFlag
    /// raises exception if Index at most 0 or Index &gt; ListCount().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn position_num(&self, index: i32) -> Result<bool, Failure> {
        if self.state.read().the_positions.is_null() {
            return Ok(self.state.read().the_do_dont_flag);
        }

        let mut i: i32 = Default::default();
        let mut up: i32 = iges_array1_upper(
            &self.state.read().the_positions,
            "IgesDrawCircArraySubfigure::position_num",
        )?;
        for i in 1..=up {
            if (iges_array1_item(
                &self.state.read().the_positions,
                i,
                "IgesDrawCircArraySubfigure::position_num",
            )?) == index
            {
                return Ok(self.state.read().the_do_dont_flag);
            }
        }
        return Ok(!(self.state.read().the_do_dont_flag));
    }
    /// returns the Index'th value position
    /// raises exception if Index at most 0 or Index &gt; ListCount().
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn list_position(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_positions,
            index,
            "IgesDrawCircArraySubfigure::list_position",
        )
    }
}
