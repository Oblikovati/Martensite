//! IgesDrawRectArraySubfigure.
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
/// Defines IGES Rectangular Array Subfigure Instance Entity,
/// Used to produce copies of object called the base entity,
/// arranging them in equally spaced rows and columns
IgesDrawRectArraySubfigure, IgesDrawRectArraySubfigureState, "IgesDrawRectArraySubfigure", parent IgesDataIgesEntity, {
the_base_entity: Handle<IgesDataIgesEntity>,
the_scale_factor: f64,
the_lower_left_corner: Xyz,
the_nb_columns: i32,
the_nb_rows: i32,
the_column_separation: f64,
the_row_separation: f64,
the_rotation_angle: f64,
the_do_dont_flag: bool,
the_positions: Handle<HArray1<i32>>,
}
}

impl IgesDrawRectArraySubfigure {
    iges_methods! {
    get base_entity = the_base_entity: Handle<IgesDataIgesEntity>;
    get scale_factor = the_scale_factor: f64;
    get nb_columns = the_nb_columns: i32;
    get nb_rows = the_nb_rows: i32;
    get column_separation = the_column_separation: f64;
    get row_separation = the_row_separation: f64;
    get rotation_angle = the_rotation_angle: f64;
    get do_dont_flag = the_do_dont_flag: bool;
    }
    /// This method is used to set the fields of the class
    /// RectArraySubfigure
    /// - aBase         : a base entity which is replicated
    /// - aScale        : Scale Factor
    /// - aCorner       : lower left hand corner for the entire array
    /// - nbCols        : Number of columns of the array
    /// - nbRows        : Number of rows of the array
    /// - hDisp         : Column separations
    /// - vtDisp        : Row separation
    /// - rotationAngle : Rotation angle specified in radians
    /// - allDont       : DO-DON'T flag to control which portion
    ///   to display
    /// - allNumPos     : List of positions to be or not to be
    ///   displayed
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_base: &Handle<IgesDataIgesEntity>,
        a_scale: f64,
        a_corner: Xyz,
        nb_cols: i32,
        nb_rows: i32,
        h_disp: f64,
        vt_disp: f64,
        rotation_angle: f64,
        do_dont: i32,
        all_num_pos: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        if !all_num_pos.is_null() {
            if iges_array1_lower(all_num_pos, "IgesDrawRectArraySubfigure::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDrawRectArraySubfigure : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_base_entity = a_base.clone();
            state.the_scale_factor = a_scale;
            state.the_lower_left_corner = a_corner;
            state.the_nb_columns = nb_cols;
            state.the_nb_rows = nb_rows;
            state.the_column_separation = h_disp;
            state.the_row_separation = vt_disp;
            state.the_rotation_angle = rotation_angle;
            state.the_do_dont_flag = do_dont != 0;
            state.the_positions = all_num_pos.clone();
        }
        self.init_type_and_form(412, 0);
        Ok(())
    }
    /// returns coordinates of lower left hand corner for the entire array
    pub fn lower_left_corner(&self) -> Point3 {
        let mut temp_lower_left_corner: Point3 =
            Point3::from(self.state.read().the_lower_left_corner);
        temp_lower_left_corner
    }
    /// returns Transformed coordinates of lower left corner for the array
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_lower_left_corner(&self) -> Result<Point3, Failure> {
        let mut temp_lower_left_corner: Xyz = self.state.read().the_lower_left_corner;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_lower_left_corner);
        }
        let mut temp_res: Point3 = Point3::from(temp_lower_left_corner);
        Ok(temp_res)
    }
    /// returns True if (ListCount = 0) i.e., all elements to be displayed
    pub fn display_flag(&self) -> bool {
        self.state.read().the_positions.is_null()
    }
    /// returns 0 if all replicated entities to be displayed
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn list_count(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_positions.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_positions,
                "IgesDrawRectArraySubfigure::list_count",
            )?
        })
    }
    /// returns whether Index is to be processed (DO)
    /// or not to be processed(DON'T)
    /// if (ListCount = 0) return theDoDontFlag
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
            "IgesDrawRectArraySubfigure::position_num",
        )?;
        for i in 1..=up {
            if (iges_array1_item(
                &self.state.read().the_positions,
                i,
                "IgesDrawRectArraySubfigure::position_num",
            )?) == index
            {
                return Ok(self.state.read().the_do_dont_flag);
            }
        }
        return Ok(!(self.state.read().the_do_dont_flag));
    }
    /// returns the Index'th value position
    /// raises exception if Index at most 0 or Index &gt; ListCount()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn list_position(&self, index: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_positions,
            index,
            "IgesDrawRectArraySubfigure::list_position",
        )
    }
}
